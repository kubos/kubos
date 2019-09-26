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

//! Kubos Service for interacting with the ISIS-OBC Supervisor
//!
//! # Configuration
//!
//! The service must be configured in `/etc/kubos-config.toml` with the following fields:
//!
//! - `[iobc-supervisor-service.addr]`
//!
//!     - `ip` - Specifies the service's IP address
//!     - `port` - Specifies the port on which the service will be listening for UDP packets
//!
//! For example:
//!
//! ```toml
//! [iobc-supervisor-service.addr]
//! ip = "0.0.0.0"
//! port = 8006
//! ```
//!
//! # Starting the Service
//!
//! The service should be started automatically by its init script, but may also be started manually:
//!
//! ```shell
//! $ iobc-supervisor-service
//! Kubos antenna systems service started
//! Listening on: 0.0.0.0:8006
//! ```
//!
//! # Available Fields
//!
//! ```json
//! query {
//!     ping: "pong",
//! 	supervisor: {
//! 		version: {
//! 			dummy,
//! 			spiCommandStatus,
//! 			indexOfSubsystem,
//! 			majorVersion,
//! 			minorVersion,
//! 			patchVersion,
//! 			gitHeadVersion,
//! 			serialNumber,
//! 			compileInformation,
//! 			clockSpeed,
//! 			codeType,
//! 			crc
//! 		},
//! 		housekeeping: {
//! 			dummy,
//! 			spiCommandStatus,
//! 			enableStatus: {
//! 				powerObc,
//! 				powerRtc,
//! 				supervisorMode,
//! 				busyRtc,
//! 				powerOffRtc
//! 			},
//! 			supervisorUptime,
//! 			iobcUptime,
//! 			iobcResetCount,
//! 			adcData,
//! 			adcUpdateFlag,
//! 			crc8
//! 		}
//! 	}
//! }
//!
//! mutation {
//! 	reset,
//! 	emergencyReset,
//! 	powercycle
//! }
//! ```
//!

#[macro_use]
extern crate juniper;

mod model;
mod schema;

use crate::model::Supervisor;
use crate::schema::{MutationRoot, QueryRoot};
use kubos_service::{Config, Service};
use syslog::Facility;

fn main() {
    syslog::init(
        Facility::LOG_DAEMON,
        log::LevelFilter::Debug,
        Some("iobc-supervisor-service"),
    )
    .unwrap();

    Service::new(
        Config::new("iobc-supervisor-service")
            .map_err(|err| {
                log::error!("Failed to load service config: {:?}", err);
                err
            })
            .unwrap(),
        Supervisor::new(),
        QueryRoot,
        MutationRoot,
    )
    .start();
}
