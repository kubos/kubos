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
//! query routedTelemetry(timestampGe: Integer, timestampLe: Integer, subsystem: String, parameter: String, output: String!, compress: Boolean = true): String!
//!
//! mutation insert(timestamp: Integer, subsystem: String!, parameter: String!, value: String!):{ success: Boolean!, errors: String! }
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
//!
//! ## Repeat the previous query, but route the output to compressed file `/home/system/recent_telem.tar.gz`
//! ```graphql
//! {
//!   telemetry(limit: 10, timestampGe: 1008, output: "/home/system/recent_telem")
//! }
//! ```
//!
//! ## Repeat the previous query, but route the output to uncompressed file `/home/system/recent_telem`
//! ```graphql
//! {
//!   telemetry(limit: 10, timestampGe: 1008, output: "/home/system/recent_telem", compress: false)
//! }
//! ```
//!
//! # Example Mutations
//!
//! ## Insert a new entry, allowing the service to generate the timestamp
//! ```graphql
//! mutation {
//! 	insert(subsystem: "eps", parameter: "voltage", value: "4.0") {
//! 		success,
//! 		errors
//! 	}
//! }
//! ```
//!
//! ## Insert a new entry with a custom timestamp
//! ```graphql
//! mutation {
//! 	insert(timestamp: 533, subsystem: "eps", parameter: "voltage", value: "5.1") {
//! 		success,
//! 		errors
//! 	}
//! }
//!
//! ```
//!
//! ## Delete all entries from the EPS subsystem occuring before timestamp 1003
//! ```graphql
//! mutation {
//!     delete(subsystem: "eps", timestampLe: 1004) {
//!         success,
//!         errors,
//!         entriesDeleted
//!     }
//! }
//! ```

#[macro_use]
extern crate juniper;

mod schema;
mod udp;

use crate::schema::{MutationRoot, QueryRoot, Subsystem};
use kubos_service::{Config, Service};
use kubos_telemetry_db::Database;
use syslog::Facility;

fn main() {
    syslog::init(
        Facility::LOG_DAEMON,
        log::LevelFilter::Debug,
        Some("kubos-telemetry-service"),
    )
    .unwrap();

    let config = Config::new("telemetry-service");

    let db_path = config
        .get("database")
        .expect("No database path found in config file");
    let db_path = db_path.as_str().unwrap_or("");

    let db = Database::new(&db_path);
    db.setup();

    let direct_udp = config.get("direct_port").map(|port| {
        let host = config.hosturl();
        let mut host_parts = host.split(':').map(|val| val.to_owned());
        let host_ip = host_parts.next().unwrap();

        format!("{}:{}", host_ip, port)
    });

    Service::new(
        config,
        Subsystem::new(db, direct_udp),
        QueryRoot,
        MutationRoot,
    )
    .start();
}
