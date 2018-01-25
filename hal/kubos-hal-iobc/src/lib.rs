//
// Copyright (C) 2017 Kubos Corporation
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

extern crate libc;

use std::mem;

mod ffi;

/// Structure returned by supervisor_version
#[derive(Debug)]
pub struct SupervisorVersion {
    pub dummy: u8,
    pub spi_command_status: u8,
    pub index_of_subsystem: u8,
    pub major_version: u8,
    pub minor_version: u8,
    pub patch_version: u8,
    pub git_head_version: u32,
    pub serial_number: u16,
    pub compile_information: Vec<i8>,
    pub clock_speed: u8,
    pub code_type: i8,
    pub crc: u8,
}

/// Structure for individual enable statuses
/// Used in SupervisorHousekeeping
#[derive(Debug)]
pub struct SupervisorEnableStatus {
    pub power_obc: u8,
    pub power_rtc: u8,
    pub supervisor_mode: u8,
    pub busy_rtc: u8,
    pub power_off_rtc: u8,
}

/// Structure returned by supervisor_housekeeping
#[derive(Debug)]
pub struct SupervisorHousekeeping {
    pub dummy: u8,
    pub spi_command_status: u8,
    pub enable_status: SupervisorEnableStatus,
    pub supervisor_uptime: u32,
    pub iobc_uptime: u32,
    pub iobc_reset_count: u32,
    pub adc_data: Vec<u16>,
    pub adc_update_flag: u8,
    pub crc8: u8,
}

/// Supervisor emergency reset interface
pub fn supervisor_emergency_reset() -> Result<(), String> {
    match unsafe { ffi::supervisor_emergency_reset() } {
        true => Ok(()),
        false => Err(String::from("Problem with supervisor emergency reset")),
    }
}

/// Supervisor reset interface
pub fn supervisor_reset() -> Result<(), String> {
    match unsafe { ffi::supervisor_reset() } {
        true => Ok(()),
        false => Err(String::from("Problem with supervisor reset")),
    }
}

/// Supervisor powercycle interface
pub fn supervisor_powercycle() -> Result<(), String> {
    match unsafe { ffi::supervisor_powercycle() } {
        true => Ok(()),
        false => Err(String::from("Problem with supervisor powercycle")),
    }
}

/// Converts raw bytes from iOBC into SupervisorVersion
fn convert_raw_version(raw: ffi::supervisor_version) -> SupervisorVersion {
    SupervisorVersion {
        dummy: raw.0[0] as u8,
        spi_command_status: raw.0[1] as u8,
        index_of_subsystem: raw.0[2] as u8,
        major_version: raw.0[3] as u8,
        minor_version: raw.0[4] as u8,
        patch_version: raw.0[5] as u8,
        git_head_version: {
            (raw.0[6] as u32) | (raw.0[7] as u32) << 8 | (raw.0[8] as u32) << 16
                | (raw.0[9] as u32) << 24
        },
        serial_number: { (raw.0[10] as u16) | (raw.0[11] as u16) << 8 },
        compile_information: {
            (&raw.0[12..(12 + ffi::LENGTH_COMPILE_INFORMATION)])
                .iter()
                .map(|x| *x as i8)
                .collect::<Vec<i8>>()
        },
        clock_speed: raw.0[31] as u8,
        code_type: raw.0[32] as i8,
        crc: raw.0[33] as u8,
    }
}

/// Interface for retrieving iOBC supervisor version data
pub fn supervisor_version() -> Result<SupervisorVersion, String> {
    let mut version: ffi::supervisor_version = unsafe { mem::uninitialized() };
    let version_result = unsafe { ffi::supervisor_get_version(&mut version) };
    if !version_result {
        Err(String::from("Problem retrieving supervisor version"))
    } else {
        Ok(convert_raw_version(version))
    }
}

/// Converts raw bytes from iOBC into SupervisorHousekeeping
fn convert_raw_housekeeping(raw: ffi::supervisor_housekeeping) -> SupervisorHousekeeping {
    SupervisorHousekeeping {
        dummy: raw.0[0] as u8,
        spi_command_status: raw.0[1] as u8,
        enable_status: SupervisorEnableStatus {
            // We bitmask rather than split the int
            // across bitfields
            power_obc: (raw.0[2] as u8) & 0x1,
            power_rtc: ((raw.0[2] as u8) & 0x2) >> 1,
            supervisor_mode: ((raw.0[2] as u8) & 0x4) >> 2,
            busy_rtc: ((raw.0[2] as u8) & 0x20) >> 5,
            power_off_rtc: ((raw.0[2] as u8) & 0x40) >> 6,
        },
        supervisor_uptime: {
            (raw.0[3] as u32) | (raw.0[4] as u32) << 8 | (raw.0[5] as u32) << 16
                | (raw.0[6] as u32) << 24
        },
        iobc_uptime: {
            (raw.0[7] as u32) | (raw.0[8] as u32) << 8 | (raw.0[9] as u32) << 16
                | (raw.0[10] as u32) << 24
        },
        iobc_reset_count: {
            (raw.0[11] as u32) | (raw.0[12] as u32) << 8 | (raw.0[13] as u32) << 16
                | (raw.0[14] as u32) << 24
        },
        adc_data: {
            // combining bytes into 16-bit uints
            let mut v = Vec::<u16>::new();
            for i in 0..(ffi::SUPERVISOR_NUMBER_OF_ADC_CHANNELS) {
                v.push((raw.0[15 + 2 * i] as u16) | (raw.0[15 + 2 * i + 1] as u16) << 8);
            }
            v
        },
        adc_update_flag: raw.0[35] as u8,
        crc8: raw.0[36] as u8,
    }
}

