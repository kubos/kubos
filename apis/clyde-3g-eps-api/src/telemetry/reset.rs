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

/// Common Reset Telemetry Structure
pub struct ResetTelemetry {
    pub motherboard: u8,
    pub daughterboard: Option<u8>,
}

/// Each of these reset telemetry commands may return two bytes or four bytes,
/// depending on whether or not a daughterboard exists. The first two bytes
/// will always represent the motherboard, the second two will represent
/// the daughterboard. All counters roll over at 255 to 0.
make_reset_telemetry!(
    // Get Number of Brown-out Resets
    BrownOut => 0x31,
    // Get Number of Automatic Software Resets
    // If the on-board microcontroller has experienced a malfunction, such as being stuck
    // in a loop, it will reset itself into a pre-defined initial state.
    AutomaticSoftware => 0x32,
    // Get Number of Manual Resets
    // This is the count of the number of times the device has been manually reset using
    // the Reset command.
    Manual => 0x33,
    // Get Number of Communications Watchdog Resets
    // The device will reset itself if it does not receive any
    // data via i2c for a predefined length of time. The communications node keeps a count
    // of the number of times such an event has taken place.
    Watchdog => 0x34,
);

pub fn parse(data: &[u8]) -> Result<ResetTelemetry, Error> {
    if data.len() == 2 {
        Ok(ResetTelemetry {
            motherboard: data[1],
            daughterboard: None,
        })
    } else if data.len() == 4 {
        Ok(ResetTelemetry {
            motherboard: data[1],
            daughterboard: Some(data[3]),
        })
    } else {
        throw!(EpsError::BadData)
    }
}
