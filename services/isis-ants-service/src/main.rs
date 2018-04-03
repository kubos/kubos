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

#![warn(missing_docs)]

#[cfg(test)]
#[macro_use]
extern crate failure;
#[cfg(not(test))]
extern crate failure;

extern crate iron;
extern crate isis_ants_api;
#[macro_use]
extern crate juniper;
extern crate juniper_iron;
extern crate logger;
extern crate mount;
#[macro_use]
extern crate serde_json;

use iron::prelude::*;
use isis_ants_api::KI2CNum;
use juniper_iron::{GraphQLHandler, GraphiQLHandler};
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod macros;
mod model;
mod objects;
mod schema;

// Create a connection to the underlying AntS device with each GraphQL request
// and use it as the endpoint for queries and mutations
fn context_factory(_: &mut Request) -> schema::Context {
    //TODO: Figure out how to actually pass through the configuration values when this changes to a pure UDP implementation
    schema::Context { subsystem: model::Subsystem::new(KI2CNum::KI2C1, 0x31, 0x32, 4, 10) }
}

#[allow(dead_code)]
struct Config {
    primary: u8,
    secondary: u8,
    antennas: u8,
    wd_timeout: u32,
}

fn main() {

    let default = json!({
                    "isis-ants-service": {
                        "addr": "0.0.0.0",
                        "port": 8080,
                        "primary": 0x31,
                        "secondary": 0x32,
                        "antennas": 4,
                        "wd_timeout": 10
                    }
                });

    let mut raw = String::new();
    let filename = env::args().nth(1).unwrap_or("".to_owned());

    let master_config: Value =
        match File::open(filename)
            .map(|mut f| f.read_to_string(&mut raw))
            .and_then(|_x| serde_json::from_str(&raw).map_err(|err| err.into())) {
            Ok(v) => v,
            _ => {
                println!("Failed to get configuration. Using default values");
                default
            }
        };

    let host = master_config["isis-ants-service"]["addr"].to_string();
    let port = master_config["isis-ants-service"]["port"].to_string();

    let addr = format!("{}:{}", host.trim_matches('"'), port.trim_matches('"'));

    #[allow(unused_variables)]
    let config = Config {
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

    let graphql_endpoint =
        GraphQLHandler::new(context_factory, schema::QueryRoot, schema::MutationRoot);

    let graphiql_endpoint = GraphiQLHandler::new("/");

    let mut mount = mount::Mount::new();
    mount.mount("/", graphql_endpoint);
    mount.mount("/graphiql", graphiql_endpoint);

    let (logger_before, logger_after) = logger::Logger::new(None);

    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let host = env::var("LISTEN").unwrap_or(addr.to_owned());
    println!("GraphQL server started on {}", host);
    Iron::new(chain).http(host.as_str()).unwrap();
}
