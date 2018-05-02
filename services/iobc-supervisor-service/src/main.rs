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

#[macro_use]
extern crate juniper;
extern crate kubos_service;

mod model;
mod schema;

use kubos_service::{Config, Service};
use model::Supervisor;
use schema::{MutationRoot, QueryRoot};

fn main() {
    Service::new(
        Config::new("iobc-supervisor-service"),
        Supervisor::new(),
        QueryRoot,
        MutationRoot,
    ).start();
}
