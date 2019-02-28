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

/// Last Error
///
/// If an error has been generated after attempting to execute a user’s command
/// the value 0xFFFF is returned. To find out the details of the last error,
/// send the command 0x03 followed by the data byte 0x00. This will return
/// the code of the last error generated. The first two bytes returned represent
/// the Motherboard’s error code, the second two bytes represent the Daughterboard’s.

/// Bitflags struct holding last error information.
bitflags! {
    /// Bitflags struct holding last error information.
    #[derive(Default)]
    pub struct ErrorCode: u8 {
        /// CRC code does not match data
        const BAD_CRC = 0x10;
        /// Unknown command received
        const UNKNOWN_COMMAND = 0x01;
        /// Supplied data incorrect when processing command
        const COMMAND_DATA_INCORRECT = 0x02;
        /// Selected channel does not exist
        const CHANNEL_DOES_NOT_EXIST = 0x03;
        /// Selected channel is currently inactive
        const CHANNEL_INACTIVE = 0x04;
        /// A reset had to occur
        const RESET_OCCURRED = 0x13;
        /// There was an error with the ADC acquisition
        const BAD_ADC_ACQUISITION = 0x14;
        /// Reading from EEPROM generated an error
        const FAIL_READING_EEPROM = 0x20;
        /// Generic warning about an error on the internal SPI bus
        const INTERNAL_SPI_ERROR = 0x30;
    }
}

/// Struct holding EPS last error information
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LastError {
    /// Last error reported by motherboard
    pub motherboard: ErrorCode,
    /// Last error reported by daughterboard
    pub daughterboard: Option<ErrorCode>,
}

pub fn parse(data: &[u8]) -> EpsResult<LastError> {
    if data.len() == 2 {
        Ok(LastError {
            motherboard: ErrorCode::from_bits(data[1]).unwrap_or_default(),
            daughterboard: None,
        })
    } else if data.len() == 4 {
        Ok(LastError {
            motherboard: ErrorCode::from_bits(data[1]).unwrap_or_default(),
            daughterboard: ErrorCode::from_bits(data[3]),
        })
    } else {
        Err(EpsError::parsing_failure("Last Error"))
    }
}

pub fn command() -> Command {
    Command {
        cmd: 0x03,
        data: vec![0x00],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_motherboard() {
        assert_eq!(
            LastError {
                motherboard: ErrorCode::BAD_CRC,
                daughterboard: None,
            },
            parse(&vec![0x00, 0x10]).unwrap()
        );
    }

    #[test]
    fn test_parse_motherboard_daughterboard() {
        assert_eq!(
            LastError {
                motherboard: ErrorCode::COMMAND_DATA_INCORRECT,
                daughterboard: Some(ErrorCode::CHANNEL_INACTIVE),
            },
            parse(&vec![0x00, 0x02, 0x00, 0x04]).unwrap()
        );
    }

    #[test]
    fn test_parse_bad_data_len() {
        assert_eq!(
            EpsError::parsing_failure("Last Error"),
            parse(&vec![]).err().unwrap()
        );
    }
}
