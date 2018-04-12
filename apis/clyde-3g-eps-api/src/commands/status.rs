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

use i2c_api::Command;

bitflags! {
    #[derive(Default)]
    pub struct Status: u8 {
        const LAST_COMMAND_FAILED = 0b0000001;
        const WATCHDOG_ERROR = 0b0000010;
        const BAD_COMMAND_DATA = 0b0000100;
        const BAD_COMMAND_CHANNEL = 0b0001000;
        const ERROR_READING_EEPROM = 0b0010000;
        const POWER_ON_RESET = 0b0100000;
        const BROWN_OUT_RESET = 0b1000000;
    }
}

impl Status {
    pub fn parse(data: &[u8]) -> Self {
        if data.len() > 0 {
            match Status::from_bits(data[0]) {
                Some(s) => s,
                None => Status::default(),
            }
        } else {
            Status::default()
        }
    }

    pub fn command() -> Command {
        Command {
            cmd: 0x01,
            data: vec![0x00],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_no_flags() {
        assert_eq!(Status::default(), Status::parse(&vec![]));
    }

    #[test]
    fn test_parse_last_cmd_failed() {
        assert_eq!(Status::LAST_COMMAND_FAILED, Status::parse(&vec![0b1]));
    }

    #[test]
    fn test_parse_watchdog_error() {
        assert_eq!(Status::WATCHDOG_ERROR, Status::parse(&vec![0b10]));
    }

    #[test]
    fn test_parse_bad_command_data() {
        assert_eq!(Status::BAD_COMMAND_DATA, Status::parse(&vec![0b100]));
    }

    #[test]
    fn test_parse_bad_command_channel() {
        assert_eq!(Status::BAD_COMMAND_CHANNEL, Status::parse(&vec![0b1000]));
    }

    #[test]
    fn test_parse_error_reading_eeprom() {
        assert_eq!(Status::ERROR_READING_EEPROM, Status::parse(&vec![0b10000]));
    }

    #[test]
    fn test_parse_power_on_reset() {
        assert_eq!(Status::POWER_ON_RESET, Status::parse(&vec![0b100000]));
    }

    #[test]
    fn test_parse_brown_out_reset() {
        assert_eq!(Status::BROWN_OUT_RESET, Status::parse(&vec![0b1000000]));
    }

    #[test]
    fn test_parse_all() {
        assert_eq!(Status::all(), Status::parse(&vec![0b1111111]));
    }
}
