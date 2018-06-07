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

#![deny(missing_docs)]
#![deny(warnings)]

//! Kubos Service for interacting with a [NovAtel OEM6 High Precision GNSS Receiver](https://www.novatel.com/products/gnss-receivers/oem-receiver-boards/oem6-receivers/)
//!
//! # Configuration
//!
//! The service can be configured in the `/home/system/etc/config.toml` with the following fields:
//!
//! - `bus` - Specifies the UART bus the OEM6 is connected to
//! - `ip` - Specifies the service's IP address
//! - `port` - Specifies the port on which the service will be listening for UDP packets
//!
//! For example:
//!
//! ```toml
//! [novatel-oem6-service]
//! bus = "/dev/ttyS4"
//!
//! [novatel-oem6-service.addr]
//! ip = "127.0.0.1"
//! port = 8082
//! ```
//!
//! # Starting the Service
//!
//! The service should be started automatically by its init script, but may also be started manually:
//!
//! ```toml
//! $ novatel-oem6-service
//! Kubos OEM6 service started
//! Listening on: 10.63.1.20:8082
//! ```
//!
//! # Available Fields
//!
//! ```json
//! query {
//!     ack,
//!     config,
//!     errors,
//!     lockInfo {
//!         position,
//!         time {
//!             ms,
//!             week
//!         },
//!         velocity
//!     },
//!     lockStatus {
//!         positionStatus,
//!         positionType,
//!         time {
//!             ms,
//!             week
//!         },
//!         timeStatus,
//!         velocityStatus,
//!         velocityType
//!     },
//!     power{
//!         state,
//!         uptime
//!     },
//!     systemStatus {
//!         errors,
//!         status
//!     },
//!     telemetry{
//!         debug {
//!             components {
//!                 bootVersion,
//!                 compType,
//!                 compileDate,
//!                 compileTime,
//!                 hwVersion,
//!                 model,
//!                 serialNum,
//!                 swVersion,
//!             },
//!             numComponents
//!         },
//!         nominal{
//!             lockInfo {
//!                 position,
//!                 time {
//!                     ms,
//!                     week
//!                 },
//!                 velocity
//!             },
//!             lockStatus {
//!                 positionStatus,
//!                 positionType,
//!                 time {
//!                     ms,
//!                     week
//!                 },
//!                 timeStatus,
//!                 velocityStatus,
//!                 velocityType
//!             },
//!             systemStatus {
//!                 errors,
//!                 status
//!             }
//!         }
//!     },
//!     testResults {
//!         errors,
//!         success,
//!         telemetryDebug {
//!             components {
//!                 bootVersion,
//!                 compType,
//!                 compileDate,
//!                 compileTime,
//!                 hwVersion,
//!                 model,
//!                 serialNum,
//!                 swVersion,
//!             },
//!             numComponents
//!         },
//!         telemetryNominal{
//!             lockInfo {
//!                 position,
//!                 time {
//!                     ms,
//!                     week
//!                 },
//!                 velocity
//!             },
//!             lockStatus {
//!                 positionStatus,
//!                 positionType,
//!                 time {
//!                     ms,
//!                     week
//!                 },
//!                 timeStatus,
//!                 velocityStatus,
//!                 velocityType
//!             },
//!             systemStatus {
//!                 errors,
//!                 status
//!             }
//!         }
//!     }
//! }
//!
//! mutation {
//!     configureHardware(config: [{option: ConfigOption, hold: Boolean, interval: Float, offset: Float},...]) {
//!         config,
//!         errors,
//!         success
//!     },
//!     controlPower,
//!     errors,
//!     issueRawCommand(command: String){
//!         errors,
//!         success
//!     },
//!     noop {
//!         errors,
//!         success
//!     },
//!     testHardware(test: TestType) {
//!         ... on IntegrationTestResults {
//!             errors,
//!             success,
//!             telemetryDebug {
//!                 components {
//!                     bootVersion,
//!                     compType,
//!                     compileDate,
//!                     compileTime,
//!                     hwVersion,
//!                     model,
//!                     serialNum,
//!                     swVersion,
//!                 },
//!                 numComponents
//!             },
//!             telemetryNominal{
//!                 lockInfo {
//!                     position,
//!                     time {
//!                         ms,
//!                         week
//!                     },
//!                     velocity
//!                 },
//!                 lockStatus {
//!                     positionStatus,
//!                     positionType,
//!                     time {
//!                         ms,
//!                         week
//!                     },
//!                     timeStatus,
//!                     velocityStatus,
//!                     velocityType
//!                 },
//!                 systemStatus {
//!                     errors,
//!                     status
//!                 }
//!             }
//!         }
//!         ... on HardwareTestResults {
//!             data,
//!             errors,
//!             success
//!         }
//!     }
//! }
//! ```
//!

#![recursion_limit = "256"]

extern crate failure;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate kubos_service;
extern crate novatel_oem6_api;
#[cfg(test)]
#[macro_use]
extern crate serde_json;

mod model;
mod objects;
mod schema;
#[cfg(test)]
mod tests;

use kubos_service::{Config, Service};
use model::{LockData, Subsystem};
use novatel_oem6_api::OEMResult;
use schema::{MutationRoot, QueryRoot};
use std::sync::Arc;

fn main() -> OEMResult<()> {
    let config = Config::new("novatel-oem6-service");
    let bus = config
        .get("bus")
        .expect("No 'bus' value found in 'novatel-oem6-service' section of config");
    let bus = bus.as_str().unwrap();

    let subsystem = Subsystem::new(bus, Arc::new(LockData::new()))?;

    Service::new(config, subsystem, QueryRoot, MutationRoot).start();

    Ok(())
}
