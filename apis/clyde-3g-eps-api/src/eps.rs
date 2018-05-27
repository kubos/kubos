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

use commands::*;
use eps_api::EpsResult;
use i2c_hal::Connection;
use telemetry;

/// Eps structure containing low level connection and functionality
/// required for commanding and requesting telemetry from Eps device.
pub struct Eps {
    connection: Connection,
}

impl Eps {
    /// Constructor
    ///
    /// Creates new instance of Eps structure.
    ///
    /// # Arguments
    /// - connection - A `Box<Stream>` used as low-level connection to eps hardware
    pub fn new(connection: Connection) -> Self {
        Eps { connection }
    }

    /// Get Board Status
    ///
    /// The status bytes are designed to supply operational data about the I2C Node.
    pub fn get_board_status(&self) -> EpsResult<board_status::BoardStatus> {
        board_status::parse(&self.connection.transfer(board_status::command(), 2)?)
    }

    /// Get Checksum
    ///
    /// This command instructs the node to self-inspect its ROM contents in order
    /// to generate a checksum. The value retrieved can be used to determine whether
    /// the contents of the ROM have changed during the operation of the device.
    pub fn get_checksum(&self) -> EpsResult<checksum::Checksum> {
        checksum::parse(&self.connection.transfer(checksum::command(), 50)?)
    }

    /// Get Version
    ///
    /// The version number of the firmware will be returned on this command.
    /// The revision number returns the current revision of the firmware that is
    /// present on the board. The firmware number returns the current firmware on the board.
    pub fn get_version_info(&self) -> EpsResult<version::VersionInfo> {
        version::parse(&self.connection.transfer(version::command(), 2)?)
    }

    /// Get Last Error
    ///
    /// If an error has been generated after attempting to execute a user's command,
    /// this command can be used to retrieve details about the error.
    pub fn get_last_error(&self) -> EpsResult<last_error::LastError> {
        last_error::parse(&self.connection.transfer(last_error::command(), 2)?)
    }

    /// Manual Reset
    ///
    /// If required the user can reset the TTC node using this command. When issued,
    /// the board will reset within 1 second. This command will result in the board
    /// being brought up in its defined initial condition. Resetting the board in
    /// this fashion will increment the Manual Reset Counter.
    pub fn manual_reset(&self) -> EpsResult<()> {
        self.connection.write(manual_reset::command())?;
        Ok(())
    }

    /// Reset Communications Watchdog
    ///
    /// Any valid command will reset the communications watchdog timer. If the user
    /// does not require any telemetry from the board, this command can be sent
    /// to reset the communications watchdog.
    pub fn reset_comms_watchdog(&self) -> EpsResult<()> {
        self.connection.write(reset_comms_watchdog::command())?;
        Ok(())
    }

    /// Get Motherboard Telemetry
    ///
    /// This command is used to request telemetry items from the motherboard's
    /// telemetry node.
    ///
    /// # Arguments
    /// `telem_type` - Variant of `telemetry::motherboard::Type` to request
    pub fn get_motherboard_telemetry(
        &self,
        telem_type: telemetry::motherboard::Type,
    ) -> EpsResult<f32> {
        telemetry::motherboard::parse(
            &self
                .connection
                .transfer(telemetry::motherboard::command(telem_type), 20)?,
            telem_type,
        )
    }

    /// Get Daughterboard Telemetry
    ///
    /// This command is used to request telemetry items from the daughterboard's
    /// telemetry node.
    ///
    /// # Arguments
    /// `telem_type` - Variant of `telemetry::daughterboard::Type` to request
    pub fn get_daughterboard_telemetry(
        &self,
        telem_type: telemetry::daughterboard::Type,
    ) -> EpsResult<f32> {
        telemetry::daughterboard::parse(
            &self
                .connection
                .transfer(telemetry::daughterboard::command(telem_type), 20)?,
            telem_type,
        )
    }

    /// Get Reset Telemetry
    ///
    /// This command is used to request telemetry items regarding various
    /// reset conditions on both the motherboard and daughterboard.
    ///
    /// # Arguments
    /// `telem_type` - Variant of `telemetry::daughterboard::ResetType` to request
    pub fn get_reset_telemetry(
        &self,
        telem_type: telemetry::reset::ResetType,
    ) -> EpsResult<telemetry::reset::ResetTelemetry> {
        telemetry::reset::parse(&self
            .connection
            .transfer(telemetry::reset::command(telem_type), 20)?)
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
    /// 'period' - Watchdog period to set in minutes
    pub fn set_comms_watchdog_period(&self, period: u8) -> EpsResult<()> {
        self.connection
            .write(set_comms_watchdog_period::command(period))?;
        Ok(())
    }

    /// Get Communications Watchdog Period
    ///
    /// This command provides the user with the current communications watchdog
    /// timeout that has been set. The returned value is indicated in minutes.
    pub fn get_comms_watchdog_period(&self) -> EpsResult<u8> {
        get_comms_watchdog_period::parse(&self
            .connection
            .transfer(get_comms_watchdog_period::command(), 2)?)
    }
}
