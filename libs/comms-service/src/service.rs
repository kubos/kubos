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
use crate::packet::{LinkPacket, PayloadType};
use crate::telemetry::*;
use log::info;
use std::fmt::Debug;
use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;

/// Type definition for a "read" function pointer.
pub type ReadFn<Connection> = dyn Fn(&Connection) -> CommsResult<Vec<u8>> + Send + Sync + 'static;
/// Type definition for a "write" function pointer.
pub type WriteFn<Connection> =
    dyn Fn(&Connection, &[u8]) -> CommsResult<()> + Send + Sync + 'static;

/// Struct that holds configuration data to allow users to set up a Communication Service.
#[derive(Clone)]
pub struct CommsControlBlock<ReadConnection: Clone, WriteConnection: Clone> {
    /// Function pointer to a function that defines how to read from a gateway.
    pub read: Option<Arc<ReadFn<ReadConnection>>>,
    /// Function pointers to functions that define methods for writing data over a gateway.
    pub write: Vec<Arc<WriteFn<WriteConnection>>>,
    /// Gateway connection to read from.
    pub read_conn: ReadConnection,
    /// Gateway connection to write to.
    pub write_conn: WriteConnection,
    /// Maximum number of concurrent message handlers allowed.
    pub max_num_handlers: u16,
    /// Timeout for the completion of GraphQL operations within message handlers (in milliseconds).
    pub timeout: u64,
    /// IP address of the computer that is running the communication service.
    pub ip: Ipv4Addr,
    /// Optional list of ports used by downlink endpoints that send messages to the ground.
    /// Each port in the list will be used by one downlink endpoint.
    pub downlink_ports: Option<Vec<u16>>,
}

impl<ReadConnection: Clone + Debug, WriteConnection: Clone + Debug> Debug
    for CommsControlBlock<ReadConnection, WriteConnection>
{
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
            max_num_handlers: {:?}, timeout: {:?}, ip: {:?}, downlink_ports: {:?} }}",
            read,
            write,
            self.read_conn,
            self.write_conn,
            self.max_num_handlers,
            self.timeout,
            self.ip,
            self.downlink_ports,
        )
    }
}

impl<ReadConnection: Clone, WriteConnection: Clone>
    CommsControlBlock<ReadConnection, WriteConnection>
{
    /// Creates a new instance of the CommsControlBlock
    pub fn new(
        read: Option<Arc<ReadFn<ReadConnection>>>,
        write: Vec<Arc<WriteFn<WriteConnection>>>,
        read_conn: ReadConnection,
        write_conn: WriteConnection,
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
            ip: Ipv4Addr::from_str(&config.ip)?,
            downlink_ports: config.downlink_ports,
        })
    }
}

/// Struct that enables users to start the Communication Service.
pub struct CommsService;

impl CommsService {
    /// Starts an instance of the Communication Service and its associated background threads.
    pub fn start<
        ReadConnection: Clone + Send + 'static,
        WriteConnection: Clone + Send + 'static,
        Packet: LinkPacket + Send + 'static,
    >(
        control: CommsControlBlock<ReadConnection, WriteConnection>,
        telem: &Arc<Mutex<CommsTelemetry>>,
    ) -> CommsResult<()> {
        // If desired, spawn a read thread
        if control.read.is_some() {
            let telem_ref = telem.clone();
            let control_ref = control.clone();
            thread::spawn(move || {
                read_thread::<ReadConnection, WriteConnection, Packet>(control_ref, &telem_ref)
            });
        }

        // For each provided `write()` function, spawn a downlink endpoint thread.
        if let Some(ports) = control.downlink_ports {
            for (_, (port, write)) in ports.iter().zip(control.write.iter()).enumerate() {
                let telem_ref = telem.clone();
                let port_ref = *port;
                let conn_ref = control.write_conn.clone();
                let write_ref = write.clone();
                let ip = control.ip;
                thread::spawn(move || {
                    downlink_endpoint::<ReadConnection, WriteConnection, Packet>(
                        &telem_ref, port_ref, conn_ref, &write_ref, ip,
                    );
                });
            }
        }

        info!("Communication service started");
        Ok(())
    }
}

// This thread reads from a gateway and passes received messages to message handlers.
fn read_thread<
    ReadConnection: Clone + Send + 'static,
    WriteConnection: Clone + Send + 'static,
    Packet: LinkPacket + Send + 'static,
