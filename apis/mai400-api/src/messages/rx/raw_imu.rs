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
use byteorder::{LittleEndian, ReadBytesExt};
use crc16::*;
use nom::*;
use std::io::Cursor;

/// Raw accelerometer and gyroscope data
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RawIMU {
    /// Accelerometer (X, Y, Z)  (3.9 mg/lsb)
    pub accel: [i16; 3],
    /// Gyroscope (X, Y, Z) (8.75 mdps/lsb)
    pub gyro: [i16; 3],
    /// Gyroscope temperature (-1C/lsb)
    pub gyro_temp: u8,
}

impl RawIMU {
    /// Constructor. Converts a raw data array received from the MAI-400 into a usable structure
    pub fn new(mut msg: Vec<u8>) -> Option<Self> {
        // Verify message starts with sync bytes
        let mut data = msg.split_off(2);

        let mut wrapper = Cursor::new(msg);
        let check = wrapper.read_u16::<LittleEndian>().unwrap_or(0);
        if check != AUX_SYNC {
            println!("RawIMU: Bad sync - {:x}", check);
            return None;
        }

        // Get the CRC bytes
        let len = data.len() - 2;
        let mut crc = Cursor::new(data.split_off(len));
        let crc = crc.read_u16::<LittleEndian>().unwrap_or(0);

        // Note: Yes, this is a different way of calculating the checksum than everywhere else
        let calc = State::<ARC>::calculate(&data);

        // Verify the CRC bytes at the end of the message
        match calc == crc {
            true => {
                // Convert the raw data to an official struct
                match raw_imu(&data) {
                    Ok(conv) => Some(conv.1),
                    _ => None,
                }
            }
            false => None,
        }
    }
}

named!(raw_imu(&[u8]) -> RawIMU,
    do_parse!(
        le_i16 >>
        le_i16 >>
        accel_x: le_i16 >>
        accel_y: le_i16 >>
        accel_z: le_i16 >>
        gyro_x: le_i16 >>
        gyro_y: le_i16 >>
        gyro_z: le_i16 >>
        gyro_temp: le_u8 >>
        (RawIMU {
                accel: [accel_x ,accel_y, accel_z],
                gyro: [gyro_x, gyro_y, gyro_z],
                gyro_temp
        })
    )
);
