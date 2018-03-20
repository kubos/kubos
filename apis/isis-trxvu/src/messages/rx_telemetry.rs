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

use ffi::*;
use messages::convert::*;

/// Struct for transmit telemetry
#[derive(Debug, PartialEq)]
pub struct RxTelemetry {
    pub inst_doppler_offset: f32,
    pub supply_current: f32,
    pub supply_voltage: f32,
    pub temp_oscillator: f32,
    pub temp_power_amp: f32,
    pub inst_signal_strength: f32,
}

impl RxTelemetry {
    pub fn parse(raw: &RxTelemRaw) -> RxTelemetry {
        RxTelemetry {
            inst_doppler_offset: get_doppler_offset(raw.inst_doppler_offset),
            supply_current: get_current(raw.supply_current),
            supply_voltage: get_voltage(raw.supply_voltage),
            temp_oscillator: get_temperature(raw.temp_oscillator),
            temp_power_amp: get_temperature(raw.temp_power_amp),
            inst_signal_strength: get_signal_strength(raw.inst_signal_strength),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let raw = RxTelemRaw {
            inst_doppler_offset: 1,
            supply_voltage: 1,
            supply_current: 1,
            temp_power_amp: 1,
            temp_oscillator: 1,
            inst_signal_strength: 1,
        };
        let result = RxTelemetry {
            inst_doppler_offset: -22286.648,
            supply_voltage: 0.00488,
            supply_current: 0.16643964,
            temp_power_amp: 195.52701,
            temp_oscillator: 195.52701,
            inst_signal_strength: -151.97,
        };
        assert_eq!(result, RxTelemetry::parse(&raw));
    }
}
