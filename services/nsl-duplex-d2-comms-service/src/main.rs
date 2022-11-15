//
// Copyright (C) 2019 Kubos Corporation
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

//!
//! Hardware service to allow for communications over a NSL Duplex D2 Radio.
//! This service starts up a communication service to allow transfer of UDP
//! packets over a serial link to and from the radio.
//!
//! # Configuration
//!
//! The service can be configured in the `config.toml` with the following fields:
//!
//! ```toml
//! [nsl-duplex-comms-service]
//! bus = "/dev/ttyUSB0"
//! ping_freq = 10
//!
//! [nsl-duplex-comms-service.comms]
//! max_num_handlers = 10
//! downlink_ports = [15001]
//! timeout = 15000
//! ip = "127.0.0.2"
//!
//! [nsl-duplex-comms-service.addr]
//! ip = "127.0.0.1"
//! port = 8140
//! ```
//!
//! The configuration of this service is split into three parts:
//!   - Service Specific
//!
//!     This section is found under `[nsl-duplex-comms-service]`
//!     - `bus` - Specifies which UART bus the Duplex is on
//!     - `ping_freq` - Specifies how frequently the service sends ping packets when the downlink queue is empty (seconds)
//!   - Communications Service Configs
//!
//!     This section is found under `[nsl-duplex-comms-service.comms]`
//!     - `max_num_handlers` - Maximum number of concurrent message handlers
//!     - `downlink_ports` - Optional list of downlink endpoints
//!     - `timeout` - Timeout for completion of GraphQL requests
//!     - `ip` - Local IP of satellite
//!   - GraphQL Server Configs
//!
//!     This section is found under `[nsl-duplex-comms-service.addr]`
//!     - `ip` - The service's IP address, used for its GraphQL interface
//!     - `port` - The service's port, used for its GraphQL interface
//!
//! Where `bus` specifies the UART bus the Duplex is on, `ping_freq` specifies how often
//! the downlink queue should be checked for queueing up pings, `ip` specifies the
//! service's IP address, and `port` specifies the port on which the service will be
//! listening for UDP packets.
//!
//! # Running the Service
//!
//! The service should be started automatically by its init script, but may also be started manually:
//!
//! ```bash
//! $ nsl-duplex-d2-comms-service
//! NSL Duplex Communications Service starting on /dev/ttyUSB0
//! ```
//!
//! If no config file is specified, then the service will look at `/etc/kubos-config.toml`.
//! An alternative config file may be specified on the command line at run time:
//!
//! ```bash
//! $ nsl-duplex-d2-comms-service -c config.toml
//! ```
//!
//! # Panics
//!
//! This service will panic on start if no config sections are found for the service or
//! if the `bus` parameter is not provided.
//!
//! # GraphQL Schema
//!
//! ## Queries
//!
//! ### Failed Packets Up
//!
//! Request number of bad uplink packets
//!
//! ```
//! {
//!     failedPacketsUp: Int!
//! }
//! ```
//!
//! ### Failed Packets Down
//!
//! Request number of bad downlink packets
//!
//! ```
//! {
//!     failedPacketsDown: Int!
//! }
//! ```
//!
//! ### Packets Up
//!
//! Request number of packets successfully uplinked
//!
//! ```
//! {
//!     packetsUp: Int!
//! }
//! ```
//!
//! ### Packets Down
//!
//! Request number of packets successfully downlinked
//!
//! ```
//! {
//!     packetsDown: Int!
//! }
//! ```
//!
//! ### Errors
//!
//! Request errors that have occurred
//!
//! ```
//! {
//!     errors: [String]
//! }
//! ```
//!
//! ### Modem Health
//!
//! Request current modem health information
//!
//! ```json
//! {
//!     modemHealth {
//!         resetCount: Int!
//!         currentTime: Int!
//!         currentRssi: Int!
//!         connectionStatus: Int!
//!         globalstarGateway: Int!
//!         lastContactTime: Int!
//!         lastAttemptTime: Int!
//!         callAttemptsSinceReset: Int!
//!         successfulConnectsSinceReset: Int!
//!         averageConnectionDuration: Int!
//!         connectionDurationStdDev: Int!
//!     }
//! }
//! ```
//!
//! ### Geolocation
//!
//! Request current geolocation data
//!
//! ```jso
//! {
//!     geolocation {
//!         lon: Float!
//!         lat: Float!
//!         time: Int!
//!         maxError: Int!
//!     }
//! }
//! ```
//!
//! ### Downlink Queue Count
//!
//! Request number of files in the downlink queue
//!
//! ```json
//! {
//!     downlink_queue_count: Int!
//! }
//! ```
//!
//! ### Is Alive
//!
//! Queries the modem's 'is_alive' status
//!
//! ```json
//! {
//!     alive: Boolean!
//! }
//! ```
//!
//! ## Mutations
//!
//! ### NoOp
//!
//! Execute a trivial command against the system
//!
//! ```json
//! mutation {
//!     noop: Boolean!
//! }
//! ```
//!

#![deny(warnings)]
#![deny(missing_docs)]

extern crate comms_service;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate juniper;
extern crate kubos_service;
#[macro_use]
extern crate log;
extern crate nsl_duplex_d2;

mod comms;
mod model;
mod schema;

use crate::comms::*;
use crate::model::Subsystem;
use crate::schema::{MutationRoot, QueryRoot};
use comms_service::*;
use failure::Error;
use kubos_service::{Config, Logger, Service};
use std::sync::{Arc, Mutex};

// Generic return type
type NslDuplexCommsResult<T> = Result<T, Error>;

fn main() -> NslDuplexCommsResult<()> {
    Logger::init("nsl-duplex-comms-service").unwrap();

    let service_config = Config::new("nsl-duplex-comms-service").map_err(|err| {
        error!("Failed to load service config: {:?}", err);
        err
    })?;

    let bus = service_config
        .get("bus")
        .ok_or_else(|| {
            error!("Failed to load 'bus' config value");
            "Failed to load 'bus' config value"
        })
        .unwrap()
        .as_str()
        .ok_or_else(|| {
            error!("Failed to parse 'bus' config value");
            "Failed to parse 'bus' config value"
        })
        .unwrap()
        .to_owned();

    let ping_freq = if let Some(ping_freq) = service_config.get("ping_freq") {
        ping_freq.as_integer().unwrap_or(DEFAULT_PING_FREQ as i64) as u64
    } else {
        DEFAULT_PING_FREQ
    };

    // Read configuration from config file.
    let comms_config = CommsConfig::new(service_config.clone()).map_err(|err| {
        error!("Failed to load comms config: {:?}", err);
        err
    })?;

    // Open radio serial connection
    let duplex_comms = Arc::new(Mutex::new(DuplexComms::new(&bus, ping_freq)));

    // Start keep alive loop
    let radio = duplex_comms.clone();
    std::thread::spawn(|| ping_loop(radio));

    // Control block to configure communication service.
    let controls = CommsControlBlock::new(
        Some(Arc::new(read)),
        vec![Arc::new(write)],
        duplex_comms.clone(),
        duplex_comms.clone(),
        comms_config,
    )?;

    // Initialize new `CommsTelemetry` object.
    let telem = Arc::new(Mutex::new(CommsTelemetry::default()));

    // Start communication service.
    info!("NSL Duplex Communications Service starting on {}", bus);
    CommsService::start::<Arc<Mutex<DuplexComms>>, SpacePacket>(controls, &telem)?;

    // Start up graphql server
    let subsystem = Subsystem::new(telem, duplex_comms);
    Service::new(service_config, subsystem, QueryRoot, MutationRoot).start();

    Ok(())
}
