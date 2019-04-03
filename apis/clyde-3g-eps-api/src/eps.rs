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

use crate::commands::*;
use crate::telemetry;
use eps_api::EpsResult;
use rust_i2c::{Command, Connection};
use std::thread;
use std::time::Duration;

// Observed (but undocumented) inter-command delay required is 59ms
// Rounding up to an even 60
const INTER_COMMAND_DELAY: Duration = Duration::from_millis(60);

/// Trait defining expected functionality for Clyde 3g EPS
pub trait Clyde3gEps {
    /// Get Board Status
    ///
    /// The status bytes are designed to supply operational data about the I2C Node.
    fn get_board_status(&self) -> EpsResult<board_status::BoardStatus>;

    /// Get Checksum
    ///
    /// This command instructs the node to self-inspect its ROM contents in order
    /// to generate a checksum. The value retrieved can be used to determine whether
    /// the contents of the ROM have changed during the operation of the device.
    fn get_checksum(&self) -> EpsResult<checksum::Checksum>;

    /// Get Version
    ///
    /// The version number of the firmware will be returned on this command.
    /// The revision number returns the current revision of the firmware that is
    /// present on the board. The firmware number returns the current firmware on the board.
    fn get_version_info(&self) -> EpsResult<version::VersionInfo>;

    /// Get Last Error
    ///
    /// If an error has been generated after attempting to execute a user's command,
    /// this command can be used to retrieve details about the error.
    fn get_last_error(&self) -> EpsResult<last_error::LastError>;

    /// Manual Reset
    ///
    /// If required the user can reset the TTC node using this command. When issued,
    /// the board will reset within 1 second. This command will result in the board
    /// being brought up in its defined initial condition. Resetting the board in
    /// this fashion will increment the Manual Reset Counter.
    fn manual_reset(&self) -> EpsResult<()>;

    /// Reset Communications Watchdog
    ///
    /// Any valid command will reset the communications watchdog timer. If the user
    /// does not require any telemetry from the board, this command can be sent
    /// to reset the communications watchdog.
    fn reset_comms_watchdog(&self) -> EpsResult<()>;

    /// Get Motherboard Telemetry
    ///
    /// This command is used to request telemetry items from the motherboard's
    /// telemetry node.
    ///
    /// # Arguments
    /// `telem_type` - Variant of [`MotherboardTelemetry::Type`] to request
    ///
    /// [`MotherboardTelemetry::Type`]: ./MotherboardTelemetry/enum.Type.html
    fn get_motherboard_telemetry(&self, telem_type: telemetry::motherboard::Type)
        -> EpsResult<f64>;

    /// Get Daughterboard Telemetry
    ///
    /// This command is used to request telemetry items from the daughterboard's
    /// telemetry node.
    ///
    /// # Arguments
    /// `telem_type` - Variant of [`DaughterboardTelemetry::Type`] to request
    ///
    /// [`DaughterboardTelemetry::Type`]: ./DaughterboardTelemetry/enum.Type.html
    fn get_daughterboard_telemetry(
        &self,
        telem_type: telemetry::daughterboard::Type,
    ) -> EpsResult<f64>;

    /// Get Reset Telemetry
    ///
    /// This command is used to request telemetry items regarding various
    /// reset conditions on both the motherboard and daughterboard.
    ///
    /// # Arguments
    /// `telem_type` - Variant of [`ResetTelemetry::Type`] to request
    ///
    /// [`ResetTelemetry::Type`]: ./ResetTelemetry/enum.Type.html
    fn get_reset_telemetry(
        &self,
        telem_type: telemetry::reset::Type,
    ) -> EpsResult<telemetry::reset::Data>;

