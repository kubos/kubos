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
struct ThreadProcess {
    pub sender: Sender<ChannelMessage>,
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
        .unwrap()
        .iter()
        .map(|(channel_id, data)| (*channel_id, (data.path.to_owned(), data.pid)))
        .collect();

    let chan_proto = channel_protocol::ChannelProtocol::new(host, remote, 4096);

    chan_proto.send(shell_protocol::messages::list::to_cbor(
        channel_id,
        Some(proc_list),
    )?)?;

    Ok(())
}

// Main function of process handling thread
fn thread_body(
    host: &str,
    remote: &str,
    channel_id: u32,
    timeout: Duration,
    proc_handle: ProcessHandler,
    shared_threads: Arc<Mutex<HashMap<u32, ThreadProcess>>>,
    receiver: Receiver<ChannelMessage>,
) -> () {
    let mut s_protocol =
        ShellProtocol::new(&host, &remote, channel_id, Box::new(Some(proc_handle)));

    // Receive and react to incoming shell protocol messages
    match s_protocol.message_engine(
        |d| match receiver.recv_timeout(d) {
            Ok(v) => Ok(v),
            Err(RecvTimeoutError::Timeout) => Err(ProtocolError::ReceiveTimeout),
            Err(e) => Err(ProtocolError::ReceiveError {
                err: format!("Error {:?}", e),
            }),
        },
        timeout,
    ) {
        Err(e) => warn!("Encountered errors while processing transaction: {}", e),
        _ => {}
    }

    // Remove ourselves from threads list if we are finished
    shared_threads.lock().unwrap().remove(&channel_id);
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

    let shell_message = shell_protocol::parse_message(channel_message.clone())?;

    Ok((channel_message, shell_message, source))
}

//
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
    let raw_threads: HashMap<u32, ThreadProcess> = HashMap::new();
    // Create thread sharable wrapper
    let threads = Arc::new(Mutex::new(raw_threads));

    loop {
        let host_ref = host_ip.clone();
        let timeout_ref = timeout.clone();

        let (channel_message, shell_message, message_source) = match get_message(&c_protocol) {
            Ok((c, s, m)) => (c, s, m),
            Err(e) => {
                warn!("Failed to get next message: {}", e);
                continue;
            }
        };
        let channel_id = channel_message.channel_id;

        match shell_message {
            // Gather and send back list of processes
            ShellMessage::List {
                channel_id,
                process_list: None,
            } => {
                list_processes(
                    channel_id,
                    &host_ref,
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
                if !threads.lock().unwrap().contains_key(&channel_id) {
                    let (sender, receiver): (
                        Sender<ChannelMessage>,
                        Receiver<ChannelMessage>,
                    ) = mpsc::channel();

                    let proc_handle = match ProcessHandler::spawn(command.to_owned(), args) {
                        Ok(p) => p,
                        Err(e) => {
                            warn!("Failed to spawn {:?}", e);
                            continue;
                        }
                    };

                    threads.lock().unwrap().insert(
                        channel_id,
                        ThreadProcess {
                            sender: sender.clone(),
                            pid: proc_handle.id().unwrap_or(0),
                            path: command.to_owned(),
                        },
                    );

                    let shared_threads = threads.clone();
                    thread::spawn(move || {
                        thread_body(
                            &host_ref,
                            &format!("{}", message_source),
                            channel_id,
                            timeout_ref,
                            proc_handle,
                            shared_threads,
                            receiver,
                        )
                    });
                } else {
                    warn!("Process on channel {} already exists", channel_id);
                }
            }
            // Pass along the message to existing process
            _ => {
                if let Some(process_handle) = threads.lock().unwrap().get(&channel_id) {
                    match process_handle.sender.send(channel_message) {
                        Err(e) => warn!("Error when sending to channel {}: {:?}", channel_id, e),
                        _ => {}
                    };
                }

                if !threads.lock().unwrap().contains_key(&channel_id) {
                    warn!("No sender found for {}", channel_id);
                    threads.lock().unwrap().remove(&channel_id);
                }
            }
        }
    }
}
