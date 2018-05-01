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
// CRC32 algorithm taken from Section 1.7 of the NovAtel OEM6 Firmware Reference Manual, Rev 12

const CRC32_POLY: u32 = 0xEDB8_8320;

pub struct CRC32(pub u32);

impl CRC32 {
    fn calc_val(&self, val: u8) -> u32 {
        let mut crc: u32 = val.into();
        for _num in 0..8 {
            if crc & 1 == 1 {
                crc = crc.wrapping_shr(1) ^ CRC32_POLY;
            } else {
                crc = crc.wrapping_shr(1);
            }
        }

        crc
    }

    pub fn calc_crc(&self, msg: &[u8]) -> Self {
        let mut crc: u32 = 0;

        for elem in msg.iter() {
            let val1 = crc.wrapping_shr(8);
            let arg: u32 = crc ^ (elem.clone() as u32);
            let val2 = self.calc_val(arg as u8);
            crc = val2 ^ val1;
        }

        CRC32(crc)
    }
}
