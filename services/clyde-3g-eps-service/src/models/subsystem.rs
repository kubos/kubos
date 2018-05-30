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
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Copy, Clone, GraphQLEnum)]
pub enum Mutations {
    None,
    ManualReset,
    ResetWatchdog,
    SetWatchdogPeriod,
}

fn watchdog_thread(eps: Arc<Mutex<Eps>>) {
    loop {
        thread::sleep(Duration::from_secs(60));
        let _res_ = eps.lock().unwrap().reset_comms_watchdog();
    }
}

pub struct Subsystem {
    pub eps: Arc<Mutex<Eps>>,
    pub last_mutation: Cell<Mutations>,
    pub errors: RefCell<Vec<String>>,
    watchdog_handle: thread::JoinHandle<()>,
}

impl Subsystem {
    pub fn new(bus: &str) -> EpsResult<Subsystem> {
        let eps = Arc::new(Mutex::new(Eps::new(Connection::from_path(bus, 0x2B))));
        let thread_eps = eps.clone();
        let watchdog_handle = thread::spawn(move || watchdog_thread(thread_eps));

        Ok(Subsystem {
            eps,
            last_mutation: Cell::new(Mutations::None),
            errors: RefCell::new(vec![]),
            watchdog_handle,
        })
    }

    pub fn get_motherboard_telemetry(
        &self,
        telem_type: motherboard_telemetry::Type,
    ) -> Result<f32, Error> {
        Ok(self.eps
            .lock()
            .unwrap()
            .get_motherboard_telemetry(telem_type.into())?)
    }

    pub fn get_daughterboard_telemetry(
        &self,
        telem_type: daughterboard_telemetry::Type,
    ) -> Result<f32, Error> {
        Ok(self.eps
            .lock()
            .unwrap()
            .get_daughterboard_telemetry(telem_type.into())?)
    }

    pub fn get_reset_telemetry(
        &self,
        telem_type: reset_telemetry::Type,
    ) -> Result<reset_telemetry::Data, Error> {
        Ok((self.eps
            .lock()
            .unwrap()
            .get_reset_telemetry(telem_type.into())?)
            .into())
    }

    pub fn get_comms_watchdog_period(&self) -> Result<u8, Error> {
        Ok(self.eps.lock().unwrap().get_comms_watchdog_period()?)
    }

    pub fn get_version(&self) -> Result<version::Data, Error> {
        Ok(self.eps.lock().unwrap().get_version_info()?.into())
    }

    pub fn manual_reset(&self) -> Result<(), Error> {
        Ok(self.eps.lock().unwrap().manual_reset()?)
    }

    pub fn reset_watchdog(&self) -> Result<(), Error> {
        Ok(self.eps.lock().unwrap().reset_comms_watchdog()?)
    }

    pub fn set_watchdog_period(&self, period: u8) -> Result<(), Error> {
        Ok(self.eps.lock().unwrap().set_comms_watchdog_period(period)?)
    }
}
