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

use eps_api::{EpsError, EpsResult};

/// Macro for generating `Type` enum, `parse` and `command` functions
/// for motherboard and daughterboard telemetry items.
#[macro_export]
macro_rules! make_telemetry {
    (
        $(
            $(#[$meta:meta])+
            $type: ident => {$data: expr, $parser: expr},
        )+
    ) => {

        #[derive(Clone, Copy, Debug)]
        /// Telemetry variants
        pub enum Type {
            $(
                $(#[$meta])+
                $type,
            )+
        }

        /// Telemetry parsing function
        ///
        /// # Arguments
        ///
        /// `data` - Raw telemetry data from eps
        /// `telem_type` - `Type` of telemetry to parse
        pub fn parse(data: &[u8], telem_type: Type) -> EpsResult<f32> {
            let adc_data = get_adc_result(data)?;
            Ok(match telem_type {
                $(Type::$type => $parser(adc_data),)+
            })
        }

        /// Helper function storing telemetry command information
        ///
        /// # Arguments
        ///
        /// `telem_type` - `Type` of telemetry to return command for
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

pub fn get_adc_result(data: &[u8]) -> EpsResult<f32> {
    // It appears the ADCS actually sends back 4 bytes
    // The first two contain the actual response and
    // the second two are 0s
    if data.len() < 2 {
        throw!(EpsError::parsing_failure("ADC Result"))
    } else {
        Ok(f32::from(
            u16::from(data[0]) | (u16::from(data[1]) & 0xF) << 8,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adcs_result() {
        let raw = vec![0xAB, 0xCD, 0x12, 0x34];
        let adc = get_adc_result(&raw).unwrap();

        assert_eq!(adc, 3499.0);
    }

    #[test]
    fn test_make_telemetry() {
        use rust_i2c::Command;
        const TELEM_CMD: u8 = 0x00;

        make_telemetry!(
            /// TestValue1
            TestVal1 => {vec![0xE1], |d| (10.0 * d) - 10.0},
        );

        assert_eq!(
            command(Type::TestVal1),
            Command {
                cmd: TELEM_CMD,
                data: vec![0xE1],
            }
        );
        assert_eq!(
            parse(&vec![0xAB, 0xCD, 0x0, 0x0], Type::TestVal1),
            Ok(34980.0)
        );
    }
}
