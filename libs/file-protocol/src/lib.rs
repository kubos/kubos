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

//! TODO: Crate documentation

#![deny(missing_docs)]

extern crate blake2_rfc;
extern crate cbor_protocol;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_cbor;
extern crate time;

use std::time::Duration;

mod messages;
mod parsers;
pub mod protocol;
mod storage;

pub use protocol::Protocol as FileProtocol;
pub use protocol::Role;

const CHUNK_SIZE: usize = 4096;

/// File protocol message types
#[derive(Debug, Clone)]
pub enum Message {
    /// TODO: Decide whether or not to keep this
    Sync(String),
    /// Receiver should prepare a new temporary storage folder with the specified metadata
    Metadata(String, u32),
    /// File data chunk message
    ReceiveChunk(String, u32, Vec<u8>),
    /// Receiver has successfully gotten all data chunks of the requested file
    ACK(String),
    /// Receiver is missing the specified file data chunks
    NAK(String, Option<Vec<(u32, u32)>>),
    /// (Client Only) Message requesting the recipient to receive the specified file
    ReqReceive(u64, String, String, Option<u32>),
    /// (Client Only) Message requesting the recipient to transmit the speicified file
    ReqTransmit(u64, String),
    /// (Server Only) Recipient has successfully processed a request to receive a file
    SuccessReceive(u64),
    /// (Server Only) Recipient has successfully prepared to transmit a file
    SuccessTransmit(u64, String, u32, Option<u32>),
    /// (Server Only) The transmit or receive request has failed to be completed
    Failure(u64, String),
}

/// Upload a file to the target server location
pub fn upload(port: u16, source_path: &str, target_path: &str) -> Result<(), String> {
    let f_protocol = protocol::Protocol::new(String::from("127.0.0.1"), port, Role::Client);

    info!(
        "Uploading local:{} to remote:{}",
        &source_path, &target_path
    );

    // Copy file to upload to temp storage. Calculate the hash and chunk info
    let (hash, num_chunks, mode) = storage::initialize_file(&source_path)?;

    // Q: Why not combine this and export into one message? it's really only a single extra parameter
    // Tell our destination the hash and number of chunks to expect
    f_protocol.send(messages::metadata(&hash, num_chunks).unwrap())?;

    // Send export command for file
    f_protocol.send_export(&hash, &target_path, mode)?;

    // Start the engine to send the file data chunks
    f_protocol.message_engine(Some(&hash), Duration::from_secs(2), true)?;

    Ok(())
}

/// Download a file from the target server location
pub fn download(port: u16, source_path: &str, target_path: &str) -> Result<(), String> {
    let f_protocol = protocol::Protocol::new(String::from("127.0.0.1"), port, Role::Client);

    info!(
        "Downloading remote: {} to local: {}",
        source_path, target_path
    );

    // Send our file request to the remote addr and verify that it's
    // going to be able to send it
    f_protocol.send_import(source_path)?;

    // Check the number of chunks we need to receive and then receive them
    // f_protocol.sync_and_send(&hash, Some(num_chunks))?;
    match f_protocol.message_engine(None, Duration::from_secs(1), false) {
        Ok(Some(Message::SuccessTransmit(_id, hash, _num_chunks, mode))) => {
            info!("file has been transmitted?");
            f_protocol.message_engine(Some(&hash), Duration::from_secs(2), true)?;

            info!("done recv");

            // Save received data to the requested path
            storage::finalize_file(&hash, target_path, mode)?;
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
