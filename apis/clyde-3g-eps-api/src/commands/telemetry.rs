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
use i2c_hal::Command;

const TELEM_CMD: u8 = 0x10;

pub enum Type {
    Voltage,
}

pub struct Telemetry;

impl Telemetry {
    pub fn parse(data: &[u8], telem_type: Type) -> Result<f32, EpsError> {
        match telem_type {
            Voltage => VoltageFeedingBRC1::parse(data),
        }
    }

    pub fn command(telem_type: Type) -> Command {
        match telem_type {
            Voltage => VoltageFeedingBRC1::command(),
        }
    }
}

fn get_adc_result(data: &[u8]) -> f32 {
    ((data[0] as u16) | ((data[1] as u16) & 0xF) << 4) as f32
}

mod VoltageFeedingBRC1 {
    use super::*;

    pub fn parse(data: &[u8]) -> Result<f32, EpsError> {
        Ok(0.0249 * get_adc_result(data))
    }

    pub fn command() -> Command {
        Command {
            cmd: TELEM_CMD,
            data: vec![0xE1, 0x10],
        }
    }
}
