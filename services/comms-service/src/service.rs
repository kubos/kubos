//
// Copyright (C) 2018 Kubos Corporation
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
// Contributed by: William Greer (wgreer184@gmail.com) and Sam Justice (sam.justice1@gmail.com)
//

use byteorder::{BigEndian, ByteOrder};
use errors::*;
use pnet::packet::udp::{ipv4_checksum, UdpPacket};
use pnet::packet::Packet;
use std::net::{Ipv4Addr, UdpSocket};
use std::ops::Range;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use telemetry::*;

// UDP header length.
const HEADER_LEN: usize = 8;
// Checksum location in UDP packet.
const CHKSUM_RNG: Range<usize> = 6..8;
// Communication service maximum packet size (65,535 - 20 byte IP header - 8 byte UDP header
//  - 8 byte UDP header).
const MAX_SIZE: usize = 65499;

/// Type definition for a "read" function pointer.
pub type ReadFn<T> = Fn(T) -> CommsResult<Vec<u8>> + Send + Sync + 'static;
/// Type definition for a "write" function pointer.
pub type WriteFn<T> = Fn(T, &[u8]) -> CommsResult<()> + Send + Sync + 'static;

/// Struct that holds configuration data to allow users to set up a Communication Service.
#[derive(Clone)]
pub struct CommsControlBlock<T: Clone> {
    /// Function pointer to a function that defines how to read from a gateway.
    pub read: Option<Arc<ReadFn<T>>>,
    /// Optional function pointers to functions that define methods for writing data over a gateway.
    pub write: Vec<Arc<WriteFn<T>>>,
    /// Gateway connection to read from.
    pub read_conn: T,
    /// Gateway connection to write to.
    pub write_conn: T,
    /// Starting port used to define a range of ports that are used in the message handlers
    /// that handle messages received from the ground.
    pub handler_port_min: u16,
    /// Ending port used to define a range of ports that are used in the message handlers
    /// that handle messages received from the ground.
    pub handler_port_max: u16,
    /// Timeout for the completion of GraphQL operations within message handlers (in milliseconds).
    pub timeout: u64,
    /// IP address of the ground gateway.
    pub ground_ip: Ipv4Addr,
    /// Satellite's IP address.
    pub satellite_ip: Ipv4Addr,
    /// Optional list of ports used by downlink endpoints that send messages to the ground.
    /// Each port in the list will be used by one downlink endpoint.
    pub downlink_ports: Option<Vec<u16>>,
    /// Specifies the port to which the ground gateway is bound.
    pub ground_port: Option<u16>,
}

/// Struct that enables users to start the Communication Service.
pub struct CommsService;

impl CommsService {
    /// Starts an instance of the Communication Service and its associated background threads.
    pub fn start<T: Clone + Send + 'static>(
        control: CommsControlBlock<T>,
        telem: Arc<Mutex<CommsTelemetry>>,
    ) -> CommsResult<()> {
        // Make sure a `read()` function and a `write()` function exist before starting the read thread.
        if control.read.is_some() {
            if control.write.len() > 0 {
                let telem_ref = telem.clone();
                let control_ref = control.clone();
                thread::spawn(move || read_thread(control_ref, telem_ref));
            } else {
                return Err(CommsServiceError::MissingWriteMethod.into());
            }
        }

        // For each provided `write()` function, spawn a downlink endpoint thread.
        if let Some(ports) = control.downlink_ports {
            if control.write.len() == ports.len() {
                for index in 0..control.write.len() {
                    let telem_ref = telem.clone();
                    let port_ref = ports[index];
                    let conn_ref = control.write_conn.clone();
                    let write_ref = control.write[index].clone();
                    let ground_ip = control.ground_ip.clone();
                    let sat_ip = control.satellite_ip.clone();
                    let ground_port = control.ground_port.unwrap().clone();
                    thread::spawn(move || {
                        downlink_endpoint(
                            telem_ref,
                            port_ref,
                            conn_ref,
                            write_ref,
                            ground_ip,
                            sat_ip,
                            ground_port,
                        );
                    });
                }
            } else {
                return Err(CommsServiceError::ParameterLengthMismatch.into());
            }
        }

        Ok(())
    }
}

// This thread reads from a gateway and passes received messages to message handlers.
fn read_thread<T: Clone + Send + 'static>(
    comms: CommsControlBlock<T>,
    data: Arc<Mutex<CommsTelemetry>>,
) {
    // Setup port rotation for message handlers.
    let mut port = comms.handler_port_min;

    // Take reader from control block.
    let read = comms.read.unwrap();

    loop {
        // Read bytes from the radio.
        let bytes = match (read)(comms.read_conn.clone()) {
            Ok(bytes) => bytes,
            Err(e) => {
                log_error(&data, e.to_string()).unwrap();
                continue;
            }
        };
        let byte_length = bytes.len();

        // Create a UDP packet from the received information.
        let packet = match UdpPacket::owned(bytes) {
            Some(packet) => packet,
            None => {
                log_error(&data, CommsServiceError::HeaderParsing.to_string()).unwrap();
                continue;
            }
        };

        // Verify the length of the UDP Packet.
        if byte_length != packet.get_length() as usize {
            log_error(&data, CommsServiceError::InvalidPacketLength.to_string()).unwrap();
            continue;
        }

        // Verify checksum of the UDP Packet.
        if packet.get_checksum() != ipv4_checksum(&packet, &comms.ground_ip, &comms.satellite_ip) {
            log_error(&data, CommsServiceError::InvalidChecksum.to_string()).unwrap();
            continue;
        }

        // Update number of packets up.
        log_telemetry(&data, TelemType::Up).unwrap();

        // Spawn new message handler.
        let conn_ref = comms.write_conn.clone();
        let write_ref = comms.write[0].clone();
        let data_ref = data.clone();
        let sat_ref = comms.satellite_ip.clone();
        let ground_ref = comms.ground_ip.clone();
        let time_ref = comms.timeout.clone();
        thread::spawn(move || {
            handle_message(
                data_ref, port, conn_ref, write_ref, packet, time_ref, sat_ref, ground_ref,
            );
        });

        // Increment port number.
        if port < comms.handler_port_max {
            port += 1;
        } else {
            port = comms.handler_port_min;
        }
    }
}

