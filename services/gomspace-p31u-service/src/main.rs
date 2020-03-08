//
// Copyright (C) 2017 Kubos Corporation
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

//! Kubos Service for interacting with [ISIS Antenna Systems](https://www.isispace.nl/product-category/products/antenna-systems/)
//!
//! # Configuration
//!
//! The service must be configured in `/home/system/etc/config.toml` with the following fields:
//!
//! - `[isis-ants-service.addr]`
//!
//!     - `ip` - Specifies the service's IP address
//!     - `port` - Specifies the port on which the service will be listening for UDP packets
//!
//! - `[isis-ants-service]`
//!
//!     - `bus` - Specifies the I2C bus the antenna system is connected to
//! 	- `addr` - Specifies the I2C address of the antenna system's primary microcontroller
//! 	- `wd_timeout` - Specifies the interval at which the AntS watchdog should be automatically kicked. To disable automatic kicking, this value should be `0`.
//! Example: 
//!     [gomspace-eps-service.addr]
//!     ip = "0.0.0.0"
//!     port = 8021

//!     [gomspace-eps-service]
//!     bus = "/dev/i2c-0"
//!     addr = "0x08"
//!     wd_timeout = 10
//!
//!  # Starting the Service
//!
//! The service should be started automatically by its init script, but may also be started manually:
//!

#![deny(missing_docs)]
#![recursion_limit = "256"]
#![deny(warnings)]

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
#[cfg(test)]
mod tests;

fn main()-> EpsResult<()> {

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
            println!("Failed to load bus");
        })
        .unwrap();
    let bus = bus.as_str().unwrap();

    let addr = config
        .get("i2c_addr")
        .ok_or_else(|| {
            error!("Failed to load 'addr' config value");
            format_err!("Failed to load 'addr' config value");
            println!("Failed to load addr");
        })
        .unwrap();
    let addr = addr.as_str().unwrap();

    let addr: u8 = if addr.starts_with("0x") {
        u8::from_str_radix(&addr[2..], 16).unwrap()
    } else {
        u8::from_str_radix(addr, 16).unwrap()
    };
    
    Service::new(
        config,
        Subsystem::new(bus,addr)?,
        QueryRoot,
        MutationRoot,
    )
    .start();

    Ok(())
}


