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

use nom::{float, multispace, IResult};

#[derive(Debug, PartialEq)]
pub struct GeoRecord {
    lat: f32,
    lon: f32,
    time: u32,
    max_error: u32,
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

fn parse_coord<'a>(input: &'a [u8], prefix: &'static str) -> IResult<&'a [u8], f32> {
    let (input, _) = take_until_and_consume!(input, prefix)?;
    let (input, _) = multispace(input)?;
    let (input, d) = float(input)?;
    let (input, _) = multispace(input)?;
    let (input, m) = float(input)?;
    let (input, _) = multispace(input)?;
    let (input, s) = float(input)?;
    Ok((input, d + m / 60.0 + s / 3600.0))
}

impl GeoRecord {
    pub fn parse(input: &[u8]) -> IResult<&[u8], GeoRecord> {
        println!("TODO: parse geo record {:x}", Hex(input));
        let (input, _) = take_until_and_consume!(input, "GU")?;
        let (input, n) = parse_coord(input, "N:")?;
        let (input, w) = parse_coord(input, "W:")?;

        // Fields are left justified and the entire record is end padded with spaces to a fixed 90 byte length.
        // N:DDD MM SS
        // W:DDD MM SS
        // TIME: DD MM YYYY HH:MM:SS
        // ERR:<300m to <100km

        Ok((
            input,
            GeoRecord {
                lat: n,
                lon: -w,
                time: 0,
                max_error: 0,
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
                    lat: 33.479954,
                    lon: -94.182613,
                    time: 1520527330,
                    max_error: 1500,
                },
            )),
            GeoRecord::parse(b"print this stuff!!")
        )
    }
}