    /// Set Communications Watchdog Period
    ///
    /// The Communications Watchdog by default has a value of 4 minutes set as
    /// its timeout period. If 4 minutes pass without a command being received
    /// then the device will reboot into its pre-defined initial state. This
    /// value of 4 minutes can be changed using the Set Communications Watchdog
    /// Period command, 0x21. The data byte specifies the number of minutes the
    /// communications watchdog will wait before timing out.
    ///
    /// # Arguments
    /// `period` - Watchdog period to set in minutes
    fn set_comms_watchdog_period(&self, period: u8) -> EpsResult<()>;

    /// Get Communications Watchdog Period
    ///
    /// This command provides the user with the current communications watchdog
    /// timeout that has been set. The returned value is indicated in minutes.
    fn get_comms_watchdog_period(&self) -> EpsResult<u8>;

    /// Issue Raw Command
    ///
    /// This command sends a raw command to the EPS
    fn raw_command(&self, cmd: u8, data: Vec<u8>) -> EpsResult<()>;
}

/// EPS structure containing low level connection and functionality
/// required for commanding and requesting telemetry from EPS device.
pub struct Eps {
    connection: Connection,
}

impl Eps {
    /// Constructor
    ///
    /// Creates new instance of Eps structure.
    ///
    /// # Arguments
    /// `connection` - A [`Connection`] used as low-level connection to EPS hardware
    ///
    /// [`Connection`]: ../rust_i2c/struct.Connection.html
    pub fn new(connection: Connection) -> Self {
        Eps { connection }
    }
}

impl Clyde3gEps for Eps {
    /// Get Board Status
    ///
    /// The status bytes are designed to supply operational data about the I2C Node.
    fn get_board_status(&self) -> EpsResult<board_status::BoardStatus> {
        thread::sleep(INTER_COMMAND_DELAY);
        let (command, rx_len) = board_status::command();
        board_status::parse(
            &self
                .connection
                .transfer(command, rx_len, Duration::from_millis(3))?,
        )
    }

    /// Get Checksum
    ///
    /// This command instructs the node to self-inspect its ROM contents in order
    /// to generate a checksum. The value retrieved can be used to determine whether
    /// the contents of the ROM have changed during the operation of the device.
    fn get_checksum(&self) -> EpsResult<checksum::Checksum> {
        thread::sleep(INTER_COMMAND_DELAY);
        let (command, rx_len) = checksum::command();
        checksum::parse(
            &self
                .connection
                .transfer(command, rx_len, Duration::from_millis(80))?,
        )
    }

    /// Get Version
    ///
    /// The version number of the firmware will be returned on this command.
    /// The revision number returns the current revision of the firmware that is
    /// present on the board. The firmware number returns the current firmware on the board.
    fn get_version_info(&self) -> EpsResult<version::VersionInfo> {
        thread::sleep(INTER_COMMAND_DELAY);
        let (command, rx_len) = version::command();
        version::parse(
            &self
                .connection
                .transfer(command, rx_len, Duration::from_millis(3))?,
        )
    }

    /// Get Last Error
    ///
    /// If an error has been generated after attempting to execute a user's command,
    /// this command can be used to retrieve details about the error.
    fn get_last_error(&self) -> EpsResult<last_error::LastError> {
        thread::sleep(INTER_COMMAND_DELAY);
        let (command, rx_len) = last_error::command();
        last_error::parse(
            &self
                .connection
                .transfer(command, rx_len, Duration::from_millis(3))?,
        )
    }

