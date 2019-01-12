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

//! Channel Protocol
//!
//! This protocol is used to send and receive CBOR-encoded channel based
//! messages over UDP. Each message consists of three parts: channel ID,
//! message name, message payload, all contained in a cbor array.
//!
//!   { channel_id, name, payload.. }
//!
//! The channel ID is typically used to group all messages related to a transaction
//! or logical action. An example would be all messages related to uploading
//! a single file.
//! The message name is used to determine the type of message.
//! The message payload contains any other message data.
//!
//! # Examples
//!
//! ```no_run
//! use channel_protocol::*;
//!
//! let channel_proto = ChannelProtocol::new(
//!                         &"0.0.0.0:0".to_owned(),
//!                         &"127.0.0.1:8000".to_owned(),
//!                         4096);
//!
//! let message = vec![0, 1, 1, 2];
//!
//! channel_proto.send(&message);
//! match channel_proto.recv_message(None) {
//!     Ok(message) => println!("Received: {:?}", message),
//!     Err(e) => eprintln!("Error receiving: {}", e)
//! }
//! ```

#![deny(missing_docs)]
#![deny(warnings)]

extern crate cbor_protocol;
#[macro_use]
extern crate failure;
extern crate log;
extern crate rand;
extern crate serde_cbor;

mod error;
mod parsers;
mod protocol;

pub use crate::error::ProtocolError;
pub use crate::parsers::*;
pub use crate::protocol::Message as ChannelMessage;
pub use crate::protocol::Protocol as ChannelProtocol;

use rand::Rng;

/// Generates a new random channel ID for use when initiating a
/// file transfer.
///
/// # Errors
///
/// If this function encounters any errors, it will return an error message string
///
/// # Examples
///
/// ```no_run
/// use channel_protocol::*;
///
/// let channel_id = generate_channel();
/// ```
///
pub fn generate_channel() -> u32 {
    let mut rng = rand::thread_rng();
    let channel_id: u32 = rng.gen_range(100_000, 999_999);
    channel_id
}
