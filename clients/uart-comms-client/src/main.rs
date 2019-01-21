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
// Wraps the user's data in a UDP packet and then sends it over the UART port
// The example communication service, `uart-comms-service`, should be running and listening for
// these messages.
// The service will forward the message on to the requested destination port and then return the
// response once the request has completed.
//
// Packets can be additionally encapsulated using the KISS protocol to simulate additional
// radio-specific framing. Resulting packet is `KISS<UDP<payload>>`.

mod comms;
mod kiss;

use byteorder::{BigEndian, ByteOrder};
use clap::{App, Arg};
use failure::{bail, Error};
use pnet::packet::udp::{ipv4_checksum, UdpPacket};
use std::fs::File;
use std::io::Read;
use std::net::Ipv4Addr;
use std::ops::Range;
use std::time::Duration;

// Default UDP connection information
// Note: This MUST MATCH what the communications service is expecting.
// It's used to verify the checksum of all incoming packets, so if the IP addresses are different,
// then the checksum is different and the message is rejected.
const SOURCE_PORT: u16 = 1000;
const SOURCE_IP: &str = "192.168.0.1";
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
    length: u16,
    source_ip: Ipv4Addr,
    source_port: u16,
    dest_ip: Ipv4Addr,
    dest_port: u16,
) -> Vec<u8> {
    // Create a new UDP packet.
    let mut header = [0; HEADER_LEN as usize];
    let fields = [source_port, dest_port, length + HEADER_LEN, 0];
    BigEndian::write_u16_into(&fields, &mut header);
    let mut packet = header.to_vec();
    packet.append(&mut payload.to_vec());

    // Calculate the checksum for the UDP packet.
    let packet_without_checksum = match UdpPacket::owned(packet.clone()) {
        Some(bytes) => bytes,
        None => panic!(),
    };
    let mut checksum = [0; 2];
    println!(
        "Source: {}:{}, Destination: {}:{}",
        source_ip, source_port, dest_ip, dest_port
    );
    BigEndian::write_u16(
        &mut checksum,
        ipv4_checksum(&packet_without_checksum, &source_ip, &dest_ip),
    );

    // Splice the checksum back into UDP packet.
    packet.splice(CHKSUM_RNG, checksum.iter().cloned());
    packet
}

fn main() -> ClientResult<()> {
    let args = App::new("UART Comms Client")
        .arg(
            Arg::with_name("bus")
                .help("Serial Device")
                .short("b")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("source_ip")
                .help("Source IP address")
                .short("s")
                .takes_value(true)
                .default_value(SOURCE_IP),
        )
        .arg(
            Arg::with_name("dest_ip")
                .help("Destination IP address")
                .short("d")
                .takes_value(true)
                .default_value(DEST_IP),
        )
        .arg(
            Arg::with_name("port")
                .help("Destination port")
                .short("p")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("file")
                .help("File containing data to send")
                .short("f")
                .takes_value(true)
                .conflicts_with("listen"),
        )
        .arg(
            Arg::with_name("data")
                .help("Data to send")
                .required_unless_one(&["file", "listen"])
                .conflicts_with("file")
                .conflicts_with("listen"),
        )
        .arg(
            Arg::with_name("kiss")
                .help("Enable KISS framing")
                .short("k"),
        )
        .get_matches();

    let bus = args.value_of("bus").unwrap();
    let source_ip = args.value_of("source_ip").unwrap().parse()?;
    let dest_ip = args.value_of("dest_ip").unwrap().parse()?;
    let dest_port = args.value_of("port").unwrap().parse()?;

    let query = if let Some(file) = args.value_of("file") {
        let mut raw = String::new();
        File::open(file).and_then(|mut f| f.read_to_string(&mut raw))?;
        raw
    } else {
        args.value_of("data").unwrap().to_string()
    };

    println!("Request: {}", query);

    // Wrap the request in a UDP packet
    let packet = build_packet(
        query.as_bytes(),
        query.len() as u16,
        source_ip,
        SOURCE_PORT,
        dest_ip,
        dest_port,
    );

    let packet = if args.is_present("kiss") {
        // Add KISS framing
        kiss::encode(&packet)
    } else {
        packet
    };

    let mut conn = comms::serial_init(bus)?;

    // Write our request out through the "radio"
    comms::write(&mut conn, &packet)?;

    // Get our response
    let msg = comms::read(&mut conn)?;

    let msg = if args.is_present("kiss") {
        // Parse the KISS frame
        let (frame, _, _) = kiss::decode(&msg)?;
        frame
    } else {
        msg
    };

    // Parse a UDP packet from the received information.
    let packet = match UdpPacket::owned(msg.clone()) {
        Some(packet) => packet,
        None => {
            bail!("Failed to parse UDP packet");
        }
    };

    // Verify checksum of the UDP Packet.
    let calc = ipv4_checksum(&packet, &dest_ip, &source_ip);
    if packet.get_checksum() != calc {
        eprintln!("Checksum mismatch");
    } else {
        let msg = ::std::str::from_utf8(&msg[8..])?;

        println!("Response: {}", msg);
    }

    Ok(())
}
