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
use super::storage;
use super::Message;
use cbor_protocol::Protocol as CborProtocol;
use serde_cbor::{ser, Value};
use std::cell::Cell;
use std::str;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Protocol {
    cbor_proto: CborProtocol,
    host: String,
    dest_port: Cell<u16>,
}

impl Protocol {
    pub fn new(host: String, dest_port: u16) -> Self {
        // Get a local UDP socket (Bind)
        let c_protocol = CborProtocol::new(format!("{}:0", host));

        // Set up the full connection info
        Protocol {
            cbor_proto: c_protocol,
            // Remote IP?
            host,
            dest_port: Cell::new(dest_port),
        }
    }

    /*
    // We already have a CBOR connection, we just need to setup the file system stuff
    pub fn new_listener(c_protocol: CborProtocol, destination: SocketAddr) -> Self {
        Protocol {
            cbor_proto: c_protocol,
            // Remote IP?
            host: format!("{}", destination.ip()).to_owned(),
            dest_port: destination.port(),
        }
    }
    */

    pub fn send(&self, vec: Vec<u8>) -> Result<(), String> {
        self.cbor_proto
            .send_message(&vec, &self.host, self.dest_port.get())
            .unwrap();
        Ok(())
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

        loop {
            // Listen on UDP port
            let (peer, message) = match self.cbor_proto.recv_message_peer()? {
                (peer, Some(data)) => (peer, data),
                _ => return Err("Failed to receive op result".to_owned()),
            };
            // Update our response port
            self.dest_port.set(peer.port());

            match self.on_message(message)? {
                Message::NAK(hash, chunks) => {
                    if let Some(c) = chunks {
                        self.do_upload(&hash, &c)?;
                    }
                }
                Message::ACK(ack_hash) => {
                    if ack_hash == hash {
                        return Ok(());
                    }
                }
                Message::Failure(channel, error) => {
                    return Err(format!("Transfer {} failed: {}", channel, error));
                }
                _m => {
                    return Err(format!("Unexpected message found {:?}", _m));
                }
            }
        }
    }

    // Request a file from a remote target
    pub fn send_import(&self, source_path: &str) -> Result<(String, u32, Option<u32>), String> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .and_then(|duration| {
                Ok(duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1000000)
            })
            .map_err(|err| format!("Failed to get current system time: {}", err))?;
        let channel_id: u32 = (time % 100000) as u32;

        self.send(messages::import(channel_id, source_path).unwrap())
            .unwrap();

        // Listen on UDP port
        let message = self
            .cbor_proto
            .recv_message()?
            .ok_or(format!("Failed to receive op result"))?;

