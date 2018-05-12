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

#[macro_use]
extern crate diesel;

mod db;
mod models;
mod schema;

use schema::{MutationRoot, QueryRoot};
use db::Database;
use kubos_service::{Config, Service};

fn main() {
    let config = Config::new("telemetry-service");

    let db_path = config
        .get("database")
        .expect("No database path found in config file");
    let db_path = db_path.as_str().unwrap_or("");

    let db = Database::new(&db_path);
    db.setup();

    Service::new(config, db, QueryRoot, MutationRoot).start();
}
