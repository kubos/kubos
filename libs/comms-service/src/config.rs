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

//! TOML parser for the `comms-service`. This module parses a `toml` file and returns a
//! struct containing configuration information for a `comms-service`.

use crate::errors::*;
use serde_derive::Deserialize;

/// Default maximum number of message handlers
pub const DEFAULT_MAX_HANDLERS: u16 = 50;
/// Default message handler timeout
pub const DEFAULT_TIMEOUT: u64 = 1500;

/// A struct that holds useful configuration options to use in a `comms-service` implementation.
/// Created by parsing a configuration file in the `toml` file format.
#[derive(Clone, Debug, Deserialize)]
pub struct CommsConfig {
    /// The maximum number of concurrent message handlers allowed
    /// Default: 50
    pub max_num_handlers: Option<u16>,
    /// Optional list of ports used by downlink endpoints that send messages to the ground.
    /// Each port in the list will be used by one downlink endpoint.
    pub downlink_ports: Option<Vec<u16>>,
    /// Timeout for the completion of GraphQL operations within message handlers (in milliseconds).
    /// Default: 1500
    pub timeout: Option<u64>,
    /// Required. IP address on which comms service will listen.
    pub ip: String,
}

impl CommsConfig {
    /// Builds a new configuration for a specific `comms-service`.
    /// Configuration parameters are read from the service's `config.toml` file.
    pub fn new(service_config: kubos_system::Config) -> CommsResult<Self> {
        let raw_config = service_config.get("comms").ok_or_else(|| {
            CommsServiceError::ConfigError("Unable to get `comms` config".to_owned())
        })?;

        let config: CommsConfig = raw_config.try_into().map_err(|err| {
            let msg = format!("Failed to parse config: {}", err);
            CommsServiceError::ConfigError(msg)
        })?;

        Ok(config)
    }
}
