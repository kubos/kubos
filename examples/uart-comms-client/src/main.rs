//
// Copyright (C) 2019 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Example UDP client "radio" over UART program
//
// Sends a GraphQL request wrapped in a UDP packet over the UART port
// The example communication service, `uart-comms-service`, should be running and listening for
// these messages.
// The service will forward the message on to the requested destination port and then return the
// response once the request has completed.

#[macro_use]
extern crate failure;

mod comms;

use byteorder::{BigEndian, ByteOrder};
use failure::Error;
use pnet::packet::udp::{ipv4_checksum, UdpPacket};
use std::net::Ipv4Addr;
use std::ops::Range;
use std::time::Duration;

const BUS: &str = "/dev/ttyS1";
// UDP connection information
// Note: This MUST MATCH what the communications service is expecting.
// It's used to verify the checsum of all incoming packets, so if the IP addresses are different,
// then the checksum is different and the message is rejected.
const SOURCE_PORT: u16 = 1000;
const SOURCE_IP: &str = "192.168.8.40";
const DEST_PORT: u16 = 8006; // Telemetry service port
const DEST_IP: &str = "0.0.0.0";

// UDP header length.
const HEADER_LEN: u16 = 8;
// Checksum location in UDP packet.
const CHKSUM_RNG: Range<usize> = 6..8;

// Return type for this service.
type ClientResult<T> = Result<T, Error>;

// Takes the payload and then wraps it into a UDP packet.
fn build_packet(
    payload: &[u8],
    source: u16,
    dest: u16,
    length: u16,
    sat: Ipv4Addr,
    ground: Ipv4Addr,
) -> Vec<u8> {
    // Create a new UDP packet.
    let mut header = [0; HEADER_LEN as usize];
    let fields = [source, dest, length + HEADER_LEN, 0];
    BigEndian::write_u16_into(&fields, &mut header);
    let mut packet = header.to_vec();
    packet.append(&mut payload.to_vec());

    // Calculate the checksum for the UDP packet.
    let packet_without_checksum = match UdpPacket::owned(packet.clone()) {
        Some(bytes) => bytes,
        None => panic!(),
    };
    let mut checksum = [0; 2];
    println!("Write source: {}, dest: {}", ground, sat);
    BigEndian::write_u16(
        &mut checksum,
        ipv4_checksum(&packet_without_checksum, &ground, &sat),
    );

    // Splice the checksum back into UDP packet.
    packet.splice(CHKSUM_RNG, checksum.iter().cloned());
    packet
}

fn main() -> ClientResult<()> {
    let mut conn = comms::serial_init(BUS)?;

    let sat = DEST_IP.parse()?;
    let ground = SOURCE_IP.parse()?;

    let query = "{telemetry(limit: 1){timestamp,subsystem,parameter,value}}";

    let packet = build_packet(
        query.as_bytes(),
        SOURCE_PORT,
        DEST_PORT,
        query.len() as u16,
        sat,
        ground,
    );

    println!("Writing {} bytes", packet.len());

    // Write our request out through the "radio"
    comms::write(&mut conn, &packet)?;

    // Get our response
    let msg = comms::read(&mut conn)?;

    // Parse a UDP packet from the received information.
    let packet = match UdpPacket::owned(msg.clone()) {
        Some(packet) => packet,
        None => {
            bail!("Failed to parse UDP packet");
        }
    };

    // Verify checksum of the UDP Packet.
    let calc = ipv4_checksum(&packet, &sat, &ground);
    if packet.get_checksum() != calc {
        bail!("Checksum mismatch");
    }

    let msg = ::std::str::from_utf8(&msg[8..])?;

    println!("Response: {}", msg);

    Ok(())
}
