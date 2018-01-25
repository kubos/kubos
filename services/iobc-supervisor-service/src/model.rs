//
// Copyright (C) 2017 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

extern crate kubos_hal_iobc;

use std::io::{Error, ErrorKind};

// Why create a new SupervisorVersion struct  which just holds a SupervisorVersion?
// Because of E0117 (https://doc.rust-lang.org/error-index.html#E0117)
// Basically we can't implement the (external) GraphQL traits on
// kubos_hal_iobc::SupervisorVersion because it is an external type
pub struct SupervisorVersion {
    version : kubos_hal_iobc::SupervisorVersion
}

// The GraphQL spec only defines an Integer and Float type
// These wrapper functions convert our base types (u8 mostly)
// into the more compatible i32
impl SupervisorVersion {
    pub fn dummy(&self) -> Result<i32, Error> {
        Ok(self.version.dummy as i32)
    }

    pub fn spi_command_status(&self) -> Result<i32, Error> {
        Ok(self.version.spi_command_status as i32)
    }

    pub fn index_of_subsystem(&self) -> Result<i32, Error> {
        Ok(self.version.index_of_subsystem as i32)
    }

    pub fn major_version(&self) -> Result<i32, Error> {
        Ok(self.version.major_version as i32)
    }

    pub fn minor_version(&self) -> Result<i32, Error> {
        Ok(self.version.minor_version as i32)
    }

    pub fn patch_version(&self) -> Result<i32, Error> {
        Ok(self.version.patch_version as i32)
    }

    pub fn git_head_version(&self) -> Result<i32, Error> {
        Ok(self.version.git_head_version as i32)
    }

    pub fn serial_number(&self) -> Result<i32, Error> {
        Ok(self.version.serial_number as i32)
    }

    pub fn compile_information(&self) -> Result<Vec<i32>, Error> {
        Ok(self.version.compile_information.iter()
            .map(|x| *x as i32)
            .collect::<Vec<i32>>())
    }

    pub fn clock_speed(&self) -> Result<i32, Error> {
        Ok(self.version.clock_speed as i32)
    }

    pub fn code_type(&self) -> Result<i32, Error> {
        Ok(self.version.code_type as i32)
    }

    pub fn crc(&self) -> Result<i32, Error> {
        Ok(self.version.crc as i32)
    }
}

/// Model for handler's subsystem
pub struct Supervisor;

impl Supervisor {
    pub fn new() -> Supervisor {
        Supervisor {}
    }

    pub fn version(&self) -> Result<SupervisorVersion, String> {
        match kubos_hal_iobc::supervisor_version() {
            Ok(v) => Ok(SupervisorVersion { version: v }),
            Err(e) => Err(e)
        }
    }
}
