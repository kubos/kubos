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
//! ```
//! [novatel-oem6-service]
//! ip = "127.0.0.1"
//! port = 8082
//! ```
//!
//! Where `ip` specifies the service's IP address, and `port` specifies the port which UDP requests should be sent to.
//!
//! # Starting the Service
//!
//! The service should be started automatically by its init script, but may also be started manually:
//!
//! ```
//! $ novatel-oem6-service
//! Kubos OEM6 service started
//! Listening on: 10.63.1.20:8082
//! ```
//!
//! # Available Fields
//!
//! ```json
//! TODO
//! ```
//!

#![recursion_limit = "256"]

extern crate failure;
#[macro_use]
extern crate juniper;
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
use model::Subsystem;
use novatel_oem6_api::OEMResult;
use schema::{MutationRoot, QueryRoot};

// TODO: CHANGE THE UART BUS TO UART4!!!

fn main() -> OEMResult<()> {
    Service::new(
        Config::new("novatel-oem6-service"),
        Subsystem::new("/dev/ttyS5")?,
        QueryRoot,
        MutationRoot,
    ).start();

    Ok(())
}
