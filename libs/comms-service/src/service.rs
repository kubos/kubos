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

use crate::config::*;
use crate::errors::*;
use crate::packet::{LinkPacket, LinkType};
use crate::spacepacket::SpacePacket;
use crate::telemetry::*;
use log::info;
use std::fmt::Debug;
use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Communication service maximum packet size (65,535 - 20 byte IP header - 8 byte UDP header
//  - 8 byte UDP header).
const MAX_SIZE: usize = 65499;

/// Type definition for a "read" function pointer.
pub type ReadFn<T> = Fn(&T) -> CommsResult<Vec<u8>> + Send + Sync + 'static;
/// Type definition for a "write" function pointer.
pub type WriteFn<T> = Fn(&T, &[u8]) -> CommsResult<()> + Send + Sync + 'static;

/// Struct that holds configuration data to allow users to set up a Communication Service.
#[derive(Clone)]
pub struct CommsControlBlock<T: Clone> {
    /// Function pointer to a function that defines how to read from a gateway.
    pub read: Option<Arc<ReadFn<T>>>,
    /// Function pointers to functions that define methods for writing data over a gateway.
    pub write: Vec<Arc<WriteFn<T>>>,
    /// Gateway connection to read from.
    pub read_conn: T,
    /// Gateway connection to write to.
    pub write_conn: T,
    /// Maximum number of concurrent message handlers allowed.
    pub max_num_handlers: u16,
    /// Timeout for the completion of GraphQL operations within message handlers (in milliseconds).
    pub timeout: u64,
    /// IP address of the computer that is running the communication service.
    pub satellite_ip: Ipv4Addr,
    /// Optional list of ports used by downlink endpoints that send messages to the ground.
    /// Each port in the list will be used by one downlink endpoint.
    pub downlink_ports: Option<Vec<u16>>,
    /// Specifies the port to which the ground gateway is bound.
    pub ground_port: Option<u16>,
}

impl<T: Clone + Debug> Debug for CommsControlBlock<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let read = if self.read.is_some() {
            "Some(fn)"
        } else {
            "None"
        };

        let mut write = vec![];

        if !self.write.is_empty() {
            for _n in 0..self.write.len() {
                write.push("Fn");
            }
        }

        write!(
            f,
            "CommsControlBlock {{ read: {}, write: {:?}, read_conn: {:?}, write_conn: {:?},
            max_num_handlers: {:?}, timeout: {:?}, satellite_ip: {:?},
            downlink_ports: {:?}, ground_port: {:?} }}",
            read,
            write,
            self.read_conn,
            self.write_conn,
            self.max_num_handlers,
            self.timeout,
            self.satellite_ip,
            self.downlink_ports,
            self.ground_port
        )
    }
}

impl<T: Clone> CommsControlBlock<T> {
    /// Creates a new instance of the CommsControlBlock
    pub fn new(
        read: Option<Arc<ReadFn<T>>>,
        write: Vec<Arc<WriteFn<T>>>,
        read_conn: T,
        write_conn: T,
        config: CommsConfig,
    ) -> CommsResult<Self> {
        if write.is_empty() {
            return Err(
                CommsServiceError::ConfigError("No `write` function provided".to_owned()).into(),
            );
        }

        if let Some(ports) = config.clone().downlink_ports {
            if write.len() != ports.len() {
                return Err(CommsServiceError::ConfigError(
                    "There must be a unique write function for each downlink port".to_owned(),
                )
                .into());
            }
        }

        Ok(CommsControlBlock {
            read,
            write,
            read_conn,
            write_conn,
            max_num_handlers: config.max_num_handlers.unwrap_or(DEFAULT_MAX_HANDLERS),
            timeout: config.timeout.unwrap_or(DEFAULT_TIMEOUT),
            satellite_ip: Ipv4Addr::from_str(&config.satellite_ip)?,
            downlink_ports: config.downlink_ports,
            ground_port: config.ground_port,
        })
    }
}

/// Struct that enables users to start the Communication Service.
pub struct CommsService;

impl CommsService {
    /// Starts an instance of the Communication Service and its associated background threads.
    pub fn start<T: Clone + Send + 'static>(
        control: CommsControlBlock<T>,
        telem: &Arc<Mutex<CommsTelemetry>>,
    ) -> CommsResult<()> {
        // If desired, spawn a read thread
        if control.read.is_some() {
            let telem_ref = telem.clone();
            let control_ref = control.clone();
            thread::spawn(move || read_thread(control_ref, &telem_ref));
        }

        // For each provided `write()` function, spawn a downlink endpoint thread.
        if let Some(ports) = control.downlink_ports {
            for (_, (port, write)) in ports.iter().zip(control.write.iter()).enumerate() {
                let telem_ref = telem.clone();
                let port_ref = *port;
                let conn_ref = control.write_conn.clone();
                let write_ref = write.clone();
                let sat_ip = control.satellite_ip;
                let ground_port = control.ground_port.ok_or_else(|| {
                    CommsServiceError::ConfigError("Missing ground_port parameter".to_owned())
                })?;
                thread::spawn(move || {
                    downlink_endpoint(
                        &telem_ref,
                        port_ref,
                        conn_ref,
                        &write_ref,
                        sat_ip,
                        ground_port,
                    );
                });
            }
        }

        info!("Communication service started");
        Ok(())
    }
}

