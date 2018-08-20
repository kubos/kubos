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

mod storage;

use blake2_rfc::blake2s::Blake2s;
use cbor_codec::Protocol as CborProtocol;
use serde::Serializer;
use serde_cbor::{de, ser, to_vec, Value};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::path::Path;
use std::str;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use time;

const CHUNK_SIZE: usize = 4096;

pub enum Message {
    Sync(String),
    SyncChunks(String, u32),
    ReceiveChunk(String),
    ACK(String),
    NAK(String, Option<Vec<(u32, u32)>>),
    ReqReceive(u64),
    ReqTransmit(u64),
    SuccessReceive(u64),
    SuccessTransmit(u64, String, u32, Option<u32>),
    Failure(u64, String),
}

pub struct Protocol {
    cbor_proto: CborProtocol,
    host: String,
    dest_port: u16,
}

impl Protocol {
    pub fn new(host: String, dest_port: u16) -> Self {
        // Get a local UDP socket (Bind)
        let c_protocol = CborProtocol::new(0);

        // Set up the full connection info
        Protocol {
            cbor_proto: c_protocol,
            // Remote IP?
            host,
            dest_port,
        }
    }

    // We already have a CBOR connection, we just need to setup the file system stuff
    pub fn new_listener(c_protocol: CborProtocol, destination: SocketAddr) -> Self {
        Protocol {
            cbor_proto: c_protocol,
            // Remote IP?
            host: format!("{}", destination.ip()).to_owned(),
            dest_port: destination.port(),
        }
    }

