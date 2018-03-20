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
#[derive(Debug)]
pub struct TxTelemetry {
    pub inst_rf_reflected: f32,
    pub inst_rf_forward: f32,
    pub supply_voltage: f32,
    pub supply_current: f32,
    pub temp_power_amp: f32,
    pub temp_oscillator: f32,
}

impl TxTelemetry {
    pub fn parse(raw: &TxTelemRaw) -> TxTelemetry {
        TxTelemetry {
            inst_rf_reflected: get_rf_power_dbm(raw.inst_rf_reflected),
            inst_rf_forward: get_rf_power_mw(raw.inst_rf_forward),
            supply_voltage: get_voltage(raw.supply_voltage),
            supply_current: get_current(raw.supply_current),
            temp_power_amp: get_temperature(raw.temp_power_amp),
            temp_oscillator: get_temperature(raw.temp_oscillator),
        }
    }
}
