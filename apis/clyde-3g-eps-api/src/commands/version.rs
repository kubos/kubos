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
    daughterboard: Version,
}

pub fn parse(data: &[u8]) -> Result<VersionInfo, Error> {
    if data.len() > 0 {
        let firmware_number = (data[0] as u16) | ((data[1] as u16) & 0xF) << 8;
        let revision = (data[1] & 0xF0) >> 4;
        Ok(VersionInfo {
            motherboard: Version {
                revision,
                firmware_number,
            },
            daughterboard: Version {
                revision,
                firmware_number,
            },
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
    fn test_parse() {
        assert_eq!(
            Ok(VersionInfo {
                motherboard: Version {
                    revision: 0,
                    firmware_number: 0,
                },
                daughterboard: Version {
                    revision: 0,
                    firmware_number: 0,
                },
            },),
            parse(&vec![0, 0])
        );
    }
}
