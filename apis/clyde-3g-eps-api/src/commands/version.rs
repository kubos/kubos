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

use eps_api::EpsError;
use failure::Error;
use i2c_hal::Command;

/// Version
///
/// The version number of the firmware will be returned on this command.
/// The revision number returns the current revision of the firmware that is
/// present on the board. The firmware number returns the current firmware on the board.

#[derive(Debug, Eq, PartialEq)]
pub struct Version {
    revision: u8,
    firmware_number: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct VersionInfo {
    motherboard: Version,
    daughterboard: Option<Version>,
}

fn get_firmware(num1: u8, num2: u8) -> u16 {
    (num1 as u16) | ((num2 as u16) & 0xF) << 8
}

fn get_revision(num: u8) -> u8 {
    (num & 0xF0) >> 4
}

pub fn parse(data: &[u8]) -> Result<VersionInfo, Error> {
    if data.len() == 2 {
        Ok(VersionInfo {
            motherboard: Version {
                firmware_number: get_firmware(data[0], data[1]),
                revision: get_revision(data[1]),
            },
            daughterboard: None,
        })
    } else if data.len() == 4 {
        Ok(VersionInfo {
            motherboard: Version {
                firmware_number: get_firmware(data[2], data[3]),
                revision: get_revision(data[3]),
            },
            daughterboard: Some(Version {
                firmware_number: get_firmware(data[0], data[1]),
                revision: get_revision(data[1]),
            }),
        })
    } else {
        throw!(EpsError::BadData)
    }
}

pub fn command() -> Command {
    Command {
        cmd: 0x04,
        data: vec![0x00],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_motherboard() {
        assert_eq!(
            VersionInfo {
                motherboard: Version {
                    revision: 12,
                    firmware_number: 2152,
                },
                daughterboard: None,
            },
            parse(&vec![0x68, 0xC8]).unwrap()
        )
    }

    #[test]
    fn test_parse_both() {
        assert_eq!(
            VersionInfo {
                motherboard: Version {
                    revision: 12,
                    firmware_number: 2152,
                },
                daughterboard: Some(Version {
                    revision: 15,
                    firmware_number: 100,
                }),
            },
            parse(&vec![0x64, 0xF0, 0x68, 0xC8]).unwrap()
        );
    }

    #[test]
    fn test_parse_one() {
        assert_eq!(
            EpsError::BadData,
            parse(&vec![0x0])
                .err()
                .unwrap()
                .downcast::<EpsError>()
                .unwrap()
        )
    }

    #[test]
    fn test_parse_three() {
        assert_eq!(
            EpsError::BadData,
            parse(&vec![0x0, 0x1, 0x3])
                .err()
                .unwrap()
                .downcast::<EpsError>()
                .unwrap()
        )
    }
}
