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

//#![deny(missing_docs)]

extern crate blake2_rfc;
extern crate cbor_protocol;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_cbor;
extern crate time;

pub mod messages;
mod parsers;
pub mod protocol;
pub mod storage;

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
