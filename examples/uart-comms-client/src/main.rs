extern crate failure;

use serial;
use std::time::Duration;
use std::io::prelude::*;
use serial::prelude::*;
use std::thread;

use byteorder::{BigEndian, ByteOrder};
use pnet::packet::udp::{ipv4_checksum, UdpPacket};
use pnet::packet::Packet;
use std::net::{Ipv4Addr, UdpSocket};
use std::ops::Range;

const MAX_READ: usize = 48;
const TIMEOUT: Duration = Duration::from_millis(1000);

// UDP header length.
const HEADER_LEN: u16 = 8;
// Checksum location in UDP packet.
const CHKSUM_RNG: Range<usize> = 6..8;

const SOURCE_PORT: u16 = 1000;
const SOURCE_IP: &str = "192.168.8.40";
const DEST_PORT: u16 = 8006; // Telemetry service
const DEST_IP: &str = "0.0.0.0";

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

fn main() {
    let settings = serial::PortSettings {
        baud_rate: serial::Baud115200,
        char_size: serial::Bits8,
        parity: serial::ParityNone,
        stop_bits: serial::Stop1,
        flow_control: serial::FlowNone,
    };
    
    let mut port = serial::open("/dev/ttyS1").unwrap();

    port.configure(&settings).unwrap();
    port.set_timeout(TIMEOUT).unwrap();
    
    let mut counter = 0;
    
    let sat = DEST_IP.parse().unwrap();
    let ground = SOURCE_IP.parse().unwrap();

    loop {
        //let msg = format!("Test Message {:03}", counter);
        let msg = "{telemetry(limit: 1){timestamp,subsystem,parameter,value}}";
        counter += 1;
        
        //let packet = msg.as_bytes();
        let packet = build_packet(msg.as_bytes(), SOURCE_PORT, DEST_PORT, msg.len() as u16, sat, ground);

        println!("Writing {} bytes", packet.len());
        
        if let Some(error) = port.write(&packet).err() {
            eprintln!("Write failed: {:?}", error);
        }
        
        // Get response
        let mut packet = vec![];
        loop {
            let mut buffer: Vec<u8> = vec![0; MAX_READ];
            match port.read(buffer.as_mut_slice()) {
                Ok(num) => {
                    println!("Ground read fragment: {}", num);
                    buffer.resize(num, 0);
                    packet.append(&mut buffer);
    
                    if num < MAX_READ {
                        break;
                    }
                },
                Err(ref err) => match err.kind() {
                    ::std::io::ErrorKind::TimedOut => {
                        eprintln!("Timed out waiting for response");
                        panic!();
                    },
                    _ => panic!()
                }
            };
        }
        
        let msg = packet.clone();
        
        // Create a UDP packet from the received information.
        let packet = match UdpPacket::owned(packet) {
            Some(packet) => packet,
            None => {
                panic!("Failed to parse UDP packet");
            }
        };
        
        eprintln!("re-constructed udp packet");

        // Verify checksum of the UDP Packet.
        let calc = ipv4_checksum(&packet, &sat, &ground);
        println!("Ground read: Source: {}, Dest: {}, Checksum-real: {}, Checksum-calc: {}",
                &sat, &ground, packet.get_checksum(), calc
        );
        if packet.get_checksum() != calc {
            // The most likely reason that this would fail is if the source and dest IP addresses
            // don't match what we're expecting
            panic!("Checksum mismatch");
        }
        
        let msg = ::std::str::from_utf8(&msg[8..]).unwrap();
        
        println!("Response: {}", msg);
        
        thread::sleep(Duration::from_secs(2));
    }
}