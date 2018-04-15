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
use telemetry::TELEM_CMD;

pub fn parse(adc_data: f32) -> f32 {
    0.0249 * adc_data
}

pub fn command() -> Command {
    Command {
        cmd: TELEM_CMD,
        data: vec![0xE1, 0x10],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data: f32 = 100.0;
        assert_eq!(2.49, parse(data));
    }
}
