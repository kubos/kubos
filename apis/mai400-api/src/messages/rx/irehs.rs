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

/// IR Earth Horizon Sensor telemetry data
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IREHSTelemetry {
    /// IREHS A Thermopile Sensors 12-bit ADC (0.8059 mV per lsb)
    /// [earth limb, earth reference, space reference, wide FOV]
    pub thermopiles_a: [u16; 4],
    /// IREHS B Thermopile Sensors 12-bit ADC (0.8059 mV per lsb)
    /// [earth limb, earth reference, space reference, wide FOV]
    pub thermopiles_b: [u16; 4],
    /// Thermistor Temp 12-bit ADC for A (0.8059 mV per lsb)
    /// [earth limb, earth reference, space reference, wide FOV]
    pub temp_a: [u16; 4],
    /// Thermistor Temp 12-bit ADC for B (0.8059 mV per lsb)
    /// [earth limb, earth reference, space reference, wide FOV]
    pub temp_b: [u16; 4],
    /// Calculated dip angle of Earth limb for A in degrees
    pub dip_angle_a: i32,
    /// Calculated dip angle of Earth limb for B in degrees
    pub dip_angle_b: i32,
    /// Degradation codes for thermopiles
    /// [{A}, {B}]
    pub solution_degraded: [ThermopileFlags; 8],
}

impl IREHSTelemetry {
    /// Constructor. Converts a raw data array received from the MAI-400 into a usable structure
    pub fn new(mut msg: Vec<u8>) -> Option<Self> {
        // Verify message starts with sync bytes
        let mut data = msg.split_off(2);

        let mut wrapper = Cursor::new(msg);
        let check = wrapper.read_u16::<LittleEndian>().unwrap_or(0);
        if check != AUX_SYNC {
            return None;
        }

        // Get the CRC bytes
        let len = data.len() - 2;
        let mut crc = Cursor::new(data.split_off(len));
        let crc = crc.read_u16::<LittleEndian>().unwrap_or(0);

        // Note: Yes, this is a different way of calculating the checksum than everywhere else
        let calc = State::<ARC>::calculate(&data);

        // Verify the CRC bytes at the end of the message
        if calc == crc {
            // Convert the raw data to an official struct
            match irehs_telem(&data) {
                Ok(conv) => Some(conv.1),
                _ => None,
            }
        } else {
            None
        }
    }
}

named!(irehs_telem(&[u8]) -> IREHSTelemetry,
    do_parse!(
        le_i16 >>
        le_i16 >>
        thermopiles_a_earth_limb: le_u16 >>
        thermopiles_a_earth_ref: le_u16 >>
        thermopiles_a_space_ref: le_u16 >>
        thermopiles_a_wide_fov: le_u16 >>
        thermopiles_b_earth_limb: le_u16 >>
        thermopiles_b_earth_ref: le_u16 >>
        thermopiles_b_space_ref: le_u16 >>
        thermopiles_b_wide_fov: le_u16 >>
        temp_a_earth_limb: le_u16 >>
        temp_a_earth_ref: le_u16 >>
        temp_a_space_ref: le_u16 >>
        temp_a_wide_fov: le_u16 >>
        temp_b_earth_limb: le_u16 >>
        temp_b_earth_ref: le_u16 >>
        temp_b_space_ref: le_u16 >>
        temp_b_wide_fov: le_u16 >>
        dip_angle_a: le_i32 >>
        dip_angle_b: le_i32 >>
        solution_degraded_earth_limb_a: le_u8 >>
        solution_degraded_earth_ref_a: le_u8 >>
        solution_degraded_space_ref_a: le_u8 >>
        solution_degraded_wide_fov_a: le_u8 >>
        solution_degraded_earth_limb_b: le_u8 >>
        solution_degraded_earth_ref_b: le_u8 >>
        solution_degraded_space_ref_b: le_u8 >>
        solution_degraded_wide_fov_b: le_u8 >>
        (IREHSTelemetry {
                thermopiles_a: [
                    thermopiles_a_earth_limb,
                    thermopiles_a_earth_ref,
                    thermopiles_a_space_ref,
                    thermopiles_a_wide_fov
                ],
                thermopiles_b: [
                    thermopiles_b_earth_limb,
                    thermopiles_b_earth_ref,
                    thermopiles_b_space_ref,
                    thermopiles_b_wide_fov
                ],
                temp_a: [
                    temp_a_earth_limb,
                    temp_a_earth_ref,
                    temp_a_space_ref,
                    temp_a_wide_fov
                ],
                temp_b: [
                    temp_b_earth_limb,
                    temp_b_earth_ref,
                    temp_b_space_ref,
                    temp_b_wide_fov
                ],
                dip_angle_a,
                dip_angle_b,
                solution_degraded: [
                    ThermopileFlags::from_bits_truncate(solution_degraded_earth_limb_a),
                    ThermopileFlags::from_bits_truncate(solution_degraded_earth_ref_a),
                    ThermopileFlags::from_bits_truncate(solution_degraded_space_ref_a),
                    ThermopileFlags::from_bits_truncate(solution_degraded_wide_fov_a),
                    ThermopileFlags::from_bits_truncate(solution_degraded_earth_limb_b),
                    ThermopileFlags::from_bits_truncate(solution_degraded_earth_ref_b),
                    ThermopileFlags::from_bits_truncate(solution_degraded_space_ref_b),
                    ThermopileFlags::from_bits_truncate(solution_degraded_wide_fov_b)
                ]
        })
    )
);

bitflags! {
    /// Thermopile error flags
    #[derive(Default)]
    pub struct ThermopileFlags: u8 {
        /// Dip angle exceeded senser system limit
        const DIP_ANGLE_LIMIT = 0x01;
        /// Sun in FOV using sun ephemeris
        const SUN_IN_EPHEMERIS = 0x02;
        /// Thermopile is saturated
        const THERMOPILE_SAT = 0x04;
        /// No communications
        const NO_COMM = 0x08;
        /// Earth reference is bad
        const BAD_EARTH_REF = 0x10;
        /// Using auxiliary wide FOV sensor
        const AUX_WIDE_FOV = 0x20;
    }
}

impl ThermopileFlags {
    /// Convert the flags byte into a vector containing the string representations
    /// of all flags present.
    ///
    /// # Examples
    ///
    /// ```
    /// use mai400_api::*;
    ///
    /// # fn func() -> MAIResult<()> {
    /// let flags = ThermopileFlags::NO_COMM | ThermopileFlags::BAD_EARTH_REF;
    ///
    /// let conv = flags.to_vec();
    ///
    /// assert_eq!(conv, vec!["NO_COMM", "BAD_EARTH_REF"]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub fn to_vec(self) -> Vec<String> {
        format!("{:?}", self)
            .to_owned()
            .split(" | ")
            .map(|x| x.to_string())
            .collect()
    }
}
