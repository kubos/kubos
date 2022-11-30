//
// Copyright (C) 2019 Kubos Corporation
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

//! Service for interacting with a [Clyde Space 3G EPS](https://www.aac-clyde.space/satellite-bits/eps)
//!
//! # Configuration
//!
//! The service can be configured in the `/etc/kubos-config.toml` with the following fields:
//!
//! ```toml
//! [clyde-3g-eps-service]
//! bus = "/dev/i2c-1"
//!
//! [clyde-3g-eps-service.addr]
//! ip = "127.0.0.1"
//! port = 8100
//! ```
//!
//! Where `bus` specifies the I2C bus the EPS is on, `ip` specifies the
//! service's IP address, and `port` specifies the port on which the service will be
//! listening for UDP packets.
//!
//! # Running the Service
//!
//! The service should be started automatically by its init script, but may also be started manually:
//!
//! ```bash
//! $ clyde-3g-eps-service
//! Listening on: 127.0.0.1:8100
//! ```
//!
//! If no config file is specified, then the service will look at `/etc/kubos-config.toml`.
//! An alternative config file may be specified on the command line at run time:
//!
//! ```bash
//! $ clyde-3g-eps-service -c config.toml
//! ```
//!
//! # Panics
//!
//! Attempts to grab `bus` from Configuration and will `panic!` if not found.
//!
//! # GraphQL Schema
//!
//! ## Queries
//!
//! ### ACK
//!
//! Fetch the last mutation which was executed by the service.
//! Returns a [subsystem::Mutations](models/subsystem/enum.Mutations.html) value.
//!
//! ```json
//! {
//!     ack: Mutation!
//! }
//! ```
//!
//! ### Errors
//!
//! Fetch all errors encountered by the service since the last time this field was queried
//!
//! ```json
//! {
//!     errors: [String]
//! }
//! ```
//!
//! ### Power
//!
//! Get the system power status.
//! Returns a [PowerState](models/enum.PowerState.html) value.
//!
//! ```json
//! {
//!        power {
//!         motherboard: PowerState,
//!          daughterboard: PowerState,
//!        }
//! }
//! ```
//!
//! ### Telemetry
//!
//! #### Version Info
//!
//! Fetch the version information for the EPS' motherboard and daughterboard (if present).
//!
//! ```json
//! {
//!     telemetry {
//!         version {
//!             motherboard {
//!                 revision: Int!,
//!                 firmwareNumber: Int!
//!             },
//!             daughterboard {
//!                 revision: Int,
//!                 firmwareNumber: Int
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! #### Reset Count
//!
//! Get the number of board resets, by category.
//! Note: If any value exceeds 255, it will automatically roll over to 0.
//!
//! ```json
//! {
//!     telemetry {
//!         reset {
//!             brownOut
//!                 motherboard: Int!
//!                 daughterboard: Int
//!             automaticSoftware
//!                 motherboard: Int!
//!                 daughterboard: Int
//!             manual
//!                 motherboard: Int!
//!                 daughterboard: Int
//!             watchdog
//!                 motherboard: Int!
//!                 daughterboard: Int
//!         }
//!     }
//! }
//! ```
//!
//! #### Motherboard Telemetry
//!
//! Fetch telemetry data for the motherboard. All returned values are automatically converted from their original raw data.
//! Refer to Table 11-7 of the EPS' User Manual for more information.
//!
//!  ```json
//! {
//!     telemetry {
//!         motherboard {
//!             VoltageFeedingBcr1: Float!
//!             CurrentBcr1Sa1a: Float!
//!             CurrentBcr1Sa1b: Float!
//!             ArrayTempSa1a: Float!
//!             ArrayTempSa1b: Float!
//!             SunDetectorSa1a: Float!
//!             SunDetectorSa1b: Float!
//!             VoltageFeedingBcr2: Float!
//!             CurrentBcr2Sa2a: Float!
//!             CurrentBcr2Sa2b: Float!
//!             ArrayTempSa2a: Float!
//!             ArrayTempSa2b: Float!
//!             SunDetectorSa2a: Float!
//!             SunDetectorSa2b: Float!
//!             VoltageFeedingBcr3: Float!
//!             CurrentBcr3Sa3a: Float!
//!             CurrentBcr3Sa3b: Float!
//!             ArrayTempSa3a: Float!
//!             ArrayTempSa3b: Float!
//!             SunDetectorSa3a: Float!
//!             SunDetectorSa3b: Float!
//!             BcrOutputCurrent: Float!
//!             BcrOutputVoltage: Float!
//!             CurrentDraw3V3: Float!
//!             CurrentDraw5V: Float!
//!             OutputCurrent12V: Float!
//!             OutputVoltage12V: Float!
//!             OutputCurrentBattery: Float!
//!             OutputVoltageBattery: Float!
//!             OutputCurrent5v: Float!
//!             OutputVoltage5v: Float!
//!             OutputCurrent33v: Float!
//!             OutputVoltage33v: Float!
//!             OutputVoltageSwitch1: Float!
//!             OutputCurrentSwitch1: Float!
//!             OutputVoltageSwitch2: Float!
//!             OutputCurrentSwitch2: Float!
//!             OutputVoltageSwitch3: Float!
//!             OutputCurrentSwitch3: Float!
//!             OutputVoltageSwitch4: Float!
//!             OutputCurrentSwitch4: Float!
//!             OutputVoltageSwitch5: Float!
//!             OutputCurrentSwitch5: Float!
//!             OutputVoltageSwitch6: Float!
//!             OutputCurrentSwitch6: Float!
//!             OutputVoltageSwitch7: Float!
//!             OutputCurrentSwitch7: Float!
//!             OutputVoltageSwitch8: Float!
//!             OutputCurrentSwitch8: Float!
//!             OutputVoltageSwitch9: Float!
//!             OutputCurrentSwitch9: Float!
//!             OutputVoltageSwitch10: Float!
//!             OutputCurrentSwitch10: Float!
//!             BoardTemperature: Float!
//!         }
//!     }
//! }
//! ```
//!
//! ### Daughterboard Telemetry
//!
//! Fetch telemetry data for the motherboard. All returned values are automatically converted from their original raw data.
//! Refer to Table 11-8 of the EPS's User Manual for more information.
//!
//! ```json
//! {
//!     telemetry {
//!         daughterboard {
//!             VoltageFeedingBcr4: Float
//!             CurrentBcr4Sa4a: Float
//!             CurrentBcr4Sa4b: Float
//!             ArrayTempSa4a: Float
//!             ArrayTempSa4b: Float
//!             SunDetectorSa4a: Float
//!             SunDetectorSa4b: Float
//!             VoltageFeedingBcr5: Float
//!             CurrentBcr5Sa5a: Float
//!             CurrentBcr5Sa5b: Float
//!             ArrayTempSa5a: Float
//!             ArrayTempSa5b: Float
//!             SunDetectorSa5a: Float
//!             SunDetectorSa5b: Float
//!             VoltageFeedingBcr6: Float
//!             CurrentBcr6Sa6a: Float
//!             CurrentBcr6Sa6b: Float
//!             ArrayTempSa6a: Float
//!             ArrayTempSa6b: Float
//!             SunDetectorSa6a: Float
//!             SunDetectorSa6b: Float
//!             VoltageFeedingBcr7: Float
//!             CurrentBcr7Sa7a: Float
//!             CurrentBcr7Sa7b: Float
//!             ArrayTempSa7a: Float
//!             ArrayTempSa7b: Float
//!             SunDetectorSa7a: Float
//!             SunDetectorSa7b: Float
//!             VoltageFeedingBcr8: Float
//!             CurrentBcr8Sa8a: Float
//!             CurrentBcr8Sa8b: Float
//!             ArrayTempSa8a: Float
//!             ArrayTempSa8b: Float
//!             SunDetectorSa8a: Float
//!             SunDetectorSa8b: Float
//!             VoltageFeedingBcr9: Float
//!             CurrentBcr9Sa9a: Float
//!             CurrentBcr9Sa9b: Float
//!             ArrayTempSa9a: Float
//!             ArrayTempSa9b: Float
//!             SunDetectorSa9a: Float
//!             SunDetectorSa9b: Float
//!             BoardTemperature: Float
//!         }
//!     }
//! }
//! ```
//!
//! #### Watchdog Period
//!
//! Fetch the current watchdog timeout period, in minutes
//!
//! ```json
//! {
//!     telemetry {
//!         watchdogPeriod: Int!
//!     }
//! }
//! ```
//!
//! #### Last EPS Error
//!
//! Fetch the last error which was encountered by the system while executing a command.
//! Returns a [last_error::Error](models/last_error/enum.Error.html) value
//!
//! ```json
//! {
//!     telemetry {
//!         lastEpsError {
//!             motherboard: Error!
//!             daughterboard: Error
//!         }
//!     }
//! }
//! ```
//!
//! #### Board Status
//!
//! Check the status of the motherboard and daughterboard.
//! Returns [status flags](../clyde_3g_eps_api/struct.StatusCode.html) indicating any recent errors.
//!
//! ```json
//! {
//!     telemetry {
//!         boardStatus {
//!             motherboard: Status!
//!             daugherboard: Status
//!         }
//!     }
//! }
//! ```
//!
//! ## Mutations
//!
//! ### No-Op
//!
//! Execute a trivial command against the system.
//!
//! ```json
//! mutation {
//!     noop {
//!         success: Boolean!
//!         errors: String!
//!     }
//! }
//! ```
//!
//! ### Manual Reset
//!
//! Manually reset the EPS.
//!
//! ```json
//! mutation {
//!     manualReset {
//!         success: Boolean!
//!         errors: String!
//!     }
//! }
//! ```
//!
//! ### Reset Watchdog
//!
//! Reset the communications watchdog timer.
//!
//! ```json
//! mutation {
//!     resetWatchdog {
//!         success: Boolean!
//!         errors: String!
//!     }
//! }
//! ```
//!
//! ### Set Watchdog Period
//!
//! Set the communications watchdog timeout period.
//!
//! - period: New timeout period, in minutes
//!
//! ```json
//! mutation {
//!     setWatchdogPeriod(period: Int!) {
//!         success: Boolean!
//!         errors: String!
//!     }
//! }
//! ```
//!
//! ### Issue Raw Command
//!
//! Pass a custom command through to the system
//!
//! - command: Decimal value of the command byte to send
//! - data: Decimal values of the command parameters to send. Should be `[0]` if there are no additional parameters required.
//!
//! ```json
//! mutation {
//!     issueRawCommand(command: Int!, data: [Int!]) {
//!         success: Boolean!
//!         errors: String!
//!     }
//! }
//! ```
//!
//! ### Test Hardware
//!
//! Perform a system test
//!
//! - test: Specific test to perform. Should be `HARDWARE`
//!
//! ```json
//!  mutation {
//!      testHardware(test: TestType) {
//!          success: Boolean!
//!          errors: String!
//!      }
//!  }
//! ```

