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

/// Checksum
///
/// This command instructs the node to self-inspect its ROM contents in order
/// to generate a checksum. The value retrieved can be used to determine whether
/// the contents of the ROM have changed during the operation of the device.

#[derive(Debug, Eq, PartialEq)]
pub struct Checksum {
    /// Motherboard ROM checksum
    pub motherboard: u16,
    /// Daughterboard ROM checksum
    pub daughterboard: Option<u16>,
}

impl Default for Checksum {
    fn default() -> Self {
        Checksum {
            motherboard: 0,
            daughterboard: None,
        }
    }
}

fn parse_checksum(data1: u8, data2: u8) -> u16 {
    u16::from(data1) | (u16::from(data2) << 8)
}

pub fn parse(data: &[u8]) -> EpsResult<Checksum> {
    println!("Checksum {:?}", data);
    if data.len() == 4 {
        Ok(Checksum {
            motherboard: parse_checksum(data[2], data[3]),
            daughterboard: Some(parse_checksum(data[0], data[1])),
        })
    } else if data.len() == 2 {
        Ok(Checksum {
            motherboard: parse_checksum(data[0], data[1]),
            daughterboard: None,
        })
    } else {
        throw!(EpsError::invalid_data(data))
    }
}

pub fn command() -> Command {
    Command {
        cmd: 0x05,
        data: vec![0x00],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_motherboard_version() {
        assert_eq!(
            Checksum {
                motherboard: 0b1010010110100101,
                daughterboard: None,
            },
            parse(&vec![0b10100101, 0b10100101]).unwrap()
        );
    }

    #[test]
    fn test_parse_both_versions() {
        assert_eq!(
            Checksum {
                motherboard: 3084,
                daughterboard: Some(771),
            },
            parse(&vec![0b0011, 0b0011, 0b1100, 0b1100]).unwrap()
        );
    }

    #[test]
    fn test_parse_one_byte() {
        assert_eq!(
            EpsError::InvalidData {
                data: String::from("\u{0}"),
            },
            parse(&vec![0]).err().unwrap()
        );
    }

    #[test]
    fn test_parse_three_bytes() {
        assert_eq!(
            EpsError::InvalidData {
                data: String::from("\u{1}\u{2}\u{3}"),
            },
            parse(&vec![1, 2, 3]).err().unwrap()
        );
    }
}