        // Parse the received message
        match self.on_message(message)? {
            Message::SuccessTransmit(id, hash, num_chunks, mode) => {
                if (id as u32) == channel_id {
                    Ok((hash, num_chunks, mode))
                } else {
                    return Err("Channel ID mismatch".to_owned());
                }
            }
            Message::Failure(id, error) => {
                return Err(format!(
                    "Failed to request file {}. Error returned from service: {}",
                    id, error
                ))
            }
            _ => return Err("Received unexpected response to import request".to_owned()),
        }
    }

    // Figure out if/what chunks are missing and send the hash and info back to the remote addr
    // Q: This copies ACK/NAK. Should it replace them? Or use them?
    pub fn sync_and_send(&self, hash: &str, num_chunks: Option<u32>) -> Result<(), String> {
        // TODO: Create some way to break out of this loop if we never receive all the chunks
        loop {
            let (result, _chunks) = storage::local_sync(hash, num_chunks)?;

            self.send(messages::ack_or_nak(hash, num_chunks).unwrap())
                .unwrap();

            if result == true {
                // We've received all the chunks we were expecting. Time to go home.
                break;
            }

            // Try to receive the missing chunks
            loop {
                // Listen on UDP port
                // TODO: Make timeout a config option
                // TODO: Make timeout 'receive chunk' message-specific
                match self.cbor_proto.recv_message_timeout(Duration::from_secs(1)) {
                    // Parse the received message
                    Ok(Some(message)) => match self.on_message(message) {
                        Ok(_) => { /* TODO: Verify that we got a ReceiveChunk message? */ }
                        Err(err) => eprintln!("Failed to parse message: {}", err),
                    },
                    Ok(None) => { /* TODO: Handle pause or resume messages? */ }
                    Err(None) => {
                        // We timed out of receiving a new chunk. Let's go see if we got everything
                        break;
                    }
                    Err(Some(err)) => {
                        // Something went wrong while we were receiving
                        // Let's quit while we're ahead
                        return Err(err);
                    }
                }
            }
        }

        Ok(())
    }

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
        println!("-> {{ {}, true }}", channel_id);
        let vec = ser::to_vec_packed(&(channel_id, true)).unwrap();
        self.cbor_proto
            .send_message(&vec, &self.host, self.dest_port.get())
            .unwrap();
        Ok(())
    }

    pub fn send_failure(&self, channel_id: u64, error: &str) -> Result<(), String> {
        println!("-> {{ {}, false, {} }}", channel_id, error);
        let vec = ser::to_vec_packed(&(channel_id, false, error)).unwrap();
        self.cbor_proto
            .send_message(&vec, &self.host, self.dest_port.get())
            .unwrap();
        Ok(())
    }

    // Received message handler/parser
    pub fn on_message(&self, message: Value) -> Result<Message, String> {
        let data = match message {
            Value::Array(val) => val.to_owned(),
            _ => return Err("Unable to parse message: Data not an array".to_owned()),
        };
        let mut pieces = data.iter();

        let first_param: Value = pieces
            .next()
            .ok_or(format!("Unable to parse message: No contents"))?
            .to_owned();

        match first_param {
            // It's a channel ID
            Value::U64(channel_id) => {
                match pieces.next().ok_or(format!(
                    "Unable to parse message: No param after channel ID"
                ))? {
                    Value::String(operation) => {
                        match operation.as_ref() {
                            "export" => {
                                // It's an export request: { channel_id, "export", hash, path [, mode] }

                                let hash =
                                    match pieces.next().ok_or(format!(
                                        "Unable to parse export message: No hash param"
                                    ))? {
                                        Value::String(val) => val,
                                        _ => return Err(
                                            "Unable to parse export message: Invalid hash param"
                                                .to_owned(),
                                        ),
                                    };

                                let path =
                                    match pieces.next().ok_or(format!(
                                        "Unable to parse export message: No path param"
                                    ))? {
                                        Value::String(val) => val,
                                        _ => return Err(
                                            "Unable to parse export message: Invalid path param"
                                                .to_owned(),
                                        ),
                                    };

                                let mode = match pieces.next() {
                                    Some(Value::U64(num)) => Some(*num as u32),
                                    _ => None,
                                };

                                return Ok(Message::ReqReceive(
                                    channel_id,
                                    hash.to_owned(),
                                    path.to_owned(),
                                    mode,
                                ));
                            }
                            "import" => {
                                // It's an import request: { channel_id, "import", path }
                                let path =
                                    match pieces.next().ok_or(format!(
                                        "Unable to parse import message: No path param"
                                    ))? {
                                        Value::String(val) => val,
                                        _ => return Err(
                                            "Unable to parse import message: Invalid path param"
                                                .to_owned(),
                                        ),
                                    };

                                // TODO: Actual logic for an import request
                                self.send(messages::local_import(channel_id, path).unwrap())
                                    .unwrap();

                                return Ok(Message::ReqTransmit(channel_id));
                            }
                            _ => return Err(format!("Unable to parse message: Unknown operation")),
                        }
                    }
                    Value::Bool(result) => {
                        // It's an import/export op result
                        // Good - { channel_id, true, ...values }

                        match result {
                            true => {
                                // Good - { channel_id, true, ...values }
                                if let Some(piece) = pieces.next() {
                                    // It's a good result after an 'import' operation
                                    let hash = match piece {
                                        Value::String(val) => val,
                                        _ => return Err(
                                            "Unable to parse success message: Invalid hash param"
                                                .to_owned(),
                                        ),
                                    };

                                    let num_chunks = match pieces.next().ok_or(format!(
                                        "Unable to parse success message: No num_chunks param"
                                    ))? {
                                        Value::U64(val) => *val,
                                        _ => return Err(
                                            "Unable to parse success message: Invalid num_chunks param"
                                                .to_owned(),
                                        ),
                                    };

                                    let mode = match pieces.next() {
                                        Some(Value::U64(val)) => Some(*val as u32),
                                        _ => None,
                                    };

                                    // Return the file info
                                    return Ok(Message::SuccessTransmit(
                                        channel_id,
                                        hash.to_string(),
                                        num_chunks as u32,
                                        mode,
                                    ));
                                } else {
                                    // It's a good result after an 'export' operation
                                    return Ok(Message::SuccessReceive(channel_id));
                                }
                            }
                            false => {
                                // Bad - { channel_id, false, error_message}
                                let error =
                                    match pieces.next().ok_or(format!(
                                        "Unable to parse failure message: No error param"
                                    ))? {
                                        Value::String(val) => val,
                                        _ => return Err(
                                            "Unable to parse failure message: Invalid error param"
                                                .to_owned(),
                                        ),
                                    };

                                return Ok(Message::Failure(channel_id, error.to_owned()));
                            }
                        }
                    }
                    _ => {
                        return Err(format!(
                            "Unable to parse message: Unknown param after channel ID"
                        ))
                    }
                }
            }
            // It's a hash value
            Value::String(hash) => {
                if let Some(second_param) = pieces.next() {
                    match second_param {
                        Value::Bool(true) => {
                            // It's an ACK: { hash, true, num_chunks }
                            // Our data transfer (export) completed succesfully
                            // self.stop_push(&hash)?;

                            //TODO: Do something with the third param? (num_chunks)
                            // Doesn't look like we do anything with num_chunks
                            return Ok(Message::ACK(hash));
                        }
                        Value::Bool(false) => {
                            // It's a NAK: { hash, false, 1, 4, 6, 7 }
                            // Some number of chunks were not received by the remote addr

                            let mut remaining_chunks: Vec<(u32, u32)> = vec![];
                            let mut chunk_nums: Vec<u32> = vec![];
                            for entry in pieces {
                                if let Value::U64(chunk_num) = entry {
                                    chunk_nums.push(*chunk_num as u32);
                                }
                            }

                            for chunk in chunk_nums.chunks(2) {
                                let first = chunk[0];
                                let last = chunk[1];
                                remaining_chunks.push((first, last));
                            }

                            return Ok(Message::NAK(hash, Some(remaining_chunks)));
                        }
                        Value::U64(num) => {
                            if let Some(third_param) = pieces.next() {
                                if let Value::Bytes(data) = third_param {
                                    // It's a data chunk message: { hash, chunk_index, data }
                                    // Store the new chunk
                                    storage::store_chunk(&hash, *num as u32, data).unwrap();

                                    return Ok(Message::ReceiveChunk(hash));
                                } else {
                                    return Err(format!(
                                        "Unable to parse chunk message: Invalid data format"
                                    ));
                                }
                            } else {
                                // It's a sync message: { hash, num_chunks }
                                // TODO: Whoever processes this message should do the sync_and_send
                                //self.sync_and_send(&hash, Some(*num as u32));
                                return Ok(Message::SyncChunks(hash, *num as u32));
                            }
                        }
                        _ => {
                            return Err(format!("Unable to parse message: Unknown param after hash"))
                        }
                    }
                } else {
                    // It's a sync message: { hash }
                    // TODO: Whoever processes this message should do the sync_and_send
                    //self.sync_and_send(&hash, None)?;
                    return Ok(Message::Sync(hash));
                }
            }
            _ => return Err(format!("Unable to parse message: Unknown first param type")),
        }
    }
}
