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
//! # Queries
//!
//! ## Ping
//!
//! Test query to verify service is running without attempting
//! to communicate with the underlying subsystem
//!
//! ```json
//! {
//!     ping: "pong"
//! }
//! ```
//!
//! ## ACK
//!
//! Get the last run mutation
//!
//! ```json
//! {
//!     ack: AckCommand
//! }
//! ```
//!
//! ## Errors
//!
//! Get all errors encountered since the last time this field was queried
//!
//! ```json
//! {
//!     errors: [String]
//! }
//! ```
//!
//! ## Power Status
//!
//! Get the current power state of the system
//!
//! Note: `uptime` is included as an available field in order to conform to
//!       the Kubos Service Outline, but cannot be implemented for this device,
//!       so the value will be 1 if the device is on and 0 if the device is off
//!
//! ```json
//! {
//!     power {
//!         state: PowerState,
//!         uptime: Int
//!     }
//! }
//! ```
//!
//! ## Configuration
//!
//! Get the current configuration of the system
//!
//! Stretch goal: implement the LOGLIST command
//!
//! ```json
//! {
//!     config: "Not Implemented"
//! }
//! ```
//!
//! ## Test Results
//!
//! Get the test results of the last run test
//!
//! ```json
//! {
//!     testResults{
//!         success,
//!         telemetryNominal{...},
//!         telemetryDebug{...}
//!     }
//! }
//! ```
//!
//! ## System Status
//!
//! Get the current system status and errors
//!
//! ```json
//! {
//!     systemStatus {
//!        errors: Vec<String>,
//!        status: Vec<String>
//!     }
//! }
//! ```
//!
//! ## Lock Status
//!
//! Get current status of position information gathering
//!
//! ```json
//! {
//!     lockStatus {
//!         positionStatus: SolutionStatus,
//!           positionType: PosVelType,
//!           time {
//!             ms: Int,
//!               week: Int
//!           },
//!         timeStatus: RefTimeStatus,
//!         velocityStatus: SolutionStatus,
//!           velocityType: PosVelType
//!     }
//! }
//! ```
//!
//! ## Lock Information
//!
//! Get the last known good position information
//!
//! ```json
//! {
//!     lockInfo {
//!        position: Vec<Float>,
//!        time {
//!            ms: Int,
//!            week: Int
//!        },
//!        velocity: Vec<Float>
//!     }
//! }
//! ```
//!
//! ## Telemetry
//!
//! Get current telemetry information for the system
//!
//! ```json
//! {
//!     telemetry{
//!         debug {
//!             components: [{
//!                 bootVersion: String,
//!                 compType: Int,
//!                 compileDate: String,
//!                 compileTime: String,
//!                 hwVersion: String,
//!                 model: String,
//!                 serialNum: String,
//!                 swVersion: String,
//!             }],
//!             numComponents: Int
//!         },
//!         nominal{
//!             lockInfo {...},
//!             lockStatus {...},
//!             systemStatus: Vec<String>
//!         }
//!     }
//! }
//! ```
//!
//! # Mutations
//!
//! ## Errors
//!
//! Get all errors encountered while processing this GraphQL request
//!
//! Note: This will only return errors thrown by fields which have
//! already been processed, so it is recommended that this field be specified last.
//!
//! ```json
//! mutation {
//!     errors: [String]
//! }
//! ```
//!
//! ## No-Op
//!
//! Execute a trivial command against the system
//!
//! ```json
//! mutation {
//!     noop {
//!         errors: String,
//!         success: Boolean
//!    }
//! }
//! ```
//!
//! ## Set Power State
//!
//! Control the power state of the system
//!
//! Note: Power control of the GPS device will be done by the GPSRM service
//!
//! ```json
//! mutation {
//!     controlPower: "Not Implemented"
//! }
//! ```
//!
//! ## Configuration
//!
//! Configure the system
//!
//! - config: Vector of configuration requests (ConfigStruct)
//!   - option: Configuration operation which should be performed
//!   - hold: For `LOG_*` requests, specifies whether this request should be excluded
//!           from removal by future 'UNLOG_ALL' requests.
//!           For `UNLOG_ALL` requests, specifies whether the 'hold' value in previous
//!           `LOG_*` requests should be ignored.
//!   - interval: Interval at which log messages should be generated.
//!               Note: Only applies to `LOG_POSITION_DATA` requests. Ignored otherwise
//!   - offset: Offset of interval at which log messages should be generated.
//!             Note: Only applies to `LOG_POSITION_DATA` requests. Ignored otherwise
//!
//! ```json
//! mutation {
//!     configureHardware(config: [{option: ConfigOption, hold: Boolean, interval: Float, offset: Float},...]) {
//!         config: String
//!         errors: String,
//!         success: Boolean,
//!     }
//! }
//! ```
//!
//! ## System Self-Test
//!
//! Run a system self-test
//!
//! - test: Type of self-test to perform
//!
//! ```json
//! mutation {
//!     testHardware(test: TestType) {
//!         ... on IntegrationTestResults {
//!             errors: String,
//!             success: Boolean,
//!             telemetryNominal{...},
//!             telemetryDebug{...}
//!         }
//!         ... on HardwareTestResults {
//!             errors: "Not Implemented",
//!             success: true,
//!             data: Empty
//!         }
//!    }
//! }
//! ```
//!
//! ## Passthrough
//!
//! Pass a custom command through to the system
//!
//! - command: String containing the hex values to be sent (ex. "C3")
//!          It will be converted to a byte array before transfer.
//!
//! ```json
//! mutation {
//!     issueRawCommand(command: String) {
//!         errors: String,
//!         success: Boolean,
//!         response: String
//!     }
//! }
//! ```
//!

#![recursion_limit = "256"]

#[macro_use]
extern crate juniper;

mod model;
mod objects;
mod schema;
#[cfg(test)]
mod tests;

use kubos_service::{Config, Service};
use crate::model::{LockData, Subsystem};
use novatel_oem6_api::OEMResult;
pub use crate::objects::*;
use crate::schema::{MutationRoot, QueryRoot};
use std::sync::Arc;
use syslog::Facility;

fn main() -> OEMResult<()> {
    syslog::init(
        Facility::LOG_DAEMON,
        log::LevelFilter::Debug,
        Some("novatel-oem6-service"),
    )
    .unwrap();

    let config = Config::new("novatel-oem6-service");
    let bus = config
        .get("bus")
        .expect("No 'bus' value found in 'novatel-oem6-service' section of config");
    let bus = bus.as_str().unwrap();

    let subsystem = Subsystem::new(bus, Arc::new(LockData::new()))?;

    Service::new(config, subsystem, QueryRoot, MutationRoot).start();

    Ok(())
}
