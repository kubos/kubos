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

use byteorder::{LittleEndian, WriteBytesExt};
use super::*;

pub trait Message {
    fn serialize(&self) -> Vec<u8>;
}

pub struct SetAcsMode {
    pub id: u8,
    pub mode: u8,
    pub qbi_cmd: [i16; 4],
}

impl Default for SetAcsMode {
    fn default() -> Self {
        SetAcsMode {
            id: 0,
            mode: 0,
            qbi_cmd: [0; 4],
        }
    }
}

impl Message for SetAcsMode {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![];

        vec.write_u16::<LittleEndian>(SYNC).unwrap();
        vec.push(self.id);
        vec.push(self.mode);
        vec.write_i16::<LittleEndian>(self.qbi_cmd[0]).unwrap();
        vec.write_i16::<LittleEndian>(self.qbi_cmd[1]).unwrap();
        vec.write_i16::<LittleEndian>(self.qbi_cmd[2]).unwrap();
        vec.write_i16::<LittleEndian>(self.qbi_cmd[3]).unwrap();
        vec.append(&mut vec![0; 26]);
        vec
    }
}

pub struct SetAcsModeSun {
    pub id: u8,
    pub mode: u8,
    pub sun_angle_enable: i16,
    pub sun_rot_angle: f32,
}

impl Default for SetAcsModeSun {
    fn default() -> Self {
        SetAcsModeSun {
            id: 0,
            mode: 0,
            sun_angle_enable: 0,
            sun_rot_angle: 0.0,
        }
    }
}

impl Message for SetAcsModeSun {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![];

        vec.write_u16::<LittleEndian>(SYNC).unwrap();
        vec.push(self.id);
        vec.push(self.mode);
        vec.write_i16::<LittleEndian>(self.sun_angle_enable)
            .unwrap();
        vec.write_f32::<LittleEndian>(self.sun_rot_angle).unwrap();
        vec.append(&mut vec![0; 28]);
        vec
    }
}

pub struct SetGPSTime {
    pub id: u8,
    pub gps_time: u32,
}

impl Default for SetGPSTime {
    fn default() -> Self {
        SetGPSTime {
            id: 0x44,
            gps_time: 0,
        }
    }
}

impl Message for SetGPSTime {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![];

        vec.write_u16::<LittleEndian>(SYNC).unwrap();
        vec.push(self.id);
        vec.write_u32::<LittleEndian>(self.gps_time).unwrap();
        vec.append(&mut vec![0; 31]);
        vec
    }
}

pub struct SetRV {
    pub id: u8,
    pub eci_pos: [f32; 3],
    pub eci_vel: [f32; 3],
    pub time_epoch: u32,
}

impl Default for SetRV {
    fn default() -> Self {
        SetRV {
            id: 0x41,
            eci_pos: [0.0, 0.0, 0.0],
            eci_vel: [0.0, 0.0, 0.0],
            time_epoch: 0,
        }
    }
}

impl Message for SetRV {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![];

        vec.write_u16::<LittleEndian>(SYNC).unwrap();
        vec.push(self.id);
        vec.write_f32::<LittleEndian>(self.eci_pos[0]).unwrap();
        vec.write_f32::<LittleEndian>(self.eci_pos[1]).unwrap();
        vec.write_f32::<LittleEndian>(self.eci_pos[2]).unwrap();
        vec.write_f32::<LittleEndian>(self.eci_vel[0]).unwrap();
        vec.write_f32::<LittleEndian>(self.eci_vel[1]).unwrap();
        vec.write_f32::<LittleEndian>(self.eci_vel[2]).unwrap();
        vec.write_u32::<LittleEndian>(self.time_epoch).unwrap();
        vec.append(&mut vec![0; 7]);
        vec
    }
}

pub struct RequestReset(pub [u8; 38]);

impl Default for RequestReset {
    fn default() -> Self {
        let mut array = [0; 38];
        array[0] = 0x90; // SYNC byte 1
        array[1] = 0xEB; // SYNC byte 2
        array[2] = 0x5A; // Command ID
        RequestReset(array)
    }
}

impl Message for RequestReset {
    fn serialize(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

pub struct ConfirmReset([u8; 38]);

impl Default for ConfirmReset {
    fn default() -> Self {
        let mut array = [0; 38];
        array[0] = 0x90; // SYNC byte 1
        array[1] = 0xEB; // SYNC byte 2
        array[2] = 0xF1; // Command ID
        ConfirmReset(array)
    }
}

impl Message for ConfirmReset {
    fn serialize(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}