// This thread reads from a gateway and passes received messages to message handlers.
fn read_thread<T: Clone + Send + 'static>(
    comms: CommsControlBlock<T>,
    data: &Arc<Mutex<CommsTelemetry>>,
) {
    // Take reader from control block.
    let read = comms.read.unwrap();

    // Initiate counter for handlers
    let num_handlers: Arc<Mutex<u16>> = Arc::new(Mutex::new(0));

    loop {
        // Read bytes from the radio.
        let bytes = match (read)(&comms.read_conn.clone()) {
            Ok(bytes) => bytes,
            Err(e) => {
                log_error(&data, e.to_string()).unwrap();
                continue;
            }
        };

        // Create a link packet from the received information.
        let packet = match SpacePacket::parse(&bytes) {
            Ok(packet) => packet,
            Err(e) => {
                log_telemetry(&data, &TelemType::UpFailed).unwrap();
                log_error(&data, CommsServiceError::HeaderParsing.to_string()).unwrap();
                error!("Failed to parse packet header {}", e);
                continue;
            }
        };

        // Validate the link packet
        if packet.validate() != true {
            log_telemetry(&data, &TelemType::UpFailed).unwrap();
            log_error(&data, CommsServiceError::InvalidChecksum.to_string()).unwrap();
            error!("Packet checksum failed");
            continue;
        }

        // Update number of packets up.
        log_telemetry(&data, &TelemType::Up).unwrap();
        info!("Packet successfully uplinked");

        // Check link type for appropriate message handling path
        match packet.link_type() {
            LinkType::UDP => {
                unimplemented!();
            }
            LinkType::GraphQL => {
                if let Ok(mut num_handlers) = num_handlers.lock() {
                    if *num_handlers >= comms.max_num_handlers {
                        log_error(&data, CommsServiceError::NoAvailablePorts.to_string()).unwrap();
                        error!("No message handler ports available");
                        continue;
                    } else {
                        *num_handlers += 1;
                    }
                }

                // Spawn new message handler.
                let conn_ref = comms.write_conn.clone();
                let write_ref = comms.write[0].clone();
                let data_ref = data.clone();
                let sat_ref = comms.satellite_ip;
                let time_ref = comms.timeout;
                let num_handlers_ref = num_handlers.clone();
                thread::spawn(move || {
                    let res =
                        handle_graphql_request(conn_ref, &write_ref, &packet, time_ref, sat_ref);

                    if let Ok(mut num_handlers) = num_handlers_ref.lock() {
                        *num_handlers -= 1;
                    }

                    match res {
                        Ok(_) => {
                            log_telemetry(&data_ref, &TelemType::Down).unwrap();
                            info!("UDP Packet successfully downlinked");
                        }
                        Err(e) => {
                            log_telemetry(&data_ref, &TelemType::DownFailed).unwrap();
                            log_error(&data_ref, e.to_string()).unwrap();
                            error!("UDP packet failed to downlink: {}", e.to_string());
                        }
                    }
                });
            }
        }
    }
}

// This thread sends a query/mutation to its intended destination and waits for a response.
// The thread then writes the response to the gateway.
#[allow(clippy::too_many_arguments)]
fn handle_graphql_request<T: Clone>(
    write_conn: T,
    write: &Arc<WriteFn<T>>,
    message: &SpacePacket,
    timeout: u64,
    sat_ip: Ipv4Addr,
) -> Result<(), String> {
    let payload = message.payload().to_vec();

    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(timeout))
        .build()
        .map_err(|e| e.to_string())?;

    let mut res = client
        .post(&format!("http://{}:{}", sat_ip, message.destination()))
        .body(payload)
        .send()
        .map_err(|e| e.to_string())?;

    let size = res.content_length().unwrap_or(0) as usize;
    let buf = res.text().unwrap_or_else(|_| "".to_owned());
    let buf = buf.as_bytes();

    // Take received message and wrap it in a LinkPacket
    let packet = SpacePacket::build(message.command_id(), LinkType::GraphQL, 0, &buf[0..size])
        .and_then(|packet| packet.to_bytes())
        .map_err(|e| e.to_string())?;

    // Write packet to the gateway
    write(&write_conn.clone(), &packet).map_err(|e| e.to_string())
}

// This thread reads indefinitely from a UDP socket, creating link packets from
// the UDP packet payload and then writes the link packets to a gateway.
fn downlink_endpoint<T: Clone>(
    data: &Arc<Mutex<CommsTelemetry>>,
    port: u16,
    write_conn: T,
    write: &Arc<WriteFn<T>>,
    sat_ip: Ipv4Addr,
    ground_port: u16,
) {
    // Bind the downlink endpoint to a UDP socket.
    let socket = match UdpSocket::bind((sat_ip, port)) {
        Ok(sock) => sock,
        Err(e) => return log_error(&data, e.to_string()).unwrap(),
    };

    loop {
        let mut buf = [0; MAX_SIZE];

        // Indefinitely wait for a message from any application or service.
        let (size, _address) = match socket.recv_from(&mut buf) {
            Ok(tuple) => tuple,
            Err(e) => {
                log_error(&data, e.to_string()).unwrap();
                continue;
            }
        };

        // Take received message and wrap it in a Link packet.
        let packet = match SpacePacket::build(0, LinkType::UDP, ground_port, &buf[0..size])
            .and_then(|packet| packet.to_bytes())
        {
            Ok(packet) => packet,
            Err(e) => {
                log_error(&data, e.to_string()).unwrap();
                continue;
            }
        };

        // Write packet to the gateway and update telemetry.
        match write(&write_conn.clone(), &packet) {
            Ok(_) => {
                log_telemetry(&data, &TelemType::Down).unwrap();
                info!("Packet successfully downlinked");
            }
            Err(e) => {
                log_telemetry(&data, &TelemType::DownFailed).unwrap();
                log_error(&data, e.to_string()).unwrap();
                error!("Packet failed to downlink");
            }
        };
    }
}
