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

use super::*;
use byteorder::{LittleEndian, WriteBytesExt};

pub trait Message {
    fn serialize(&self) -> Vec<u8>;
}

pub struct SetAcsMode {
    //pub hdr: MessageHeader,
    pub mode: u8,
    pub sec_vec: i32,
    pub pri_axis: i32,
    pub sec_axis: i32,
    pub qbi_cmd4: i32,
}

impl Default for SetAcsMode {
    fn default() -> Self {
        SetAcsMode {
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

        //vec.append(&mut self.hdr.serialize());
        vec.push(self.mode);
        vec.write_i32::<LittleEndian>(self.sec_vec).unwrap();
        vec.write_i32::<LittleEndian>(self.pri_axis).unwrap();
        vec.write_i32::<LittleEndian>(self.sec_axis).unwrap();
        vec.write_i32::<LittleEndian>(self.qbi_cmd4).unwrap();
        vec
    }
}

pub struct SetGPSTime {
    //pub hdr: MessageHeader,
    pub sync: u16,
    pub id: u8,
    pub gps_time: u32,
}

impl Default for SetGPSTime {
    fn default() -> Self {
        SetGPSTime {
            sync: SYNC,
            id: 0x44,
            gps_time: 0,
        }
    }
}

impl Message for SetGPSTime {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![];

        //vec.append(&mut self.hdr.serialize());
        vec.write_u16::<LittleEndian>(self.sync).unwrap();
        vec.push(self.id);
        vec.write_u32::<LittleEndian>(self.gps_time).unwrap();
        vec.append(&mut vec![0; 31]); //TODO: Remove for newer FW version
        vec
    }
}

pub struct SetRV {
    //pub hdr: MessageHeader,
    pub sync: u16,
    pub id: u8,
    pub eci_pos_x: f32,
    pub eci_pos_y: f32,
    pub eci_pos_z: f32,
    pub eci_vel_x: f32,
    pub eci_vel_y: f32,
    pub eci_vel_z: f32,
    pub time_epoch: u32,
}

impl Default for SetRV {
    fn default() -> Self {
        SetRV {
            sync: SYNC,
            id: 0x41,
            eci_pos_x: 0.0,
            eci_pos_y: 0.0,
            eci_pos_z: 0.0,
            eci_vel_x: 0.0,
            eci_vel_y: 0.0,
            eci_vel_z: 0.0,
            time_epoch: 0,
        }
    }
}

impl Message for SetRV {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![];

        //vec.append(&mut self.hdr.serialize());
        vec.write_u16::<LittleEndian>(self.sync).unwrap();
        vec.push(self.id);
        vec.write_f32::<LittleEndian>(self.eci_pos_x).unwrap();
        vec.write_f32::<LittleEndian>(self.eci_pos_y).unwrap();
        vec.write_f32::<LittleEndian>(self.eci_pos_z).unwrap();
        vec.write_f32::<LittleEndian>(self.eci_vel_x).unwrap();
        vec.write_f32::<LittleEndian>(self.eci_vel_y).unwrap();
        vec.write_f32::<LittleEndian>(self.eci_vel_z).unwrap();
        vec.write_u32::<LittleEndian>(self.time_epoch).unwrap();
        vec.append(&mut vec![0; 6]); //TODO: Remove for newer FW version
        vec
    }
}

pub struct RequestReset([u8; 40]);

impl Default for RequestReset {
    fn default() -> Self {
        let mut array = [0; 40];
        array[0] = 0x90;
        array[1] = 0xEB;
        array[2] = 0x5A;
        //TODO: Checksum
        RequestReset(array)
    }
}

pub struct ConfirmReset([u8; 40]);

impl Default for ConfirmReset {
    fn default() -> Self {
        let mut array = [0; 40];
        array[0] = 0x90;
        array[1] = 0xEB;
        array[2] = 0xF1;
        //TODO: Checksum
        ConfirmReset(array)
    }
}