/// Interface for fetching iOBC supervisor housekeeping data
pub fn supervisor_housekeeping() -> Result<SupervisorHousekeeping, String> {
    let mut raw: ffi::supervisor_housekeeping = unsafe { mem::uninitialized() };
    let result = unsafe { ffi::supervisor_get_housekeeping(&mut raw) };

    if !result {
        Err(String::from("Problem retrieving supervisor housekeeping"))
    } else {
        Ok(convert_raw_housekeeping(raw))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Accuracy testing conversion of raw bytes into non-C
    /// SupervisorVersion structure
    #[test]
    fn test_convert_version() {
        let raw: ffi::supervisor_version = ffi::supervisor_version([
                // dummy (u8)
                0,
                // spi_command_status (u8)
                1,
                // index_of_subsystem (u8)
                2,
                // major_version (u8)
                3,
                // minor version (u8)
                4,
                // patch version (u8)
                5,
                // git_head_version (u32)
                6,
                7,
                8,
                9,
                // serial_number (u16)
                10,
                11,
                // compile_information (i8 * 19)
                12,
                13,
                14,
                15,
                16,
                17,
                18,
                19,
                20,
                21,
                22,
                23,
                24,
                25,
                26,
                27,
                28,
                29,
                30,
                // clock_speed (u8)
                31,
                // code_type (i8)
                32,
                // crc8 (u8)
                33,
            ]);

        let version = convert_raw_version(raw);

        assert_eq!(version.dummy, 0);
        assert_eq!(version.spi_command_status, 1);
        assert_eq!(version.major_version, 3);
        assert_eq!(version.minor_version, 4);
        assert_eq!(version.patch_version, 5);
        assert_eq!(version.git_head_version, 151521030);
        assert_eq!(version.serial_number, 2826);
        for i in 12..31 {
            assert_eq!(version.compile_information[i - 12], i as i8);
        }
        assert_eq!(version.clock_speed, 31);
        assert_eq!(version.code_type, 32);
        assert_eq!(version.crc, 33);
    }

    /// Accuracy testing conversion of raw bytes into non-C
    /// SupervisorHousekeeping structure
    #[test]
    fn test_convert_housekeeping() {
        let raw: ffi::supervisor_housekeeping = ffi::supervisor_housekeeping([
                // dummy (u8), spi_command_status (u8), enable_status (u8)
                0,
                1,
                // enable_status (u8) is a bitfield in the C structure
                // power_obc : 1
                // power_rtc : 1
                // supervisor_mode : 1
                // padding : 2
                // busy_rtc : 1
                // power_off_rtc : 1
                // padding: 1
                // Using 34 gives us -
                // 0 0 1 0 0 0 1 0
                // which results in alternating 1/0 field values
                34,
                // super_uptime (u32)
                3,
                2,
                1,
                0,
                // iobc_uptime (u32)
                4,
                3,
                2,
                1,
                // iobc_reset_count (u32)
                5,
                4,
                3,
                2,
                // adc_data (u16 * 10)
                0,
                1,
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                9,
                10,
                11,
                12,
                13,
                14,
                15,
                16,
                17,
                18,
                19,
                // adc_update_flag (u8)
                12,
                // crc8
                13,
            ]);

        let housekeeping = convert_raw_housekeeping(raw);

        assert_eq!(housekeeping.dummy, 0);
        assert_eq!(housekeeping.spi_command_status, 1);
        assert_eq!(housekeeping.enable_status.power_obc, 0);
        assert_eq!(housekeeping.enable_status.power_rtc, 1);
        assert_eq!(housekeeping.enable_status.supervisor_mode, 0);
        assert_eq!(housekeeping.enable_status.busy_rtc, 1);
        assert_eq!(housekeeping.enable_status.power_off_rtc, 0);
        assert_eq!(housekeeping.supervisor_uptime, 66051);
        assert_eq!(housekeeping.iobc_uptime, 16909060);
        assert_eq!(housekeeping.iobc_reset_count, 33752069);
        assert_eq!(housekeeping.adc_data[0], 256);
        assert_eq!(housekeeping.adc_data[1], 770);
        assert_eq!(housekeeping.adc_data[2], 1284);
        assert_eq!(housekeeping.adc_data[3], 1798);
        assert_eq!(housekeeping.adc_data[4], 2312);
        assert_eq!(housekeeping.adc_data[5], 2826);
        assert_eq!(housekeeping.adc_data[6], 3340);
        assert_eq!(housekeeping.adc_data[7], 3854);
        assert_eq!(housekeeping.adc_data[8], 4368);
        assert_eq!(housekeeping.adc_data[9], 4882);
        assert_eq!(housekeeping.adc_update_flag, 12);
        assert_eq!(housekeeping.crc8, 13);
    }
}
