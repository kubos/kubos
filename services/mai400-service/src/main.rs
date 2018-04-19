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
extern crate failure;
#[cfg(not(test))]
extern crate failure;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate kubos_service;
extern crate mai400_api;

mod model;
mod objects;
mod schema;

use kubos_service::{Config, Service};
use model::{Subsystem, ReadData};
use schema::{MutationRoot, QueryRoot};
use std::sync::Arc;

fn main() {
    Service::new(
        Config::new("mai400-service"),
        Subsystem::new("/dev/ttyS5".to_owned(), Arc::new(ReadData::new())),
        QueryRoot,
        MutationRoot,
    ).start();
}
