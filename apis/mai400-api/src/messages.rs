/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
//#![feature(associated_consts)]

use byteorder::{LittleEndian, WriteBytesExt};
use nom::*;

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

        //TODO: Verify that we want to make the sync variable
        //little endian...
        vec.write_u16::<LittleEndian>(self.sync).unwrap();
        vec.write_u16::<LittleEndian>(self.data_len).unwrap();
        vec.push(self.msg_id);
        vec.push(self.addr);
        vec
    }
}

pub trait Message {
    fn serialize(&self) -> Vec<u8>;
}

pub struct GetInfo {
    hdr: MessageHeader,
}

impl Default for GetInfo {
    fn default() -> Self {
        GetInfo {
            hdr: MessageHeader {
                msg_id: 0x1D,
                ..Default::default()
            },
        }
    }
}

impl Message for GetInfo {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![];

        vec.append(&mut self.hdr.serialize());
        //vec.write_u16::<LittleEndian>(self.crc).unwrap();
        vec
    }
}

pub struct SetAcsMode {
    pub hdr: MessageHeader,
    pub mode: u8,
    pub sec_vec: i32,
    pub pri_axis: i32,
    pub sec_axis: i32,
    pub qbi_cmd4: i32,
}

impl Default for SetAcsMode {
    fn default() -> Self {
        SetAcsMode {
            hdr: MessageHeader {
                data_len: 17,
                ..Default::default()
            },
            mode: 0,
            sec_vec: 0,
            pri_axis: 0,
            sec_axis: 0,
            qbi_cmd4: 0,
        }
    }
}

impl Message for SetAcsMode {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![];

        vec.append(&mut self.hdr.serialize());
        vec.push(self.mode);
        vec.write_i32::<LittleEndian>(self.sec_vec).unwrap();
        vec.write_i32::<LittleEndian>(self.pri_axis).unwrap();
        vec.write_i32::<LittleEndian>(self.sec_axis).unwrap();
        vec.write_i32::<LittleEndian>(self.qbi_cmd4).unwrap();
        vec
    }
}

pub struct ConfigInfo {
    pub hdr: MessageHeader,
    pub model: u16,
    pub serial: u16,
    pub major: u8,
    pub minor: u8,
    pub build: u16,
    pub n_ehs: u8,
    pub ehs_type: EHSType,
    pub n_st: u8,
    pub st_type: StarTracker,
    pub crc: u16,
}

impl ConfigInfo {
    pub fn new(msg: &[u8]) -> Self {
        configinfo(msg).unwrap().1
    }
}

named!(configinfo(&[u8]) -> ConfigInfo,
    do_parse!(
        sync: le_u16 >>
        data_len: le_u16 >>
        msg_id: le_u8 >>
        addr: le_u8 >>
        model: le_u16 >>
        serial: le_u16 >>
        major: le_u8 >>
        minor: le_u8 >>
        build: le_u16 >>
        n_ehs: le_u8 >> 
        ehs_type: le_u8 >>//TODO: Enum conversion
        n_st: le_u8 >>
        st_type: le_u8 >> //TODO: Enum conversion
        crc: le_u16 >>
        (ConfigInfo{
                hdr: MessageHeader {
                    sync, 
                    data_len, 
                    msg_id, 
                    addr
                }, 
                model, 
                serial, 
                major, 
                minor,
                build, 
                n_ehs,
                ehs_type: match ehs_type {
                    0 => EHSType::Internal,
                    _ => EHSType::External
                },
                n_st,
                st_type: match st_type {
                    0 => StarTracker::MAISextant,
                    _ => StarTracker::Vectronic,
                }, 
                crc})
    )
);

pub struct StandardTelemetry {}

pub enum Response {
    Config(ConfigInfo),
    StdTelem(StandardTelemetry),
}

pub enum EHSType {
    Internal,
    External,
}

pub enum StarTracker {
    MAISextant,
    Vectronic,
}
