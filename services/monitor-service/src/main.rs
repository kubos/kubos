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

//! Service for monitoring KubOS Linux processes, memory, and CPU usage
extern crate failure;
#[macro_use]
extern crate juniper;
extern crate kubos_service;
#[macro_use]
extern crate text_io;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

use kubos_service::{Config, Service};
use schema::{MutationRoot, QueryRoot};

mod meminfo;
mod objects;
mod process;
mod schema;

fn main() {
    let config = Config::new("monitor-service");

    Service::new(
        config,
        (),
        QueryRoot,
        MutationRoot,
    ).start();
}
