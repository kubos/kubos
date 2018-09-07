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
//!     // Tell our destination the hash and number of chunks to expect
//!     f_protocol.send_metadata(&hash, num_chunks)?;
//!
//!     // Give the service can have time to set up the temporary storage directory
//!     ::std::thread::sleep(Duration::from_millis(1));
//!
//!     // Send export command for file
//!     f_protocol.send_export(&hash, &target_path, mode)?;
//!
//!     // Start the engine to send the file data chunks
//!     Ok(f_protocol.message_engine(Duration::from_millis(10), State::Transmitting)?)
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
//!     # ::std::fs::File::create("service.txt").unwrap();
//!     let source_path = "service.txt";
//!     let target_path = "client.txt";
//!
//!     // Send our file request to the remote addr and verify that it's
//!     // going to be able to send it
//!     f_protocol.send_import(source_path)?;
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
//!     Ok(f_protocol.message_engine(Duration::from_millis(10), state)?)
//! }
//! ```
//!

#![deny(missing_docs)]

extern crate blake2_rfc;
extern crate cbor_protocol;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_cbor;
extern crate time;

mod messages;
mod parsers;
pub mod protocol;
mod storage;

pub use protocol::Protocol as FileProtocol;
pub use protocol::State;

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
    /// (Client Only) Message requesting the recipient to transmit the specified file
    ReqTransmit(u64, String),
    /// (Server Only) Recipient has successfully processed a request to receive a file
    SuccessReceive(u64),
    /// (Server Only) Recipient has successfully prepared to transmit a file
    SuccessTransmit(u64, String, u32, Option<u32>),
    /// (Server Only) The transmit or receive request has failed to be completed
    Failure(u64, String),
}
