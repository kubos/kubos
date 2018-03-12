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

use nom::{IResult, le_u16};
use nom::simple_errors::Context;

/// Struct for transmit telemetry
#[derive(Debug)]
pub struct RxTelemetry {
    pub inst_doppler_offset: u16,
    pub supply_current: u16,
    pub supply_voltage: u16,
    pub temp_oscillator: u16,
    pub temp_power_amp: u16,
    pub inst_signal_strength: u16,
}

impl RxTelemetry {
    pub fn parse(input: &[u8]) -> IResult<&[u8], RxTelemetry> {
        let (input, inst_doppler_offset) = le_u16(input)?;
        let (input, supply_current) = le_u16(input)?;
        let (input, supply_voltage) = le_u16(input)?;
        let (input, temp_oscillator) = le_u16(input)?;
        let (input, temp_power_amp) = le_u16(input)?;
        let (input, inst_signal_strength) = le_u16(input)?;

        Ok((
            input,
            RxTelemetry {
                inst_doppler_offset,
                supply_current,
                supply_voltage,
                temp_oscillator,
                temp_power_amp,
                inst_signal_strength,
            },
        ))
    }
}
