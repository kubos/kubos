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

//! TOML parser for the `comms-service`. This module parses a `toml` file and returns a
//! struct containing configuration information for a `comms-service`.

use crate::errors::*;
use serde_derive::Deserialize;

/// Default message handler timeout
pub const DEFAULT_TIMEOUT: u64 = 1500;

/// A struct that holds useful configuration options to use in a `ground-comms-service` implementation.
/// Created by parsing a configuration file in the `toml` file format.
#[derive(Clone, Debug, Deserialize)]
pub struct CommsConfig {
    /// Timeout for something....
    /// Default: 1500
    pub timeout: Option<u64>,
    /// Required. IP address of the ground comms service
    pub ground_ip: String,
    /// Specifies the port on which the ground comms service listens to the gateway
    pub ground_port: u16,
    /// Required. IP address of the ground gateway.
    pub gateway_ip: String,
    /// Specifies the port to which the ground gateway listens to ground comms.
    pub gateway_port: u16,
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
