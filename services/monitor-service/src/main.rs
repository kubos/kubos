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

//! Service for monitoring KubOS Linux processes, memory, and CPU usage
//!
//! # GraphQL Schema
//!
//! ```graphql
//! schema {
//!     query: Query
//! }
//!
//! type Query {
//!     ping: String!
//!     memInfo: MemInfo!
//!     ps(pids: [Int!] = null): [ProcInfo!]!
//! }
//!
//! type MemInfo {
//!     total: Int
//!     free: Int
//!     available: Int
//!     lowFree: Int
//! }
//!
//! type ProcInfo {
//!     pid: Int!
//!     uid: Int
//!     gid: Int
//!     usr: String
//!     grp: String
//!     state: String
//!     ppid: Int
//!     mem: Int
//!     rss: Int
//!     threads: Int
//!     cmd: String
//! }
//! ```

#[macro_use]
extern crate juniper;

use crate::schema::{MutationRoot, QueryRoot};
use kubos_service::{Config, Logger, Service};
use log::error;

mod meminfo;
mod objects;
#[macro_use]
mod process;
mod schema;
mod userinfo;

fn main() {
    Logger::init("kubos-monitor-service").unwrap();

    let config = Config::new("monitor-service")
        .map_err(|err| {
            error!("Failed to load service config: {:?}", err);
            err
        })
        .unwrap();

    Service::new(config, (), QueryRoot, MutationRoot).start();
}
