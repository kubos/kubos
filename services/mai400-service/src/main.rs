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
#![recursion_limit="256"]

#[cfg(test)]
#[macro_use]
extern crate double;
//#[cfg(test)]
//#[macro_use]
//extern crate failure;
//#[cfg(not(test))]
extern crate failure;
extern crate i2c_linux;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate kubos_service;
//#[macro_use]
extern crate mai400_api;
#[cfg(test)]
#[macro_use]
extern crate serde_json;

mod model;
mod objects;
mod schema;
#[cfg(test)]
mod tests;

use i2c_linux::I2c;
use kubos_service::{Config, Service};
use model::{Subsystem, ReadData};
use schema::{MutationRoot, QueryRoot};
use std::sync::Arc;

fn i2c_cmds() {
    // Make sure the power line that goes to the MAI is turned on

    let mut i2c = I2c::from_path("/dev/i2c-1").unwrap();
    i2c.smbus_set_slave_address(0x1F, false).unwrap();

    i2c.i2c_write_block_data(0x03, &[0xF0]).unwrap();
    i2c.i2c_write_block_data(0x01, &[0x0E]).unwrap();
}

fn main() {

    i2c_cmds();

    Service::new(
        Config::new("mai400-service"),
        Subsystem::new("/dev/ttyS5".to_owned(), Arc::new(ReadData::new())),
        QueryRoot,
        MutationRoot,
    ).start();
}
