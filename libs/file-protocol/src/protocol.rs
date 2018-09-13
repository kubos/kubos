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

//! File transfer protocol module

use super::messages;
use super::parsers;
use super::storage;
use super::Message;
use cbor_protocol::Protocol as CborProtocol;
use rand::{self, Rng};
use serde_cbor::Value;
use std::cell::Cell;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::str;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// How many times do we read no messages
// while holding before killing the thread
const HOLD_TIMEOUT: u16 = 5;

/// File protocol information structure
pub struct Protocol {
    prefix: String,
    cbor_proto: CborProtocol,
    remote_addr: Cell<SocketAddr>,
}

/// Current state of the file protocol transaction
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum State {
    /// Neutral state, neither transmitting nor receiving
    Holding {
        /// Number of consecutive times the holding state has been hit
        count: u16,
        /// Previous state to return to once we exit the holding state
        prev_state: Box<State>,
    },
    /// Preparing to receive file chunks
    StartReceive {
        /// Destination file path
        path: String,
    },
    /// Currently receiving a file
    Receiving {
        /// Transaction identifier
        channel_id: u32,
        /// File hash
        hash: String,
        /// Destination file path
        path: String,
        /// File mode
        mode: Option<u32>,
    },
    /// All file chunks have been received
    ReceivingDone {
        /// Transaction identifier
        channel_id: u32,
        /// File hash
        hash: String,
        /// Destination file path
        path: String,
        /// File mode
        mode: Option<u32>,
    },
    /// Currenty transmitting a file
    Transmitting,
    /// All file chunks have been transmitted
    TransmittingDone,
    /// Finished transmitting/receiving, thread or process may end
    Done,
}

impl Protocol {
    /// Create a new file protocol instance using an automatically assigned UDP socket
    ///
    /// # Arguments
    ///
    /// * host_ip - The local IP address
    /// * remote_addr - The remote IP and port to communicate with
    /// * prefix - Temporary storage directory prefix
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will panic
    ///
    /// # Examples
    ///
    /// ```
    /// use file_protocol::*;
    ///
    /// let f_protocol = FileProtocol::new("0.0.0.0", "192.168.0.1:7000", Some("my/file/storage".to_owned()));
    /// ```
    ///
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
    ///
    /// # Arguments
    ///
    /// * socket - The local socket to use for communication
    /// * remote_addr - The remote IP and port to communicate with
    /// * prefix - Temporary storage directory prefix
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will panic
    ///
    /// # Examples
    ///
    /// ```
    /// use file_protocol::*;
    /// use std::net::UdpSocket;
    ///
    /// let socket = UdpSocket::bind("0.0.0.0:8000").unwrap();
    ///
    /// let f_protocol = FileProtocol::new_from_socket(socket, "192.168.0.1:7000", None);
    /// ```
    ///
    pub fn new_from_socket(socket: UdpSocket, remote_addr: &str, prefix: Option<String>) -> Self {
        Protocol {
            prefix: prefix.unwrap_or("file-transfer".to_owned()),
            cbor_proto: CborProtocol::new_from_socket(socket),
            remote_addr: Cell::new(remote_addr.parse::<SocketAddr>().unwrap()),
        }
    }

    /// Send CBOR packet to the destination port
    ///
    /// # Arguments
    ///
    /// * vec - CBOR packet to send
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate file_protocol;
    /// extern crate serde_cbor;
    ///
    /// use file_protocol::*;
    /// use serde_cbor::ser;
    ///
    /// let f_protocol = FileProtocol::new("0.0.0.0", "0.0.0.0:7000", None);
    /// let message = ser::to_vec_packed(&"ping").unwrap();
    ///
    /// f_protocol.send(message);
    /// ```
    ///
    pub fn send(&self, vec: Vec<u8>) -> Result<(), String> {
        self.cbor_proto.send_message(&vec, self.remote_addr.get())?;
        Ok(())
    }

    /// Receive a file protocol message
    ///
    /// # Arguments
    ///
    /// * timeout - Maximum time to wait for a reply. If `None`, will block indefinitely
    ///
    /// # Errors
    ///
    /// - If this function times out, it will return `Err(None)`
    /// - If this function encounters any errors, it will return an error message string
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate file_protocol;
    ///
    /// use file_protocol::*;
    /// use std::time::Duration;
    ///
    /// let f_protocol = FileProtocol::new("0.0.0.0", "0.0.0.0:7000", None);
    ///
    /// let message = match f_protocol.recv(Some(Duration::from_secs(1))) {
    /// 	Ok(data) => data,
    /// 	Err(None) => {
    ///			println!("Timeout waiting for message");
    ///			return;
    ///		}
    /// 	Err(Some(err)) => panic!("Failed to receive message: {}", err),
    /// };
    /// ```
    ///
    pub fn recv(&self, timeout: Option<Duration>) -> Result<Option<Value>, Option<String>> {
        match timeout {
            Some(value) => self.cbor_proto.recv_message_timeout(value),
            None => self.cbor_proto.recv_message(),
        }
    }

