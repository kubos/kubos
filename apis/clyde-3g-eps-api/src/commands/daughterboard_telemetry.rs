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
    // Voltage feeding BRC4 (V)
    VoltageFeedingBcr4 => {vec![0xE1, 0x40], |d| 0.0249 * d},
    // Current BCR4, Connector SA4A (A)
    CurrentBcr4Sa4a => {vec![0xE1, 0x44], |d| 0.0009775 * d},
    // Current BCR4, Connection SA4B (A)
    CurrentBcr4Sa4b => {vec![0xE1, 0x45], |d| 0.0009775 * d},
    // Array Temp, Connector SA4A (*C)
    ArrayTempSa4a => {vec![0xE1, 0x48], |d| (0.4963 * d) - 273.15},
    // Array Temp, Connector SA4B (*C)
    ArrayTempSa4b => {vec![0xE1, 0x49], |d| (0.4963 * d) - 273.15},
    // Sun Detector, Connector SA4A (W/m^2)
    SunDetectorSa4a => {vec![0xE1, 0x4C], |d| d},
    // Sun Detector, Connector SA4B (W/m^2)
    SunDetectorSa4b => {vec![0xE1, 0x4D], |d| d},

    // Voltage Feeding BCR5 (V)
    VoltageFeedingBcr5 => {vec![0xE1, 0x50], |d| 0.0249 * d},
    // Current BCR5, Connection SA5A (A)
    CurrentBcr5Sa5a => {vec![0xE1, 0x54], |d| 0.0009775 * d},
    // Current BCR5, Connector SA5B (A)
    CurrentBcr5Sa5b => {vec![0xE1, 0x55], |d| 0.0009775 * d},
    // Array Temp, Connector SA5A (*C)
    ArrayTempSa5a => {vec![0xE1, 0x58], |d| (0.4963 * d) - 273.15},
    // Array Temp, Connector SA5B (*C)
    ArrayTempSa5b => {vec![0xE1, 0x59], |d| (0.4963 * d) - 273.15},
    // Sun Detector, Connector SA5A (W/m^2)
    SunDetectorSa5a => {vec![0xE1, 0x5C], |d| d},
    // Sun Detector, Connector SA5B (W/m^2)
    SunDetectorSa5b => {vec![0xE1, 0x5D], |d| d},

    // Voltage Feeding BCR6 (V)
    VoltageFeedingBcr6 => {vec![0xE1, 0x60], |d| 0.0099706 * d},
    // Current BCR6, Connection SA6A (A)
    CurrentBcr6Sa6a => {vec![0xE1, 0x64], |d| 0.0009775 * d},
    // Current BCR6, Connector SA6B (A)
    CurrentBcr6Sa6b => {vec![0xE1, 0x65], |d| 0.0009775 * d},
    // Array Temp, Connector SA6A (*C)
    ArrayTempSa6a => {vec![0xE1, 0x68], |d| (0.4963 * d) - 273.15},
    // Array Temp, Connector SA6B (*C)
    ArrayTempSa6b => {vec![0xE1, 0x69], |d| (0.4963 * d) - 273.15},
    // Sun Detector, Connector SA6A (W/m^2)
    SunDetectorSa6a => {vec![0xE1, 0x6C], |d| d},
    // Sun Detector, Connector SA6B (W/m^2)
    SunDetectorSa6b => {vec![0xE1, 0x6D], |d| d},

    // Voltage Feeding BCR7 (V)
    VoltageFeedingBcr7 => {vec![0xE1, 0x70], |d| 0.0099706 * d},
    // Current BCR7, Connection SA7A (A)
    CurrentBcr7Sa7a => {vec![0xE1, 0x74], |d| 0.0009775 * d},
    // Current BCR7, Connector SA7B (A)
    CurrentBcr7Sa7b => {vec![0xE1, 0x75], |d| 0.0009775 * d},
    // Array Temp, Connector SA7A (*C)
    ArrayTempSa7a => {vec![0xE1, 0x78], |d| (0.4963 * d) - 273.15},
    // Array Temp, Connector SA7B (*C)
    ArrayTempSa7b => {vec![0xE1, 0x79], |d| (0.4963 * d) - 273.15},
    // Sun Detector, Connector SA7A (W/m^2)
    SunDetectorSa7a => {vec![0xE1, 0x7C], |d| d},
    // Sun Detector, Connector SA7B (W/m^2)
    SunDetectorSa7b => {vec![0xE1, 0x7D], |d| d},

    // Voltage Feeding BCR8 (V)
    VoltageFeedingBcr8 => {vec![0xE1, 0x80], |d| 0.0099706 * d},
    // Current BCR8, Connection SA8A (A)
    CurrentBcr8Sa8a => {vec![0xE1, 0x84], |d| 0.0009775 * d},
    // Current BCR8, Connector SA8B (A)
    CurrentBcr8Sa8b => {vec![0xE1, 0x85], |d| 0.0009775 * d},
    // Array Temp, Connector SA8A (*C)
    ArrayTempSa8a => {vec![0xE1, 0x88], |d| (0.4963 * d) - 273.15},
    // Array Temp, Connector SA8B (*C)
    ArrayTempSa8b => {vec![0xE1, 0x89], |d| (0.4963 * d) - 273.15},
    // Sun Detector, Connector SA8A (W/m^2)
    SunDetectorSa8a => {vec![0xE1, 0x8C], |d| d},
    // Sun Detector, Connector SA8B (W/m^2)
    SunDetectorSa8b => {vec![0xE1, 0x8D], |d| d},

    // Voltage Feeding BCR9 (V)
    VoltageFeedingBcr9 => {vec![0xE1, 0x90], |d| 0.0099706 * d},
    // Current BC96, Connection SA9A (A)
    CurrentBcr9Sa9a => {vec![0xE1, 0x94], |d| 0.0009775 * d},
    // Current BCR9, Connector SA9B (A)
    CurrentBcr9Sa9b => {vec![0xE1, 0x95], |d| 0.0009775 * d},
    // Array Temp, Connector SA9A (*C)
    ArrayTempSa9a => {vec![0xE1, 0x98], |d| (0.4963 * d) - 273.15},
    // Array Temp, Connector SA9B (*C)
    ArrayTempSa9b => {vec![0xE1, 0x99], |d| (0.4963 * d) - 273.15},
    // Sun Detector, Connector SA9A (W/m^2)
    SunDetectorSa9a => {vec![0xE1, 0x9C], |d| d},
    // Sun Detector, Connector SA9B (W/m^2)
    SunDetectorSa9b => {vec![0xE1, 0x9D], |d| d},

    // Board Temperature (*C)
    BoardTemperature => {vec![0xE3, 0x88], |d| (0.372434 * d) - 273.15},
);
