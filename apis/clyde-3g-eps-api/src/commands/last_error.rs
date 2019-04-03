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

use eps_api::{EpsError, EpsResult};
use rust_i2c::Command;

/// Last Error
///
/// If an error has been generated after attempting to execute a user’s command
/// the value 0xFFFF is returned. To find out the details of the last error,
/// send the command 0x03 followed by the data byte 0x00. This will return
/// the code of the last error generated. The first two bytes returned represent
/// the Motherboard’s error code, the second two bytes represent the Daughterboard’s.

/// Possible last error values
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorCode {
    /// No error was encountered
    None = 0x00,
    /// Unknown command received
    UnknownCommand = 0x01,
    /// Supplied data incorrect when processing command
    CommandDataIncorrect = 0x02,
    /// Selected channel does not exist
    ChannelDoesNotExist = 0x03,
    /// Selected channel is currently inactive
    ChannelInactive = 0x04,
    /// CRC code does not match data
    BadCRC = 0x10,
    /// A reset had to occur
    ResetOccurred = 0x13,
    /// There was an error with the ADC acquisition
    BadADCAcquisition = 0x14,
    /// Reading from EEPROM generated an error
    FailReadingEEPROM = 0x20,
    /// Generic warning about an error on the internal SPI bus
    InternalSPIError = 0x30,
    /// The command to fetch the last error failed
    CommandError = 0xFF,
    /// Catch all for future error values
    UnknownError,
}

impl ErrorCode {
    fn from_u8(value: u8) -> ErrorCode {
        match value {
            0x00 => ErrorCode::None,
            0x01 => ErrorCode::UnknownCommand,
            0x02 => ErrorCode::CommandDataIncorrect,
            0x03 => ErrorCode::ChannelDoesNotExist,
            0x04 => ErrorCode::ChannelInactive,
            0x10 => ErrorCode::BadCRC,
            0x13 => ErrorCode::ResetOccurred,
            0x14 => ErrorCode::BadADCAcquisition,
            0x20 => ErrorCode::FailReadingEEPROM,
            0x30 => ErrorCode::InternalSPIError,
            0xFF => ErrorCode::CommandError,
            _ => ErrorCode::UnknownError,
        }
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
            motherboard: ErrorCode::from_u8(data[1]),
            daughterboard: None,
        })
    } else if data.len() == 4 {
        Ok(LastError {
            motherboard: ErrorCode::from_u8(data[1]),
            daughterboard: Some(ErrorCode::from_u8(data[3])),
        })
    } else {
        Err(EpsError::parsing_failure("Last Error"))
    }
}

pub fn command() -> (Command, usize) {
    (
        Command {
            cmd: 0x03,
            data: vec![0x00],
        },
        4,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_motherboard() {
        assert_eq!(
            LastError {
                motherboard: ErrorCode::BadCRC,
                daughterboard: None,
            },
            parse(&vec![0x00, 0x10]).unwrap()
        );
    }

    #[test]
    fn test_parse_motherboard_daughterboard() {
        assert_eq!(
            LastError {
                motherboard: ErrorCode::CommandDataIncorrect,
                daughterboard: Some(ErrorCode::ChannelInactive),
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
