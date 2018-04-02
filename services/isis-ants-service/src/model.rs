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

/*
 Master TODO:
 - Add descriptions to everything?
 - Make code more readable (maybe move all enums/structs to own file?
 - Maybe move macros to new common kubos crate
 */

use failure::Fail;
use isis_ants_api::{AntS, AntsTelemetry, DeployStatus, KANTSAnt, KANTSController, KI2CNum};
use std::cell::RefCell;
use std::io::Error;
use std::str;

pub struct Subsystem {
    ants: AntS,
    pub errors: RefCell<Vec<String>>,
}

impl Subsystem {
    pub fn new() -> Subsystem {
        //TODO: Make the AntS configuration params not hard coded
        let subsystem = Subsystem {
            ants: AntS::new(KI2CNum::KI2C1, 0x31, 0x32, 4, 10).unwrap(),
            errors: RefCell::new(vec![]),
        };

        subsystem
    }

    // Queries

    pub fn get_arm_status(&self) -> Result<ArmStatus, Error> {
        let (_errors, _success, deploy) = run!(self.ants.get_deploy(), self.errors);
        let armed = deploy.unwrap_or_default().sys_armed;

        let status = match armed {
            true => ArmStatus::Armed,
            false => ArmStatus::Disarmed,
        };

        Ok(status)
    }


    pub fn get_deploy_status(&self) -> Result<GetDeployResponse, Error> {
        let (_errors, _success, deploy) = run!(self.ants.get_deploy(), self.errors);

        let mut status = DeploymentStatus::Error;

        //TODO: What if there aren't 4 antennas?
        let deploy = deploy.unwrap_or_default();
        if !deploy.ant_1_not_deployed && !deploy.ant_2_not_deployed &&
            !deploy.ant_3_not_deployed && !deploy.ant_4_not_deployed
        {
            // If all antennas are not-not-deployed, then the system is fully deployed
            status = DeploymentStatus::Deployed;

        } else if deploy.ant_1_stopped_time || deploy.ant_2_stopped_time ||
                   deploy.ant_3_stopped_time || deploy.ant_4_stopped_time
        {
            // If any antennas failed to deploy, mark the system as in an error state
            // Note: A successful deployment should clear this flag for an antenna
            status = DeploymentStatus::Error;

        } else if !deploy.ant_1_not_deployed || !deploy.ant_2_not_deployed ||
                   !deploy.ant_3_not_deployed || !deploy.ant_4_not_deployed
        {
            // If there aren't any errors, but some of the antannas have been deployed,
            // mark it as a partial deployment
            // Note: This should be overridden by "InProgress". The only other way
            // a partial deployment is possible (without any errors) is if someone
            // manually deploys a single antenna
            status = DeploymentStatus::Partial

        } else if deploy.ant_1_not_deployed && deploy.ant_2_not_deployed &&
                   deploy.ant_3_not_deployed && deploy.ant_4_not_deployed
        {
            // Otherwise, if they're all marked as not-deployed, then we can
            // assume that the system is currently safely stowed
            status = DeploymentStatus::Stowed;
        }

        // An antenna can be deployed while in any of the other states, so go
        // ahead and override the status if there's an active deployment happening
        if deploy.ant_1_active || deploy.ant_2_active || deploy.ant_3_active ||
            deploy.ant_4_active
        {
            status = DeploymentStatus::InProgress;
        }

        Ok(GetDeployResponse {
            deploy_status: status,
            details: deploy,
        })
    }

    pub fn get_power(&self) -> Result<GetPowerResponse, Error> {
        let (_errors, _success, uptime) = run!(self.ants.get_uptime(), self.errors);

        let state = match uptime.unwrap_or_default() {
            0 => PowerState::Off,
            _ => PowerState::On,
        };

        Ok(GetPowerResponse {
            state: state,
            uptime: uptime.unwrap_or_default(),
        })
    }

    pub fn get_telemetry_debug(&self) -> Result<TelemetryDebug, Error> {
        let telemetry = TelemetryDebug {
            ant1: AntennaStats {
                act_count: run!(self.ants.get_activation_count(KANTSAnt::Ant1), self.errors)
                    .2
                    .unwrap_or_default(),
                act_time: run!(self.ants.get_activation_time(KANTSAnt::Ant1), self.errors)
                    .2
                    .unwrap_or_default(),
            },
            ant2: AntennaStats {
                act_count: run!(self.ants.get_activation_count(KANTSAnt::Ant2), self.errors)
                    .2
                    .unwrap_or_default(),
                act_time: run!(self.ants.get_activation_time(KANTSAnt::Ant2), self.errors)
                    .2
                    .unwrap_or_default(),
            },
            ant3: AntennaStats {
                act_count: run!(self.ants.get_activation_count(KANTSAnt::Ant3), self.errors)
                    .2
                    .unwrap_or_default(),
                act_time: run!(self.ants.get_activation_time(KANTSAnt::Ant3), self.errors)
                    .2
                    .unwrap_or_default(),
            },
            ant4: AntennaStats {
                act_count: run!(self.ants.get_activation_count(KANTSAnt::Ant4), self.errors)
                    .2
                    .unwrap_or_default(),
                act_time: run!(self.ants.get_activation_time(KANTSAnt::Ant4), self.errors)
                    .2
                    .unwrap_or_default(),
            },
        };

        Ok(telemetry)
    }

