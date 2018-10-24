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

extern crate cbor_protocol;
extern crate channel_protocol;
extern crate failure;
extern crate kubos_system;
#[macro_use]
extern crate log;
extern crate serde_cbor;
extern crate shell_protocol;
extern crate simplelog;

use channel_protocol::ChannelMessage;
use kubos_system::Config as ServiceConfig;
use shell_protocol::{ProcessHandler, ProtocolError, ShellMessage, ShellProtocol};
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct ProcessHandle {
    pub sender: Sender<ChannelMessage>,
    pub pid: u32,
    pub path: String,
}

// We need this in this lib.rs file so we can build integration tests
pub fn recv_loop(config: ServiceConfig) -> Result<(), failure::Error> {
    // Get and bind our UDP listening socket
    let host = config.hosturl();

    // Extract our local IP address so we can spawn child sockets later
    let mut host_parts = host.split(':').map(|val| val.to_owned());
    let host_ip = host_parts.next().unwrap();

    let c_protocol = cbor_protocol::Protocol::new(host.clone(), 4096);

    let timeout = config
        .get("timeout")
        .and_then(|val| {
            val.as_integer()
                .and_then(|num| Some(Duration::from_secs(num as u64)))
        }).unwrap_or(Duration::from_millis(2));

    // Setup map of channel IDs to thread channels
    let raw_threads: HashMap<u32, ProcessHandle> = HashMap::new();
    // Create thread sharable wrapper
    let threads = Arc::new(Mutex::new(raw_threads));

    loop {
        // Listen on UDP port
        let (source, first_message) = match c_protocol.recv_message_peer() {
            Ok((source, first_message)) => (source, first_message),
            Err(e) => {
                warn!("Failed to receive message: {}", e);
                continue;
            }
        };

        let host_ref = host_ip.clone();
        let timeout_ref = timeout.clone();

        let parsed_channel_message = match channel_protocol::parse_message(first_message) {
            Ok(parsed_message) => parsed_message,
            Err(e) => {
                warn!("Error parsing channel message: {:?}", e);
                continue;
            }
        };
        let channel_id = parsed_channel_message.channel_id;

        let parsed_shell_message =
            match shell_protocol::parse_message(parsed_channel_message.clone()) {
                Ok(shell_message) => shell_message,
                Err(e) => {
                    warn!("Error parsing shell message: {:?}", e);
                    continue;
                }
            };

        match parsed_shell_message {
            // Send back list of processes
            ShellMessage::List {
                channel_id,
                process_list,
            } => {
                println!("got list");
                for (k, v) in threads.lock().unwrap().iter() {
                    println!("{:?} -> {:?}", k, v);
                }

                let proc_list: HashMap<u32, (String, u32)> = threads
                    .lock()
                    .unwrap()
                    .iter()
                    .map(|(channel_id, data)| (*channel_id, (data.path.to_owned(), data.pid)))
                    .collect();

                let chan_sender =
                    channel_protocol::ChannelProtocol::new(&host_ref, &format!("{}", source), 4096);

                chan_sender.send(shell_protocol::messages::list::to_cbor(
                    channel_id,
                    Some(proc_list),
                )?);

                continue;
            }
            // Spawn a new thread for this process
            ShellMessage::Spawn {
                channel_id,
                command,
                args,
            } => {
                if !threads.lock().unwrap().contains_key(&channel_id) {
                    let (sender, receiver): (
                        Sender<ChannelMessage>,
                        Receiver<ChannelMessage>,
                    ) = mpsc::channel();
                    // Break the processing work off into its own thread so we can
                    // listen for requests from other clients
                    let shared_threads = threads.clone();
                    thread::spawn(move || {
                        let proc_handle = match ProcessHandler::spawn(command.to_owned(), args) {
                            Ok(p) => p,
                            Err(e) => {
                                warn!("Failed to spawn {:?}", e);
                                return;
                            }
                        };

                        shared_threads.lock().unwrap().insert(
                            channel_id,
                            ProcessHandle {
                                sender: sender.clone(),
                                pid: proc_handle.id().unwrap_or(0),
                                path: command.to_owned(),
                            },
                        );

                        let mut s_protocol = ShellProtocol::new(
                            &host_ref,
                            &format!("{}", source),
                            channel_id,
                            Box::new(Some(proc_handle)),
                        );

                        // Listen, process, and react to the remaining messages in the
                        // requested operation
                        match s_protocol.message_engine(
                            |d| match receiver.recv_timeout(d) {
                                Ok(v) => Ok(v),
                                Err(RecvTimeoutError::Timeout) => {
                                    Err(ProtocolError::ReceiveTimeout)
                                }
                                Err(e) => Err(ProtocolError::ReceiveError {
                                    err: format!("Error {:?}", e),
                                }),
                            },
                            timeout_ref,
                        ) {
                            Err(e) => {
                                warn!("Encountered errors while processing transaction: {}", e)
                            }
                            _ => {}
                        }

                        // Remove ourselves from threads list if we are finished
                        shared_threads.lock().unwrap().remove(&channel_id);
                    });
                } else {
                    warn!("Process on channel {} already exists", channel_id);
                }
            }
            // Pass along the message to existing process
            _ => {
                if let Some(process_handle) = threads.lock().unwrap().get(&channel_id) {
                    match process_handle.sender.send(parsed_channel_message) {
                        Err(e) => warn!("Error when sending to channel {}: {:?}", channel_id, e),
                        _ => {}
                    };
                } else {
                    warn!("No sender found for {}", channel_id);
                    threads.lock().unwrap().remove(&channel_id);
                }
            }
        }
    }
}
