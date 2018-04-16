//
// Copyright (C) 2017 Kubos Corporation
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

//! A collection of structures and functions used to create hardware services
//! in the kubos linux ecosystem.
//!
//! # Use
//!
//! The basic use of the kubos_service crate is through the Service structure.
//! This structure provides an interface for creating a new service instance,
//! configuring it with a hardware subsystem and Juniper Query/Mutation objects.
//! It also provides a starting entry point and basic configuration file parsing.
//!
//! ## In Services
//!
//! Services should only link to the `kubos_service` crate if they have a
//! hardware device they want to expose over the service interface (currently GraphQL/UDP).
//!
//! ### Examples
//!
//! # Creating and starting a simple service.
//! ```rust,ignore
//! use kubos_service::{Config, Service};
//! use model::Subsystem;
//! use schema::{MutationRoot, QueryRoot};
//!
//! Service::new(
//!     Config::new("service-name"),
//!     Subsystem::new(),
//!     QueryRoot,
//!     MutationRoot,
//! ).start();
//! ```
//!
//! # Using the service config info to configure the subsystem.
//! ```rust,ignore
//! use kubos_service::{Config, Service};
//! use model::Subsystem;
//! use schema::{MutationRoot, QueryRoot};
//!
//! let config = Config::new("example-service");
//! let subsystem = Subsystem { bus = config["bus"] ) };
//! Service::new(
//!     config,
//!     subsystem,
//!     query,
//!     mutation
//! ).start();
//! ```
//!
//! # Running a service with the default config file (`/home/system/etc/config.toml`).
//! ```bash
//! $ ./example-service
//! ```
//!
//! # Running a service with a custom config file.
//! ```bash
//! $ ./example-service -c config.toml
//! ```

extern crate getopts;
extern crate juniper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate toml;

mod config;
mod service;

pub use config::Config;
pub use service::{Context, Service};
