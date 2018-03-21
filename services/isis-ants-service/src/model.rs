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

use isis_ants_api::{AntS, KANTSAnt, KI2CNum, AntsTelemetry, KANTSController, DeployStatus};

use std::io::Error;

//TODO: Change to boolean?
#[derive(GraphQLEnum)]
pub enum ArmStatus {
    Armed,
    Disarmed,
}

#[derive(GraphQLEnum, Clone)]
pub enum DeploymentStatus {
    Deployed,
    InProgress,
    Partial,
    Stowed,
    Error,
}

#[derive(GraphQLEnum)]
pub enum ArmState {
    Arm,
    Disarm,
}

#[derive(GraphQLEnum, Clone)]
pub enum PowerState {
    On,
    Off,
    Reset,
}

#[derive(GraphQLEnum, Clone)]
pub enum ConfigureController {
    Primary,
    Secondary,
}

#[derive(GraphQLEnum, Clone)]
pub enum TestType {
    Integration,
    Hardware,
}

#[derive(GraphQLEnum)]
pub enum DeployType {
    All,
    Antenna1,
    Antenna2,
    Antenna3,
    Antenna4,
}

pub struct GetDeployResponse {
    pub deploy_status: DeploymentStatus,
    pub details: DeployStatus,
}

/// Model for power queries
pub struct GetPowerResponse {
    pub state: PowerState,
    pub uptime: u32,
}

#[derive(GraphQLObject)]
pub struct GenericResponse {
    pub errors: String,
    pub success: bool,
}

pub type NoopResponse = GenericResponse;
pub type ArmResponse = GenericResponse;
pub type DeployResponse = GenericResponse;

/// Model for power mutations
#[derive(GraphQLObject)]
pub struct ControlPowerResponse {
    pub errors: String,
    pub success: bool,
    pub power: PowerState,
}

#[derive(GraphQLObject)]
pub struct ConfigureHardwareResponse {
    pub errors: String,
    pub success: bool,
    pub config: ConfigureController,
}

#[derive(GraphQLEnum)]
pub enum TelemetryType {
    Nominal,
    Debug,
}

pub enum Telemetry {
    Nominal(TelemetryNominal),
    Debug(TelemetryDebug),
}

pub struct TelemetryNominal(pub AntsTelemetry);

pub struct AntennaStats {
    pub act_count: u8,
    pub act_time: u16,
}

pub struct TelemetryDebug {
    pub ant1: AntennaStats,
    pub ant2: AntennaStats,
    pub ant3: AntennaStats,
    pub ant4: AntennaStats,
}

pub struct Subsystem {
    ants: AntS,
}

#[derive(GraphQLObject)]
pub struct TestResults {
    pub success: bool,
    pub telemetry_nominal: TelemetryNominal,
    pub telemetry_debug: TelemetryDebug,
}

#[derive(GraphQLObject)]
pub struct RawCommandResponse {
    pub errors: String,
    pub success: bool,
    pub response: String,
}

impl Subsystem {
    pub fn new() -> Subsystem {
        let subsystem = Subsystem { ants: AntS::new(KI2CNum::KI2C1, 0x31, 0x32, 4, 10).unwrap() };

        subsystem
        //TODO: error handling
    }

    pub fn get_arm_status(&self) -> Result<ArmStatus, Error> {
        let deploy = self.ants.get_deploy().unwrap();
        let armed = deploy.sys_armed;

        let status = match armed {
            true => ArmStatus::Armed,
            false => ArmStatus::Disarmed,
        };

        Ok(status)
    }

    pub fn get_power(&self) -> Result<GetPowerResponse, Error> {
        let uptime = self.ants.get_uptime().unwrap();

        let state = match uptime {
            0 => PowerState::Off,
            _ => PowerState::On,
        };

        Ok(GetPowerResponse {
            state: state,
            uptime: uptime,
        })
    }

