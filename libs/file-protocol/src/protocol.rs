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
use crate::error::ProtocolError;
use cbor_protocol::Protocol as CborProtocol;
use log::{error, info, warn};
use rand::{self, Rng};
use serde_cbor::Value;
use std::cell::Cell;
use std::net::SocketAddr;
use std::str;
use std::thread;
use std::time::Duration;

/// Configuration data for Protocol
#[derive(Clone)]
pub struct ProtocolConfig {
    // Name of folder used to store protocol metadata
    storage_prefix: String,
    // Chunk size used in transfers
    transfer_chunk_size: usize,
    // How many times do we read and timeout
    // while in the Hold state before stopping
    hold_count: u16,
    // Duration of delay between individual chunk transmission
    inter_chunk_delay: Duration,
    // Max number of chunks to transmit in one go
    max_chunks_transmit: Option<u32>,
    // Chunk size used in storage hashing
    hash_chunk_size: usize,
}

impl ProtocolConfig {
    /// Creates new ProtocolConfig struct
    pub fn new(
        storage_prefix: Option<String>,
        transfer_chunk_size: usize,
        hold_count: u16,
        inter_chunk_delay: u64,
        max_chunks_transmit: Option<u32>,
        hash_chunk_size: usize,
    ) -> Self {
        ProtocolConfig {
            storage_prefix: storage_prefix.unwrap_or_else(|| "file-storage".to_owned()),
            transfer_chunk_size,
            hold_count,
            inter_chunk_delay: Duration::from_millis(inter_chunk_delay),
            max_chunks_transmit,
            hash_chunk_size,
        }
    }
}

