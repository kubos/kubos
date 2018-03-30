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

use failure::Fail;
use isis_ants_api::{AntS, KANTSAnt, KI2CNum, AntsTelemetry, KANTSController, DeployStatus};

use std::io::Error;
use std::cell::RefCell;
use std::str;

#[derive(GraphQLEnum, Clone, Copy)]
pub enum AckCommand {
    None,
    Noop,
    ControlPower,
    ConfigureHardware,
    TestHardware,
    IssueRawCommand,
    Arm,
    Deploy,
}

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

#[derive(Default)]
pub struct TelemetryNominal(pub AntsTelemetry);

#[derive(Default)]
pub struct AntennaStats {
    pub act_count: u8,
    pub act_time: u16,
}

#[derive(Default)]
pub struct TelemetryDebug {
    pub ant1: AntennaStats,
    pub ant2: AntennaStats,
    pub ant3: AntennaStats,
    pub ant4: AntennaStats,
}

#[derive(GraphQLEnum)]
pub enum TestType {
    Integration,
    Hardware,
}
pub enum TestResults {
    Integration(IntegrationTestResults),
    Hardware(HardwareTestResults),
}

#[derive(GraphQLObject)]
pub struct IntegrationTestResults {
    pub success: bool,
    pub telemetry_nominal: TelemetryNominal,
    pub telemetry_debug: TelemetryDebug,
}

#[derive(GraphQLObject)]
pub struct HardwareTestResults {
    pub success: bool,
    pub data: String,
}

#[derive(GraphQLObject)]
pub struct RawCommandResponse {
    pub errors: String,
    pub success: bool,
    pub response: String,
}

pub struct Subsystem {
    ants: AntS,
    pub errors: RefCell<String>, //TODO: Consider making a vector of strings
}

// Iterate through a failure::Error and concatenate the error
// and all its causes into a single string
// TODO: Is there a good way to enforce delimiter formatting?
// (ie must be String, str, or char)
macro_rules! process_errors {
	($err:ident) => (process_errors!($err, '\n'));
	($err:ident, $delim:expr) => {{	
		{
			let mut results = String::new();
			let mut chain = $err.causes();
			
		    if let Some(err) = chain.next() {
		    	results.push_str(&format!("{}", err));
		
		        for err in chain {
		            results.push_str(&format!("{}{}", $delim, err));
		        }
		    }

		    results
		}
	}};
}

macro_rules! push_err {
	($master:expr, $err:expr) => {{
	        // TODO: Might change the master err string to a master err Vec<String>
	        // Might be easier to process/consume later
	        let mut err = $master.borrow_mut();
	        err.push_str($err);
		}}
}

// Execute a function and return a tuple containing:
//   a) A String with any errors which were encountered
//   b) A boolean to indicate whether the function ran successfully
// Optionally:
//   Add the error string to the master error string for later consumption
macro_rules! run {
	($func:expr) => {{
			let (errors, success, data) = match $func {
		        Ok(v) => (String::from(""), true, Some(v)),
		        Err(e) => (process_errors!(e, ", "), false, None),
		    };
			
			(errors, success, data)
		}};
	($func:expr, $master:expr) => {{
		{
			let (errors, success, data) = run!($func);
			
			// We want to know which function threw these particular errors, 
			// but we don't want to print the entire expression, so using split
			// to go from
			//     self.my.func(arg1, arg2)
			// to this
			//     func
			// TODO: This isn't perfect or particularly pretty. Is there a better way?
			let mut name = stringify!($func).split('(').next().unwrap();
			name = name.split(&[':','.'][..]).last().unwrap();
			push_err!($master, &format!("{}: {}", name, errors));
	        
			(errors, success, data)
		}
	}};
}

impl Subsystem {
    pub fn new() -> Subsystem {
        let subsystem = Subsystem {
            ants: AntS::new(KI2CNum::KI2C1, 0x31, 0x32, 4, 10).unwrap(),
            errors: RefCell::new("".to_owned()),
        };

        subsystem
        //TODO: error handling
    }

    pub fn get_arm_status(&self) -> Result<ArmStatus, Error> {
        let (_errors, _success, deploy) = run!(self.ants.get_deploy(), self.errors);
        let armed = deploy.unwrap_or_default().sys_armed;

        let status = match armed {
            true => ArmStatus::Armed,
            false => ArmStatus::Disarmed,
        };

        Ok(status)
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

    pub fn get_telemetry_nominal(&self) -> Result<TelemetryNominal, Error> {
        let (_errors, _success, telemetry) = run!(self.ants.get_system_telemetry(), self.errors);

        Ok(TelemetryNominal(telemetry.unwrap_or_default()))
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

    pub fn get_test_results(&self) -> Result<IntegrationTestResults, Error> {
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
                push_err!(self.errors, "controlPower: Invalid power state");

                Ok(ControlPowerResponse {
                    power: state,
                    errors: String::from("Invalid power state"),
                    success: false,
                })
            }

        }
    }

    pub fn arm(&self, state: ArmState) -> Result<ArmResponse, Error> {
        let (errors, success, _data) = match state {
            ArmState::Arm => run!(self.ants.arm(), self.errors),
            ArmState::Disarm => run!(self.ants.disarm(), self.errors),
        };

        Ok(ArmResponse { errors, success })
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