    pub fn get_deploy_status(&self) -> Result<GetDeployResponse, Error> {
        let deploy = self.ants.get_deploy().unwrap();

        let mut status = DeploymentStatus::Error;

        //TODO: What if there aren't 4 antennas?

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

    pub fn get_telemetry_nominal(&self) -> Result<TelemetryNominal, Error> {
        let telemetry = TelemetryNominal(self.ants.get_system_telemetry().unwrap());

        Ok(telemetry)
    }

    pub fn get_telemetry_debug(&self) -> Result<TelemetryDebug, Error> {
        let telemetry = TelemetryDebug {
            ant1: AntennaStats {
                act_count: self.ants.get_activation_count(KANTSAnt::Ant1).unwrap(),
                act_time: self.ants.get_activation_time(KANTSAnt::Ant1).unwrap(),
            },
            ant2: AntennaStats {
                act_count: self.ants.get_activation_count(KANTSAnt::Ant2).unwrap(),
                act_time: self.ants.get_activation_time(KANTSAnt::Ant2).unwrap(),
            },
            ant3: AntennaStats {
                act_count: self.ants.get_activation_count(KANTSAnt::Ant3).unwrap(),
                act_time: self.ants.get_activation_time(KANTSAnt::Ant3).unwrap(),
            },
            ant4: AntennaStats {
                act_count: self.ants.get_activation_count(KANTSAnt::Ant4).unwrap(),
                act_time: self.ants.get_activation_time(KANTSAnt::Ant4).unwrap(),
            },
        };

        Ok(telemetry)
    }

    pub fn get_test_results(&self) -> Result<TestResults, Error> {
        //TODO: Just fetch previous results instead?
        let nominal = self.get_telemetry_nominal().unwrap();
        let debug = self.get_telemetry_debug().unwrap();

        Ok(TestResults {
            success: true,
            telemetry_nominal: nominal,
            telemetry_debug: debug,
        })
    }

    pub fn noop(&self) -> Result<NoopResponse, Error> {
        self.ants.watchdog_kick().unwrap();

        Ok(NoopResponse {
            errors: String::from(""),
            success: true,
        })
    }

    pub fn configure_hardware(
        &self,
        controller: ConfigureController,
    ) -> Result<ConfigureHardwareResponse, Error> {
        let conv = match controller {
            ConfigureController::Primary => KANTSController::Primary,
            ConfigureController::Secondary => KANTSController::Secondary,
        };

        self.ants.configure(conv).unwrap();

        Ok(ConfigureHardwareResponse {
            config: controller,
            errors: String::from(""),
            success: true,
        })
    }

    pub fn control_power(&self, state: PowerState) -> Result<ControlPowerResponse, Error> {
        match state {
            PowerState::Reset => {
                self.ants.reset().unwrap();
                //TODO: convert/print error

                Ok(ControlPowerResponse {
                    power: state,
                    errors: String::from(""),
                    success: true,
                })

            } 
            _ => Ok(ControlPowerResponse {
                power: state,
                errors: String::from("Invalid power state"),
                success: false,
            }),

        }
    }

    pub fn arm(&self, state: ArmState) -> Result<ArmResponse, Error> {
        match state {
            ArmState::Arm => self.ants.arm().unwrap(),
            ArmState::Disarm => self.ants.disarm().unwrap(),
        };

        Ok(ArmResponse {
            errors: String::from(""),
            success: true,
        })
    }

    pub fn deploy(&self, ant: DeployType, force: bool, time: i32) -> Result<DeployResponse, Error> {

        let mut conv = time as u8;

        if time > 255 {
            conv = 255;
            //TODO: Set error string
        }

        match ant {
            DeployType::All => {
                self.ants.auto_deploy(conv).unwrap();
            }
            DeployType::Antenna1 => {
                self.ants.deploy(KANTSAnt::Ant1, force, conv).unwrap();
            }
            DeployType::Antenna2 => {
                self.ants.deploy(KANTSAnt::Ant2, force, conv).unwrap();
            }
            DeployType::Antenna3 => {
                self.ants.deploy(KANTSAnt::Ant3, force, conv).unwrap();
            }
            DeployType::Antenna4 => {
                self.ants.deploy(KANTSAnt::Ant4, force, conv).unwrap();
            }
        }

        Ok(DeployResponse {
            errors: String::from(""),
            success: true,
        })

    }

    pub fn test_hardware(&self, _test: TestType) -> Result<TestResults, Error> {
        //TODO: Handle hardware vs integration testing?
        let nominal = self.get_telemetry_nominal().unwrap();
        let debug = self.get_telemetry_debug().unwrap();

        Ok(TestResults {
            success: true,
            telemetry_nominal: nominal,
            telemetry_debug: debug,
        })
    }

    pub fn passthrough(&self, command: String, rx_len: i32) -> Result<RawCommandResponse, Error> {
        let tx: &[u8] = command.as_bytes();
        let mut rx = Vec::with_capacity(rx_len as usize);

        self.ants.passthrough(tx, rx.as_mut_slice()).unwrap();

        Ok(RawCommandResponse {
            errors: String::from(""),
            success: true,
            response: String::from(""),
        })
    }
}
