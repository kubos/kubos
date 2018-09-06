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

//! TODO: Module documentation

use super::messages;
use super::parsers;
use super::storage;
use super::Message;
use cbor_protocol::Protocol as CborProtocol;
use serde_cbor::Value;
use std::cell::Cell;
use std::net::UdpSocket;
use std::str;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::net::SocketAddr;

// How many times do we read no messages
// while holding before killing the thread
const HOLD_TIMEOUT: u16 = 5;

/// File Protocol information Structure
pub struct Protocol {
    prefix: String,
    cbor_proto: CborProtocol,
    remote_addr: Cell<SocketAddr>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum State {
    // Neutral state, neither transmitting nor receiving
    Holding {
        count: u16,
        prev_state: Box<State>,
    },
    // Receiving a file has been requested
    StartReceive {
        path: String,
    },
    // Currently receiving a file
    Receiving {
        channel_id: u64,
        hash: String,
        path: String,
        mode: Option<u32>,
    },
    ReceivingDone {
        channel_id: u64,
        hash: String,
        path: String,
        mode: Option<u32>,
    },
    // Currenty transmitting a file
    Transmitting,
    TransmittingDone,
    // Finished transmitting/receiving, thread or process may end
    Done,
}

impl Protocol {
    /// Create a new file protocol instance using an automatically assigned UDP socket
    pub fn new(host_ip: &str, remote_addr: &str, prefix: Option<String>) -> Self {
        // Get a local UDP socket (Bind)
        let c_protocol = CborProtocol::new(format!("{}:0", host_ip));

        // Set up the full connection info
        Protocol {
            prefix: prefix.unwrap_or("file-transfer".to_owned()),
            cbor_proto: c_protocol,
            remote_addr: Cell::new(remote_addr.parse::<SocketAddr>().unwrap()),
        }
    }

    /// Create a new file protocol instance using a specific UDP socket
    pub fn new_from_socket(socket: UdpSocket, remote_addr: &str, prefix: Option<String>) -> Self {
        Protocol {
            prefix: prefix.unwrap_or("file-transfer".to_owned()),
            cbor_proto: CborProtocol::new_from_socket(socket),
            remote_addr: Cell::new(remote_addr.parse::<SocketAddr>().unwrap()),
        }
    }

    /// Wrap the specified data into a CBOR packet and then send to the destination port
    pub fn send(&self, vec: Vec<u8>) -> Result<(), String> {
        self.cbor_proto
            .send_message(&vec, self.remote_addr.get())
            .unwrap();
        Ok(())
    }

    pub fn recv(&self, timeout: Option<Duration>) -> Result<Option<Value>, Option<String>> {
        match timeout {
            Some(value) => self.cbor_proto.recv_message_timeout(value),
            None => self.cbor_proto.recv_message(),
        }
    }

    /// Request a file from a remote target
    pub fn send_metadata(&self, hash: &str, num_chunks: u32) -> Result<(), String> {
        self.send(messages::metadata(&hash, num_chunks).unwrap())
    }

    /// Request remote target to receive file from host
    pub fn send_export(&self, hash: &str, target_path: &str, mode: u32) -> Result<(), String> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .and_then(
                |duration| Ok(duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1000000),
            )
            .map_err(|err| format!("Failed to get current system time: {}", err))?;
        let channel_id: u32 = (time % 100000) as u32;

        self.send(messages::export_request(channel_id, hash, target_path, mode).unwrap())
            .unwrap();

        Ok(())
    }

    /// Request a file from a remote target
    pub fn send_import(&self, source_path: &str) -> Result<(), String> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .and_then(
                |duration| Ok(duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1000000),
            )
            .map_err(|err| format!("Failed to get current system time: {}", err))?;
        let channel_id: u32 = (time % 100000) as u32;

