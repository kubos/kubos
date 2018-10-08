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

extern crate cbor_protocol;
extern crate log;
extern crate serde_cbor;
#[macro_use]
extern crate failure;
extern crate rand;

mod error;
mod parsers;
mod protocol;

pub use error::ProtocolError;
pub use parsers::*;
pub use protocol::Message as ChannelMessage;
pub use protocol::Protocol as ChannelProtocol;

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
    let channel_id: u32 = rng.gen_range(100000, 999999);
    channel_id
}
