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
//! in the Kubos Linux ecosystem.
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
//! hardware device they want to expose over the service interface (currently GraphQL/HTTP).
//!
//! ## Configuration
//!
//! Services which use this crate have the option of using a local configuration file
//! or falling back on default config values. The service will search for the configuration
//! file at this location `/home/system/etc/config.toml` unless otherwise specified with
//! the `-c` flag at run time.
//!
//! The service configuration file uses the Toml format and is expected to use the
//! following layout:
//!
//! ```toml,ignore
//! [service-name]
//! config-key = "value"
//! config-key2 = 123
//!
//! # This section and values are needed for all services
//! [service-name.addr]
//! ip = "127.0.0.1"
//! port = 8082
//! ```
//!
//! The `[service-name.addr]` section is required for all services and is used to set
//! the ip/port on which the service will listen for messages. Any service specific
//! configuration values can be specified directly under the `[service-name]` section.
//! Note - the `service-name` used in the sections must match the name used when creating
//! the `Config` instance inside your service.
//!
//! ### Examples
//!
//! # Creating and starting a simple service.
//!
//! ```rust,ignore
//! use kubos_service::{Config, Service};
//! use model::Subsystem;
//! use schema::{MutationRoot, QueryRoot};
//!
//! Service::new(
//!     Config::new("service-name").unwrap(),
//!     Subsystem::new(),
//!     QueryRoot,
//!     MutationRoot,
//! ).start();
//! ```
//!
//! # Using the service config info to configure the subsystem.
//!
//! ```rust,ignore
//! use kubos_service::{Config, Service};
//! use model::Subsystem;
//! use schema::{MutationRoot, QueryRoot};
//!
//! let config = Config::new("example-service").unwrap();
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
//!
//! ```bash
//! $ ./example-service
//! ```
//!
//! # Running a service with a custom config file.
//!
//! ```bash
//! $ ./example-service -c config.toml
//! ```

mod macros;
mod service;

pub use crate::service::{Context, Service};
pub use kubos_system::Config;