        self.send(messages::import_request(channel_id, source_path).unwrap())
            .unwrap();
        Ok(())
    }

    /// Create temporary folder for chunks
    /// Stream copy file from mutable space to immutable space
    /// Move folder to hash of contents
    pub fn initialize_file(&self, source_path: &str) -> Result<(String, u32, u32), String> {
        storage::initialize_file(&self.prefix, source_path)
    }

    /// Verify the integrity of received file data and then transfer into the requested permanent file location.
    /// Notify the connection peer of the results
    ///
    /// Verifies:
    ///     a) All of the chunks of a file have been received
    ///     b) That the calculated hash of said chunks matches the expected hash
    ///
    pub fn finalize_file(
        &self,
        channel_id: u64,
        hash: &str,
        target_path: &str,
        mode: Option<u32>,
    ) -> Result<(), String> {
        match storage::finalize_file(&self.prefix, hash, target_path, mode) {
            Ok(_) => {
                self.send(messages::operation_success(channel_id).unwrap())?;
                return Ok(());
            }
            Err(e) => {
                self.send(messages::operation_failure(channel_id, &e).unwrap())?;
                return Err(e);
            }
        }
    }

    /// Store a files metadata into the appropriate temporary storage location
    pub fn store_meta(&self, hash: &str, num_chunks: u32) -> Result<(), String> {
        storage::store_meta(&self.prefix, hash, num_chunks)
    }

    /// Send all requested chunks of a file to the remote destination
    pub fn send_chunks(&self, hash: &str, chunks: &[(u32, u32)]) -> Result<(), String> {
        for (first, last) in chunks {
            for chunk_index in *first..*last {
                let chunk = storage::load_chunk(&self.prefix, hash, chunk_index).unwrap();
                self.send(messages::chunk(hash, chunk_index, &chunk).unwrap())
                    .unwrap();

                thread::sleep(Duration::from_millis(1));
            }
        }
        Ok(())
    }

    /// Listen for and process file protocol messages
    /// TODO: Make this more descriptive
    pub fn message_engine(&self, timeout: Duration, start_state: State) -> Result<(), String> {
        let mut state = start_state.clone();
        loop {
            // Listen on UDP port
            let message = match self.cbor_proto.recv_message_peer_timeout(timeout) {
                Ok((peer, Some(message))) => {
                    // Update our response port
                    self.remote_addr.set(peer);

                    // If we previously timed out, restore the old state
                    if let State::Holding { count, prev_state } = state {
                        state = *prev_state;
                    }

                    message
                }
                // I don't think recv_message reports the difference between
                // no message/timeout and an actual error
                // TODO: Fix that
                _ => match state.clone() {
                    State::Receiving {
                        channel_id,
                        hash,
                        path,
                        mode,
                    } => {
                        match storage::validate_file(&self.prefix, &hash, None) {
                            Ok((true, _)) => {
                                self.send(messages::ack(&hash, None).unwrap()).unwrap();
                                state = State::ReceivingDone {
                                    channel_id,
                                    hash: hash.clone(),
                                    path: path.clone(),
                                    mode,
                                };
                            }
                            Ok((false, chunks)) => {
                                self.send(messages::nak(&hash, &chunks).unwrap()).unwrap();
                                state = State::Holding {
                                    count: 0,
                                    prev_state: Box::new(state.clone()),
                                };
                                continue;
                            }
                            Err(e) => return Err(e),
                        };

                        match self.finalize_file(channel_id, &hash, &path, mode) {
                            Ok(_) => {
                                return Ok(());
                            }
                            Err(e) => {
                                // We need a way to error out here...
                                state = State::Holding {
                                    count: 0,
                                    prev_state: Box::new(state.clone()),
                                };
                                continue;
                            }
                        }
                    }
                    State::ReceivingDone {
                        channel_id,
                        hash,
                        path,
                        mode,
                    } => {
                        // We've got all the chunks of data we want.
                        // Stitch it back together and verify the hash of the official file
                        self.finalize_file(channel_id, &hash, &path, mode)?;
                        return Ok(());
                    }
                    State::Done => {
                        return Ok(());
                    }
                    State::Holding { count, prev_state } => {
                        if count > HOLD_TIMEOUT {
                            return Ok(());
                        } else {
                            state = State::Holding {
                                count: count + 1,
                                prev_state,
                            };
                            continue;
                        }
                    }
                    other => {
                        state = State::Holding {
                            count: 0,
                            prev_state: Box::new(state.clone()),
                        };
                        continue;
                    }
                },
            };

            match self.process_message(message, state.clone()) {
                Ok(new_state) => state = new_state,
                Err(e) => return Err(e),
            }

            match state.clone() {
                State::ReceivingDone {
                    channel_id,
                    hash,
                    path,
                    mode,
                } => {
                    // We've got all the chunks of data we want.
                    // Stitch it back together and verify the hash of the official file
                    self.finalize_file(channel_id, &hash, &path, mode)?;
                    return Ok(());
                }
                State::Done => return Ok(()),
                _ => continue,
            };
        }
    }

    /// Process a file protocol message
    pub fn process_message(&self, message: Value, state: State) -> Result<State, String> {
        let parsed_message = parsers::parse_message(message);
        let new_state;
        match parsed_message.to_owned() {
            Ok(parsed_message) => {
                match &parsed_message {
                    Message::Sync(hash) => {
                        info!("<- {{ {} }}", hash);
                        new_state = state.clone();
                    }
                    Message::Metadata(hash, num_chunks) => {
                        info!("<- {{ {}, {} }}", hash, num_chunks);
                        storage::store_meta(&self.prefix, &hash, *num_chunks).unwrap();
                        new_state = state.clone();
                    }
                    Message::ReceiveChunk(hash, chunk_num, data) => {
                        info!("<- {{ {}, {}, chunk_data }}", hash, chunk_num);
                        storage::store_chunk(&self.prefix, &hash, *chunk_num, &data).unwrap();
                        new_state = state.clone();
                    }
                    Message::ACK(ack_hash) => {
                        info!("<- {{ {}, true }}", ack_hash);
                        // TODO: Figure out hash verification here
                        new_state = State::TransmittingDone;
                    }
                    Message::NAK(hash, Some(missing_chunks)) => {
                        info!("<- {{ {}, false, {:?} }}", hash, missing_chunks);
                        self.send_chunks(&hash, &missing_chunks)?;
                        new_state = State::Transmitting;
                    }
                    Message::NAK(hash, None) => {
                        info!("<- {{ {}, false }}", hash);
                        // TODO: Maybe trigger a failure?
                        new_state = state.clone();
                    }
                    Message::ReqReceive(channel_id, hash, path, mode) => {
                        info!(
                            "<- {{ {}, export, {}, {}, {:?} }}",
                            channel_id, hash, path, mode
                        );
                        // The client wants to send us a file.
                        // See what state the file is currently in on our side
                        match storage::validate_file(&self.prefix, hash, None) {
                            Ok((true, _)) => {
                                // We've already got all the file data in temporary storage
                                self.send(messages::ack(&hash, None).unwrap()).unwrap();

                                new_state = State::ReceivingDone {
                                    channel_id: *channel_id,
                                    hash: hash.to_string(),
                                    path: path.to_string(),
                                    mode: *mode,
                                };
                            }
                            Ok((false, chunks)) => {
                                // We're missing some number of data chunks of the requrested file
                                self.send(messages::nak(&hash, &chunks).unwrap()).unwrap();
                                new_state = State::Receiving {
                                    channel_id: *channel_id,
                                    hash: hash.to_string(),
                                    path: path.to_string(),
                                    mode: *mode,
                                };
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    Message::ReqTransmit(channel_id, path) => {
                        info!("<- {{ {}, import, {} }}", channel_id, path);
                        // Set up the requested file for transmission
                        match storage::initialize_file(&self.prefix, path) {
                            Ok((hash, num_chunks, mode)) => {
                                // It worked, let the requester know we're ready to send
                                self.send(
                                    messages::import_setup_success(
                                        *channel_id,
                                        &hash,
                                        num_chunks,
                                        mode,
                                    ).unwrap(),
                                ).unwrap();

                                new_state = State::Transmitting;
                            }
                            Err(error) => {
                                // It failed. Let the requester know that we can't transmit
                                // the file they want.
                                self.send(
                                    messages::operation_failure(*channel_id, &error).unwrap(),
                                ).unwrap();

                                new_state = State::Done;
                            }
                        }
                    }
                    Message::SuccessReceive(channel_id) => {
                        info!("<- {{ {}, true }}", channel_id);
                        new_state = State::Done;
                    }
                    Message::SuccessTransmit(channel_id, hash, num_chunks, mode) => {
                        match mode {
                            Some(value) => info!(
                                "<- {{ {}, true, {}, {}, {} }}",
                                channel_id, hash, num_chunks, value
                            ),
                            None => {
                                info!("<- {{ {}, true, {}, {} }}", channel_id, hash, num_chunks)
                            }
                        }

                        // TODO: handle channel_id mismatch
                        match storage::validate_file(&self.prefix, hash, Some(*num_chunks)) {
                            Ok((true, _)) => {
                                self.send(messages::ack(&hash, Some(*num_chunks)).unwrap())
                                    .unwrap();
                                new_state = match state.clone() {
                                    State::StartReceive { path } => State::ReceivingDone {
                                        channel_id: *channel_id,
                                        hash: hash.to_string(),
                                        path: path.to_string(),
                                        mode: *mode,
                                    },
                                    _ => State::Done,
                                };
                            }
                            Ok((false, chunks)) => {
                                self.send(messages::nak(&hash, &chunks).unwrap()).unwrap();
                                new_state = match state.clone() {
                                    State::StartReceive { path } => State::Receiving {
                                        channel_id: *channel_id,
                                        hash: hash.to_string(),
                                        path: path.to_string(),
                                        mode: *mode,
                                    },
                                    _ => state.clone(),
                                };
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    Message::Failure(channel_id, error_message) => {
                        info!("<- {{ {}, false, {} }}", channel_id, error_message);
                        return Err(format!(
                            "Transmission failure on channel {}. Error returned from server: {}",
                            channel_id, error_message
                        ));
                    }
                }
                Ok(new_state)
            }
            Err(e) => {
                info!("<- what did we get?? {}", e);
                Err(e)
            }
        }
    }
}
