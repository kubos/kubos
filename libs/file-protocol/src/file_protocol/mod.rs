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
use serde_cbor::{ser, to_vec, Value};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use time;
//use std::sync::mpsc::{channel, Receiver, Sender};
//use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

const CHUNK_SIZE: usize = 4096;

pub struct Protocol {
    cbor_proto: CborProtocol,
    host: String,
    dest_port: u16,
    //result_receiver: Receiver<(u32, String, <str as Trait>::Split)>,
}

impl Protocol {
    pub fn new(host: String, dest_port: u16) -> Self {
        // Get a local UDP socket (Bind)
        let c_protocol = CborProtocol::new(0);

        //let (result_sender, result_receiver) = channel();

        // Spawn the thread which will listen for messages from the server
        //thread::spawn(move || listen_thread(result_sender));

        // Set up the full connection info
        Protocol {
            cbor_proto: c_protocol,
            // Remote IP?
            host,
            dest_port,
            //result_receiver,
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
    pub fn send_export(&self, hash: &str, target_path: &str, mode: u16) -> Result<(), String> {
        println!("-> {{ export, {}, {}, {} }}", hash, target_path, mode);
        let vec = ser::to_vec_packed(&("export", hash, target_path, mode)).unwrap();

        self.cbor_proto
            .send_message(&vec, &self.host, self.dest_port)
            .unwrap();

        //TODO: Send the actual file
        Ok(())
    }

    // Request a file from a remote target
    pub fn send_import(&self, source_path: &str) -> Result<(String, u32, Option<u16>), String> {
        //local channel_id = (os.time() + uv.hrtime()) % 0x100000
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .and_then(
                |duration| Ok(duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1000000),
            )
            .map_err(|err| format!("Failed to get current system time: {}", err))?;
        let channel_id: u32 = (time % 100000) as u32;

        println!("-> {{ import, {} }}", source_path);
        let vec = ser::to_vec_packed(&(channel_id, "import", source_path)).unwrap();

        self.cbor_proto
            .send_message(&vec, &self.host, self.dest_port)
            .unwrap();

        /*
        let (channel_id, result, data) = self.result_receiver
            .recv()
            .map_err(|err| format!("Failed to receive op result: {}", err))?;
            */
        // Listen on UDP port
        let message = self.cbor_proto
            .recv_message()?
            .ok_or(format!("Failed to receive op result"))?;

        // Parse the received message
        if let Some((channel_id, result, data)) = self.on_message(message)? {
            let mut pieces = data.iter();
            match result {
                true => {
                    //let mut pieces = data.iter();
                    let hash = match pieces
                        .next()
                        .ok_or(format!("Unable to parse success message: No hash param"))?
                    {
                        Value::String(val) => val,
                        _ => {
                            return Err("Unable to parse success message: Invalid hash param".to_owned())
                        }
                    };

                    let num_chunks = match pieces.next().ok_or(format!(
                        "Unable to parse success message: No num_chunks param"
                    ))? {
                        Value::U64(val) => *val,
                        _ => {
                            return Err("Unable to parse success message: Invalid num_chunks param"
                                .to_owned())
                        }
                    };

                    //let mode = pieces.next().and_then(|val| Some(val as u16));

                    // Return the file info
                    Ok((hash.to_string(), num_chunks as u32, Some(0) /*mode*/))
                }
                false => {
                    return Err(format!(
                        "Failed to request file import: {:?}",
                        pieces
                            .next()
                            .ok_or(format!("Unable to parse failure message: No error param"))?
                    ));
                }
            }
        } else {
            Err(format!("Failed to receive success message response"))
        }
    }

    // Figure out if/what chunks are missing and send the hash and info back to the remote addr
    pub fn sync_and_send(&self, hash: &str, num_chunks: Option<u32>) -> Result<(), String> {
        let (result, chunks) = storage::local_sync(hash, num_chunks)?;
        // TODO: chunks will eventually be more than a single value, so we'll need to iter through the list
        // and add each missing chunk # to the vector
        println!("-> {{ {}, {:?} }}", hash, result);
        let vec = ser::to_vec_packed(&(hash, result, chunks)).unwrap();

        self.cbor_proto
            .send_message(&vec, &self.host, self.dest_port)
            .unwrap();
        Ok(())
    }

    /// Create temporary folder for chunks
    /// Stream copy file from mutable space to immutable space
    /// Move folder to hash of contents
    pub fn local_import(&self, source_path: &str) -> Result<(String, u32, u16), String> {
        storage::local_import(source_path)
    }

    // Copy temporary data chunks into permanent file?
    pub fn local_export(
        &self,
        hash: &str,
        target_path: &str,
        mode: Option<u16>,
    ) -> Result<(), String> {
        storage::local_export(hash, target_path, mode)
    }

    // Request a download to start
    fn start_push(&self, hash: &str, chunks: Option<Vec<u32>>) -> Result<(), String> {
        unimplemented!();
    }

    // Request a download to stop
    fn stop_push(&self, hash: &str) -> Result<(), String> {
        unimplemented!();
    }

    // Send a single file chunk to the remote address
    fn send_chunk(&self, hash: &str, index: u32, chunk: &[u8]) -> Result<(), String> {
        unimplemented!();
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

    /*
    fn listen_thread(&self) -> Result<(), String> {
        loop {
            // Listen on UDP port
            if let Some(message) = self.cbor_proto.recv_message()? {
                // Call on_message with any received messages
                self.on_message(message);
            }
        }
    }
    */

    // Received message handler/parser
    fn on_message(&self, message: Value) -> Result<Option<(u32, bool, Vec<Value>)>, String> {
        let data = match message {
            Value::Array(val) => val.to_owned(),
            _ => return Err("Unable to parse message: Data not an array".to_owned()),
        };
        let mut pieces = data.iter();

        let first_param: Value = pieces
            .next()
            .ok_or(format!("Unable to parse message: No contents"))?
            .to_owned();

        // TODO: verify channel ID number type
        match first_param {
            // It's a channel ID
            Value::U64(channel_id) => match pieces.next().ok_or(format!(
                "Unable to parse message: No param after channel ID"
            ))? {
                Value::String(operation) => {
                    match operation.as_ref() {
                        "export" => {
                            // It's an export request: { channel_id, "export", hash, path [, mode] }

                            let hash = match pieces
                                .next()
                                .ok_or(format!("Unable to parse export message: No hash param"))?
                            {
                                Value::String(val) => val,
                                _ => {
                                    return Err("Unable to parse export message: Invalid hash param"
                                        .to_owned())
                                }
                            };

                            let path = match pieces
                                .next()
                                .ok_or(format!("Unable to parse export message: No path param"))?
                            {
                                Value::String(val) => val,
                                _ => {
                                    return Err("Unable to parse export message: Invalid path param"
                                        .to_owned())
                                }
                            };

                            let mode = match pieces.next() {
                                Some(Value::U64(num)) => Some(*num as u16),
                                _ => None,
                            };

                            match storage::local_export(hash, path, mode) {
                                Ok(results) => {
                                    // TODO: Results might need to be unpacked from tuple
                                    println!("-> {{ {}, true, {:?} }}", channel_id, results);
                                    let vec =
                                        ser::to_vec_packed(&(channel_id, true, results)).unwrap();

                                    self.cbor_proto
                                        .send_message(&vec, &self.host, self.dest_port)
                                        .unwrap();
                                }
                                Err(error) => {
                                    println!("-> {{ {}, false, {} }}", channel_id, error);
                                    let vec =
                                        ser::to_vec_packed(&(channel_id, false, error)).unwrap();

                                    self.cbor_proto
                                        .send_message(&vec, &self.host, self.dest_port)
                                        .unwrap();
                                }
                            }
                        }
                        "import" => {
                            // It's an import request: { channel_id, "import", path }
                            let path = match pieces
                                .next()
                                .ok_or(format!("Unable to parse import message: No path param"))?
                            {
                                Value::String(val) => val,
                                _ => {
                                    return Err("Unable to parse import message: Invalid path param"
                                        .to_owned())
                                }
                            };

                            match storage::local_import(path) {
                                Ok(results) => {
                                    // TODO: Results might need to be unpacked from tuple
                                    println!("-> {{ {}, true, {:?} }}", channel_id, results);
                                    let vec =
                                        ser::to_vec_packed(&(channel_id, true, results)).unwrap();

                                    self.cbor_proto
                                        .send_message(&vec, &self.host, self.dest_port)
                                        .unwrap();
                                }
                                Err(error) => {
                                    println!("-> {{ {}, false, {} }}", channel_id, error);
                                    let vec =
                                        ser::to_vec_packed(&(channel_id, false, error)).unwrap();

                                    self.cbor_proto
                                        .send_message(&vec, &self.host, self.dest_port)
                                        .unwrap();
                                }
                            }
                        }
                        _ => return Err(format!("Unable to parse message: Unknown operation")),
                    }
                }
                Value::Bool(result) => {
                    // It's an import/export op result
                    // Good - { channel_id, true, ...values }
                    // Bad - { channel_id, false, error_message}

                    // Return result to caller
                    return Ok(Some((
                        channel_id as u32,
                        *result,
                        pieces.map(|val| val.clone()).collect(),
                    )));
                }
                _ => {
                    return Err(format!(
                        "Unable to parse message: Unknown param after channel ID"
                    ))
                }
            },
            // It's a hash value
            Value::String(hash) => {
                if let Some(second_param) = pieces.next() {
                    match second_param {
                        Value::Bool(true) => {
                            // It's an ACK: { hash, true, num_chunks }
                            // Our data transfer (export) completed succesfully
                            self.stop_push(&hash)?;

                            //TODO: Do something with the third param? (num_chunks)
                        }
                        Value::Bool(false) => {
                            // It's a NAK: { hash, false, 1, 4, 6, 7 }
                            // Some number of chunks were not received by the remote addr
                            let mut remaining_chunks: Vec<u32> = vec![];
                            for entry in pieces {
                                if let Value::U64(chunk_num) = entry {
                                    remaining_chunks.push(*chunk_num as u32);
                                }
                            }

                            if remaining_chunks.len() > 0 {
                                self.start_push(&hash, Some(remaining_chunks))?
                            } else {
                                return Err(format!(
                                    "Unable to parse NAK message: Missing missing chunks"
                                ));
                            }
                        }
                        Value::U64(num) => {
                            if let Some(third_param) = pieces.next() {
                                if let Value::Bytes(data) = third_param {
                                    // It's a data chunk message: { hash, chunk_index, data }
                                    // Store the new chunk
                                    storage::store_chunk(&hash, *num as u32, data);
                                    // TODO: receive_timeouts
                                    //let timer = receive_timeouts(&hash);
                                    self.sync_and_send(&hash, None);
                                } else {
                                    return Err(format!(
                                        "Unable to parse chunk message: Invalid data format"
                                    ));
                                }
                            } else {
                                // It's a sync message: { hash, num_chunks }
                                self.sync_and_send(&hash, Some(*num as u32));
                            }
                        }
                        _ => {
                            return Err(format!("Unable to parse message: Unknown param after hash"))
                        }
                    }
                } else {
                    // It's a sync message: { hash }
                    self.sync_and_send(&hash, None)?;
                }
            }
            _ => return Err(format!("Unable to parse message: Unknown first param type")),
        }

        Ok(None)
    }
}
