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

//! Main module used for interacting with the underlying EPS API

use crate::models::*;
use clyde_3g_eps_api::{Checksum, Clyde3gEps, Eps};
use eps_api::EpsResult;
use failure::Error;
use rust_i2c::*;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

/// Enum for tracking the last mutation executed
#[derive(Copy, Clone, Debug, Eq, Hash, GraphQLEnum, PartialEq)]
pub enum Mutations {
    /// No mutation has been run since the service was started
    None,
    /// No-op
    Noop,
    /// Manual reset
    ManualReset,
    /// Raw passthrough command
    RawCommand,
    /// Watchdog reset
    ResetWatchdog,
    /// Set watchdog period
    SetWatchdogPeriod,
    /// Hardware test
    TestHardware,
}

fn watchdog_thread(eps: Arc<Mutex<Box<Clyde3gEps + Send>>>) {
    loop {
        thread::sleep(Duration::from_secs(60));
        let _res_ = eps.lock().unwrap().reset_comms_watchdog();
    }
}

/// Main structure for controlling and accessing system resources
#[derive(Clone)]
pub struct Subsystem {
    /// Underlying EPS object
    pub eps: Arc<Mutex<Box<Clyde3gEps + Send>>>,
    /// Last mutation executed
    pub last_mutation: Arc<RwLock<Mutations>>,
    /// Errors accumulated over all queries and mutations
    pub errors: Arc<RwLock<Vec<String>>>,
    /// Watchdog kicking thread handle
    pub watchdog_handle: Arc<Mutex<thread::JoinHandle<()>>>,
    /// Last known checksum of EPS ROM
    pub checksum: Arc<Mutex<Checksum>>,
}

impl Subsystem {
    /// Create a new subsystem instance for the service to use
    pub fn new(eps: Box<Clyde3gEps + Send>) -> EpsResult<Self> {
        let eps = Arc::new(Mutex::new(eps));
        let thread_eps = eps.clone();
        let watchdog = thread::spawn(move || watchdog_thread(thread_eps));

        Ok(Self {
            eps,
            last_mutation: Arc::new(RwLock::new(Mutations::None)),
            errors: Arc::new(RwLock::new(vec![])),
            watchdog_handle: Arc::new(Mutex::new(watchdog)),
            checksum: Arc::new(Mutex::new(Checksum::default())),
        })
    }

    /// Create the underlying EPS object and then create a new subsystem which will use it
    pub fn from_path(bus: &str) -> EpsResult<Self> {
        let clyde_eps: Box<Clyde3gEps + Send> =
            Box::new(Eps::new(Connection::from_path(bus, 0x2B)));
        Subsystem::new(clyde_eps)
    }
}

impl Subsystem {
    /// Get the requested telemetry item from the motherboard
    pub fn get_motherboard_telemetry(
        &self,
        telem_type: motherboard_telemetry::Type,
    ) -> Result<f64, String> {
        let result = run!(
            self.eps
                .lock()
                .unwrap()
                .get_motherboard_telemetry(telem_type.into()),
            self.errors
        )?;

        Ok(result)
    }

    /// Get the requested telemetry item from the daughterboard
    pub fn get_daughterboard_telemetry(
        &self,
        telem_type: daughterboard_telemetry::Type,
    ) -> Result<f64, String> {
        let eps = self.eps.lock().unwrap();
        Ok(run!(
            eps.get_daughterboard_telemetry(telem_type.into()),
            self.errors
        )?)
    }

    /// Get the specific type of reset counts
    pub fn get_reset_telemetry(
        &self,
        telem_type: reset_telemetry::Type,
    ) -> Result<reset_telemetry::Data, String> {
        let eps = self.eps.lock().unwrap();
        Ok(run!(eps.get_reset_telemetry(telem_type.into()), self.errors)?.into())
    }

    /// Get the current watchdog period setting
    pub fn get_comms_watchdog_period(&self) -> Result<u8, String> {
        let eps = self.eps.lock().unwrap();
        Ok(run!(eps.get_comms_watchdog_period(), self.errors)?)
    }

    /// Get the system version information
    pub fn get_version(&self) -> Result<version::VersionData, String> {
        let eps = self.eps.lock().unwrap();
        Ok(run!(eps.get_version_info(), self.errors)?.into())
    }

    /// Get the current board status
    pub fn get_board_status(&self) -> Result<board_status::BoardData, String> {
        let eps = self.eps.lock().unwrap();
        Ok(run!(eps.get_board_status(), self.errors)?.into())
    }

    /// Get the last error the EPS encountered
    pub fn get_last_eps_error(&self) -> Result<last_error::Data, String> {
        let eps = self.eps.lock().unwrap();
        Ok(run!(eps.get_last_error(), self.errors)?.into())
    }

    /// Get the current power state of the EPS
    pub fn get_power(&self) -> Result<GetPowerResponse, String> {
        let eps = self.eps.lock().unwrap();
        if let Ok(data) = eps.get_version_info() {
            let daughterboard = if data.daughterboard.is_some() {
                PowerState::On
            } else {
                PowerState::Off
            };

            Ok(GetPowerResponse {
                motherboard: PowerState::On,
                daughterboard,
            })
        } else {
            Ok(GetPowerResponse {
                motherboard: PowerState::Off,
                daughterboard: PowerState::Off,
            })
        }
    }

    /// Trigger a manual reset of the EPS
    pub fn manual_reset(&self) -> Result<MutationResponse, String> {
        let eps = self.eps.lock().unwrap();
        match run!(eps.manual_reset(), self.errors) {
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

    /// Kick the I2C watchdog
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

    /// Set the I2C watchdog timeout period
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

    /// Pass raw command values through to the EPS
    pub fn raw_command(&self, command: u8, data: Vec<u8>) -> Result<MutationResponse, String> {
        let eps = self.eps.lock().unwrap();
        match run!(eps.raw_command(command, data), self.errors) {
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

    /// Run hardware tests to check system health
    pub fn test_hardware(&self) -> Result<MutationResponse, String> {
        let eps = self.eps.lock().unwrap();
        match run!(eps.get_checksum(), self.errors) {
            Ok(new_data) => {
                let mut errors = vec![];
                let mut success = true;

                let mut old_data = self.checksum.lock().unwrap();

                // Compare the new checksum values to the previously fetched ones
                if old_data.motherboard != 0 {
                    if old_data.motherboard != new_data.motherboard {
                        success = false;
                        errors.push(format!(
                            "Motherboard checksum changed: {} -> {}",
                            old_data.motherboard, new_data.motherboard
                        ));
                    }

                    if old_data.daughterboard != new_data.daughterboard {
                        success = false;
                        errors.push(format!(
                            "Daughterboard checksum changed: {:?} -> {:?}",
                            old_data.daughterboard, new_data.daughterboard
                        ));
                    }
                }

                // Update the stored values
                old_data.motherboard = new_data.motherboard;
                old_data.daughterboard = new_data.daughterboard;

                Ok(MutationResponse {
                    success,
                    errors: errors.join(". "),
                })
            }
            Err(e) => Ok(MutationResponse {
                success: false,
                errors: e,
            }),
        }
    }

    /// Record the last mutation executed by the service
    pub fn set_last_mutation(&self, mutation: Mutations) {
        if let Ok(mut last_cmd) = self.last_mutation.write() {
            *last_cmd = mutation;
        }
    }

    /// Fetch all errors since the last time this function was called, then clear the errors storage
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
