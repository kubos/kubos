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

mod rx;
mod tx;

use byteorder::{LittleEndian, WriteBytesExt};
pub use self::rx::*;
pub use self::tx::*;

pub const SYNC: u16 = 0xEB90;
pub const HDR_SZ: usize = 6;
pub const FRAME_SZ: usize = HDR_SZ + 2;

#[repr(C, packed)]
pub struct MessageHeader {
    pub sync: u16,
    pub data_len: u16,
    pub msg_id: u8,
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
