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
use kubos_service::MutationResponse;
use models::*;
use rust_i2c::*;
use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex};
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

fn watchdog_thread(eps: Arc<Mutex<Eps>>) {
    loop {
        thread::sleep(Duration::from_secs(60));
        let _res_ = eps.lock().unwrap().reset_comms_watchdog();
    }
}

pub trait Subsystem {
    fn get_motherboard_telemetry(&self, telem_type: motherboard_telemetry::Type) -> EpsResult<f32>;
    fn get_daughterboard_telemetry(
        &self,
        telem_type: daughterboard_telemetry::Type,
    ) -> EpsResult<f32>;
    fn get_reset_telemetry(
        &self,
        telem_type: reset_telemetry::Type,
    ) -> EpsResult<reset_telemetry::Data>;
    fn get_comms_watchdog_period(&self) -> EpsResult<u8>;
    fn get_version(&self) -> EpsResult<version::Data>;
    fn get_board_status(&self) -> EpsResult<board_status::Data>;
    fn get_last_eps_error(&self) -> EpsResult<last_error::Data>;
    fn manual_reset(&self) -> EpsResult<MutationResponse>;
    fn reset_watchdog(&self) -> EpsResult<MutationResponse>;
    fn set_watchdog_period(&self, period: u8) -> EpsResult<MutationResponse>;
    fn raw_command(&self, command: u8, data: Vec<u8>) -> EpsResult<MutationResponse>;
    fn get_last_mutation(&self) -> Mutations;
    fn set_last_mutation(&self, mutation: Mutations);
    fn get_errors(&self) -> EpsResult<Vec<String>>;
}

pub struct RealSubsystem {
    pub eps: Arc<Mutex<Eps>>,
    pub last_mutation: Cell<Mutations>,
    pub errors: RefCell<Vec<String>>,
    pub watchdog_handle: thread::JoinHandle<()>,
}

impl RealSubsystem {
    pub fn new(bus: &str) -> EpsResult<RealSubsystem> {
        let eps = Arc::new(Mutex::new(Eps::new(Connection::from_path(bus, 0x2B))));
        let thread_eps = eps.clone();
        let watchdog_handle = thread::spawn(move || watchdog_thread(thread_eps));

        Ok(RealSubsystem {
            eps,
            last_mutation: Cell::new(Mutations::None),
            errors: RefCell::new(vec![]),
            watchdog_handle,
        })
    }
}

impl Subsystem for RealSubsystem {
    fn get_motherboard_telemetry(&self, telem_type: motherboard_telemetry::Type) -> EpsResult<f32> {
        Ok(self.eps
            .lock()
            .unwrap()
            .get_motherboard_telemetry(telem_type.into())?)
    }

    fn get_daughterboard_telemetry(
        &self,
        telem_type: daughterboard_telemetry::Type,
    ) -> EpsResult<f32> {
        Ok(self.eps
            .lock()
            .unwrap()
            .get_daughterboard_telemetry(telem_type.into())?)
    }

    fn get_reset_telemetry(
        &self,
        telem_type: reset_telemetry::Type,
    ) -> EpsResult<reset_telemetry::Data> {
        Ok((self.eps
            .lock()
            .unwrap()
            .get_reset_telemetry(telem_type.into())?)
            .into())
    }

    fn get_comms_watchdog_period(&self) -> EpsResult<u8> {
        Ok(self.eps.lock().unwrap().get_comms_watchdog_period()?)
    }

    fn get_version(&self) -> EpsResult<version::Data> {
        Ok(self.eps.lock().unwrap().get_version_info()?.into())
    }

    fn get_board_status(&self) -> EpsResult<board_status::Data> {
        Ok(self.eps.lock().unwrap().get_board_status()?.into())
    }

    fn get_last_eps_error(&self) -> EpsResult<last_error::Data> {
        Ok(self.eps.lock().unwrap().get_last_error()?.into())
    }

    fn manual_reset(&self) -> EpsResult<MutationResponse> {
        match self.eps.lock().unwrap().manual_reset() {
            Ok(_v) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => throw!(e),
        }
    }

    fn reset_watchdog(&self) -> EpsResult<MutationResponse> {
        match self.eps.lock().unwrap().reset_comms_watchdog() {
            Ok(_v) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => throw!(e),
        }
    }

    fn set_watchdog_period(&self, period: u8) -> EpsResult<MutationResponse> {
        match self.eps.lock().unwrap().set_comms_watchdog_period(period) {
            Ok(_v) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => throw!(e),
        }
    }

    fn raw_command(&self, command: u8, data: Vec<u8>) -> EpsResult<MutationResponse> {
        match self.eps.lock().unwrap().raw_command(command, data) {
            Ok(_v) => Ok(MutationResponse {
                success: true,
                errors: "".to_string(),
            }),
            Err(e) => throw!(e),
        }
    }

    fn get_last_mutation(&self) -> Mutations {
        Mutations::None
    }

    fn set_last_mutation(&self, _mutation: Mutations) {
        ()
    }

    fn get_errors(&self) -> EpsResult<Vec<String>> {
        match self.errors.try_borrow_mut() {
            Ok(mut master_vec) => {
                let current = master_vec.clone();
                master_vec.clear();
                master_vec.shrink_to_fit();
                Ok(current)
            }
            _ => Ok(vec![
                "Error: Failed to borrow master errors vector".to_string(),
            ]),
        }
    }
}
