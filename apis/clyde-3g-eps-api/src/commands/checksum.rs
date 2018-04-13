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
use i2c_hal::Command;

#[derive(Debug, Eq, PartialEq)]
pub struct Checksum {
    /// Motherboard ROM checksum
    pub motherboard: u16,
    /// Daughterboard ROM checksum
    pub daughterboard: u16,
}

impl Default for Checksum {
    fn default() -> Self {
        Checksum {
            motherboard: 0,
            daughterboard: 0,
        }
    }
}

impl Checksum {
    pub fn parse(data: &[u8]) -> Result<Self, EpsError> {
        if data.len() == 4 {
            Ok(Checksum {
                motherboard: data[2] as u16 | (data[3] as u16) << 8,
                daughterboard: data[0] as u16 | (data[1] as u16) << 8,
            })
        } else if data.len() == 2 {
            Ok(Checksum {
                motherboard: data[0] as u16 | (data[1] as u16) << 8,
                daughterboard: 0,
            })
        } else {
            Err(EpsError::BadData)
        }
    }

    pub fn command() -> Command {
        Command {
            cmd: 0x05,
            data: vec![0x00],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_motherboard_version() {
        assert_eq!(
            Ok(Checksum {
                motherboard: 0b1010010110100101,
                daughterboard: 0,
            }),
            Checksum::parse(&vec![0b10100101, 0b10100101])
        );
    }

    #[test]
    fn test_parse_both_versions() {
        assert_eq!(
            Ok(Checksum {
                motherboard: 3084,
                daughterboard: 771,
            }),
            Checksum::parse(&vec![0b0011, 0b0011, 0b1100, 0b1100])
        );
    }

    #[test]
    fn test_parse_one_byte() {
        assert_eq!(Err(EpsError::BadData), Checksum::parse(&vec![0]));
    }

    #[test]
    fn test_parse_three_bytes() {
        assert_eq!(Err(EpsError::BadData), Checksum::parse(&vec![1, 2, 3]));
    }
}
