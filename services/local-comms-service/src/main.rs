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

#![deny(missing_docs)]
#![deny(warnings)]

//!
//! Local comms service for testing against a local system or services.
//! Creates a single listening UDP interface which accepts UDP packets
//! with embedded Space Packets, and passes the Space Packet payload
//! along to the appropriate service.
//!
//! Telemetry queries will be added as desired for testing.
//!

use crate::comms::*;
use comms_service::*;
use failure::Error;
use kubos_system::logger as ServiceLogger;
use kubos_system::Config as ServiceConfig;
use log::error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod comms;

// Return type for the ethernet service.
type LocalCommsServiceResult<T> = Result<T, Error>;

fn main() -> LocalCommsServiceResult<()> {
    // Setup new system logging for ethernet service.
    ServiceLogger::init("local-comms-service").unwrap();

    // Get the main service configuration from the system's config.toml file
    let service_config = ServiceConfig::new("local-comms-service").map_err(|err| {
        error!("Failed to load service config: {:?}", err);
        err
    })?;

    let gateway_ip = service_config
        .get("gateway_ip")
        .ok_or_else(|| {
            error!("No 'gateway_ip' parameter in config");
            "No 'gateway_ip' parameter in config"
        })
        .unwrap()
        .as_str()
        .ok_or_else(|| {
            error!("Failed to parse 'gateway_ip' config value");
            "Failed to parse 'gateway_ip' config value"
        })
        .unwrap()
        .to_owned();

    let gateway_port = service_config
        .get("gateway_port")
        .ok_or_else(|| {
            error!("No 'gateway_port' parameter in config");
            "No 'gateway_port' parameter in config"
        })
        .unwrap()
        .as_integer()
        .ok_or_else(|| {
            error!("Failed to parse 'gateway_port' config value");
            "Failed to parse 'gateway_port' config value"
        })
        .unwrap() as u16;

    let listening_ip = service_config
        .get("listening_ip")
        .ok_or_else(|| {
            error!("No 'listening_ip' parameter in config");
            "No 'listening_ip' parameter in config"
        })
        .unwrap()
        .as_str()
        .ok_or_else(|| {
            error!("Failed to parse 'listening_ip' config value");
            "Failed to parse 'listening_ip' config value"
        })
        .unwrap()
        .to_owned();

    let listening_port = service_config
        .get("listening_port")
        .ok_or_else(|| {
            error!("No 'listening_port' parameter in config");
            "No 'listening_port' parameter in config"
        })
        .unwrap()
        .as_integer()
        .ok_or_else(|| {
            error!("Failed to parse 'listening_port' config value");
            "Failed to parse 'listening_port' config value"
        })
        .unwrap() as u16;

    // Pull out our communication settings
    let config = CommsConfig::new(service_config).map_err(|err| {
        error!("Failed to initialize CommsConfig: {:?}", err);
        err
    })?;

    let conn = Arc::new(Mutex::new(
        LocalComms::new(&listening_ip, listening_port, &gateway_ip, gateway_port).map_err(
            |err| {
                error!("Failed to initialize LocalComms: {:?}", err);
                err
            },
        )?,
    ));

    // Control block to configure communication service.
    let controls = CommsControlBlock::new(
        Some(Arc::new(read)),
        vec![Arc::new(write)],
        conn.clone(),
        conn,
        config,
    )
    .map_err(|err| {
        error!("Failed to initialize CommsControlBlock: {:?}", err);
        err
    })?;

    // Initialize new `CommsTelemetry` object.
    let telem = Arc::new(Mutex::new(CommsTelemetry::default()));

    // Start communication service.
    CommsService::start::<Arc<Mutex<LocalComms>>, SpacePacket>(controls, &telem).map_err(
        |err| {
            error!("Failed to start comms service: {:?}", err);
            err
        },
    )?;

    // We will eventually start the GraphQL service here.
    loop {
        thread::sleep(Duration::from_millis(1))
    }
}
