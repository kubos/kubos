#![deny(missing_docs)]
#![deny(warnings)]

//!
//! This library allows users to define and start communication services within their hardware services.
//!
//! # Example Usage
//!
//! ```rust,no_run
//! # extern crate comms_service;
//!
//! use comms_service::*;
//! use std::net::{Ipv4Addr, UdpSocket};
//! use std::sync::{Arc, Mutex};
//!
//! // Example setup.
//! fn read(socket: Arc<UdpSocket>) -> CommsResult<Vec<u8>> { Ok(vec![]) }
//! fn write(socket: Arc<UdpSocket>, data: &[u8]) -> CommsResult<()> { Ok(()) }
//!
//! // Defining connections.
//! let read_conn = Arc::new(UdpSocket::bind(("192.168.8.1", 13000)).unwrap());
//! let write_conn = Arc::new(UdpSocket::bind(("192.168.8.1", 13001)).unwrap());
//!
//! // Putting everything into the control block.
//! let controls = CommsControlBlock {
//!     read: Some(Arc::new(read)),
//!     write: vec![Arc::new(write)],
//!     read_conn,
//!     write_conn,
//!     handler_port_min: 13002,
//!     handler_port_max: 13099,
//!     timeout: 1500,
//!     ground_ip: Ipv4Addr::new(192, 168, 8, 40),
//!     satellite_ip: Ipv4Addr::new(192, 168, 8, 1),
//!     downlink_ports: Some(vec![13011]),
//!     ground_port: Some(9001)
//! };
//!
//! // Get telemetry from communication service.
//! let telem = Arc::new(Mutex::new(CommsTelemetry::default()));
//!
//! // Start communication service.
//! CommsService::start(controls, telem);
//! ```
//!
//! ## Comms Service Config File Format
//!
//! ```toml
//! [service-name]
//! handler_port_min = 13002
//! handler_port_max = 13010
//! downlink_ports = [13011]
//! ground-port = 9001
//! timeout = 1500
//! ground-ip = "192.168.8.1"
//! satellite-ip = "192.168.8.2"
//! ```

#[macro_use]
extern crate failure;
extern crate pnet;
#[macro_use]
extern crate juniper;
extern crate byteorder;
extern crate toml;

mod config;
mod errors;
mod service;
mod telemetry;

/// Communication Service library.
pub use service::*;

/// Communication Service errors.
pub use errors::*;

/// Communication Service telemetry.
pub use telemetry::CommsTelemetry;

/// Communication Service configuration parsing.
pub use config::*;
