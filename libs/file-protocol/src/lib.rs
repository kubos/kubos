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

//! Kubos File Transfer Protocol
//!
//! # Examples
//!
//! ```no_run
//! extern crate file_protocol;
//!
//! use file_protocol::*;
//! use std::time::Duration;
//!
//! fn upload() -> Result<(), String> {
//!     let f_protocol = FileProtocol::new("0.0.0.0", "0.0.0.0:7000", Some("storage/dir".to_owned()));
//!
//!     # ::std::fs::File::create("client.txt").unwrap();
//!     let source_path = "client.txt";
//!     let target_path = "service.txt";
//!
//!     // Copy file to upload to temp storage. Calculate the hash and chunk info
//!     let (hash, num_chunks, mode) = f_protocol.initialize_file(&source_path)?;
//!
//!     // Generate channel id
//!     let channel_id = f_protocol.generate_channel()?;
//!
//!     // Tell our destination the hash and number of chunks to expect
//!     f_protocol.send_metadata(channel_id, &hash, num_chunks)?;
//!
//!     // Send export command for file
//!     f_protocol.send_export(channel_id, &hash, &target_path, mode)?;
//!
//!     // Start the engine to send the file data chunks
//!     Ok(f_protocol.message_engine(|d| f_protocol.recv(Some(d)), Duration::from_millis(10), State::Transmitting)?)
//! }
//! ```
//!
//! ```no_run
//! extern crate file_protocol;
//!
//! use file_protocol::*;
//! use std::time::Duration;
//!
//! fn download() -> Result<(), String> {
//!     let f_protocol = FileProtocol::new("0.0.0.0", "0.0.0.0:8000", None);
//!
//!     let channel_id = f_protocol.generate_channel()?;
//!     # ::std::fs::File::create("service.txt").unwrap();
//!     let source_path = "service.txt";
//!     let target_path = "client.txt";
//!
//!     // Send our file request to the remote addr and verify that it's
//!     // going to be able to send it
//!     f_protocol.send_import(channel_id, source_path)?;
//!
//!     // Wait for the request reply
//!     let reply = match f_protocol.recv(None) {
//!         Ok(Some(message)) => message,
//!         Ok(None) => return Err("Failed to import file".to_owned()),
//!         Err(Some(error)) => return Err(format!("Failed to import file: {}", error)),
//!         Err(None) => return Err("Failed to import file".to_owned()),
//!     };
//!
//!     let state = f_protocol.process_message(
//!         reply,
//!         State::StartReceive {
//!             path: target_path.to_string(),
//!         },
//!     )?;
//!
//!     Ok(f_protocol.message_engine(|d| f_protocol.recv(Some(d)), Duration::from_millis(10), state)?)
//! }
//! ```
//!

#![deny(missing_docs)]

extern crate blake2_rfc;
extern crate cbor_protocol;
#[macro_use]
extern crate log;
extern crate rand;
extern crate serde;
extern crate serde_cbor;
extern crate time;

mod messages;
mod parsers;
pub mod protocol;
mod storage;

pub use protocol::Protocol as FileProtocol;
pub use protocol::State;

pub use parsers::parse_channel_id;

const CHUNK_SIZE: usize = 4096;

/// File protocol message types
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Message {
    /// TODO: Decide whether or not to keep this
    Sync(u32, String),
    /// Receiver should prepare a new temporary storage folder with the specified metadata
    Metadata(u32, String, u32),
    /// File data chunk message
    ReceiveChunk(u32, String, u32, Vec<u8>),
    /// Receiver has successfully gotten all data chunks of the requested file
    ACK(u32, String),
    /// Receiver is missing the specified file data chunks
    NAK(u32, String, Option<Vec<(u32, u32)>>),
    /// (Client Only) Message requesting the recipient to receive the specified file
    ReqReceive(u32, String, String, Option<u32>),
    /// (Client Only) Message requesting the recipient to transmit the specified file
    ReqTransmit(u32, String),
    /// (Server Only) Recipient has successfully processed a request to receive a file
    SuccessReceive(u32),
    /// (Server Only) Recipient has successfully prepared to transmit a file
    SuccessTransmit(u32, String, u32, Option<u32>),
    /// (Server Only) The transmit or receive request has failed to be completed
    Failure(u32, String),
}

#[cfg(test)]
mod tests {
    use super::{messages, parsers, Message};
    use serde_cbor::{self, de, ser, Value};

    #[test]
    fn create_parse_export_request() {
        let channel_id = 10;
        let hash = "abcdedf".to_owned();
        let target_path = "/path/to/file".to_owned();
        let mode = 0o623;

        let raw = messages::export_request(channel_id, &hash, &target_path, mode).unwrap();

        let msg = parsers::parse_message(de::from_slice(&raw).unwrap());

        assert_eq!(
            msg,
            Ok(Message::ReqReceive(
                channel_id,
                hash,
                target_path,
                Some(mode)
            ))
        );
    }

    #[test]
    fn create_parse_sync() {
        let channel_id = 10;
        let hash = "abcdefg".to_owned();
        let num_chunks = 100;

        let raw = messages::sync(channel_id, &hash).unwrap();
        let msg = parsers::parse_message(de::from_slice(&raw).unwrap());

        assert_eq!(msg, Ok(Message::Sync(channel_id, hash)));
    }

    #[test]
    fn create_parse_metadata() {
        let channel_id = 10;
        let hash = "abcdefg".to_owned();
        let num_chunks = 100;

        let raw = messages::metadata(channel_id, &hash, num_chunks).unwrap();
        let msg = parsers::parse_message(de::from_slice(&raw).unwrap());

        assert_eq!(msg, Ok(Message::Metadata(channel_id, hash, num_chunks)));
    }

    #[test]
    fn create_parse_chunk() {
        let channel_id = 10;
        let hash = "abcdefg".to_owned();
        let chunk_num = 10;
        let chunk_data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];

        let raw = messages::chunk(channel_id, &hash, chunk_num, &chunk_data).unwrap();
        let msg = parsers::parse_message(de::from_slice(&raw).unwrap());

        assert_eq!(
            msg,
            Ok(Message::ReceiveChunk(
                channel_id, hash, chunk_num, chunk_data
            ))
        );
    }

    #[test]
    fn create_parse_ack() {
        let channel_id = 14;
        let hash = "abcdefg".to_owned();
        let num_chunks = 10;

        let raw = messages::ack(channel_id, &hash, Some(num_chunks)).unwrap();
        let msg = parsers::parse_message(de::from_slice(&raw).unwrap());

        assert_eq!(msg, Ok(Message::ACK(channel_id, hash)));
    }

    #[test]
    fn create_parse_nak() {
        let channel_id = 11;
        let hash = "abcdefg".to_owned();
        let missing_chunks = vec![0, 1, 4, 10];
        let chunk_ranges: Vec<(u32, u32)> = vec![(0, 1), (4, 10)];

        let raw = messages::nak(channel_id, &hash, &missing_chunks).unwrap();
        let msg = parsers::parse_message(de::from_slice(&raw).unwrap());

        assert_eq!(msg, Ok(Message::NAK(channel_id, hash, Some(chunk_ranges))));
    }
}
