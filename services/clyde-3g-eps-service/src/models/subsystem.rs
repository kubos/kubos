//
// Copyright (C) 2018 Kubos Corporation
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

use clyde_3g_eps_api::Eps;
use eps_api::EpsResult;
use failure::Error;
use i2c_hal::*;
use models::*;
use std::cell::{Cell, RefCell};

pub enum AckCommand {
    None,
}

pub struct Subsystem {
    pub eps: Eps,
    pub last_cmd: Cell<AckCommand>,
    pub errors: RefCell<Vec<String>>,
}

impl Subsystem {
    pub fn new(bus: &str) -> EpsResult<Subsystem> {
        let eps = Eps::new(Connection::from_path(bus, 0x2B));

        Ok(Subsystem {
            eps,
            last_cmd: Cell::new(AckCommand::None),
            errors: RefCell::new(vec![]),
        })
    }

    pub fn get_motherboard_telemetry(
        &self,
        telem_type: motherboard_telemetry::Type,
    ) -> Result<f32, Error> {
        Ok(self.eps.get_motherboard_telemetry(telem_type.into())?)
    }

    pub fn get_daughterboard_telemetry(
        &self,
        telem_type: daughterboard_telemetry::Type,
    ) -> Result<f32, Error> {
        Ok(self.eps.get_daughterboard_telemetry(telem_type.into())?)
    }

    pub fn get_reset_telemetry(
        &self,
        telem_type: reset_telemetry::Type,
    ) -> Result<reset_telemetry::Data, Error> {
        Ok((self.eps.get_reset_telemetry(telem_type.into())?).into())
    }

    pub fn get_comms_watchdog_period(&self) -> Result<u8, Error> {
        Ok(self.eps.get_comms_watchdog_period()?)
    }

    pub fn get_version(&self) -> Result<version::Data, Error> {
        Ok(self.eps.get_version_info()?.into())
    }
}