>(
    comms: CommsControlBlock<ReadConnection, WriteConnection>,
    data: &Arc<Mutex<CommsTelemetry>>,
) {
    // Take reader from control block.
    let read = comms.read.unwrap();

    // Initiate counter for handlers
    #[cfg(features = "graphql")]
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
        let packet = match Packet::parse(&bytes) {
            Ok(packet) => packet,
            Err(e) => {
                log_telemetry(&data, &TelemType::UpFailed).unwrap();
                log_error(&data, CommsServiceError::HeaderParsing.to_string()).unwrap();
                error!("Failed to parse packet header {}", e);
                continue;
            }
        };

        // Validate the link packet
        if !packet.validate() {
            log_telemetry(&data, &TelemType::UpFailed).unwrap();
            log_error(&data, CommsServiceError::InvalidChecksum.to_string()).unwrap();
            error!("Packet checksum failed");
            continue;
        }

        // Update number of packets up.
        log_telemetry(&data, &TelemType::Up).unwrap();
        info!("Packet successfully uplinked");

        // Check link type for appropriate message handling path
        match packet.payload_type() {
            PayloadType::Unknown(value) => {
                log_error(
                    &data,
                    CommsServiceError::UnknownPayloadType(value).to_string(),
                )
                .unwrap();
                error!("Unknown payload type encountered: {}", value);
            }
            PayloadType::UDP => {
                let sat_ref = comms.ip;
                let data_ref = data.clone();

                thread::spawn(move || match handle_udp_passthrough(packet, sat_ref) {
                    Ok(_) => {
                        log_telemetry(&data_ref, &TelemType::Down).unwrap();
                        info!("UDP Packet successfully uplinked");
                    }
                    Err(e) => {
                        log_telemetry(&data_ref, &TelemType::DownFailed).unwrap();
                        log_error(&data_ref, e.to_string()).unwrap();
                        error!("UDP packet failed to uplink: {}", e.to_string());
                    }
                });
            }
            #[cfg(features = "graphql")]
            PayloadType::GraphQL => {
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
                let sat_ref = comms.ip;
                let time_ref = comms.timeout;
                let num_handlers_ref = num_handlers.clone();
                thread::spawn(move || {
                    let res =
                        handle_graphql_request(conn_ref, &write_ref, packet, time_ref, sat_ref);

                    if let Ok(mut num_handlers) = num_handlers_ref.lock() {
                        *num_handlers -= 1;
                    }

                    match res {
                        Ok(_) => {
                            log_telemetry(&data_ref, &TelemType::Down).unwrap();
                            info!("GraphQL Packet successfully downlinked");
                        }
                        Err(e) => {
                            log_telemetry(&data_ref, &TelemType::DownFailed).unwrap();
                            log_error(&data_ref, e.to_string()).unwrap();
                            error!("GraphQL packet failed to downlink: {}", e.to_string());
                        }
                    }
                });
            }
        }
    }
}

// This thread sends a query/mutation to its intended destination and waits for a response.
// The thread then writes the response to the gateway.
#[cfg(features = "graphql")]
#[allow(clippy::boxed_local)]
fn handle_graphql_request<WriteConnection: Clone, Packet: LinkPacket>(
    write_conn: WriteConnection,
    write: &Arc<WriteFn<WriteConnection>>,
    message: Box<Packet>,
    timeout: u64,
    sat_ip: Ipv4Addr,
) -> Result<(), String> {
    use std::time::Duration;

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
    let packet = Packet::build(message.command_id(), PayloadType::GraphQL, 0, &buf[0..size])
        .and_then(|packet| packet.to_bytes())
        .map_err(|e| e.to_string())?;

    // Write packet to the gateway
    write(&write_conn.clone(), &packet).map_err(|e| e.to_string())
}

// This function takes a Packet with PayloadType::UDP and sends the payload over a
// UdpSocket to the specified destination.
#[allow(clippy::boxed_local)]
fn handle_udp_passthrough<Packet: LinkPacket>(
    message: Box<Packet>,
    sat_ip: Ipv4Addr,
) -> Result<(), String> {
    let socket = UdpSocket::bind((sat_ip, 0)).map_err(|e| e.to_string())?;

    socket
        .send_to(&message.payload(), (sat_ip, message.destination()))
        .map_err(|e| e.to_string())
        .map(|_c| ())
}

// This thread reads indefinitely from a UDP socket, creating link packets from
// the UDP packet payload and then writes the link packets to a gateway.
fn downlink_endpoint<ReadConnection: Clone, WriteConnection: Clone, Packet: LinkPacket>(
    data: &Arc<Mutex<CommsTelemetry>>,
    port: u16,
    write_conn: WriteConnection,
    write: &Arc<WriteFn<WriteConnection>>,
    sat_ip: Ipv4Addr,
) {
    // Bind the downlink endpoint to a UDP socket.
    let socket = match UdpSocket::bind((sat_ip, port)) {
        Ok(sock) => sock,
        Err(e) => return log_error(&data, e.to_string()).unwrap(),
    };

    loop {
        let mut buf = vec![0; Packet::max_size()];

        // Indefinitely wait for a message from any application or service.
        let (size, _address) = match socket.recv_from(&mut buf) {
            Ok(tuple) => tuple,
            Err(e) => {
                log_error(&data, e.to_string()).unwrap();
                continue;
            }
        };

        // Take received message and wrap it in a Link packet.
        // Setting port to 0 because we don't know the ground port...
        // That is known by the ground comms service
        let packet = match Packet::build(0, PayloadType::UDP, 0, &buf[0..size])
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
