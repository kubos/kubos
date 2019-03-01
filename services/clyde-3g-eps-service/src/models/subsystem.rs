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

use clyde_3g_eps_api::{Clyde3gEps, Eps};
use eps_api::EpsResult;
use failure::Error;
//use kubos_service::MutationResponse;
use crate::models::*;
use rust_i2c::*;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

#[derive(Copy, Clone, Debug, Eq, Hash, GraphQLEnum, PartialEq)]
pub enum Mutations {
    None,
    ManualReset,
    RawCommand,
    ResetWatchdog,
    SetWatchdogPeriod,
}

fn watchdog_thread(eps: Arc<Mutex<Box<Clyde3gEps + Send>>>) {
    loop {
        thread::sleep(Duration::from_secs(60));
        let _res_ = eps.lock().unwrap().reset_comms_watchdog();
    }
}

#[derive(Clone)]
pub struct Subsystem {
    pub eps: Arc<Mutex<Box<Clyde3gEps + Send>>>,
    pub last_mutation: Arc<RwLock<Mutations>>,
    pub errors: Arc<RwLock<Vec<String>>>,
    pub watchdog_handle: Arc<Mutex<thread::JoinHandle<()>>>,
}

impl Subsystem {
    pub fn new(eps: Box<Clyde3gEps + Send>) -> EpsResult<Self> {
        let eps = Arc::new(Mutex::new(eps));
        let thread_eps = eps.clone();
        let watchdog = thread::spawn(move || watchdog_thread(thread_eps));

        Ok(Self {
            eps,
            last_mutation: Arc::new(RwLock::new(Mutations::None)),
            errors: Arc::new(RwLock::new(vec![])),
            watchdog_handle: Arc::new(Mutex::new(watchdog)),
        })
    }

    pub fn from_path(bus: &str) -> EpsResult<Self> {
        let clyde_eps: Box<Clyde3gEps + Send> =
            Box::new(Eps::new(Connection::from_path(bus, 0x2B)));
        Subsystem::new(clyde_eps)
    }
}

impl Subsystem {
    pub fn get_motherboard_telemetry(
        &self,
        telem_type: motherboard_telemetry::Type,
    ) -> Result<f64, String> {
        let result = run!(self
            .eps
            .lock()
            .unwrap()
            .get_motherboard_telemetry(telem_type.into()),
            self.errors)?;
        
        Ok(result)
    }

    pub fn get_daughterboard_telemetry(
        &self,
        telem_type: daughterboard_telemetry::Type,
    ) -> Result<f64, String> {
        let eps = self.eps.lock().unwrap();
        Ok(run!(eps.get_daughterboard_telemetry(telem_type.into()), self.errors)?)
    }

    pub fn get_reset_telemetry(
        &self,
        telem_type: reset_telemetry::Type,
    ) -> Result<reset_telemetry::Data, String> {
        let eps = self.eps.lock().unwrap();
        Ok(run!(eps.get_reset_telemetry(telem_type.into()), self.errors)?.into())
    }

    pub fn get_comms_watchdog_period(&self) -> Result<u8, String> {
        let eps = self.eps.lock().unwrap();
        Ok(run!(eps.get_comms_watchdog_period(), self.errors)?)
    }

    pub fn get_version(&self) -> Result<version::VersionData, String> {
        let eps = self.eps.lock().unwrap();
        Ok(run!(eps.get_version_info(), self.errors)?.into())
    }

    pub fn get_board_status(&self) -> Result<board_status::Data, String> {
        let eps = self.eps.lock().unwrap();
        Ok(run!(eps.get_board_status(), self.errors)?.into())
    }

    pub fn get_last_eps_error(&self) -> Result<last_error::Data, String> {
        let eps = self.eps.lock().unwrap();
        Ok(run!(eps.get_last_error(), self.errors)?.into())
    }

    pub fn manual_reset(&self) -> Result<MutationResponse, String> {
        let eps = self.eps.lock().unwrap();
        match run!(eps.manual_reset(), self.errors) {
            // TODO: What does manual_reset return?
            Ok(_v) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => Ok(MutationResponse {
                success: false,
                errors: e,
            }),
        }
    }

    pub fn reset_watchdog(&self) -> Result<MutationResponse, String> {
        let eps = self.eps.lock().unwrap();
        match run!(eps.reset_comms_watchdog(), self.errors) {
            Ok(_v) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => Ok(MutationResponse {
                success: false,
                errors: e,
            }),
        }
    }

    pub fn set_watchdog_period(&self, period: u8) -> Result<MutationResponse, String> {
        let eps = self.eps.lock().unwrap();
        match run!(eps.set_comms_watchdog_period(period), self.errors) {
            Ok(_v) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => Ok(MutationResponse {
                success: false,
                errors: e,
            }),
        }
    }

    pub fn raw_command(&self, command: u8, data: Vec<u8>) -> Result<MutationResponse, String> {
        let eps = self.eps.lock().unwrap();
        match run!(eps.raw_command(command, data), self.errors) {
            // TODO: do something with the returned data?
            Ok(_v) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => Ok(MutationResponse {
                success: false,
                errors: e,
            }),
        }
    }

    pub fn set_last_mutation(&self, mutation: Mutations) {
        if let Ok(mut last_cmd) = self.last_mutation.write() {
            *last_cmd = mutation;
        }
    }

    pub fn get_errors(&self) -> EpsResult<Vec<String>> {
        match self.errors.write() {
            Ok(mut master_vec) => {
                let current = master_vec.clone();
                master_vec.clear();
                master_vec.shrink_to_fit();
                Ok(current)
            }
            _ => Ok(vec![
                "Error: Failed to borrow master errors vector".to_string()
            ]),
        }
    }
}
