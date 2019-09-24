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

use failure::Error;
use isis_ants_api::*;
use kubos_service::{process_errors, push_err, run};
use log::info;
use std::str;
use std::sync::{Arc, Mutex, RwLock};

use crate::objects::*;

#[derive(Clone)]
pub struct Subsystem {
    pub ants: Arc<Mutex<Box<dyn IAntS>>>,
    pub count: u8,
    pub controller: Arc<RwLock<ConfigureController>>,
    pub errors: Arc<RwLock<Vec<String>>>,
    pub last_cmd: Arc<RwLock<AckCommand>>,
}

impl Subsystem {
    pub fn new(
        bus: &str,
        primary: u8,
        secondary: u8,
        count: u8,
        timeout: u32,
    ) -> AntSResult<Subsystem> {
        let ants: Arc<Mutex<Box<dyn IAntS>>> = Arc::new(Mutex::new(Box::new(AntS::new(
            bus, primary, secondary, count, timeout,
        )?)));

        info!("Kubos antenna systems service started");

        Ok(Subsystem {
            ants,
            count,
            controller: Arc::new(RwLock::new(ConfigureController::Primary)),
            errors: Arc::new(RwLock::new(vec![])),
            last_cmd: Arc::new(RwLock::new(AckCommand::None)),
        })
    }

    // Queries

    #[allow(clippy::clone_on_copy)]
    pub fn get_config(&self) -> AntSResult<ConfigureController> {
        let controller = self
            .controller
            .read()
            .map_err(|_| AntsError::GenericError)?
            .clone();
        Ok(controller)
    }

    pub fn get_arm_status(&self) -> AntSResult<ArmStatus> {
        let result = run!(self.ants.lock().unwrap().get_deploy(), self.errors);
        let armed = result.unwrap_or_default().sys_armed;

        Ok(if armed {
            ArmStatus::Armed
        } else {
            ArmStatus::Disarmed
        })
    }

    pub fn get_deploy_status(&self) -> AntSResult<GetDeployResponse> {
        let result = run!(self.ants.lock().unwrap().get_deploy(), self.errors);

        let mut status = DeploymentStatus::Error;

        let deploy = result.clone().unwrap_or_default();

        // If our API call threw any errors, just go ahead and quit now
        if result.is_err() {
            return Ok(GetDeployResponse {
                status,
                details: deploy,
            });
        }

        let mut deployed = !deploy.ant_1_not_deployed && !deploy.ant_2_not_deployed;
        if self.count > 2 {
            deployed = deployed && !deploy.ant_3_not_deployed;
        }
        if self.count > 3 {
            deployed = deployed && !deploy.ant_4_not_deployed;
        }

        if deployed {
            // If all antennas are not-not-deployed, then the system is fully deployed
            status = DeploymentStatus::Deployed;
        } else if deploy.ant_1_stopped_time
            || deploy.ant_2_stopped_time
            || deploy.ant_3_stopped_time
            || deploy.ant_4_stopped_time
        {
            // If any antennas failed to deploy, mark the system as in an error state
            // Note: A successful deployment should clear this flag for an antenna
            status = DeploymentStatus::Error;
        } else if !deploy.ant_1_not_deployed
            || !deploy.ant_2_not_deployed
            || (!deploy.ant_3_not_deployed && self.count > 2)
            || (!deploy.ant_4_not_deployed && self.count > 3)
        {
            // If there aren't any errors, but some of the antannas have been deployed,
            // mark it as a partial deployment
            // Note: This should be overridden by "InProgress". The only real way
            // a partial deployment is possible (without any errors) is if someone
            // manually deploys a single antenna
            status = DeploymentStatus::Partial
        } else if deploy.ant_1_not_deployed
            && deploy.ant_2_not_deployed
            && (deploy.ant_3_not_deployed || self.count < 3)
            && (deploy.ant_4_not_deployed || self.count < 4)
        {
            // Otherwise, if they're all marked as not-deployed, then we can
            // assume that the system is currently safely stowed
            status = DeploymentStatus::Stowed;
        }

        // An antenna can be deployed while in any of the other states, so go
        // ahead and override the status if there's an active deployment happening
        if deploy.ant_1_active || deploy.ant_2_active || deploy.ant_3_active || deploy.ant_4_active
        {
            status = DeploymentStatus::InProgress;
        }

        Ok(GetDeployResponse {
            status,
            details: deploy,
        })
    }

    pub fn get_power(&self) -> AntSResult<GetPowerResponse> {
        let result = run!(self.ants.lock().unwrap().get_uptime(), self.errors);
        let uptime = result.unwrap_or_default();

        let state = match uptime {
            0 => PowerState::Off,
            _ => PowerState::On,
        };

        Ok(GetPowerResponse { state, uptime })
    }

    pub fn get_telemetry(&self) -> AntSResult<Telemetry> {
        let nominal = run!(
            self.ants.lock().unwrap().get_system_telemetry(),
            self.errors
        )
        .unwrap_or_default();

        let debug = TelemetryDebug {
            ant1: get_stats(&self.ants, &self.errors, KANTSAnt::Ant1),
            ant2: get_stats(&self.ants, &self.errors, KANTSAnt::Ant2),
            ant3: get_stats(&self.ants, &self.errors, KANTSAnt::Ant3),
            ant4: get_stats(&self.ants, &self.errors, KANTSAnt::Ant4),
        };

        Ok(Telemetry {
            nominal: TelemetryNominal(nominal),
            debug,
        })
    }

    pub fn get_test_results(&self) -> AntSResult<IntegrationTestResults> {
        self.integration_test()
    }

