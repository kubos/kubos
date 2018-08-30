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

pub mod messages;
mod parsers;
pub mod protocol;
pub mod storage;

pub use protocol::Protocol as FileProtocol;
pub use protocol::State;

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