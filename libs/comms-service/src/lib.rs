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
// Contributed by: William Greer (wgreer184@gmail.com) and Sam Justice (sam.justice1@gmail.com)
//

// #![deny(missing_docs)]
#![deny(warnings)]

//!
//! This library allows users to define and start communication services within their hardware services.
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use comms_service::*;
//! use std::net::{Ipv4Addr, UdpSocket};
//! use std::sync::{Arc, Mutex};
//!
//! // Example setup.
//! fn read(socket: &Arc<UdpSocket>) -> CommsResult<Vec<u8>> { Ok(vec![]) }
//! fn write(socket: &Arc<UdpSocket>, data: &[u8]) -> CommsResult<()> { Ok(()) }
//!
//! # fn func() -> CommsResult<()> {
//! // Defining connections.
//! let read_conn = Arc::new(UdpSocket::bind(("192.168.8.1", 13000)).unwrap());
//! let write_conn = Arc::new(UdpSocket::bind(("192.168.8.1", 13001)).unwrap());
//!
//! // Fetching communications settings from the common config.toml file.
//! let service_config = kubos_system::Config::new("service-name");
//! let comms_config = CommsConfig::new(service_config)?;
//!
//! // Putting everything into the control block.
//! let controls = CommsControlBlock::new(
//!     Some(Arc::new(read)),
//!     vec![Arc::new(write)],
//!     read_conn,
//!     write_conn,
//!     comms_config
//! )?;
//!
//! // Get telemetry from communication service.
//! let telem = Arc::new(Mutex::new(CommsTelemetry::default()));
//!
//! // Start communication service.
//! CommsService::start(controls, &telem);
//! # Ok(())
//! # }
//! ```
//!
//! ## Comms Service Config File Format
//!
//! ```toml
//! [service-name.comms]
//! max_num_handlers = 50
//! downlink_ports = [13011]
//! timeout = 1500"
//! satellite_ip = "192.168.8.2"
//! ```

#[macro_use]
extern crate juniper;

#[macro_use]
extern crate log;

extern crate byteorder;
extern crate failure;

mod config;
mod errors;
mod packet;
mod service;
mod spacepacket;
mod telemetry;

#[cfg(test)]
mod tests;

/// Communication Service library.
pub use crate::service::*;

/// Communication Service errors.
pub use crate::errors::*;

/// Communication Service telemetry.
pub use crate::telemetry::CommsTelemetry;

/// Communication Service configuration parsing.
pub use crate::config::*;

pub use packet::LinkPacket;
pub use packet::LinkType;
pub use spacepacket::SpacePacket;
