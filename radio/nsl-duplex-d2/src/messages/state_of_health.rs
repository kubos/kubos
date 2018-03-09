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

use nom::{IResult, be_u32, be_u8};

#[derive(Debug, PartialEq)]
pub struct StateOfHealth {
    reset_count: u32, // (4 byte integer) Current epoch reset count, starts at 0, incremented for each power system reset, persistent over the life of the mission
    current_time: u32, // (4 byte integer) Current time (seconds) from start of most recent reset
    current_rssi: u8, // (1 byte integer) Current RSSI (Received Signal Strength Indicator), 0 to 4
    connection_status: u8, // (1 byte integer) Connection status, 0 (connected) or 1 (disconnected)
    globalstar_gateway: u8, // (1 byte integer) Globalstar gateway connected to, proprietary ID, 0 to 255
    last_contact_time: u32, // (4 byte integer) Last contact time, seconds since latest reset
    last_attempt_time: u32, // (4 byte integer) Last attempt time, seconds since latest reset
    call_attempts_since_reset: u32, // (4 byte integer) Count of call attempts since latest reset
    successful_connects_since_reset: u32, // (4 byte integer) Count of successful connects since latest reset
    average_connection_duration: u32,     // (4 byte integer) Average connection duration (seconds)
    connection_duration_std_dev: u32, // (4 byte integer) Connection duration standard deviation (seconds)
}

impl StateOfHealth {
    pub fn parse(input: &[u8]) -> IResult<&[u8], StateOfHealth> {
        let (input, _) = tag!(input, b"GU")?;
        let (input, reset_count) = be_u32(input)?;
        let (input, current_time) = be_u32(input)?;
        let (input, current_rssi) = be_u8(input)?;
        let (input, connection_status) = be_u8(input)?;
        let (input, globalstar_gateway) = be_u8(input)?;
        let (input, last_contact_time) = be_u32(input)?;
        let (input, last_attempt_time) = be_u32(input)?;
        let (input, call_attempts_since_reset) = be_u32(input)?;
        let (input, successful_connects_since_reset) = be_u32(input)?;
        let (input, average_connection_duration) = be_u32(input)?;
        let (input, connection_duration_std_dev) = be_u32(input)?;
        Ok((
            input,
            StateOfHealth {
                reset_count,
                current_time,
                current_rssi,
                connection_status,
                globalstar_gateway,
                last_contact_time,
                last_attempt_time,
                call_attempts_since_reset,
                successful_connects_since_reset,
                average_connection_duration,
                connection_duration_std_dev,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::StateOfHealth;
    #[test]
    fn it_parses() {
        assert_eq!(
            Ok((
                &b"extra"[..],
                StateOfHealth {
                    reset_count: 1,
                    current_time: 2,
                    current_rssi: 3,
                    connection_status: 4,
                    globalstar_gateway: 5,
                    last_contact_time: 6,
                    last_attempt_time: 7,
                    call_attempts_since_reset: 8,
                    successful_connects_since_reset: 9,
                    average_connection_duration: 10,
                    connection_duration_std_dev: 11,
                }
            )),
            StateOfHealth::parse(b"GU\x00\x00\x00\x01\x00\x00\x00\x02\x03\x04\x05\x00\x00\x00\x06\x00\x00\x00\x07\x00\x00\x00\x08\x00\x00\x00\x09\x00\x00\x00\x0a\x00\x00\x00\x0bextra")
        )
    }
}