/// File protocol information structure
pub struct Protocol {
    cbor_proto: CborProtocol,
    remote_addr: Cell<SocketAddr>,
    config: ProtocolConfig,
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
    /// ```no_run
    /// use file_protocol::*;
    ///
    /// let config = FileProtocolConfig::new(Some("my/file/storage".to_owned()), 1024, 5, 1, None, 2048);
    /// let f_protocol = FileProtocol::new("0.0.0.0:8000", "192.168.0.1:7000", config);
    /// ```
    ///
    pub fn new(host_addr: &str, remote_addr: &str, config: ProtocolConfig) -> Self {
        // Get a local UDP socket (Bind)
        let c_protocol = CborProtocol::new(host_addr, config.transfer_chunk_size);

        // Set up the full connection info
        Protocol {
            cbor_proto: c_protocol,
            remote_addr: Cell::new(
                remote_addr
                    .parse::<SocketAddr>()
                    .map_err(|err| {
                        error!("Failed to parse remote_addr: {:?}", err);
                        err
                    })
                    .unwrap(),
            ),
            config,
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
    /// ```no_run
    /// use file_protocol::*;
    /// use serde_cbor::ser;
    ///
    /// let config = FileProtocolConfig::new(None, 1024, 5, 1, None, 2048);
    /// let f_protocol = FileProtocol::new("0.0.0.0:8000", "0.0.0.0:7000", config);
    /// let message = ser::to_vec_packed(&"ping").unwrap();
    ///
    /// f_protocol.send(&message);
    /// ```
    ///
    pub fn send(&self, vec: &[u8]) -> Result<(), ProtocolError> {
        self.cbor_proto.send_message(vec, self.remote_addr.get())?;
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
    /// ```no_run
    /// use file_protocol::*;
    /// use std::time::Duration;
    ///
    /// let config = FileProtocolConfig::new(None, 1024, 5, 1, None, 2048);
    /// let f_protocol = FileProtocol::new("0.0.0.0:8000", "0.0.0.0:7000", config);
    ///
    /// let message = match f_protocol.recv(Some(Duration::from_secs(1))) {
    ///     Ok(data) => data,
    ///     Err(ProtocolError::ReceiveTimeout) =>  {
    ///         println!("Timeout waiting for message");
    ///         return;
    ///     }
    ///     Err(err) => panic!("Failed to receive message: {}", err),
    /// };
    /// ```
    ///
    pub fn recv(&self, timeout: Option<Duration>) -> Result<Value, ProtocolError> {
        match timeout {
            Some(value) => Ok(self.cbor_proto.recv_message_timeout(value)?),
            None => Ok(self.cbor_proto.recv_message()?),
        }
    }

    /// Generates a new random channel ID for use when initiating a
    /// file transfer.
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
    /// let config = FileProtocolConfig::new(None, 1024, 5, 1, None, 2048);
    /// let f_protocol = FileProtocol::new("0.0.0.0:8000", "0.0.0.0:7000", config);
    ///
    /// let channel_id = f_protocol.generate_channel();
    /// ```
    ///
    pub fn generate_channel(&self) -> Result<u32, ProtocolError> {
        let mut rng = rand::thread_rng();
        let channel_id: u32 = rng.gen_range(100_000, 999_999);
        Ok(channel_id)
    }

    /// Send a file's metadata information to the remote target
    ///
    /// # Arguments
    ///
    /// * channel_id - Channel ID for transaction
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
    /// let config = FileProtocolConfig::new(None, 1024, 5, 1, None, 2048);
    /// let f_protocol = FileProtocol::new("0.0.0.0:8000", "0.0.0.0:7000", config);
    ///
    /// # ::std::fs::File::create("client.txt").unwrap();
    ///
    /// let (hash, num_chunks, _mode) = f_protocol.initialize_file("client.txt").unwrap();
    /// let channel_id = f_protocol.generate_channel().unwrap();
    /// f_protocol.send_metadata(channel_id, &hash, num_chunks);
    /// ```
    ///
    pub fn send_metadata(
        &self,
        channel_id: u32,
        hash: &str,
        num_chunks: u32,
    ) -> Result<(), ProtocolError> {
        self.send(&messages::metadata(channel_id, hash, num_chunks)?)
    }

    /// Send a request to cleanup the remote storage folder
    pub fn send_cleanup(&self, channel_id: u32, hash: Option<String>) -> Result<(), ProtocolError> {
        self.send(&messages::cleanup(channel_id, hash)?)
    }

    /// Request remote target to receive file from host
    ///
    /// # Arguments
    ///
    /// * channel_id - Channel ID used for transaction
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
    /// ```no_run
    /// use file_protocol::*;
    ///
    /// let config = FileProtocolConfig::new(None, 1024, 5, 1, None, 2048);
    /// let f_protocol = FileProtocol::new("0.0.0.0:8000", "0.0.0.0:7000", config);
    ///
    /// # ::std::fs::File::create("client.txt").unwrap();
    ///
    /// let (hash, _num_chunks, mode) = f_protocol.initialize_file("client.txt").unwrap();
    /// let channel_id = f_protocol.generate_channel().unwrap();
    /// f_protocol.send_export(channel_id, &hash, "final/dir/service.txt", mode);
    /// ```
    ///
    pub fn send_export(
        &self,
        channel_id: u32,
        hash: &str,
        target_path: &str,
        mode: u32,
    ) -> Result<(), ProtocolError> {
        self.send(&messages::export_request(
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
    /// ```no_run
    /// use file_protocol::*;
    ///
    /// let config = FileProtocolConfig::new(None, 1024, 5, 1, None, 2048);
    /// let f_protocol = FileProtocol::new("0.0.0.0:8000", "0.0.0.0:7000", config);
    /// let channel_id = f_protocol.generate_channel().unwrap();
    ///
    /// f_protocol.send_import(channel_id, "service.txt");
    /// ```
    ///
    pub fn send_import(&self, channel_id: u32, source_path: &str) -> Result<(), ProtocolError> {
        self.send(&messages::import_request(channel_id, source_path)?)?;
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
    /// ```no_run
    /// use file_protocol::*;
    ///
    /// let config = FileProtocolConfig::new(None, 1024, 5, 1, None, 2048);
    /// let f_protocol = FileProtocol::new("0.0.0.0:8000", "0.0.0.0:7000", config);
    ///
    /// # ::std::fs::File::create("client.txt").unwrap();
    ///
    /// let (_hash, _num_chunks, _mode) = f_protocol.initialize_file("client.txt").unwrap();
    /// ```
    ///
    pub fn initialize_file(&self, source_path: &str) -> Result<(String, u32, u32), ProtocolError> {
        storage::initialize_file(
            &self.config.storage_prefix,
            source_path,
            self.config.transfer_chunk_size,
            self.config.hash_chunk_size,
        )
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
    ) -> Result<(), ProtocolError> {
        match storage::finalize_file(
            &self.config.storage_prefix,
            hash,
            target_path,
            mode,
            self.config.hash_chunk_size,
        ) {
            Ok(_) => {
                self.send(&messages::operation_success(channel_id, hash)?)?;
                storage::delete_file(&self.config.storage_prefix, hash)?;
                Ok(())
            }
            Err(e) => {
                self.send(&messages::operation_failure(channel_id, &format!("{}", e))?)?;
                Err(e)
            }
        }
    }

    /// Send all requested chunks of a file to the remote destination
    ///
    /// # Arguments
    /// * channel_id - ID of channel to communicate over
    /// * hash - Hash of file corresponding to chunks
    /// * chunks - List of chunk ranges to transmit
    fn send_chunks(
        &self,
        channel_id: u32,
        hash: &str,
        chunks: &[(u32, u32)],
    ) -> Result<(), ProtocolError> {
        let mut chunks_transmitted = 0;
        for (first, last) in chunks {
            for chunk_index in *first..*last {
                match storage::load_chunk(&self.config.storage_prefix, hash, chunk_index) {
                    Ok(c) => self.send(&messages::chunk(channel_id, hash, chunk_index, &c)?)?,
                    Err(e) => {
                        warn!("Failed to load chunk {}:{} : {}", hash, chunk_index, e);
                        storage::delete_file(&self.config.storage_prefix, hash)?;
                        return Err(ProtocolError::CorruptFile(hash.to_string()));
                    }
                };
                if let Some(max_chunks_transmit) = self.config.max_chunks_transmit {
                    chunks_transmitted += 1;
                    if chunks_transmitted >= max_chunks_transmit {
                        return Ok(());
                    }
                }

                thread::sleep(self.config.inter_chunk_delay);
            }
        }
        Ok(())
    }

    /// Listen for and process file protocol messages
    ///
    /// # Arguments
    ///
    /// * pump - Function which returns the next message for processing
    /// * timeout - Maximum time to listen for a single message
    /// * start_state - Current transaction state
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, it will return an error message string
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use file_protocol::*;
    /// use std::time::Duration;
    ///
    /// let config = FileProtocolConfig::new(None, 1024, 5, 1, None, 2048);
    /// let f_protocol = FileProtocol::new("0.0.0.0:8000", "0.0.0.0:7000", config);
    ///
    /// f_protocol.message_engine(
    ///     |d| f_protocol.recv(Some(d)),
    ///     Duration::from_millis(10),
    ///     &State::Transmitting
    /// );
    /// ```
    ///
    pub fn message_engine<F>(
        &self,
        pump: F,
        timeout: Duration,
        start_state: &State,
    ) -> Result<(), ProtocolError>
    where
        F: Fn(Duration) -> Result<Value, ProtocolError>,
    {
        let mut state = start_state.clone();
        loop {
            // Listen on UDP port
            let message = match pump(timeout) {
                Ok(message) => {
                    // If we previously timed out, restore the old state
                    if let State::Holding { prev_state, .. } = state {
                        state = *prev_state;
                    }

                    message
                }
                Err(ProtocolError::ReceiveTimeout) => match state.clone() {
                    State::Receiving {
                        channel_id,
                        hash,
                        path,
                        mode,
                    } => {
                        match storage::validate_file(&self.config.storage_prefix, &hash, None) {
                            Ok((true, _)) => {
                                self.send(&messages::ack(channel_id, &hash, None)?)?;
                                state = State::ReceivingDone {
                                    channel_id,
                                    hash: hash.clone(),
                                    path: path.clone(),
                                    mode,
                                };
                            }
                            Ok((false, chunks)) => {
                                self.send(&messages::nak(channel_id, &hash, &chunks)?)?;
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
                        if count > self.config.hold_count {
                            match prev_state.as_ref() {
                                State::Holding { .. } => return Ok(()),
                                _other => {
                                    return Err(ProtocolError::ReceiveTimeout);
                                }
                            }
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
                Err(e) => return Err(e),
            };

            match self.process_message(message, &state) {
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
    /// use file_protocol::*;
    /// use std::time::Duration;
    ///
    /// let config = FileProtocolConfig::new(None, 1024, 5, 1, None, 2048);
    /// let f_protocol = FileProtocol::new("0.0.0.0:8000", "0.0.0.0:7000", config);
    ///
    /// if let Ok(message) = f_protocol.recv(Some(Duration::from_millis(100))) {
    ///     let _state = f_protocol.process_message(
    ///            message,
    ///            &State::StartReceive {
    ///                path: "target/dir/file.bin".to_owned()
    ///         }
    ///        );
    /// }
    /// ```
    ///
    pub fn process_message(&self, message: Value, state: &State) -> Result<State, ProtocolError> {
        let parsed_message = parsers::parse_message(message)?;
        let new_state = match &parsed_message {
            Message::Sync(channel_id, hash) => {
                info!("<- {{ {}, {} }}", channel_id, hash);
                state.clone()
            }
            Message::Metadata(channel_id, hash, num_chunks) => {
                info!("<- {{ {}, {}, {} }}", channel_id, hash, num_chunks);
                storage::store_meta(&self.config.storage_prefix, hash, *num_chunks)?;
                State::StartReceive {
                    path: hash.to_owned(),
                }
            }
            Message::ReceiveChunk(channel_id, hash, chunk_num, data) => {
                info!(
                    "<- {{ {}, {}, {}, chunk_data }}",
                    channel_id, hash, chunk_num
                );
                storage::store_chunk(&self.config.storage_prefix, hash, *chunk_num, data)?;
                state.clone()
            }
            Message::ACK(_channel_id, ack_hash) => {
                info!("<- {{ {}, true }}", ack_hash);
                // TODO: Figure out hash verification here
                State::TransmittingDone
            }
            Message::NAK(channel_id, hash, Some(missing_chunks)) => {
                info!(
                    "<- {{ {}, {}, false, {:?} }}",
                    channel_id, hash, missing_chunks
                );
                match self.send_chunks(*channel_id, hash, missing_chunks) {
                    Ok(()) => {}
                    Err(error) => self.send(&messages::operation_failure(
                        *channel_id,
                        &format!("{}", error),
                    )?)?,
                };
                State::Transmitting
            }
            Message::NAK(channel_id, hash, None) => {
                info!("<- {{ {}, {}, false }}", channel_id, hash);
                // TODO: Maybe trigger a failure?
                state.clone()
            }
            Message::ReqReceive(channel_id, hash, path, mode) => {
                info!(
                    "<- {{ {}, export, {}, {}, {:?} }}",
                    channel_id, hash, path, mode
                );
                // The client wants to send us a file.
                // See what state the file is currently in on our side
                match storage::validate_file(&self.config.storage_prefix, hash, None) {
                    Ok((true, _)) => {
                        // We've already got all the file data in temporary storage
                        self.send(&messages::ack(*channel_id, hash, None)?)?;

                        State::ReceivingDone {
                            channel_id: *channel_id,
                            hash: hash.to_string(),
                            path: path.to_string(),
                            mode: *mode,
                        }
                    }
                    Ok((false, chunks)) => {
                        // We're missing some number of data chunks of the requrested file
                        self.send(&messages::nak(*channel_id, hash, &chunks)?)?;
                        State::Receiving {
                            channel_id: *channel_id,
                            hash: hash.to_string(),
                            path: path.to_string(),
                            mode: *mode,
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            Message::ReqTransmit(channel_id, path) => {
                info!("<- {{ {}, import, {} }}", channel_id, path);
                // Set up the requested file for transmission
                match self.initialize_file(path) {
                    Ok((hash, num_chunks, mode)) => {
                        // It worked, let the requester know we're ready to send
                        self.send(&messages::import_setup_success(
                            *channel_id,
                            &hash,
                            num_chunks,
                            mode,
                        )?)?;

                        State::Transmitting
                    }
                    Err(error) => {
                        // It failed. Let the requester know that we can't transmit
                        // the file they want.
                        self.send(&messages::operation_failure(
                            *channel_id,
                            &format!("{}", error),
                        )?)?;

                        State::Done
                    }
                }
            }
            Message::SuccessReceive(channel_id, hash) => {
                info!("<- {{ {}, true }}", channel_id);
                storage::delete_file(&self.config.storage_prefix, hash)?;
                State::Done
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
                match storage::validate_file(&self.config.storage_prefix, hash, Some(*num_chunks)) {
                    Ok((true, _)) => {
                        self.send(&messages::ack(*channel_id, hash, Some(*num_chunks))?)?;
                        match state.clone() {
                            State::StartReceive { path } => State::ReceivingDone {
                                channel_id: *channel_id,
                                hash: hash.to_string(),
                                path,
                                mode: *mode,
                            },
                            _ => State::Done,
                        }
                    }
                    Ok((false, chunks)) => {
                        self.send(&messages::nak(*channel_id, hash, &chunks)?)?;
                        match state.clone() {
                            State::StartReceive { path } => State::Receiving {
                                channel_id: *channel_id,
                                hash: hash.to_string(),
                                path,
                                mode: *mode,
                            },
                            _ => state.clone(),
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            Message::Failure(channel_id, error_message) => {
                info!("<- {{ {}, false, {} }}", channel_id, error_message);
                return Err(ProtocolError::TransmissionError {
                    channel_id: *channel_id,
                    error_message: error_message.to_string(),
                });
            }
            Message::Cleanup(channel_id, Some(hash)) => {
                info!("<- {{ {}, cleanup, {} }}", channel_id, hash);
                storage::delete_file(&self.config.storage_prefix, hash)?;
                State::Done
            }
            Message::Cleanup(channel_id, None) => {
                info!("< {{ {}, cleanup }}", channel_id);
                storage::delete_storage(&self.config.storage_prefix)?;
                State::Done
            }
        };
        Ok(new_state)
    }
}
