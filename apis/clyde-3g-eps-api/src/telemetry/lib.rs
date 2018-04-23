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

#[macro_export]
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

#[macro_export]
macro_rules! make_reset_telemetry {
    (
        $($type: ident => $cmd: expr,)+
    ) => {

        #[derive(Clone, Copy)]
        pub enum ResetType {
            $($type,)+
        }

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

pub fn get_adc_result(data: &[u8]) -> Result<f32, EpsError> {
    if data.len() != 2 {
        Err(EpsError::BadData)
    } else {
        Ok(((data[0] as u16) | ((data[1] as u16) & 0xF) << 4) as f32)
    }
}
