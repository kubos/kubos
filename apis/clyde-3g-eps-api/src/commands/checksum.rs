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
use nom::le_u16;
use rust_i2c::Command;

/// Checksum
///
/// This command instructs the node to self-inspect its ROM contents in order
/// to generate a checksum. The value retrieved can be used to determine whether
/// the contents of the ROM have changed during the operation of the device.

#[derive(Clone, Debug, Eq, PartialEq)]
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

fn parse_checksum(data: &[u8]) -> EpsResult<u16> {
    match le_u16(data) {
        Ok((_rem, res)) => Ok(res),
        Err(_) => Err(EpsError::parsing_failure("Checksum")),
    }
}

pub fn parse(data: &[u8]) -> EpsResult<Checksum> {
    if data.len() == 4 {
        Ok(Checksum {
            motherboard: parse_checksum(&data[2..])?,
            daughterboard: Some(parse_checksum(data)?),
        })
    } else if data.len() == 2 {
        Ok(Checksum {
            motherboard: parse_checksum(data)?,
            daughterboard: None,
        })
    } else {
        throw!(EpsError::parsing_failure("Checksum"))
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
    fn test_parse_motherboard_checksum() {
        assert_eq!(
            Checksum {
                motherboard: 0xCDAB,
                daughterboard: None,
            },
            parse(&vec![0xAB, 0xCD]).unwrap()
        );
    }

    #[test]
    fn test_parse_both_checksum() {
        assert_eq!(
            Checksum {
                motherboard: 0x3412,
                daughterboard: Some(0xCDAB),
            },
            parse(&vec![0xAB, 0xCD, 0x12, 0x34]).unwrap()
        );
    }

    #[test]
    fn test_parse_one_byte() {
        assert_eq!(
            EpsError::parsing_failure("Checksum"),
            parse(&vec![0]).err().unwrap()
        );
    }

    #[test]
    fn test_parse_three_bytes() {
        assert_eq!(
            EpsError::parsing_failure("Checksum"),
            parse(&vec![1, 2, 3]).err().unwrap()
        );
    }
}
