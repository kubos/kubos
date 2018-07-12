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

//! Kubos Service for interacting with [ISIS Antenna Systems](https://www.isispace.nl/product-category/products/antenna-systems/)
//!
//! Configuration is done via a configuration file. This should be specified as a command line argument...
//!
//! # Examples
//!
//! TODO: Example calling the process
//!
//! # Available Fields
//!
//! ```json
//! query {
//!     ack
//!     armStatus
//!     config
//!     deploymentStatus {
//!         status,
//!         sysBurnActive,
//!         sysIgnoreDeploy,
//!         sysArmed,
//!         ant1NotDeployed,
//!         ant1StoppedTime,
//!         ant1Active,
//!         ant2NotDeployed,
//!         ant2StoppedTime,
//!         ant2Active,
//!         ant3NotDeployed,
//!         ant3StoppedTime,
//!         ant3Active,
//!         ant4NotDeployed,
//!         ant4StoppedTime,
//!         ant4Active
//!     }
//!     power {
//!         state,
//!         uptime
//!     }
//!     nominal: telemetry(telem: NOMINAL) {
//!         ... on TelemetryNominal {
//!             rawTemp,
//!             uptime,
//!             sysBurnActive,
//!             sysIgnoreDeploy,
//!             sysArmed,
//!             ant1NotDeployed,
//!             ant1StoppedTime,
//!             ant1Active,
//!             ant2NotDeployed,
//!             ant2StoppedTime,
//!             ant2Active,
//!             ant3NotDeployed,
//!             ant3StoppedTime,
//!             ant3Active,
//!             ant4NotDeployed,
//!             ant4StoppedTime,
//!             ant4Active
//!     }}
//!     debug: telemetry(telem: DEBUG) {
//!         ... on TelemetryDebug {
//!             ant1ActivationCount,
//!             ant1ActivationTime,
//!             ant2ActivationCount,
//!             ant2ActivationTime,
//!             ant3ActivationCount,
//!             ant3ActivationTime,
//!             ant4ActivationCount,
//!             ant4ActivationTime,
//!         }
//!     }
//!     testResults{
//!         success,
//!         telemetryNominal{...},
//!         telemetryDebug{...}
//!     }
//!     errors
//! }
//!
//! mutation {
//!     arm(state: ArmState) {
//!         errors,
//!         success
//!     }
//!     configureHardware(config: ConfigureController) {
//!         errors,
//!         success,
//!         config
//!     }
//!     controlPower(state: PowerState) {
//!         errors,
//!         success,
//!         power
//!     }
//!     deploy(ant: DeployType, force: bool, time: i32) {
//!         errors,
//!         success
//!     }
//!     issueRawCommand(command: String, rx_len: i32) {
//!         errors,
//!         success,
//!         response
//!     }
//!     noop {
//!         errors,
//!         success
//!     }
//!     integration: testHardware(test: INTEGRATION) {
//!         ... on IntegrationTestRsults {
//!             errors,
//!             success,
//!             telemetryNominal{...},
//!             telemetryDebug{...}
//!         }
//!     }
//!     hardware: testHardware(test: HARDWARE) {
//!         ... on HardwareTestResults {
//!             errors,
//!             success,
//!             data
//!         }
//!     }
//! }
//! ```
//!

#![deny(missing_docs)]
#![recursion_limit = "256"]

#[cfg(test)]
#[macro_use]
extern crate double;
extern crate failure;
extern crate isis_ants_api;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate kubos_service;
#[cfg(test)]
#[macro_use]
extern crate serde_json;

use isis_ants_api::AntSResult;
use kubos_service::{Config, Service};
use model::Subsystem;
use schema::{MutationRoot, QueryRoot};

mod model;
mod objects;
mod schema;
#[cfg(test)]
mod tests;

/*
fn main() {
    let default = json!({
                    "isis-ants-service": {
                        "addr": "0.0.0.0",
                        "port": 8080,
                        "bus": "KI2C1",
                        "primary": 0x31,
                        "secondary": 0x32,
                        "antennas": 4,
                        "wd_timeout": 10
                    }
                });

    let config = Config {
        //TODO: bus
        bus: KI2CNum::KI2C1,
        primary: master_config["isis-ants-service"]["primary"]
            .as_u64()
            .unwrap_or(0x31) as u8,
        secondary: master_config["isis-ants-service"]["secondary"]
            .as_u64()
            .unwrap_or(0x32) as u8,
        antennas: master_config["isis-ants-service"]["antennas"]
            .as_u64()
            .unwrap_or(4) as u8,
        wd_timeout: master_config["isis-ants-service"]["wd_timeout"]
            .as_u64()
            .unwrap_or(10) as u32,
    };
}
*/

fn main() -> AntSResult<()> {
    let config = Config::new("isis-ants-service");

    let bus = config
        .get("bus")
        .expect("No 'bus' value found in 'isis-ants-service' section of config");
    let bus = bus.as_str().unwrap();

    let primary = config
        .get("primary")
        .expect("No 'primary' value found in 'isis-ants-service' section of config");
    let primary = primary.as_str().unwrap();
    let primary: u8 = match primary.starts_with("0x") {
        true => u8::from_str_radix(&primary[2..], 16).unwrap(),
        false => u8::from_str_radix(primary, 16).unwrap(),
    };

    let secondary = config
        .get("secondary")
        .expect("No 'secondary' value found in 'isis-ants-service' section of config");
    let secondary = secondary.as_str().unwrap();
    let secondary: u8 = match secondary.starts_with("0x") {
        true => u8::from_str_radix(&secondary[2..], 16).unwrap(),
        false => u8::from_str_radix(secondary, 16).unwrap(),
    };

    let antennas = config
        .get("antennas")
        .expect("No 'antennas' value found in 'isis-ants-service' section of config");
    let antennas = antennas.as_integer().unwrap() as u8;

    let wd_timeout = config
        .get("wd_timeout")
        .expect("No 'wd_timeout' value found in 'isis-ants-service' section of config");
    let wd_timeout = wd_timeout.as_integer().unwrap() as u32;

    Service::new(
        config,
        Subsystem::new(bus, primary, secondary, antennas, wd_timeout)?,
        QueryRoot,
        MutationRoot,
    ).start();

    Ok(())
}
