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

use chrono::{DateTime, NaiveDateTime, Utc};
use nom::simple_errors::Context;
use nom::{
    float, map_res, multispace, tag, take_until, take_until_and_consume, Err, ErrorKind, IResult,
};
use std::str::from_utf8;

#[derive(Debug, PartialEq)]
/// Struct for storing geo-records.
pub struct GeoRecord {
    /// Modem' longitude at the time of the last connection
    pub lon: f32,
    /// Modem's latitude at the time of the last connection
    pub lat: f32,
    /// Time of modem's last connection
    pub time: i64,
    /// Approximate error of location data (<300m to <100km)
    pub max_error: u32,
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
            .map_err(|_| Err::Error(Context::Code(input, ErrorKind::Tag)))?,
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
    /// Parse GeoRecord
    pub fn parse(input: &[u8]) -> IResult<&[u8], GeoRecord> {
        let (input, _) = take_until_and_consume!(input, "GU")?;
        let (input, _) = take_until_and_consume!(input, "N:")?;
        let (input, n) = parse_coord(input)?;
        let (input, _) = take_until_and_consume!(input, "W:")?;
        let (input, w) = parse_coord(input)?;
        let (input, time) = parse_date(input)?;
        let (input, _) = take_until_and_consume!(input, "ERR: < ")?;
        let (input, max_error) = float(input)?;
        let max_error = max_error as u32;
        let (input, _) = multispace(input)?;
        let (input, unit) = take_until!(input, "\n")?;
        let (input, _) = multispace(input)?;
        let (input, _) = tag!(input, "OK")?;
        let (input, _) = multispace(input)?;
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
                max_error,
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
                    lat: 40.482_502,
                    lon: -85.493_89,
                    time: 1_514_898_642,
                    max_error: 5000,
                },
            )),
            GeoRecord::parse(b"GU\nN: 040 28 57\nW: 085 29 38\nTIME: 02 01 2018 13:10:42\nERR: < 5 km\n\nOK\n                     extra")
        )
    }
}
