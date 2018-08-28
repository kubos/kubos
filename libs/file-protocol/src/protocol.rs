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

use super::messages;
use super::parsers;
use super::storage;
use super::Message;
use cbor_protocol::Protocol as CborProtocol;
use serde_cbor::{ser, Value};
use std::cell::Cell;
use std::net::UdpSocket;
use std::str;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Eq, PartialEq)]
pub enum Role {
    Client,
    Server,
}

pub struct Protocol {
    cbor_proto: CborProtocol,
    host: String,
    dest_port: Cell<u16>,
    role: Role,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum State {
    StartReceive(String),
    Receiving(u64, String, String, Option<u32>),
    ReceivingDone,
    Transmitting,
    TransmittingDone,
    Holding,
    Done,
}

impl Protocol {
    pub fn new(host: String, dest_port: u16, role: Role) -> Self {
        // Get a local UDP socket (Bind)
        let c_protocol = CborProtocol::new(format!("{}:0", host));

        // Set up the full connection info
        Protocol {
            cbor_proto: c_protocol,
            // Remote IP?
            host,
            dest_port: Cell::new(dest_port),
            role,
        }
    }

    pub fn new_from_socket(socket: UdpSocket, host: String, dest_port: u16, role: Role) -> Self {
        Protocol {
            cbor_proto: CborProtocol::new_from_socket(socket),
            host,
            dest_port: Cell::new(dest_port),
            role,
        }
    }

    pub fn send(&self, vec: Vec<u8>) -> Result<(), String> {
        self.cbor_proto
            .send_message(&vec, &self.host, self.dest_port.get())
            .unwrap();
        Ok(())
    }

    pub fn recv(&self, timeout: Option<Duration>) -> Result<Option<Value>, Option<String>> {
        self.cbor_proto.recv_message_timeout(Duration::from_secs(1))
    }

    // Request remote target to receive file from host
    pub fn send_export(&self, hash: &str, target_path: &str, mode: u32) -> Result<(), String> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .and_then(|duration| {
                Ok(duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1000000)
            })
            .map_err(|err| format!("Failed to get current system time: {}", err))?;
        let channel_id: u32 = (time % 100000) as u32;

        self.send(messages::export(channel_id, hash, target_path, mode).unwrap())
            .unwrap();

        Ok(())
    }

    // Request a file from a remote target
    pub fn send_import(&self, source_path: &str) -> Result<(), String> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .and_then(|duration| {
                Ok(duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1000000)
            })
            .map_err(|err| format!("Failed to get current system time: {}", err))?;
        let channel_id: u32 = (time % 100000) as u32;

