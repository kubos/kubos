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

use i2c_hal::Stream;

use commands::*;
use eps_api::EpsError;

pub struct Eps {
    connection: Box<Stream>,
}

impl Eps {
    pub fn new(connection: Box<Stream>) -> Self {
        Eps { connection }
    }

    /// Retrieves operational status data from TTC node
    pub fn get_board_status(&self) -> Result<board_status::BoardStatus, EpsError> {
        board_status::parse(&self.connection.transfer(board_status::command())?)
    }

    /// Retrieves checksum of TTC node ROM
    pub fn get_checksum(&self) -> Result<checksum::Checksum, EpsError> {
        checksum::parse(&self.connection.transfer(checksum::command())?)
    }

    /// Retrieves firmware and board revision information
    pub fn get_version_info(&self) -> Result<version::VersionInfo, EpsError> {
        version::parse(&self.connection.transfer(version::command())?)
    }

    /// Retrieves details of last error generated
    pub fn get_last_error(&self) -> Result<last_error::LastError, EpsError> {
        last_error::parse(&self.connection.transfer(last_error::command())?)
    }

    /// Performs manual reset of TTC node
    pub fn manual_reset(&self) -> Result<(), EpsError> {
        Ok(self.connection.write(reset::command())?)
    }

    /// Retrieves telemetry
    pub fn get_telemetry(&self, telem_type: telemetry::Type) -> Result<f32, EpsError> {
        telemetry::parse(
            &self.connection.transfer(telemetry::command(telem_type))?,
            telem_type,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error;

    // struct MockConn;

    // impl Stream for MockConn {
    //     fn write(&self, data: &[u8]) -> Result<(), Error> {
    //         Ok(())
    //     }

    //     fn read(&self, length: usize) -> Result<Vec<u8>, Error> {
    //         Ok(vec![])
    //     }
    // }
}
