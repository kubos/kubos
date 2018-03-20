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

// Returns Voltage in volts
pub fn get_voltage(raw: u16) -> f32 {
    (raw as f32) * 0.00488
}

/// Returns Current in milliamps
pub fn get_current(raw: u16) -> f32 {
    (raw as f32) * 0.16643964
}

/// Returns Temperature in degrees Celsius
pub fn get_temperature(raw: u16) -> f32 {
    (raw as f32) * -0.07669 + 195.6037
}

/// Returns Doppler shift in hertz
pub fn get_doppler_offset(raw: u16) -> f32 {
    (raw as f32) * 13.352 - 22300.0
}

/// Returns Received signal strength power in decibel-milliwatts
pub fn get_signal_strength(raw: u16) -> f32 {
    (raw as f32) * 0.03 - 152.0
}

/// Returns RF reflected power in decibel-milliwatts
pub fn get_rf_power_dbm(raw: u16) -> f32 {
    20.0 * ((raw as f32) * 0.00767).log10()
}

/// Return RF reflected power in milliwatts
pub fn get_rf_power_mw(raw: u16) -> f32 {
    let ten: f32 = 10.0;
    (raw as f32) * (raw as f32) * ten.powf(-2.0) * 0.00005887
}
