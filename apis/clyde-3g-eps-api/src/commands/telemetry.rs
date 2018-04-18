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

macro_rules! make_telemetry {
    (
        $($type: ident => {$data: expr, $parser: expr},)+
    ) => {

        #[derive(Clone, Copy)]
        pub enum Type {
            $($type,)+
        }

            pub fn parse(data: &[u8], telem_type: Type) -> Result<f32, EpsError> {
                let adc_data = get_adc_result(data)?;
                Ok(match telem_type {
                    $(Type::$type => $parser(adc_data),)+
                })
            }

            pub fn command(telem_type: Type) -> Command {
                Command {
                    cmd: TELEM_CMD,
                    data: match telem_type {
                        $(Type::$type => $data,)+
                    }
                }
            }
    }
}

fn get_adc_result(data: &[u8]) -> Result<f32, EpsError> {
    if data.len() != 2 {
        Err(EpsError::BadData)
    } else {
        Ok(((data[0] as u16) | ((data[1] as u16) & 0xF) << 4) as f32)
    }
}


make_telemetry!(
    // Voltage feeding BRC1 (V)
    VoltageFeedingBcr1 => {vec![0xE1, 0x10], |d| 0.0249 * d},
    // Current BCR1, Connector SA1A (A)
    CurrentBcr1Sa1a => {vec![0xE1, 0x14], |d| 0.0009775 * d},
    // Current BCR1, Connection SA1B (A)
    CurrentBcr1Sa1b => {vec![0xE1, 0x15], |d| 0.0009775 * d},
    // Array Temp, Connector SA1A (*C)
    ArrayTempSa1a => {vec![0xE1, 0x18], |d| (0.4963 * d) - 273.15},
    // Array Temp, Connector SA1B (*C)
    ArrayTempSa1b => {vec![0xE1, 0x19], |d| (0.4963 * d) - 273.15},
    // Sun Detector, Connector SA1A (W/m^2)
    SunDetectorSa1a => {vec![0xE1, 0x1C], |d| d},
    // Sun Detector, Connector SA1B (W/m^2)
    SunDetectorSa1b => {vec![0xE1, 0x1D], |d| d},

    // Voltage Feeding BCR2 (V)
    VoltageFeedingBcr2 => {vec![0xE1, 0x20], |d| 0.0249 * d},
    // Current BCR2, Connection SA2A (A)
    CurrentBcr2Sa2a => {vec![0xE1, 0x24], |d| 0.0009775 * d},
    // Current BCR2, Connector SA2B (A)
    CurrentBcr2Sa2b => {vec![0xE1, 0x25], |d| 0.0009775 * d},
    // Array Temp, Connector SA2A (*C)
    ArrayTempSa2a => {vec![0xE1, 0x28], |d| (0.4963 * d) - 273.15},
    // Array Temp, Connector SA2B (*C)
    ArrayTempSa2b => {vec![0xE1, 0x29], |d| (0.4963 * d) - 273.15},
    // Sun Detector, Connector SA2A (W/m^2)
    SunDetectorSa2a => {vec![0xE1, 0x2C], |d| d},
    // Sun Detector, Connector SA2B (W/m^2)
    SunDetectorSa2b => {vec![0xE1, 0x2D], |d| d},

    // Voltage Feeding BCR3 (V)
    VoltageFeedingBcr3 => {vec![0xE1, 0x30], |d| 0.0099706 * d},
    // Current BCR3, Connection SA3A (A)
    CurrentBcr3Sa3a => {vec![0xE1, 0x34], |d| 0.0009775 * d},
    // Current BCR3, Connector SA3B (A)
    CurrentBcr3Sa3b => {vec![0xE1, 0x35], |d| 0.0009775 * d},
    // Array Temp, Connector SA3A (*C)
    ArrayTempSa3a => {vec![0xE1, 0x38], |d| (0.4963 * d) - 273.15},
    // Array Temp, Connector SA3B (*C)
    ArrayTempSa3b => {vec![0xE1, 0x39], |d| (0.4963 * d) - 273.15},
    // Sun Detector, Connector SA3A (W/m^2)
    SunDetectorSa3a => {vec![0xE1, 0x3C], |d| d},
    // Sun Detector, Connector SA3B (W/m^2)
    SunDetectorSa3b => {vec![0xE1, 0x3D], |d| d},

    // BCR Output Current (mA)
    BcrOutputCurrent => {vec![0xE2, 0x84], |d| 14.662757 * d},
    // BCR Output Voltage (V)
    BcrOutputVoltage => {vec![0xE2, 0x80], |d| 0.008993157 * d},
    // 3V3 Current Draw of EPS (A)
    CurrentDraw3V3 => {vec![0xE2, 0x05], |d| 0.001327547 * d},
    // 5V Current Draw of EPS (A)
    CurrentDraw5V => {vec![0xE2, 0x15], |d| 0.001327547 * d},

    // Output Current of 12V bus (A)
    OutputCurrent12V => {vec![0xE2, 0x34], |d| 0.00207 * d},
    // Output Voltage of 12V bus (V)
    OutputVoltage12V => {vec![0xE2, 0x30], |d| 0.01349 * d},
    // Output Current of Battery Bus (A)
    OutputCurrentBattery => {vec![0xE2, 0x24], |d| 0.005237 *d},
    // Output Voltage of Battery Bus (V)
    OutputVoltageBattery => {vec![0xE2, 0x20], |d| 0.008978 *d},
    // Output Current of 5V bus (A)
    OutputCurrent5v => {vec![0xE2, 0x14], |d| 0.005237 * d},
    // Output Voltage of 5V bus (V)
    OutputVoltage5v => {vec![0xE2, 0x10], |d| 0.005865 * d},
    // Output Current of 3.3V Bus (A)
    OutputCurrent33v => {vec![0xE2, 0x04], |d| 0.005237 *d},
    // Output Voltage of 3.3V Bus (V)
    OutputVoltage33v => {vec![0xE2, 0x00], |d| 0.004311 *d},

    // Output Voltage Switch 1 (V)
    OutputVoltageSwitch1 => {vec![0xE4, 0x10], |d| 0.01349 * d},
    // Output Current Switch 1 (A)
    OutputCurrentSwitch1 => {vec![0xE4, 0x14], |d| 0.001328 * d},
    // Output Voltage Switch 2 (V)
    OutputVoltageSwitch2 => {vec![0xE4, 0x20], |d| 0.01349 * d},
    // Output Current Switch 2 (A)
    OutputCurrentSwitch2 => {vec![0xE4, 0x24], |d| 0.001328 * d},
    // Output Voltage Switch 3 (V)
    OutputVoltageSwitch3 => {vec![0xE4, 0x30], |d| 0.008993 * d},
    // Output Current Switch 3 (A)
    OutputCurrentSwitch3 => {vec![0xE4, 0x34], |d| 0.001328 * d},
    // Output Voltage Switch 4 (V)
    OutputVoltageSwitch4 => {vec![0xE4, 0x40], |d| 0.008993 * d},
    // Output Current Switch 4 (A)
    OutputCurrentSwitch4 => {vec![0xE4, 0x44], |d| 0.001328 * d},
    // Output Voltage Switch 5 (V)
    OutputVoltageSwitch5 => {vec![0xE4, 0x50], |d| 0.005865 * d},
    // Output Current Switch 5 (A)
    OutputCurrentSwitch5 => {vec![0xE4, 0x54], |d| 0.001328 * d},
    // Output Voltage Switch 6 (V)
    OutputVoltageSwitch6 => {vec![0xE4, 0x60], |d| 0.005865 * d},
    // Output Current Switch 6 (A)
    OutputCurrentSwitch6 => {vec![0xE4, 0x64], |d| 0.001328 * d},
    // Output Voltage Switch 7 (V)
    OutputVoltageSwitch7 => {vec![0xE4, 0x70], |d| 0.005865 * d},
    // Output Current Switch 7 (A)
    OutputCurrentSwitch7 => {vec![0xE4, 0x74], |d| 0.001328 * d},
    // Output Voltage Switch 8 (V)
    OutputVoltageSwitch8 => {vec![0xE4, 0x80], |d| 0.004311 * d},
    // Output Current Switch 8 (A)
    OutputCurrentSwitch8 => {vec![0xE4, 0x84], |d| 0.001328 * d},
    // Output Voltage Switch 9 (V)
    OutputVoltageSwitch9 => {vec![0xE4, 0x90], |d| 0.004311 * d},
    // Output Current Switch 9 (A)
    OutputCurrentSwitch9 => {vec![0xE4, 0x94], |d| 0.001328 * d},
    // Output Voltage Switch 10 (V)
    OutputVoltageSwitch10 => {vec![0xE4, 0xA0], |d| 0.004311 * d},
    // Output Current Switch 10 (A)
    OutputCurrentSwitch10 => {vec![0xE4, 0xA4], |d| 0.001328 * d},

    // Board Temperature (*C)
    BoardTemperature => {vec![0xE3, 0x08], |d| (0.372434 * d) - 273.15},
);
