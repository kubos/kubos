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

#![allow(dead_code)]

use byteorder::{LittleEndian, WriteBytesExt};

pub mod rx;
mod tx;

pub use self::rx::*;
pub use self::tx::*;

pub const SYNC: [u8; 3] = [0xAA, 0x44, 0x12];
pub const HDR_LEN: u8 = 28;
pub const RESP_HDR_LEN: u8 = HDR_LEN + 4;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Header {
    pub sync: [u8; 3],
    pub hdr_len: u8,
    pub msg_id: u16,
    pub msg_type: u8, //Can probably be hardcoded. Bit fields
    pub port_addr: u8,
    pub msg_len: u16,
    pub seq: u16,
    pub idle_time: u8,   //Ignore for TX
    pub time_status: u8, //TODO: enum?
    pub week: u16,
    pub ms: i32,
    pub recv_status: u32, //Ignore for TX
    pub recv_ver: u16,    //Ignore for TX
}

impl Header {
    fn new(msg_id: u16, msg_len: u16) -> Self {
        Header {
            sync: SYNC,
            hdr_len: HDR_LEN,
            msg_id,
            msg_type: 0, // Measurement source = Primary antenna, Format = Binary, Response bit = Original message. TODO: Verify
            port_addr: Port::ThisPort as u8,
            msg_len,
            seq: 0,         // Always zero. We're only sending the one message
            idle_time: 0,   // TODO: calculate
            time_status: 0, // TODO: ...figure out...
            week: 0,        // TODO: calculate
            ms: 0,          // TODO: calculate
            recv_status: 0, // Ignored for TX
            recv_ver: 0,    // Ignored for TX
        }
    }
}

impl Message for Header {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = SYNC.to_vec();

        vec.push(self.hdr_len);
        vec.write_u16::<LittleEndian>(self.msg_id).unwrap();
        vec.push(self.msg_type);
        vec.push(self.port_addr);
        vec.write_u16::<LittleEndian>(self.msg_len).unwrap();
        vec.write_u16::<LittleEndian>(self.seq).unwrap();
        vec.push(self.idle_time);
        vec.push(self.time_status);
        vec.write_u16::<LittleEndian>(self.week).unwrap();
        vec.write_i32::<LittleEndian>(self.ms).unwrap();
        vec.write_u32::<LittleEndian>(self.recv_status).unwrap();
        vec.push(0);
        vec.push(0);
        vec.write_u16::<LittleEndian>(self.recv_ver).unwrap();

        vec
    }
}

pub enum Port {
    COM1 = 32,
    ThisPort = 192,
}
