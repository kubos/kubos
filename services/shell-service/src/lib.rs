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

#![allow(clippy::block_in_if_condition_stmt)]

use channel_protocol::{ChannelMessage, ChannelProtocol};
use failure::bail;
use kubos_system::Config as ServiceConfig;
use log::{error, info, warn};
use shell_protocol::{ProcessHandler, ProtocolError, ShellMessage, ShellProtocol};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct ThreadProcess {
    pub sender: Sender<(ChannelMessage, SocketAddr)>,
    pub pid: u32,
    pub path: String,
}

// Create process list and send back to requester
fn list_processes(
    channel_id: u32,
    host: &str,
    remote: &str,
    threads: &Arc<Mutex<HashMap<u32, ThreadProcess>>>,
) -> Result<(), failure::Error> {
    let proc_list: HashMap<u32, (String, u32)> = threads
        .lock()
        .map_err(|err| {
            error!("Failed to get threads mutex: {:?}", err);
            err
        })
        .unwrap()
        .iter()
        .map(|(channel_id, data)| (*channel_id, (data.path.to_owned(), data.pid)))
        .collect();

    let chan_proto =
        channel_protocol::ChannelProtocol::new(host, remote, shell_protocol::CHUNK_SIZE);

    chan_proto.send(&shell_protocol::messages::list::to_cbor(
        channel_id,
        Some(proc_list),
    )?)?;

    Ok(())
}

// Spawn new process and spin up thread for handling it
fn spawn_process(
    channel_id: u32,
    command: &str,
    args: Option<Vec<String>>,
    host_addr: &str,
    remote_addr: &str,
    timeout: Duration,
    shared_threads: Arc<Mutex<HashMap<u32, ThreadProcess>>>,
) -> Result<(u32, Sender<(ChannelMessage, SocketAddr)>), failure::Error> {
    #[allow(clippy::type_complexity)]
    let (sender, receiver): (
        Sender<(ChannelMessage, SocketAddr)>,
        Receiver<(ChannelMessage, SocketAddr)>,
    ) = mpsc::channel();

    let proc_handle = match ProcessHandler::spawn(command, args) {
        Ok(p) => p,
        Err(e) => {
            bail!("Failed to spawn {:?}", e);
        }
    };
    let pid = proc_handle.id();

    let channel_protocol = ChannelProtocol::new(host_addr, remote_addr, shell_protocol::CHUNK_SIZE);

    channel_protocol.send(&shell_protocol::messages::pid::to_cbor(
        channel_id,
        proc_handle.id(),
    )?)?;

    thread::spawn(move || {
        thread_body(
            channel_protocol,
            channel_id,
            timeout,
            proc_handle,
            &shared_threads,
            &receiver,
        )
    });

    Ok((pid, sender))
}

// Main function of process handling thread
fn thread_body(
    channel_protocol: ChannelProtocol,
    channel_id: u32,
    timeout: Duration,
    proc_handle: ProcessHandler,
    shared_threads: &Arc<Mutex<HashMap<u32, ThreadProcess>>>,
    receiver: &Receiver<(ChannelMessage, SocketAddr)>,
) {
    let mut s_protocol = ShellProtocol::new(channel_protocol, channel_id, Box::new(proc_handle));

    // Receive and react to incoming shell protocol messages
    if let Err(e) = s_protocol.message_engine(
        |d| match receiver.recv_timeout(d) {
            Ok((v, s)) => Ok((v, s)),
            Err(RecvTimeoutError::Timeout) => Err(ProtocolError::ReceiveTimeout),
            Err(e) => Err(ProtocolError::ReceiveError {
                err: format!("Error {:?}", e),
            }),
        },
        timeout,
    ) {
        warn!("Encountered errors while processing transaction: {}", e);
    }

    // Remove ourselves from threads list once we are finished
    shared_threads
        .lock()
        .map_err(|err| {
            error!("Failed to get threads mutex: {:?}", err);
            err
        })
        .unwrap()
        .remove(&channel_id);
}

// Retrieves and parses next shell message
fn get_message(
    cbor_proto: &cbor_protocol::Protocol,
) -> Result<
    (
        channel_protocol::ChannelMessage,
        shell_protocol::ShellMessage,
        std::net::SocketAddr,
    ),
    failure::Error,