#![deny(missing_docs)]
#![deny(warnings)]

#[macro_use]
extern crate juniper;
#[macro_use]
extern crate kubos_service;

pub mod models;
pub mod schema;
#[cfg(test)]
mod tests;

use crate::models::subsystem::Subsystem;
use crate::schema::mutation::Root as MutationRoot;
use crate::schema::query::Root as QueryRoot;
use kubos_service::{Config, Logger, Service};
use log::error;

fn main() {
    Logger::init("clyde-3g-eps-service").unwrap();

    let config = Config::new("clyde-3g-eps-service")
        .map_err(|err| {
            error!("Failed to load service config: {:?}", err);
            err
        })
        .unwrap();
    let bus = config
        .get("bus")
        .ok_or_else(|| {
            error!("Failed to load 'bus' config value");
            "Failed to load 'bus' config value"
        })
        .unwrap();
    let bus = bus
        .as_str()
        .ok_or_else(|| {
            error!("Failed to parse 'bus' config value");
            "Failed to parse 'bus' config value"
        })
        .unwrap();

    let subsystem: Box<Subsystem> = Box::new(
        Subsystem::from_path(bus)
            .map_err(|err| {
                error!("Failed to create subsystem: {:?}", err);
                err
            })
            .unwrap(),
    );

    Service::new(config, subsystem, QueryRoot, MutationRoot).start();
}
