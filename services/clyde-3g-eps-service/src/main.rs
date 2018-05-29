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

// #![deny(missing_docs)]
// #![deny(warnings)]

extern crate clyde_3g_eps_api;
extern crate eps_api;
extern crate failure;
extern crate i2c_hal;
#[macro_use]
extern crate juniper;
extern crate kubos_service;

mod models;
mod schema;

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