        self.send(messages::import(channel_id, source_path).unwrap())
            .unwrap();
        Ok(())
    }

    // // Figure out if/what chunks are missing and send the hash and info back to the remote addr
    // // Q: This copies ACK/NAK. Should it replace them? Or use them?
    // pub fn sync_and_send(&self, hash: &str, num_chunks: Option<u32>) -> Result<(), String> {
    //     // TODO: Create some way to break out of this loop if we never receive all the chunks
    //     loop {
    //         let (result, _chunks) = storage::local_sync(hash, num_chunks)?;

    //         self.send(messages::ack_or_nak(hash, num_chunks).unwrap())
    //             .unwrap();

    //         if result == true {
    //             // We've received all the chunks we were expecting. Time to go home.
    //             break;
    //         }

    //         // Try to receive the missing chunks
    //         loop {
    //             // Listen on UDP port
    //             // TODO: Make timeout a config option
    //             // TODO: Make timeout 'receive chunk' message-specific
    //             match self.cbor_proto.recv_message_timeout(Duration::from_secs(1)) {
    //                 // Parse the received message
    //                 Ok(Some(message)) => match self.on_message(message, State::Holding, Some(hash))
    //                 {
    //                     Ok(_) => { /* TODO: Verify that we got a ReceiveChunk message? */ }
    //                     Err(err) => eprintln!("Failed to parse message: {}", err),
    //                 },
    //                 Ok(None) => { /* TODO: Handle pause or resume messages? */ }
    //                 Err(None) => {
    //                     // We timed out of receiving a new chunk. Let's go see if we got everything
    //                     break;
    //                 }
    //                 Err(Some(err)) => {
    //                     // Something went wrong while we were receiving
    //                     // Let's quit while we're ahead
    //                     return Err(err);
    //                 }
    //             }
    //         }
    //     }

    //     Ok(())
    // }

    pub fn local_export(
        &self,
        hash: &str,
        target_path: &str,
        mode: Option<u32>,
    ) -> Result<(), String> {
        storage::local_export(hash, target_path, mode)
    }

    pub fn store_meta(&self, hash: &str, num_chunks: u32) -> Result<(), String> {
        storage::store_meta(hash, num_chunks)
    }

    // Request a download to start
    fn start_push(&self, _hash: &str, _chunks: Option<Vec<u32>>) -> Result<(), String> {
        unimplemented!();
    }

    // Request a download to stop
    fn stop_push(&self, _hash: &str) -> Result<(), String> {
        unimplemented!();
    }

    // This is the guts of a coroutine which appears to have been
    // spawned when the module file-protocol.lua is loaded...
    pub fn do_upload(&self, hash: &str, chunks: &[(u32, u32)]) -> Result<(), String> {
        for (first, last) in chunks {
            for chunk_index in *first..*last {
                let chunk = storage::load_chunk(hash, chunk_index).unwrap();
                self.send(messages::chunk(hash, chunk_index, &chunk).unwrap())
                    .unwrap();
            }
        }
        Ok(())
    }

    pub fn send_success(&self, channel_id: u64) -> Result<(), String> {
        info!("-> {{ {}, true }}", channel_id);
        let vec = ser::to_vec_packed(&(channel_id, true)).unwrap();
        self.cbor_proto
            .send_message(&vec, &self.host, self.dest_port.get())
            .unwrap();
        Ok(())
    }

    pub fn send_failure(&self, channel_id: u64, error: &str) -> Result<(), String> {
        info!("-> {{ {}, false, {} }}", channel_id, error);
        let vec = ser::to_vec_packed(&(channel_id, false, error)).unwrap();
        self.cbor_proto
            .send_message(&vec, &self.host, self.dest_port.get())
            .unwrap();
        Ok(())
    }

    pub fn message_engine(
        &self,
        hash: Option<&str>,
        timeout: Duration,
        start_state: State,
        pump: bool,
    ) -> Result<Option<Message>, String> {
        let mut last_message: Option<Message> = None;
        let mut state = start_state.clone();
        loop {
            // Listen on UDP port
            let message = match self.cbor_proto.recv_message_peer_timeout(timeout) {
                Ok((peer, Some(message))) => {
                    // Update our response port
                    self.dest_port.set(peer.port());
                    message
                }
                _ => {
                    match state.clone() {
                        State::Receiving(channel_id, hash, path, mode) => {
                            match self.local_export(&hash, &path, mode) {
                                Ok(_) => {
                                    self.send_success(channel_id).unwrap();
                                    break;
                                }
                                Err(e) => {
                                    self.send_failure(channel_id, &e).unwrap();
                                    continue;
                                }
                            }
                        }
                        _ => continue,
                    }
                }
            };
            
            let (new_message, new_state) = self.on_message(message, &state.clone(), hash)?;
            state = new_state;
            match state {
                State::Done => break,
                _ => continue,
            }

            if !pump {
                break;
            }
        }
        Ok(last_message)
    }

    pub fn on_message(
        &self,
        message: Value,
        state: &State,
        hash: Option<&str>,
    ) -> Result<(Option<Message>, State), String> {
        let parsed_message = parsers::parse_message(message);
        let mut new_state = State::Holding;
        match parsed_message.to_owned() {
            Ok(parsed_message) => {
                match &parsed_message {
                    Message::Sync(hash) => {
                        info!("<- {{ {} }}", hash);
                        new_state = state.clone();
                    }
                    Message::SyncChunks(hash, num_chunks) => {
                        info!("<- {{ {}, {} }}", hash, num_chunks);
                        storage::store_meta(&hash, *num_chunks).unwrap();
                        new_state = state.clone();
                    }
                    Message::ReceiveChunk(hash, chunk_num, data) => {
                        info!("<- {{ {}, {}, chunk_data }}", hash, chunk_num);
                        storage::store_chunk(&hash, *chunk_num, &data).unwrap();
                        new_state = state.clone();
                    }
                    Message::ACK(ack_hash) => {
                        info!("<- {{ {}, true }}", ack_hash);
                        // TODO: Figure out hash verification here
                        new_state = State::Done;
                    }
                    Message::NAK(hash, Some(missing_chunks)) => {
                        info!("<- {{ {}, false, {:?} }}", hash, missing_chunks);
                        self.do_upload(&hash, &missing_chunks)?;
                        new_state = State::Transmitting;
                    }
                    Message::NAK(hash, None) => {
                        info!("<- {{ {}, false }}", hash);
                        new_state = state.clone();
                    }
                    Message::ReqReceive(channel_id, hash, path, Some(mode)) => {
                        info!(
                            "<- {{ {}, export, {}, {}, {} }}",
                            channel_id, hash, path, mode
                        );
                        // The client wants to send us a file.
                        // Go listen for the chunks.
                        // Note: Won't return until we've received all of them.
                        // (so could potentially never return)
                        // TODO: handle channel_id mismatch
                        self.send(messages::ack_or_nak(&hash, None).unwrap())
                            .unwrap();
                        new_state = State::Receiving(
                            *channel_id,
                            hash.to_string(),
                            path.to_string(),
                            Some(*mode),
                        );
                    }
                    Message::ReqReceive(channel_id, hash, path, None) => {
                        info!("<- {{ {}, export, {}, {} }}", channel_id, hash, path);
                        new_state =
                            State::Receiving(*channel_id, hash.to_string(), path.to_string(), None);
                    }
                    Message::ReqTransmit(channel_id, path) => {
                        info!("<- {{ {}, import, {} }}", channel_id, path);
                        self.send(messages::local_import(*channel_id, &path).unwrap())
                            .unwrap();
                        new_state = State::Transmitting;
                    }
                    Message::SuccessReceive(channel_id) => {
                        info!("<- {{ {}, true }}", channel_id);
                        new_state = state.clone();
                    }
                    Message::SuccessTransmit(channel_id, hash, num_chunks, Some(mode)) => {
                        info!(
                            "<- {{ {}, true, {}, {}, {} }}",
                            channel_id, hash, num_chunks, mode
                        );
                        // TODO: handle channel_id mismatch
                        self.send(messages::ack_or_nak(&hash, Some(*num_chunks)).unwrap())
                            .unwrap();
                        // new_state = State::Receiving;
                        match state {
                            State::StartReceive(path) => {
                                new_state = State::Receiving(*channel_id, hash.to_string(), path.to_string(), Some(*mode));
                            },
                            _ => {
                                new_state = state.clone();
                            }
                        }
                    }
                    Message::SuccessTransmit(channel_id, hash, num_chunks, None) => {
                        info!("<- {{ {}, true, {}, {} }}", channel_id, hash, num_chunks);
                        // TODO: handle channel_id mismatch
                        self.send(messages::ack_or_nak(&hash, Some(*num_chunks)).unwrap())
                            .unwrap();
                        new_state = state.clone();
                    }
                    Message::Failure(channel_id, error_message) => {
                        info!("<- {{ {}, false, {} }}", channel_id, error_message);
                        new_state = state.clone();
                        return Err(format!(
                            "Transmission failure on channel {}. Error returned from server: {}",
                            channel_id, error_message
                        ));
                    }
                }
                Ok((Some(parsed_message), new_state))
            }
            Err(e) => {
                info!("<- what did we get?? {}", e);
                match state {
                    State::Receiving(channel_id, hash, path, mode) => {
                        match self.local_export(&hash, &path, *mode) {
                            Ok(_) => {
                                self.send_success(*channel_id).unwrap();
                                Ok((None, State::Done))
                            }
                            Err(e) => {
                                self.send_failure(*channel_id, &e).unwrap();
                                Ok((
                                    None,
                                    State::Receiving(
                                        *channel_id,
                                        hash.to_string(),
                                        path.to_string(),
                                        *mode,
                                    ),
                                ))
                            }
                        }
                    }
                    _ => Ok((None, State::Holding)),
                }
            }
        }
    }
}