    /// Generate channel id
    pub fn generate_channel(&self) -> Result<u32, String> {
        let mut rng = rand::thread_rng();
        let channel_id: u32 = rng.gen_range(100000, 999999);
        Ok(channel_id)
    }

    /// Send a file's metadata information to the remote target
    ///
    /// # Arguments
    ///
    /// * hash - BLAKE2s hash of file
    /// * num_chunks - Number of data chunks needed for file
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use file_protocol::*;
    ///
    /// let f_protocol = FileProtocol::new("0.0.0.0", "0.0.0.0:7000", None);
    ///
    /// # ::std::fs::File::create("client.txt").unwrap();
    ///
    /// let (hash, num_chunks, _mode) = f_protocol.initialize_file("client.txt").unwrap();
    /// f_protocol.send_metadata(&hash, num_chunks);
    /// ```
    ///
    pub fn send_metadata(
        &self,
        channel_id: u32,
        hash: &str,
        num_chunks: u32,
    ) -> Result<(), String> {
        self.send(messages::metadata(channel_id, &hash, num_chunks)?)
    }

    /// Request remote target to receive file from host
    ///
    /// # Arguments
    ///
    /// * hash - BLAKE2s hash of file
    /// * target_path - Destination file path
    /// * mode - File mode
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```
    /// use file_protocol::*;
    ///
    /// let f_protocol = FileProtocol::new("0.0.0.0", "0.0.0.0:7000", None);
    ///
    /// # ::std::fs::File::create("client.txt").unwrap();
    ///
    /// let (hash, _num_chunks, mode) = f_protocol.initialize_file("client.txt").unwrap();
    /// f_protocol.send_export(&hash, "final/dir/service.txt", mode);
    /// ```
    ///
    pub fn send_export(
        &self,
        channel_id: u32,
        hash: &str,
        target_path: &str,
        mode: u32,
    ) -> Result<(), String> {
        self.send(messages::export_request(
            channel_id,
            hash,
            target_path,
            mode,
        )?)?;

        Ok(())
    }

    /// Request a file from a remote target
    ///
    /// # Arguments
    ///
    /// * source_path - File remote target should send
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```
    /// use file_protocol::*;
    ///
    /// let f_protocol = FileProtocol::new("0.0.0.0", "0.0.0.0:7000", None);
    ///
    /// f_protocol.send_import("service.txt");
    /// ```
    ///
    pub fn send_import(&self, channel_id: u32, source_path: &str) -> Result<(), String> {
        self.send(messages::import_request(channel_id, source_path)?)?;
        Ok(())
    }

    /// Prepare a file for transfer
    ///
    /// Imports the file into temporary storage and calculates the BLAKE2s hash
    ///
    /// # Arguments
    ///
    /// * source_path - File to initialize for transfer
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```
    /// use file_protocol::*;
    ///
    /// let f_protocol = FileProtocol::new("0.0.0.0", "0.0.0.0:7000", None);
    ///
    /// # ::std::fs::File::create("client.txt").unwrap();
    ///
    /// let (_hash, _num_chunks, _mode) = f_protocol.initialize_file("client.txt").unwrap();
    /// ```
    ///
    pub fn initialize_file(&self, source_path: &str) -> Result<(String, u32, u32), String> {
        storage::initialize_file(&self.prefix, source_path)
    }

    // Verify the integrity of received file data and then transfer into the requested permanent file location.
    // Notify the connection peer of the results
    //
    // Verifies:
    //     a) All of the chunks of a file have been received
    //     b) That the calculated hash of said chunks matches the expected hash
    //
    fn finalize_file(
        &self,
        channel_id: u32,
        hash: &str,
        target_path: &str,
        mode: Option<u32>,
    ) -> Result<(), String> {
        match storage::finalize_file(&self.prefix, hash, target_path, mode) {
            Ok(_) => {
                self.send(messages::operation_success(channel_id)?)?;
                return Ok(());
            }
            Err(e) => {
                self.send(messages::operation_failure(channel_id, &e)?)?;
                return Err(e);
            }
        }
    }

    // Send all requested chunks of a file to the remote destination
    fn send_chunks(
        &self,
        channel_id: u32,
        hash: &str,
        chunks: &[(u32, u32)],
    ) -> Result<(), String> {
        for (first, last) in chunks {
            for chunk_index in *first..*last {
                let chunk = storage::load_chunk(&self.prefix, hash, chunk_index)?;
                self.send(messages::chunk(channel_id, hash, chunk_index, &chunk)?)?;

                thread::sleep(Duration::from_millis(1));
            }
        }
        Ok(())
    }

