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
use rust_i2c::Command;

/// Set Communications Watchdog Period
///
/// The Communications Watchdog by default has a value of 4 minutes set as
/// its timeout period. If 4 minutes pass without a command being received
/// then the device will reboot into its pre-defined initial state. This
/// value of 4 minutes can be changed using the Set Communications Watchdog
/// Period command, 0x21. The data byte specifies the number of minutes the
/// communications watchdog will wait before timing out.
///
/// A minimum value of 1 minute or a maximum of 90 minutes can be set.
/// The device will always reboot with a timeout value of 4 minutes set.
/// If an invalid value is specified then the device will generate a Data Error.
pub mod set_comms_watchdog_period {
    use super::*;

    pub fn command(period: u8) -> Command {
        Command {
            cmd: 0x21,
            data: vec![period],
        }
    }
}

/// Get Communications Watchdog Period
///
/// This command provides the user with the current communications watchdog
/// timeout that has been set. The returned value is indicated in minutes.
pub mod get_comms_watchdog_period {
    use super::*;

    pub fn parse(data: &[u8]) -> EpsResult<u8> {
        if data.len() == 2 {
            Ok(data[1])
        } else {
            throw!(EpsError::invalid_data(data))
        }
    }

    pub fn command() -> Command {
        Command {
            cmd: 0x20,
            data: vec![0x00],
        }
    }
}
