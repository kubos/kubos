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

use nom::{float, multispace, Err, ErrorKind, IResult};
use nom::simple_errors::Context;
use std::str::from_utf8;
use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Debug, PartialEq)]
pub struct GeoRecord {
    lon: f32,
    lat: f32,
    time: i64,
    max_error: u32,
}

fn parse_coord(input: &[u8]) -> IResult<&[u8], f32> {
    let (input, _) = multispace(input)?;
    let (input, d) = float(input)?;
    let (input, _) = multispace(input)?;
    let (input, m) = float(input)?;
    let (input, _) = multispace(input)?;
    let (input, s) = float(input)?;
    Ok((input, d + m / 60.0 + s / 3600.0))
}

fn parse_date(input: &[u8]) -> IResult<&[u8], i64> {
    let (input, _) = take_until_and_consume!(input, "TIME:")?;
    let (input, date) = map_res!(input, take_until!("\n"), from_utf8)?;
    let dt = DateTime::<Utc>::from_utc(
        NaiveDateTime::parse_from_str(date, "%d %m %Y %H:%M:%S")
            .or(Err(Err::Error(Context::Code(input, ErrorKind::Tag))))?,
        Utc,
    );
    Ok((input, dt.timestamp()))
}

impl GeoRecord {
    // Fields are left justified and the entire record is end padded with spaces to a fixed 90 byte length.
    // N:DDD MM SS
    // W:DDD MM SS
    // TIME: DD MM YYYY HH:MM:SS
    // ERR:<300m to <100km
    pub fn parse(input: &[u8]) -> IResult<&[u8], GeoRecord> {
        let (input, _) = take_until_and_consume!(input, "GU")?;
        let (input, message) = take!(input, 90)?;
        let (message, _) = take_until_and_consume!(message, "N:")?;
        let (message, n) = parse_coord(message)?;
        let (message, _) = take_until_and_consume!(message, "W:")?;
        let (message, w) = parse_coord(message)?;
        let (message, time) = parse_date(message)?;
        let (message, _) = take_until_and_consume!(message, "ERR: < ")?;
        let (message, max_error) = float(message)?;
        let max_error = max_error as u32;
        let (message, _) = multispace(message)?;
        let (_, unit) = take_until!(message, "\n")?;
        println!("unit {:?}", unit);
        let max_error = match unit {
            b"km" => max_error * 1000,
            _ => max_error,
        };

        Ok((
            input,
            GeoRecord {
                lon: -w,
                lat: n,
                time,
                max_error: max_error,
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
                    lat: 40.482502,
                    lon: -85.49389,
                    time: 1514898642,
                    max_error: 5000,
                },
            )),
            GeoRecord::parse(b"GU\nN: 040 28 57\nW: 085 29 38\nTIME: 02 01 2018 13:10:42\nERR: < 5 km\n\nOK\n                     extra")
        )
    }
}