> {
    let (source, message) = cbor_proto.recv_message_peer()?;

    let channel_message = channel_protocol::parse_message(message)?;

    let shell_message = shell_protocol::parse_message(&channel_message.clone())?;

    Ok((channel_message, shell_message, source))
}

// Starts and runs the main loop receiving new shell protocol messages
pub fn recv_loop(config: &ServiceConfig) -> Result<(), failure::Error> {
    // Get and bind our UDP listening socket
    let host = config
        .hosturl()
        .ok_or_else(|| failure::format_err!("Unable to fetch addr for service"))?;

    // Extract our local IP address so we can spawn child sockets later
    let mut host_parts = host.split(':').map(|val| val.to_owned());
    let host_addr = host_parts
        .next()
        .ok_or_else(|| failure::format_err!("Failed to parse service IP address"))?;

    let c_protocol =
        cbor_protocol::Protocol::new(&host.clone(), shell_protocol::CHUNK_SIZE as usize);

    let timeout = config
        .get("timeout")
        .and_then(|val| val.as_integer().map(|num| Duration::from_secs(num as u64)))
        .unwrap_or(Duration::from_millis(2));

    // Setup map of channel IDs to thread channels
    let raw_threads: HashMap<u32, ThreadProcess> = HashMap::new();
    // Create thread sharable wrapper
    let threads = Arc::new(Mutex::new(raw_threads));

    loop {
        let (channel_message, shell_message, message_source) = match get_message(&c_protocol) {
            Ok((c, s, m)) => (c, s, m),
            Err(e) => {
                warn!("Failed to get next message: {}", e);
                continue;
            }
        };
        let channel_id = channel_message.channel_id;
        let remote_addr = format!("{}", message_source);

        match shell_message {
            // Gather and send back list of processes
            ShellMessage::List {
                channel_id,
                process_list: None,
            } => {
                info!("<- {{ {}, list }}", channel_id);
                list_processes(
                    channel_id,
                    &host_addr,
                    &format!("{}", message_source),
                    &threads,
                )?;
                continue;
            }
            // Spawn up a new process & thread
            ShellMessage::Spawn {
                channel_id,
                command,
                args,
            } => {
                info!("<- {{ {}, spawn, {}, {:?} }}", channel_id, command, args);
                if !threads
                    .lock()
                    .map_err(|err| {
                        error!("Failed to get threads mutex: {:?}", err);
                        err
                    })
                    .unwrap()
                    .contains_key(&channel_id)
                {
                    if let Ok((pid, sender)) = spawn_process(
                        channel_id,
                        &command,
                        args,
                        &host_addr,
                        &remote_addr,
                        timeout,
                        threads.clone(),
                    ) {
                        threads
                            .lock()
                            .map_err(|err| {
                                error!("Failed to get threads mutex: {:?}", err);
                                err
                            })
                            .unwrap()
                            .insert(
                                channel_id,
                                ThreadProcess {
                                    sender: sender.clone(),
                                    pid,
                                    path: command.to_owned(),
                                },
                            );
                    }
                } else {
                    warn!("Process on channel {} already exists", channel_id);
                }
            }
            // Pass along the message to existing process
            _ => {
                if let Some(process_handle) = threads
                    .lock()
                    .map_err(|err| {
                        error!("Failed to get threads mutex: {:?}", err);
                        err
                    })
                    .unwrap()
                    .get(&channel_id)
                {
                    if let Err(e) = process_handle
                        .sender
                        .send((channel_message, message_source))
                    {
                        warn!("Error when sending to channel {}: {:?}", channel_id, e);
                    }
                }

                if !threads
                    .lock()
                    .map_err(|err| {
                        error!("Failed to get threads mutex: {:?}", err);
                        err
                    })
                    .unwrap()
                    .contains_key(&channel_id)
                {
                    warn!("No session found for {}", channel_id);
                    let channel_protocol =
                        ChannelProtocol::new(&host_addr, &remote_addr, shell_protocol::CHUNK_SIZE);

                    channel_protocol.send(&shell_protocol::messages::error::to_cbor(
                        channel_id,
                        &format!("No session found on channel {}", channel_id),
                    )?)?;
                    threads
                        .lock()
                        .map_err(|err| {
                            error!("Failed to get threads mutex: {:?}", err);
                            err
                        })
                        .unwrap()
                        .remove(&channel_id);
                }
            }
        }
    }
}