    // Mutations

    pub fn arm(&self, state: ArmState) -> AntSResult<ArmResponse> {
        let result = match state {
            ArmState::Arm => run!(self.ants.lock().unwrap().arm(), self.errors),
            ArmState::Disarm => run!(self.ants.lock().unwrap().disarm(), self.errors),
        };

        Ok(ArmResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }

    pub fn configure_hardware(
        &self,
        controller: ConfigureController,
    ) -> AntSResult<ConfigureHardwareResponse> {
        let conv = match controller {
            ConfigureController::Primary => KANTSController::Primary,
            ConfigureController::Secondary => KANTSController::Secondary,
        };

        let result = run!(self.ants.lock().unwrap().configure(conv), self.errors);

        if result.is_ok() {
            let mut curr_controller = self
                .controller
                .write()
                .map_err(|_| AntsError::GenericError)?;
            *curr_controller = controller;
        }

        Ok(ConfigureHardwareResponse {
            config: controller,
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }

    pub fn control_power(&self, state: PowerState) -> AntSResult<ControlPowerResponse> {
        match state {
            PowerState::Reset => {
                let result = run!(self.ants.lock().unwrap().reset(), self.errors);

                Ok(ControlPowerResponse {
                    power: state,
                    success: result.is_ok(),
                    errors: match result {
                        Ok(_) => "".to_owned(),
                        Err(err) => err,
                    },
                })
            }
            _ => {
                push_err!(self.errors, "controlPower: Invalid power state".to_owned());

                Ok(ControlPowerResponse {
                    power: state,
                    errors: String::from("Invalid power state"),
                    success: false,
                })
            }
        }
    }

    pub fn deploy(&self, ant: DeployType, force: bool, time: i32) -> AntSResult<DeployResponse> {
        let conv = if time > 255 { 255 } else { time as u8 };

        let result = match ant {
            DeployType::All => run!(self.ants.lock().unwrap().auto_deploy(conv), self.errors),
            DeployType::Antenna1 => run!(
                self.ants
                    .lock()
                    .unwrap()
                    .deploy(KANTSAnt::Ant1, force, conv),
                self.errors
            ),
            DeployType::Antenna2 => run!(
                self.ants
                    .lock()
                    .unwrap()
                    .deploy(KANTSAnt::Ant2, force, conv),
                self.errors
            ),
            DeployType::Antenna3 => run!(
                self.ants
                    .lock()
                    .unwrap()
                    .deploy(KANTSAnt::Ant3, force, conv),
                self.errors
            ),
            DeployType::Antenna4 => run!(
                self.ants
                    .lock()
                    .unwrap()
                    .deploy(KANTSAnt::Ant4, force, conv),
                self.errors
            ),
        };

        Ok(DeployResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }

    pub fn integration_test(&self) -> AntSResult<IntegrationTestResults> {
        let nom_result = run!(
            self.ants.lock().unwrap().get_system_telemetry(),
            self.errors
        );

        let debug_errors: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(vec![]));

        let debug = TelemetryDebug {
            ant1: get_stats(&self.ants, &debug_errors, KANTSAnt::Ant1),
            ant2: get_stats(&self.ants, &debug_errors, KANTSAnt::Ant2),
            ant3: get_stats(&self.ants, &debug_errors, KANTSAnt::Ant3),
            ant4: get_stats(&self.ants, &debug_errors, KANTSAnt::Ant4),
        };

        let debug_errors = Arc::try_unwrap(debug_errors)
            .map_err(|_| AntsError::GenericError)?
            .into_inner()
            .map_err(|_| AntsError::GenericError)?;

        let success = nom_result.is_ok() && debug_errors.is_empty();
        let mut errors = String::new();

        let telemetry_nominal = match nom_result {
            Ok(data) => TelemetryNominal(data),
            Err(err) => {
                errors.push_str(&format!("Nominal: {}", err));
                TelemetryNominal::default()
            }
        };

        if !debug_errors.is_empty() {
            if !errors.is_empty() {
                errors.push_str("; ");
            }
            let concat = debug_errors.join(", ");
            errors.push_str(&format!("Debug: {}", concat));
            push_err!(self.errors, format!("get_test_results(debug): {}", concat));
        }

        Ok(IntegrationTestResults {
            errors,
            success,
            telemetry_nominal,
            telemetry_debug: debug,
        })
    }

    pub fn noop(&self) -> AntSResult<NoopResponse> {
        let result = run!(self.ants.lock().unwrap().watchdog_kick(), self.errors);

        Ok(NoopResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }

    pub fn passthrough(&self, command: String, rx_len: i32) -> AntSResult<RawCommandResponse> {
        // Convert the hex values in the string into actual hex values
        // Ex. "c3c2" -> [0xc3, 0xc2]
        let tx: Vec<u8> = command
            .as_bytes()
            .chunks(2)
            .map(|chunk| u8::from_str_radix(str::from_utf8(chunk).unwrap(), 16).unwrap())
            .collect();

        let mut rx: Vec<u8> = vec![0; rx_len as usize];

        let result = run!(
            self.ants
                .lock()
                .unwrap()
                .passthrough(tx.as_slice(), rx.as_mut_slice()),
            self.errors
        );

        // Convert the response hex values into a String for the GraphQL output
        // Note: This is in BIG ENDIAN format
        Ok(match result {
            Ok(_) => RawCommandResponse {
                success: true,
                errors: "".to_owned(),
                response: rx
                    .iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect::<String>(),
            },
            Err(err) => RawCommandResponse {
                success: false,
                errors: err,
                response: "".to_owned(),
            },
        })
    }
}
