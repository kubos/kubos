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

pub mod rx;
mod tx;

use byteorder::{LittleEndian, WriteBytesExt};
pub use self::rx::*;
pub use self::tx::*;

/// IRIG-106 sync word
pub const SYNC: u16 = 0xEB90;
/// Size of message header (packed)
pub const HDR_SZ: usize = 6;

/// Header for all sent and received messages
#[derive(Clone, Debug, PartialEq)]
pub struct MessageHeader {
    /// IRIG-106 sync word
    pub sync: u16,
    /// Length of data in message, not including header or CRC bytes
    pub data_len: u16,
    /// Message ID
    pub msg_id: u8,
    /// Endpoint device address
    pub addr: u8,
}

impl Default for MessageHeader {
    fn default() -> Self {
        MessageHeader {
            sync: SYNC,
            data_len: 0,
            msg_id: 0,
            addr: 0,
        }
    }
}

impl MessageHeader {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![];

        vec.write_u16::<LittleEndian>(self.sync).unwrap();
        vec.write_u16::<LittleEndian>(self.data_len).unwrap();
        vec.push(self.msg_id);
        vec.push(self.addr);
        vec
    }
}
