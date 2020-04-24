//
// Copyright (C) 2020 Kubos Corporation
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

// Contributed by Xueliang Bai <x.bai@sydney.edu.au> on behalf of the
// ARC Training Centre for CubeSats, UAVs & Their Applications (CUAVA) team (www.cuava.com.au)
// at the University of Sydney

//! Kubos Service for interacting with [GomSpace p31u EPS]
//!
//! # Configuration
//!
//! The service must be configured in `/home/system/etc/config.toml` with the following fields:
//!
//! - `[gomspace-p31u-service.addr]`
//!
//!     - `ip`   - Specifies the service's IP address
//!     - `port` - Specifies the port on which the service will be listening for UDP packets
//!
//! - `[gomspace-p31u-service]`
//!
//!     - `bus`  - Specifies the I2C bus the antenna system is connected to
//!     - `addr` - Specifies the I2C address of the antenna system's primary microcontroller
//!
//! Example:
//!     [gomspace-eps-service.addr]
//!     ip = "0.0.0.0"
//!     port = 8021
//!
//!     [gomspace-eps-service]
//!     bus = "/dev/i2c-0"
//!     i2c_addr = "0x08"
//!    
//! # Starting the Service
//!
//! The service should be started automatically by its init script, but may also be started manually
//!

#![deny(missing_docs)]
#![recursion_limit = "256"]
#![deny(warnings)]
#![allow(clippy::too_many_arguments)]

#[macro_use]
extern crate juniper;

use crate::model::Subsystem;
pub use crate::objects::*;
use crate::schema::{MutationRoot, QueryRoot};
use failure::format_err;
use gomspace_p31u_api::*;
use kubos_service::{Config, Service};
use log::error;
use syslog::Facility;

mod model;
mod objects;
mod schema;

fn main() -> EpsResult<()> {
    syslog::init(
        Facility::LOG_DAEMON,
        log::LevelFilter::Debug,
        Some("gomspace-eps-service"),
    )
    .unwrap();

    let config = Config::new("gomspace-eps-service")
        .map_err(|err| {
            error!("Failed to load service config: {:?}", err);
            err
        })
        .unwrap();

    let bus = config
        .get("bus")
        .ok_or_else(|| {
            error!("Failed to load 'bus' config value");
            format_err!("Failed to load 'bus' config value");
        })
        .unwrap();
    let bus = bus.as_str().unwrap();

    let addr = config
        .get("i2c_addr")
        .ok_or_else(|| {
            error!("Failed to load 'addr' config value");
            format_err!("Failed to load 'addr' config value");
        })
        .unwrap();
    let addr = addr.as_str().unwrap();

    let addr: u8 = if addr.starts_with("0x") {
        u8::from_str_radix(&addr[2..], 16).unwrap()
    } else {
        u8::from_str_radix(addr, 16).unwrap()
    };

    Service::new(config, Subsystem::new(bus, addr)?, QueryRoot, MutationRoot).start();

    Ok(())
}
