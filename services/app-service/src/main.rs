/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#![deny(warnings)]

extern crate getopts;
#[macro_use]
extern crate juniper;
extern crate kubos_app;
extern crate kubos_service;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
#[macro_use]
extern crate serde_json;
#[cfg(test)]
extern crate tempfile;
extern crate toml;
extern crate uuid;

mod registry;
mod schema;
#[cfg(test)]
mod tests;

use kubos_service::{Config, Service};
use registry::AppRegistry;

fn main() {
    let config = Config::new("app-service");
    let registry = {
        match config.get("registry-dir") {
            Some(dir) => AppRegistry::new_from_dir(dir.as_str().unwrap()),
            None => AppRegistry::new(),
        }
    };

    Service::new(config, registry, schema::QueryRoot, schema::MutationRoot).start();
}