    /// Manual Reset
    ///
    /// If required the user can reset the TTC node using this command. When issued,
    /// the board will reset within 1 second. This command will result in the board
    /// being brought up in its defined initial condition. Resetting the board in
    /// this fashion will increment the Manual Reset Counter.
    fn manual_reset(&self) -> EpsResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        self.connection.write(manual_reset::command())?;
        Ok(())
    }

    /// Reset Communications Watchdog
    ///
    /// Any valid command will reset the communications watchdog timer. If the user
    /// does not require any telemetry from the board, this command can be sent
    /// to reset the communications watchdog.
    fn reset_comms_watchdog(&self) -> EpsResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        self.connection.write(reset_comms_watchdog::command())?;
        Ok(())
    }

    /// Get Motherboard Telemetry
    ///
    /// This command is used to request telemetry items from the motherboard's
    /// telemetry node.
    ///
    /// # Arguments
    /// `telem_type` - Variant of [`MotherboardTelemetry::Type`] to request
    ///
    /// [`MotherboardTelemetry::Type`]: ./MotherboardTelemetry/enum.Type.html
    fn get_motherboard_telemetry(
        &self,
        telem_type: telemetry::motherboard::Type,
    ) -> EpsResult<f64> {
        thread::sleep(INTER_COMMAND_DELAY);
        let (command, rx_len) = telemetry::motherboard::command(telem_type);
        telemetry::motherboard::parse(
            &self
                .connection
                .transfer(command, rx_len, Duration::from_millis(20))?,
            telem_type,
        )
    }

    /// Get Daughterboard Telemetry
    ///
    /// This command is used to request telemetry items from the daughterboard's
    /// telemetry node.
    ///
    /// # Arguments
    /// `telem_type` - Variant of [`DaughterboardTelemetry::Type`] to request
    ///
    /// [`DaughterboardTelemetry::Type`]: ./DaughterboardTelemetry/enum.Type.html
    fn get_daughterboard_telemetry(
        &self,
        telem_type: telemetry::daughterboard::Type,
    ) -> EpsResult<f64> {
        thread::sleep(INTER_COMMAND_DELAY);
        let (command, rx_len) = telemetry::daughterboard::command(telem_type);
        telemetry::daughterboard::parse(
            &self
                .connection
                .transfer(command, rx_len, Duration::from_millis(20))?,
            telem_type,
        )
    }

    /// Get Reset Telemetry
    ///
    /// This command is used to request telemetry items regarding various
    /// reset conditions on both the motherboard and daughterboard.
    ///
    /// # Arguments
    /// `telem_type` - Variant of [`ResetTelemetry::Type`] to request
    ///
    /// [`ResetTelemetry::Type`]: ./ResetTelemetry/enum.Type.html
    fn get_reset_telemetry(
        &self,
        telem_type: telemetry::reset::Type,
    ) -> EpsResult<telemetry::reset::Data> {
        thread::sleep(INTER_COMMAND_DELAY);
        let (command, rx_len) = telemetry::reset::command(telem_type);
        telemetry::reset::parse(&self.connection.transfer(
            command,
            rx_len,
            Duration::from_millis(3),
        )?)
    }

    /// Set Communications Watchdog Period
    ///
    /// The Communications Watchdog by default has a value of 4 minutes set as
    /// its timeout period. If 4 minutes pass without a command being received
    /// then the device will reboot into its pre-defined initial state. This
    /// value of 4 minutes can be changed using the Set Communications Watchdog
    /// Period command, 0x21. The data byte specifies the number of minutes the
    /// communications watchdog will wait before timing out.
    ///
    /// # Arguments
    /// `period` - Watchdog period to set in minutes
    fn set_comms_watchdog_period(&self, period: u8) -> EpsResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        self.connection
            .write(set_comms_watchdog_period::command(period))?;
        Ok(())
    }

    /// Get Communications Watchdog Period
    ///
    /// This command provides the user with the current communications watchdog
    /// timeout that has been set. The returned value is indicated in minutes.
    fn get_comms_watchdog_period(&self) -> EpsResult<u8> {
        thread::sleep(INTER_COMMAND_DELAY);
        let (command, rx_len) = get_comms_watchdog_period::command();
        get_comms_watchdog_period::parse(&self.connection.transfer(
            command,
            rx_len,
            Duration::from_millis(2),
        )?)
    }

    /// Issue Raw Command
    ///
    /// This command sends a raw command to the EPS
    fn raw_command(&self, cmd: u8, data: Vec<u8>) -> EpsResult<()> {
        thread::sleep(INTER_COMMAND_DELAY);
        self.connection.write(Command { cmd, data })?;
        Ok(())
    }
}
