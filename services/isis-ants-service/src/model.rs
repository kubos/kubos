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
#![allow(dead_code)]

use isis_ants_api::{AntS, KANTSAnt, KI2CNum, AntsTelemetry};

use std::io::Error;

#[derive(GraphQLEnum)]
pub enum ArmStatus {
    Armed,
    Disarmed,
}

#[derive(GraphQLEnum, Clone)]
pub enum PowerState {
    On,
    Off,
    Reset,
}

/// Model for power queries
pub struct GetPowerResponse {
    pub state: PowerState,
    pub uptime: u32,
}

/// Model for power mutations
#[derive(GraphQLObject)]
pub struct ControlPowerResponse {
    pub errors: String,
    pub success: bool,
    pub power: PowerState,
}

#[derive(GraphQLEnum)]
pub enum Telemetry {
    Nominal,
    Debug,
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

    pub fn control_power(&self, state: PowerState) -> Result<ControlPowerResponse, Error> {
        // Send command to device here
        match state {
            PowerState::Reset => {
                println!("Resetting antenna system");
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
}
