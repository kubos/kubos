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

use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct GeoRecord {
    lat: Latitude,
    lon: Longitude,
    time: TimeStamp,
    precision: u32,
}

#[derive(Debug, PartialEq)]
pub enum Longitude {
    East(f32),
    West(f32),
}

#[derive(Debug, PartialEq)]
pub enum Latitude {
    North(f32),
    South(f32),
}

#[derive(Debug, PartialEq)]
pub struct TimeStamp {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

use std::fmt::{Formatter, LowerHex, Result};
struct Hex<'a>(pub &'a [u8]);

impl<'a> LowerHex for Hex<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl GeoRecord {
    pub fn parse(input: &[u8]) -> IResult<&[u8], GeoRecord> {
        println!("TODO: parse geo record {:x}", Hex(input));
        // Fields are left justified and the entire record is end padded with spaces to a fixed 90 byte length.
        // N:DDD MM SS
        // W:DDD MM SS
        // TIME: DD MM YYYY HH:MM:SS
        // ERR:<300m to <100km

        Ok((
            input,
            GeoRecord {
                lat: Latitude::North(33.479954),
                lon: Longitude::West(94.182613),
                time: TimeStamp {
                    year: 2018,
                    month: 3,
                    day: 7,
                    hour: 20,
                    minute: 22,
                    second: 54,
                },
                precision: 1500,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_parses() {
        assert_eq!(
            Ok((
                &b"extra"[..],
                GeoRecord {
                    lat: Latitude::North(33.479954),
                    lon: Longitude::West(94.182613),
                    time: TimeStamp {
                        year: 2018,
                        month: 3,
                        day: 7,
                        hour: 20,
                        minute: 22,
                        second: 54,
                    },
                    precision: 1500,
                },
            )),
            GeoRecord::parse(b"print this stuff!!")
        )
    }
}
