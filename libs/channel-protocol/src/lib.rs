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
mod protocol;

pub use error::ProtocolError;
pub use protocol::Protocol as ChannelProtocol;

use rand::Rng;
use serde_cbor::Value;

/// Generates a new random channel ID for use when initiating a
/// file transfer.
///
/// # Errors
///
/// If this function encounters any errors, it will return an error message string
///
/// # Examples
///
///
pub fn generate_channel() -> u32 {
    let mut rng = rand::thread_rng();
    let channel_id: u32 = rng.gen_range(100000, 999999);
    channel_id
}

/// Parse out just the channel ID from a message
pub fn parse_channel_id(message: &Value) -> Result<u32, String> {
    let data = match message {
        Value::Array(val) => val.to_owned(),
        _ => return Err("Data not an array".to_owned()),
    };

    let mut pieces = data.iter();

    let first_param: Value = pieces.next().ok_or("No contents".to_owned())?.to_owned();

    if let Value::U64(channel_id) = first_param {
        Ok(channel_id as u32)
    } else {
        Err("No channel ID found".to_owned())
    }
}
