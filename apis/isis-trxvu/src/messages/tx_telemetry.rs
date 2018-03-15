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

use ffi::*;

/// Struct for transmit telemetry
#[derive(Debug)]
pub struct TxTelemetry {
    inst_rf_reflected: u16,
    inst_rf_forward: u16,
    supply_voltage: u16,
    supply_current: u16,
    temp_power_amp: u16,
    temp_oscillator: u16,
}

impl TxTelemetry {
    pub fn parse(input: &[u8]) -> IResult<&[u8], TxTelemetry> {
        let (input, inst_rf_reflected) = le_u16(input)?;
        let (input, inst_rf_forward) = le_u16(input)?;
        let (input, supply_voltage) = le_u16(input)?;
        let (input, supply_current) = le_u16(input)?;
        let (input, temp_power_amp) = le_u16(input)?;
        let (input, temp_oscillator) = le_u16(input)?;

        Ok((
            input,
            TxTelemetry {
                inst_rf_reflected,
                inst_rf_forward,
                supply_voltage,
                supply_current,
                temp_power_amp,
                temp_oscillator,
            },
        ))
    }

    pub fn inst_rf_reflected(&self) -> f32 {
        20.0 * ((self.inst_rf_reflected as f32) * 0.00767).log10()
    }

    pub fn inst_rf_forward(&self) -> f32 {
        20.0 * ((self.inst_rf_forward as f32) * 0.00767).log10()
    }

    pub fn supply_voltage(&self) -> f32 {
        (self.supply_voltage as f32) * 0.00488
    }

    pub fn supply_current(&self) -> f32 {
        (self.supply_current as f32) * 0.16643964
    }

    pub fn power_amp_temp(&self) -> f32 {
        (self.temp_power_amp as f32) * -0.07669 + 195.6037
    }

    pub fn oscillator_temp(&self) -> f32 {
        (self.temp_oscillator as f32) * -0.07669 + 195.6037
    }
}
