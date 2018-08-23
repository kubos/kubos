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

extern crate blake2_rfc;
extern crate cbor_protocol;
extern crate serde;
extern crate serde_cbor;
extern crate time;
#[macro_use]
extern crate log;

use std::time::Duration;

mod messages;
mod parsers;
pub mod protocol;
pub mod storage;

pub use protocol::Protocol as FileProtocol;
pub use protocol::Role;

const CHUNK_SIZE: usize = 4096;

#[derive(Debug, Clone)]
pub enum Message {
    Sync(String),
    SyncChunks(String, u32),
    ReceiveChunk(String, u32, Vec<u8>),
    ACK(String),
    NAK(String, Option<Vec<(u32, u32)>>),
    ReqReceive(u64, String, String, Option<u32>),
    ReqTransmit(u64, String),
    SuccessReceive(u64),
    SuccessTransmit(u64, String, u32, Option<u32>),
    Failure(u64, String),
}

pub fn upload(port: u16, source_path: &str, target_path: &str) -> Result<(), String> {
    let f_protocol = protocol::Protocol::new(String::from("127.0.0.1"), port, Role::Client);

    info!(
        "Uploading local:{} to remote:{}",
        &source_path, &target_path
    );
    // Copy file to upload to temp storage. Calculate the hash and chunk info
    // Q: What's `mode` for? `local_import` always returns 0. Looks like it should be file permissions
    let (hash, num_chunks, mode) = storage::local_import(&source_path)?;
    // Tell our destination the hash and number of chunks to expect
    f_protocol.send(messages::sync(&hash, num_chunks).unwrap())?;
    // Send export command for file
    f_protocol.send_export(&hash, &target_path, mode)?;
    // Start the engine
    f_protocol.message_engine(Some(&hash), Duration::from_secs(2), true)?;

    Ok(())
}

pub fn download(port: u16, source_path: &str, target_path: &str) -> Result<(), String> {
    let f_protocol = protocol::Protocol::new(String::from("127.0.0.1"), port, Role::Client);

    info!(
        "Downloading remote: {} to local: {}",
        source_path, target_path
    );

    // Send our file request to the remote addr and get the returned data
    f_protocol.send_import(source_path)?;

    // Check the number of chunks we need to receive and then receive them
    // f_protocol.sync_and_send(&hash, Some(num_chunks))?;
    match f_protocol.message_engine(None, Duration::from_secs(1), false) {
        Ok(Some(Message::SuccessTransmit(_id, hash, _num_chunks, mode))) => {
            info!("file has been transmitted?");
            f_protocol.message_engine(Some(&hash), Duration::from_secs(2), true);

            info!("done recv");

            // Save received data to the requested path
            storage::local_export(&hash, target_path, mode)?;
            return Ok(());
        }
        Ok(msg) => {
            return Err(format!("Wrong first message found! {:?}", msg));
        }
        Err(msg) => {
            return Err(format!("Error message found! {:?}", msg));
        }
    }
}
