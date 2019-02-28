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

use bitflags::bitflags;
use eps_api::{EpsError, EpsResult};
use rust_i2c::Command;

/// Board Status
///
/// The status bytes are designed to supply operational data about the I2C Node.
/// To retrieve the data that represents the status, the command 0x01 should be
/// sent followed by 0x00. The meaning of each bit of the returned status bytes
/// is shown below. Please note that Data[3] is the first byte returned from the
/// EPS and Data[0] is the last.

/// EPS Board Status Codes
bitflags! {
    /// EPS Board Status Codes
    #[derive(Default)]
    pub struct StatusCode: u8 {
        /// Last Command Failed
        const LAST_COMMAND_FAILED = 0b00000001;
        /// Watchdog Error
        const WATCHDOG_ERROR = 0b00000010;
        /// Bad Command Data
        const BAD_COMMAND_DATA = 0b00000100;
        /// Bad Command Channel
        const BAD_COMMAND_CHANNEL = 0b00001000;
        /// Error Reading EEPROM
        const ERROR_READING_EEPROM = 0b00010000;
        /// Power On Reset
        const POWER_ON_RESET = 0b00100000;
        /// Brown Out Reset
        const BROWN_OUT_RESET = 0b01000000;
    }
}

/// Status of EPS Motherboard and Daughterboard
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoardStatus {
    /// Motherboard status code
    pub motherboard: StatusCode,
    /// Daughterboard status code
    pub daughterboard: Option<StatusCode>,
}

pub fn parse(data: &[u8]) -> EpsResult<BoardStatus> {
    if data.len() == 2 {
        Ok(BoardStatus {
            motherboard: StatusCode::from_bits(data[0]).unwrap_or_default(),
            daughterboard: None,
        })
    } else if data.len() == 4 {
        Ok(BoardStatus {
            motherboard: StatusCode::from_bits(data[2]).unwrap_or_default(),
            daughterboard: Some(StatusCode::from_bits(data[0]).unwrap_or_default()),
        })
    } else {
        Err(EpsError::parsing_failure("Board Status"))
    }
}

pub fn command() -> Command {
    Command {
        cmd: 0x01,
        data: vec![0x00],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_both_no_flags() {
        assert_eq!(
            BoardStatus {
                motherboard: StatusCode::default(),
                daughterboard: Some(StatusCode::default()),
            },
            parse(&vec![0x0, 0x0, 0x0, 0x0]).unwrap()
        )
    }

    #[test]
    fn test_parse_both() {
        assert_eq!(
            BoardStatus {
                motherboard: StatusCode::LAST_COMMAND_FAILED,
                daughterboard: Some(StatusCode::WATCHDOG_ERROR),
            },
            parse(&vec![0x2, 0x0, 0x1, 0x0]).unwrap()
        )
    }

    #[test]
    fn test_parse_motherboard() {
        assert_eq!(
            BoardStatus {
                motherboard: StatusCode::BAD_COMMAND_DATA,
                daughterboard: None,
            },
            parse(&vec![0x4, 0x0]).unwrap()
        )
    }
}
