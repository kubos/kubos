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

//! Service for interacting with a ClydeSpace 3G EPS
//!
//! # Configuration
//!
//! The service can be configured in the `/home/system/etc/config.toml` with the following fields:
//!
//! ```toml
//! [clyde-3g-eps-service]
//! bus = "i2c-1"
//!
//! [clyde-3g-eps-service.addr]
//! ip = "127.0.0.1"
//! port = 8089
//! ```
//!
//! Where `bus` specifies the I2c bus the eps is on, `ip` specifies the
//! service's IP address, and `port` specifies the port on which the service will be
//! listening for UDP packets.
//!
//! # Running the Service
//!
//! The service should be started automatically by its init script, but may also be started manually:
//!
//! ```bash
//! $ clyde-3g-eps-service
//! Listening on: 127.0.0.1:8089
//! ```
//!
//! If no config file is specified then the service will look at `/home/system/etc/config.toml`.
//! An alternative config file may be specified on the command line at run time:
//!
//! ```bash
//! $ clyde-3g-eps-service -c config.toml
//! ```
//!
//! # Panics
//!
//! Attempts to grab bus from Configuration and will `panic!` if not found.
//!
//! # GraphQL Schema
//!
//! ## Queries
//!
//! `type Query { ping: String }`
//!
//! `type Query { version: `[Data](models/version/struct.Data.html)` }` - Calls [Eps::get_version_info](../clyde_3g_eps_api/eps/struct.Eps.html#method.get_version_info)
//!
//! `type Query { resetTelemetry(telem_type: `[Type](models/reset_telemetry/enum.Type.html)`): `[Data](models/reset_telemetry/struct.Data.html)` }` - Calls [Eps::get_reset_telemetry](../clyde_3g_eps_api/eps/struct.Eps.html#method.get_reset_telemetry)
//!
//! `type Query { motherboardTelemetry(telem_type: `[Type](models/motherboard_telemetry/enum.Type.html)`): i32 }` - Calls [Eps::get_motherboard_telemetry](../clyde_3g_eps_api/eps/struct.Eps.html#method.get_motherboard_telemetry)
//!
//! `type Query { daughterboardTelemetry(telem_type: `[Type](models/daughterboard_telemetry/enum.Type.html)`): i32 }` - Calls [Eps::get_daughterboard_telemetry](../clyde_3g_eps_api/eps/struct.Eps.html#method.get_daughterboard_telemetry)
//!
//! `type Query { watchdogPeriod(): i32 }` - Calls [Eps::get_comms_watchdog_period](../clyde_3g_eps_api/eps/struct.Eps.html#method.get_comms_watchdog_period)
//!
//! ## Mutations
//!
//! `type Mutation { `manualReset` }` - Calls [Eps::manual_reset](../clyde_3g_eps_api/eps/struct.Eps.html#method.manual_reset)
//!
//! `type Mutation { resetWatchdog }` - Calls [Eps::reset_comms_watchdog](../clyde_3g_eps_api/eps/struct.Eps.html#method.reset_comms_watchdog)
//!
//! `type Mutation { setWatchdogPeriod(period: i32) }` - Calls [Eps::set_comms_watchdog_period](../clyde_3g_eps_api/eps/struct.Eps.html#method.set_comms_watchdog_period)

//#![deny(missing_docs)]
// #![deny(warnings)]

extern crate clyde_3g_eps_api;
extern crate eps_api;
extern crate failure;
extern crate i2c_hal;
#[macro_use]
extern crate juniper;
extern crate kubos_service;

pub mod models;
pub mod schema;

use kubos_service::{Config, Service};
use models::subsystem::Subsystem;
use schema::mutation::Root as MutationRoot;
use schema::query::Root as QueryRoot;

fn main() {
    let config = Config::new("clyde-3g-eps-service");
    let bus = config
        .get("bus")
        .expect("No eps device path found in config");
    let bus = bus.as_str().unwrap_or("");

    let subsystem = Subsystem::new(bus).unwrap();

    Service::new(config, subsystem, QueryRoot, MutationRoot).start();
}
