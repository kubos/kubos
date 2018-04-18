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

    pub fn parse(data: &[u8]) -> Result<u8, EpsError> {
        Ok(data[1])
    }

    pub fn command() -> Command {
        Command {
            cmd: 0x20,
            data: vec![0x00],
        }
    }
}

/// Reset Communications Watchdog
///
/// Any valid command will reset the communications watchdog timer. If the user
/// does not require any telemetry from the board, this command can be sent
/// to reset the communications watchdog.
pub mod reset_comms_watchdog {
    use super::*;

    pub fn command() -> Command {
        Command {
            cmd: 0x22,
            data: vec![0x00],
        }
    }
}

/// Get Number of Brown-out Resets
///
/// This counter is designed to keep track of the number of brown-out resets that
/// have occurred. This counter will roll over at 255 to 0. The first two bytes
/// outputted represent the Motherboard’s value, the second two represent the Daughterboard’s.
pub mod get_number_brown_out_resets {
    use super::*;

    pub fn parse(data: &[u8]) -> Result<(u8, u8), EpsError> {
        Ok((data[1], data[3]))
    }

    pub fn command() -> Command {
        Command {
            cmd: 0x31,
            data: vec![0x00],
        }
    }
}

/// Get Number of Automatic Software Resets
///
/// If the on-board microcontroller has experienced a malfunction, such as being stuck
/// in a loop, it will reset itself into a pre-defined initial state. Using this command,
/// 0x32, it is possible to retrieve the number of times this reset has occurred. The
/// first two bytes outputted represent the Motherboard’s value, the second two
/// represent the Daughterboard’s. This counter will roll over at 255 to 0.
pub mod get_number_automatic_software_resets {
    use super::*;

    pub fn parse(data: &[u8]) -> Result<(u8, u8), EpsError> {
        Ok((data[1], data[3]))
    }

    pub fn command() -> Command {
        Command {
            cmd: 0x32,
            data: vec![0x00],
        }
    }
}

/// Get Number of Manual Resets
///
/// A count is kept of the number of times the device has been manually reset using
/// the Reset command. Sending the command 0x33 with data byte 0x00 will return the
/// number of times the device has been reset in this fashion. The first two bytes
/// outputted represent the Motherboard’s value, the second two represent the
/// Daughterboard’s. This counter will roll over at 255 to 0.
pub mod get_number_manual_resets {
    use super::*;

    pub fn parse(data: &[u8]) -> Result<(u8, u8), EpsError> {
        Ok((data[1], data[3]))
    }

    pub fn command() -> Command {
        Command {
            cmd: 0x33,
            data: vec![0x00],
        }
    }
}

/// Get Number of Communications Watchdog Resets
///
/// As described previously, the device will reset itself if it does not receive any
/// data via i2c for a predefined length of time. The communications node keeps a count
/// of the number of times such an event has taken place. Sending the command 0x34 along
/// with the data byte 0x00 will return the number of communication watchdog resets.
/// This counter will roll over at 255 to 0.
pub mod get_number_comms_watchdog_resets {
    use super::*;

    pub fn parse(data: &[u8]) -> Result<(u8, u8), EpsError> {
        Ok((data[1], data[3]))
    }

    pub fn command() -> Command {
        Command {
            cmd: 0x33,
            data: vec![0x00],
        }
    }
}