    pub fn get_telemetry_nominal(&self) -> Result<TelemetryNominal, Error> {
        let (_errors, _success, telemetry) = run!(self.ants.get_system_telemetry(), self.errors);

        Ok(TelemetryNominal(telemetry.unwrap_or_default()))
    }

    pub fn get_test_results(&self) -> Result<IntegrationTestResults, Error> {
        let (_errors, nom_success, nominal) = run!(self.get_telemetry_nominal(), self.errors);
        let (_errors, debug_success, debug) = run!(self.get_telemetry_debug(), self.errors);

        Ok(IntegrationTestResults {
            success: nom_success && debug_success,
            telemetry_nominal: nominal.unwrap_or_default(),
            telemetry_debug: debug.unwrap_or_default(),
        })
    }

    // Mutations

    pub fn arm(&self, state: ArmState) -> Result<ArmResponse, Error> {
        let (errors, success, _data) = match state {
            ArmState::Arm => run!(self.ants.arm(), self.errors),
            ArmState::Disarm => run!(self.ants.disarm(), self.errors),
        };

        Ok(ArmResponse { errors, success })
    }

    pub fn configure_hardware(
        &self,
        controller: ConfigureController,
    ) -> Result<ConfigureHardwareResponse, Error> {
        let conv = match controller {
            ConfigureController::Primary => KANTSController::Primary,
            ConfigureController::Secondary => KANTSController::Secondary,
        };

        let (errors, success, _data) = run!(self.ants.configure(conv), self.errors);

        Ok(ConfigureHardwareResponse {
            config: controller,
            errors,
            success,
        })
    }

    pub fn control_power(&self, state: PowerState) -> Result<ControlPowerResponse, Error> {
        match state {
            PowerState::Reset => {
                let (errors, success, _data) = run!(self.ants.reset(), self.errors);
                //TODO: convert/print error

                Ok(ControlPowerResponse {
                    power: state,
                    errors,
                    success,
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

    pub fn deploy(&self, ant: DeployType, force: bool, time: i32) -> Result<DeployResponse, Error> {

        let mut conv = time as u8;

        if time > 255 {
            conv = 255;
        }

        let (errors, success, _data) = match ant {
            DeployType::All => run!(self.ants.auto_deploy(conv), self.errors),
            DeployType::Antenna1 => {
                run!(self.ants.deploy(KANTSAnt::Ant1, force, conv), self.errors)
            }
            DeployType::Antenna2 => {
                run!(self.ants.deploy(KANTSAnt::Ant2, force, conv), self.errors)
            }
            DeployType::Antenna3 => {
                run!(self.ants.deploy(KANTSAnt::Ant3, force, conv), self.errors)
            }
            DeployType::Antenna4 => {
                run!(self.ants.deploy(KANTSAnt::Ant4, force, conv), self.errors)
            }
        };

        Ok(DeployResponse { errors, success })

    }

    pub fn integration_test(&self) -> Result<IntegrationTestResults, Error> {
        let (_errors, nom_success, nominal) = run!(self.get_telemetry_nominal(), self.errors);
        let (_errors, debug_success, debug) = run!(self.get_telemetry_debug(), self.errors);

        Ok(IntegrationTestResults {
            success: nom_success && debug_success,
            telemetry_nominal: nominal.unwrap_or_default(),
            telemetry_debug: debug.unwrap_or_default(),
        })
    }

    pub fn noop(&self) -> Result<NoopResponse, Error> {

        let (errors, success, _data) = run!(self.ants.watchdog_kick(), self.errors);

        Ok(NoopResponse { errors, success })
    }

    pub fn passthrough(&self, command: String, rx_len: i32) -> Result<RawCommandResponse, Error> {

        // Convert the hex values in the string into actual hex values
        // Ex. "c3c2" -> [0xc3, 0xc2]
        let tx: Vec<u8> = command
            .as_bytes()
            .chunks(2)
            .into_iter()
            .map(|chunk| {
                u8::from_str_radix(str::from_utf8(chunk).unwrap(), 16).unwrap()
            })
            .collect();

        let mut rx: Vec<u8> = vec![0; rx_len as usize];

        let (errors, success, _data) = run!(
            self.ants.passthrough(tx.as_slice(), rx.as_mut_slice()),
            self.errors
        );

        // Convert the response hex values into a String for the GraphQL output
        // Note: This is in BIG ENDIAN format
        Ok(RawCommandResponse {
            errors,
            success,
            response: rx.iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<String>(),
        })
    }
}