    pub fn send_sync(&self, hash: &str, num_chunks: u32) -> Result<(), String> {
        println!("-> {{ {}, {} }}", hash, num_chunks);
        let vec = ser::to_vec_packed(&(hash, num_chunks)).unwrap();
        self.cbor_proto
            .send_message(&vec, &self.host, self.dest_port)
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

        println!(
            "-> {{ {}, export, {}, {}, {} }}",
            channel_id, hash, target_path, mode
        );

        let vec = ser::to_vec_packed(&(channel_id, "export", hash, target_path, mode)).unwrap();

        self.cbor_proto
            .send_message(&vec, &self.host, self.dest_port)
            .unwrap();

        // Listen on UDP port
        let message = self.cbor_proto
            .recv_message()?
            .ok_or(format!("Failed to receive op result"))?;

        // if let Some((channel_id, result, data)) = self.on_message(message)? {
        //     println!("Got {} {} {:?}", channel_id, result, data);
        // }

        match self.on_message(message)? {
            Message::NAK(hash, chunks) => {
                if let Some(c) = chunks {
                    self.do_upload(&hash, &c)?;
                }
            }
            _ => {
                println!("Got other msg");
            }
        }

        //TODO: Send the actual file
        Ok(())
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

        println!("-> {{ import, {} }}", source_path);
        let vec = ser::to_vec_packed(&(channel_id, "import", source_path)).unwrap();

        self.cbor_proto
            .send_message(&vec, &self.host, self.dest_port)
            .unwrap();

        // Listen on UDP port
        let message = self.cbor_proto
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
                    "Failed to request file. Error returned from service: {}",
                    error
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
            let (mut result, mut chunks) = storage::local_sync(hash, num_chunks)?;

            println!("-> {{ {}, {:?}, {:?} }}", hash, result, chunks);
            let mut vec = ser::to_vec_packed(&(hash, result)).unwrap();
            for chunk in chunks.iter() {
                // Add the chunk number to the end of the CBOR array
                vec.append(&mut ser::to_vec_packed(&chunk).unwrap());
                // Update length of CBOR array
                vec[0] += 1;
            }

            self.cbor_proto
                .send_message(&vec, &self.host, self.dest_port)
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

    /// Create temporary folder for chunks
    /// Stream copy file from mutable space to immutable space
    /// Move folder to hash of contents
    pub fn local_import(&self, source_path: &str) -> Result<(String, u32, u32), String> {
        storage::local_import(source_path)
    }

    // Save file chunks into the requested permanent file location
    pub fn local_export(
        &self,
        hash: &str,
        target_path: &str,
        mode: Option<u32>,
    ) -> Result<(), String> {
        storage::local_export(hash, target_path, mode)
    }

    // Request a download to start
    fn start_push(&self, hash: &str, chunks: Option<Vec<u32>>) -> Result<(), String> {
        unimplemented!();
    }

    // Request a download to stop
    fn stop_push(&self, hash: &str) -> Result<(), String> {
        println!("Stop push yo");
        Ok(())
    }

    // Send a single file chunk to the remote address
    fn send_chunk(&self, hash: &str, index: u32, chunk: &[u8]) -> Result<(), String> {
        let chunk_bytes = Value::Bytes(chunk.to_vec());
        println!("-> {{ {}, {}, {:?}", hash, index, chunk_bytes);
        let mut vec = ser::to_vec_packed(&(hash, index, chunk_bytes)).unwrap();

        self.cbor_proto
            .send_message(&vec, &self.host, self.dest_port)
            .unwrap();
        Ok(())
    }

    // This is the guts of a coroutine which appears to have been
    // spawned when the module file-protocol.lua is loaded...
    fn do_upload(&self, hash: &str, chunks: &[(u32, u32)]) -> Result<(), String> {
        let first = 0;
        let chunk = vec![0];

        for (first, last) in chunks {
            for chunk_index in *first..*last {
                let chunk = storage::load_chunk(hash, chunk_index).unwrap();
                self.send_chunk(hash, chunk_index, &chunk);
            }
        }

        Ok(())
    }

    // Send an acknowledge to the remote address
    fn send_ack(&self, hash: &str, num_chunks: u32) -> Result<(), String> {
        println!("-> {{ {}, true, {} }}", hash, num_chunks);
        let vec = ser::to_vec_packed(&(hash, true, num_chunks)).unwrap();

        self.cbor_proto
            .send_message(&vec, &self.host, self.dest_port)
            .unwrap();
        Ok(())
    }

    // Send a NAK to the remote address
    // TODO: should include missing chunks
    fn send_nak(&self, hash: &str) -> Result<(), String> {
        println!("-> {{ {}, false}}", hash);
        let vec = ser::to_vec_packed(&(hash, false)).unwrap();

        self.cbor_proto
            .send_message(&vec, &self.host, self.dest_port)
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

                                // TODO: The actual logic for an export request

                                match storage::local_export(hash, path, mode) {
                                    Ok(results) => {
                                        // TODO: Results might need to be unpacked from tuple
                                        println!("-> {{ {}, true, {:?} }}", channel_id, results);
                                        let vec = ser::to_vec_packed(&(channel_id, true, results))
                                            .unwrap();

                                        self.cbor_proto
                                            .send_message(&vec, &self.host, self.dest_port)
                                            .unwrap();
                                    }
                                    Err(error) => {
                                        println!("-> {{ {}, false, {} }}", channel_id, error);
                                        let vec = ser::to_vec_packed(&(channel_id, false, error))
                                            .unwrap();

                                        self.cbor_proto
                                            .send_message(&vec, &self.host, self.dest_port)
                                            .unwrap();
                                    }
                                }

                                return Ok(Message::ReqReceive(channel_id));
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

                                match storage::local_import(path) {
                                    Ok(results) => {
                                        // TODO: Results might need to be unpacked from tuple
                                        println!("-> {{ {}, true, {:?} }}", channel_id, results);
                                        let vec = ser::to_vec_packed(&(channel_id, true, results))
                                            .unwrap();

                                        self.cbor_proto
                                            .send_message(&vec, &self.host, self.dest_port)
                                            .unwrap();
                                    }
                                    Err(error) => {
                                        println!("-> {{ {}, false, {} }}", channel_id, error);
                                        let vec = ser::to_vec_packed(&(channel_id, false, error))
                                            .unwrap();

                                        self.cbor_proto
                                            .send_message(&vec, &self.host, self.dest_port)
                                            .unwrap();
                                    }
                                }

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
                            self.stop_push(&hash)?;

                            //TODO: Do something with the third param? (num_chunks)
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

                            // if remaining_chunks.len() > 0 {
                            //     self.start_push(&hash, Some(remaining_chunks))?
                            // } else {
                            //     return Err(format!(
                            //         "Unable to parse NAK message: Missing missing chunks"
                            //     ));
                            // }

                            return Ok(Message::NAK(hash, Some(remaining_chunks)));
                        }
                        Value::U64(num) => {
                            if let Some(third_param) = pieces.next() {
                                if let Value::Bytes(data) = third_param {
                                    // It's a data chunk message: { hash, chunk_index, data }
                                    // Store the new chunk
                                    storage::store_chunk(&hash, *num as u32, data);

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
