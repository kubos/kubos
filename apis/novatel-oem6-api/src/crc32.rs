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

fn calc_val(val: u8) -> u32 {
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

pub fn calc_crc(msg: &[u8]) -> u32 {
    let mut crc: u32 = 0;

    for elem in msg.iter() {
        let val1 = crc.wrapping_shr(8);
        let arg: u32 = crc ^ u32::from(*elem);
        let val2 = calc_val(arg as u8);
        crc = val2 ^ val1;
    }

    crc
}

#[test]
fn test_crc() {
    let input = [
        0xAA, 0x44, 0x12, 0x1C, 0x24, 0x0, 0x0, 0xC0, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20, 0x0, 0x0, 0x0, 0xF1, 0x0, 0x0,
        0x0,
    ];

    assert_eq!(calc_crc(&input), 0x8BB09602);
}