// This thread sends a query/mutation to its intended destination and waits for a response.
// The thread then writes the response to the gateway.
fn handle_message<T: Clone>(
    data: Arc<Mutex<CommsTelemetry>>,
    port: u16,
    write_conn: T,
    write: Arc<WriteFn<T>>,
    message: UdpPacket,
    timeout: u64,
    sat_ip: Ipv4Addr,
    ground_ip: Ipv4Addr,
) {
    let mut buf = [0; MAX_SIZE];

    // Bind the message handler to a port.
    let socket = match UdpSocket::bind((sat_ip, port)) {
        Ok(sock) => sock,
        Err(e) => return log_error(&data, e.to_string()).unwrap(),
    };

    // Set receive timeout for the socket.
    match socket.set_read_timeout(Some(Duration::from_millis(timeout))) {
        Ok(_) => (),
        Err(e) => return log_error(&data, e.to_string()).unwrap(),
    };

    // Send message to the intended service.
    match socket.send_to(message.payload(), (sat_ip, message.get_destination())) {
        Ok(_) => (),
        Err(e) => return log_error(&data, e.to_string()).unwrap(),
    };

    // Receive response back from the service.
    let (size, _) = match socket.recv_from(&mut buf) {
        Ok(tuple) => tuple,
        Err(e) => return log_error(&data, e.to_string()).unwrap(),
    };

    // Take received message and wrap it in a UDP packet.
    let packet = match build_packet(
        &buf[0..size],
        message.get_destination(),
        message.get_source(),
        (size + HEADER_LEN) as u16,
        sat_ip,
        ground_ip,
    ) {
        Ok(packet) => packet,
        Err(e) => return log_error(&data, e.to_string()).unwrap(),
    };

    // Write packet to the gateway and update telemetry.
    match write(write_conn.clone(), packet.as_slice()) {
        Ok(_) => log_telemetry(&data, TelemType::Down).unwrap(),
        Err(e) => log_error(&data, e.to_string()).unwrap(),
    };
}

// This thread reads indefinitely from a UDP socket and then writes received packets to a gateway.
fn downlink_endpoint<T: Clone>(
    data: Arc<Mutex<CommsTelemetry>>,
    port: u16,
    write_conn: T,
    write: Arc<WriteFn<T>>,
    ground_ip: Ipv4Addr,
    sat_ip: Ipv4Addr,
    ground_port: u16,
) {
    // Bind the downlink endpoint to a socket.
    let socket = match UdpSocket::bind((sat_ip, port)) {
        Ok(sock) => sock,
        Err(e) => return log_error(&data, e.to_string()).unwrap(),
    };

    loop {
        let mut buf = [0; MAX_SIZE];

        // Indefinitely wait for a message from any application or service.
        let (size, address) = match socket.recv_from(&mut buf) {
            Ok(tuple) => tuple,
            Err(e) => {
                log_error(&data, e.to_string()).unwrap();
                continue;
            }
        };

        // Take received message and wrap it in a UDP packet.
        let packet = match build_packet(
            &buf[0..size],
            address.port(),
            ground_port,
            (size + HEADER_LEN) as u16,
            sat_ip,
            ground_ip,
        ) {
            Ok(packet) => packet,
            Err(e) => {
                log_error(&data, e.to_string()).unwrap();
                continue;
            }
        };

        // Write packet to the gateway and update telemetry.
        match write(write_conn.clone(), packet.as_slice()) {
            Ok(_) => log_telemetry(&data, TelemType::Down).unwrap(),
            Err(e) => log_error(&data, e.to_string()).unwrap(),
        };
    }
}

// Takes the payload and then wraps it into a UDP packet.
fn build_packet(
    payload: &[u8],
    source: u16,
    dest: u16,
    length: u16,
    sat: Ipv4Addr,
    ground: Ipv4Addr,
) -> CommsResult<Vec<u8>> {
    // Create a new UDP packet.
    let mut header = [0; HEADER_LEN];
    let fields = [source, dest, length, 0];
    BigEndian::write_u16_into(&fields, &mut header);
    let mut packet = header.to_vec();
    packet.append(&mut payload.to_vec());

    // Calculate the checksum for the UDP packet.
    let packet_without_checksum = match UdpPacket::owned(packet.clone()) {
        Some(bytes) => bytes,
        None => return Err(CommsServiceError::HeaderParsing.into()),
    };
    let mut checksum = [0; 2];
    BigEndian::write_u16(
        &mut checksum,
        ipv4_checksum(&packet_without_checksum, &sat, &ground),
    );

    // Splice the checksum back into UDP packet.
    packet.splice(CHKSUM_RNG, checksum.iter().cloned());
    Ok(packet)
}
