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

//! Kubos Service for interacting with the telemetry database.
//!
//! # Configuration
//!
//! The service can be configured in the `/home/system/etc/config.toml` with the following fields:
//!
//! ```
//! [telemetry-service]
//! database = "/var/lib/telemetry.db"
//!
//! [telemetry-service.addr]
//! ip = "127.0.0.1"
//! port = 8089
//! ```
//!
//! Where `database` specifies the path to the telemetry database file, `ip` specifies the
//! service's IP address, and `port` specifies the port on which the service will be
//! listening for UDP packets.
//!
//! # Starting the Service
//!
//! The service should be started automatically by its init script, but may also be started manually:
//!
//! ```
//! $ telemetry-service
//! Listening on: 127.0.0.1:8089
//! ```
//!
//! # Panics
//!
//! Attempts to grab database path from Configuration and will `panic!` if not found.
//! Attempts to connect to database at provided path and will `panic!` if connection fails.
//! Attempts to create telemetry table and will `panic!` if table creation fails.
//!
//! # GraphQL Schema
//!
//! ```graphql
//! type Entry {
//!   timestamp: Integer!
//!   subsystem: String!
//!   parameter: String!
//!   value: Float!
//! }
//!
//! query telemetry(timestampGe: Integer, timestampLe: Integer, subsystem: String, parameter: String): Entry
//! ```
//!
//! # Example Queries
//!
//! ## Select all attributes of all telemetry entries
//! ```graphql
//! {
//!   telemetry {
//!     timestamp,
//!     subsystem,
//!     parameter,
//!     value
//!   }
//! }
//! ```
//!
//! ## Select all attributes of all telemetry entries for the eps subsystem
//! ```graphql
//! {
//!   telemetry(subsystem: "eps") {
//!     timestamp,
//!     subsystem,
//!     parameter,
//!     value
//!   }
//! }
//! ```
//!
//! ## Select all attributes of all telemetry entries for the voltage parametereter of the eps subsystem
//! ```graphql
//! {
//!   telemetry(subsystem: "eps", parameter: "voltage") {
//!     timestamp,
//!     subsystem,
//!     parameter,
//!     value
//!   }
//! }
//! ```
//!
//! ## Select all attributes of all telemetry entries occurring between the timestamps 100 and 200
//! ```graphql
//! {
//!   telemetry(timestampGe: 101, timestampLe: 199) {
//!     timestamp,
//!     subsystem,
//!     parameter,
//!     value
//!   }
//! }
//! ```
//!
//! ## Select all attributes of all telemetry entries occurring at the timestamp 101
//! ```graphql
//! {
//!   telemetry(timestampGe: 101, timestampLe: 101) {
//!     timestamp,
//!     subsystem,
//!     parameter,
//!     value
//!   }
//! }
//! ```
//!
//! ## Select ten entries occurring on or after the timestamp 1008
//! ```graphql
//! {
//!   telemetry(limit: 10, timestampGe: 1008) {
//!     timestamp,
//!     subsystem,
//!     parameter,
//!     value
//!   }
//! }
//! ```
extern crate diesel;
#[macro_use]
extern crate juniper;
extern crate kubos_service;
extern crate kubos_telemetry_db;

mod schema;

use kubos_service::{Config, Service};
use kubos_telemetry_db::Database;
use schema::{MutationRoot, QueryRoot};

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
