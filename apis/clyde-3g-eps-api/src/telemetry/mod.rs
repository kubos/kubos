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

mod temp_sa1a;
mod voltage_bcr1;
mod current_bcr1_sa1a;

const TELEM_CMD: u8 = 0x10;

pub enum Type {
    /// Voltage feeding BRC1 (V)
    VoltageFeedingBRC1,
    /// Current BRC1, Connector SA1A (A)
    IBCR1A,
    /// Array Temp, Connector SA1A (*C)
    TempSA1A,
}

pub fn parse(data: &[u8], telem_type: Type) -> Result<f32, EpsError> {
    let adc_data = get_adc_result(data);
    Ok(match telem_type {
        Voltage => voltage_bcr1::parse(adc_data),
        IBCR1A => current_bcr1_sa1a::parse(adc_data),
        TempSA1A => temp_sa1a::parse(adc_data),
    })
}

pub fn command(telem_type: Type) -> Command {
    match telem_type {
        Voltage => voltage_bcr1::command(),
        IBCR1A => current_bcr1_sa1a::command(),
        TempSA1A => temp_sa1a::command(),
    }
}

fn get_adc_result(data: &[u8]) -> f32 {
    ((data[0] as u16) | ((data[1] as u16) & 0xF) << 4) as f32
}
