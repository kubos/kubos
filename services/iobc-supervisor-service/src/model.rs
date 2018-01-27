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

// Why create a new SupervisorVersion struct  which just holds a SupervisorVersion?
// Because of E0117 (https://doc.rust-lang.org/error-index.html#E0117)
// Basically we can't implement the (external) GraphQL traits on
// kubos_hal_iobc::SupervisorVersion because it is an external type
pub struct SupervisorVersion {
    pub version : kubos_hal_iobc::SupervisorVersion
}

// The GraphQL spec only defines an Integer and Float type
// These wrapper functions convert our base types (u8 mostly)
// into the more compatible i32



pub struct SupervisorEnableStatus {
    pub raw : kubos_hal_iobc::SupervisorEnableStatus
}

pub struct SupervisorHousekeeping {
    pub raw : kubos_hal_iobc::SupervisorHousekeeping
}

/*
impl SupervisorHousekeeping {
    pub fn dummy(&self) -> Result<i32, Error> {
        Ok(self.housekeeping.dummy as i32)
    }

    pub fn spi_command_status(&self) -> Result<i32, Error> {
        Ok(self.housekeeping.spi_command_status as i32)
    }


    pub fn enable_status(&self) -> Result<SupervisorEnableStatus, Error> {
        Ok(self.housekeeping.enable_status)
    }


    pub fn supervisor_uptime(&self) -> Result<i32, Error> {
        Ok(self.housekeeping.supervisor_uptime as i32)
    }

    pub fn iobc_uptime(&self) -> Result<i32, Error> {
        Ok(self.housekeeping.iobc_uptime as i32)
    }

    pub fn iobc_reset_count(&self) -> Result<i32, Error> {
        Ok(self.housekeeping.iobc_reset_count as i32)
    }

    pub fn adc_data(&self) -> Result<Vec<i32>, Error> {
        Ok(self.housekeeping.adc_data.iter()
           .map(|x| *x as i32)
           .collect::<Vec<i32>>())
    }

    pub fn adc_update_flag(&self) -> Result<i32, Error> {
        Ok(self.housekeeping.adc_update_flag as i32)
    }

    pub fn crc8(&self) -> Result<i32, Error> {
        Ok(self.housekeeping.crc8 as i32)
    }
}*/


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

    pub fn housekeeping(&self) -> Result<SupervisorHousekeeping, String> {
        match kubos_hal_iobc::supervisor_housekeeping() {
            Ok(h) => Ok(SupervisorHousekeeping { raw : h}),
            Err(e) => Err(e)
        }
    }
}
