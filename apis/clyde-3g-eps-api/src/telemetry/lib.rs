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

        #[derive(Clone, Copy)]
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

/// Macro for generating `ResetType` enum and `command` function
/// for reset telemetry items.
#[macro_export]
macro_rules! make_reset_telemetry {
    (
        //$(#[$type: ident => $cmd: expr],)+
        $(
            $(#[$meta:meta])+
            $type: ident => $cmd: expr,
        )+
    ) => {

        #[derive(Clone, Copy)]
        /// Reset Telemetry Variants
        pub enum ResetType {
            $(
                $(#[$meta])+
                $type,
            )+
        }

        /// Helper function storing telemetry command information
        ///
        /// # Arguments
        ///
        /// `telem_type` - `ResetType` of telemetry to return command for
        pub fn command(reset_type: ResetType) -> Command {
            Command {
                cmd: match reset_type {
                    $(ResetType::$type => $cmd,)+
                },
                data: vec![0x00],
            }
        }
    }
}

pub fn get_adc_result(data: &[u8]) -> EpsResult<f32> {
    if data.len() != 2 {
        throw!(EpsError::invalid_data(data))
    } else {
        Ok(f32::from(
            u16::from(data[0]) | (u16::from(data[1]) & 0xF) << 4,
        ))
    }
}