    /// Listen for and process file protocol messages
    ///
    /// # Arguments
    ///
    /// * timeout - Maximum time to listen for a single message
    /// * start_state - Current transaction state
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate file_protocol;
    ///
    /// use file_protocol::*;
    /// use std::time::Duration;
    ///
    /// let f_protocol = FileProtocol::new("0.0.0.0", "0.0.0.0:7000", None);
    ///
    /// f_protocol.message_engine(Duration::from_millis(10), State::Transmitting);
    /// ```
    ///
    pub fn message_engine<F>(
        &self,
        pump: F,
        timeout: Duration,
        start_state: State,
    ) -> Result<(), String>
    where
        F: Fn(Duration) -> Result<Option<Value>, Option<String>>,
    {
        let mut state = start_state.clone();
        loop {
            // Listen on UDP port
            info!("engine pump {:?}", state);
            let message = match pump(timeout) {
                //let message = match self.cbor_proto.recv_message_peer_timeout(timeout) {
                Ok(Some(message)) => {
                    // Ok((peer, Some(message))) => {
                    // Update our response port
                    // self.remote_addr.set(peer);

                    // If we previously timed out, restore the old state
                    if let State::Holding {
                        count: _,
                        prev_state,
                    } = state
                    {
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
                                self.send(messages::ack(channel_id, &hash, None)?)?;
                                state = State::ReceivingDone {
                                    channel_id,
                                    hash: hash.clone(),
                                    path: path.clone(),
                                    mode,
                                };
                            }
                            Ok((false, chunks)) => {
                                self.send(messages::nak(channel_id, &hash, &chunks)?)?;
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
                                warn!("Failed to finalize file {} as {}: {}", hash, path, e);
                                // TODO: Handle finalization failures (ex. corrupted chunk file)
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
                    _ => {
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
    ///
    /// Returns the new transaction state
    ///
    /// # Arguments
    ///
    /// * message - File protocol message to process
    /// * state - Current transaction state
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate file_protocol;
    ///
    /// use file_protocol::*;
    /// use std::time::Duration;
    ///
    /// let f_protocol = FileProtocol::new("0.0.0.0", "0.0.0.0:7000", None);
    ///
    /// if let Ok(Some(message)) = f_protocol.recv(Some(Duration::from_millis(100))) {
    /// 	let _state = f_protocol.process_message(
    ///			message,
    ///			State::StartReceive {
    ///				path: "target/dir/file.bin".to_owned()
    ///         }
    ///		);
    /// }
    /// ```
    ///
    pub fn process_message(&self, message: Value, state: State) -> Result<State, String> {
        let parsed_message = parsers::parse_message(message);
        let new_state;
        match parsed_message.to_owned() {
            Ok(parsed_message) => {
                match &parsed_message {
                    Message::Sync(channel_id, hash) => {
                        info!("<- {{ {}, {} }}", channel_id, hash);
                        new_state = state.clone();
                    }
                    Message::Metadata(channel_id, hash, num_chunks) => {
                        info!("<- {{ {}, {}, {} }}", channel_id, hash, num_chunks);
                        storage::store_meta(&self.prefix, &hash, *num_chunks)?;
                        new_state = State::StartReceive {
                            path: hash.to_owned(),
                        };
                    }
                    Message::ReceiveChunk(channel_id, hash, chunk_num, data) => {
                        info!(
                            "<- {{ {}, {}, {}, chunk_data }}",
                            channel_id, hash, chunk_num
                        );
                        storage::store_chunk(&self.prefix, &hash, *chunk_num, &data)?;
                        new_state = state.clone();
                    }
                    Message::ACK(channel_id, ack_hash) => {
                        info!("<- {{ {}, true }}", ack_hash);
                        // TODO: Figure out hash verification here
                        new_state = State::TransmittingDone;
                    }
                    Message::NAK(channel_id, hash, Some(missing_chunks)) => {
                        info!(
                            "<- {{ {}, {}, false, {:?} }}",
                            channel_id, hash, missing_chunks
                        );
                        self.send_chunks(*channel_id, &hash, &missing_chunks)?;
                        new_state = State::Transmitting;
                    }
                    Message::NAK(channel_id, hash, None) => {
                        info!("<- {{ {}, {}, false }}", channel_id, hash);
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
                                self.send(messages::ack(*channel_id, &hash, None)?)?;

                                new_state = State::ReceivingDone {
                                    channel_id: *channel_id,
                                    hash: hash.to_string(),
                                    path: path.to_string(),
                                    mode: *mode,
                                };
                            }
                            Ok((false, chunks)) => {
                                // We're missing some number of data chunks of the requrested file
                                self.send(messages::nak(*channel_id, &hash, &chunks)?)?;
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
                                self.send(messages::import_setup_success(
                                    *channel_id,
                                    &hash,
                                    num_chunks,
                                    mode,
                                )?)?;

                                new_state = State::Transmitting;
                            }
                            Err(error) => {
                                // It failed. Let the requester know that we can't transmit
                                // the file they want.
                                self.send(messages::operation_failure(*channel_id, &error)?)?;

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
                                self.send(messages::ack(*channel_id, &hash, Some(*num_chunks))?)?;
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
                                self.send(messages::nak(*channel_id, &hash, &chunks)?)?;
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
