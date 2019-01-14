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

// Default values for control block configurations.
const DEFAULT_HANDLER_START: u16 = 13100;
const DEFAULT_HANDLER_END: u16 = 13149;
const DEFAULT_TIMEOUT: u64 = 1500;
static DEFAULT_GROUND_IP: &str = "192.168.8.1";
static DEFAULT_SATELLITE_IP: &str = "192.168.8.2";

/// A struct that holds useful configuration options to use in a `comms-service` implementation.
/// Created by parsing a configuration file in the `toml` file format.
#[derive(Debug, Deserialize)]
pub struct CommsConfig {
    /// Starting port used to define a range of ports that are used in the message handlers
    /// that handle messages received from the ground.
    pub handler_port_min: u16,
    /// Ending port used to define a range of ports that are used in the message handlers
    /// that handle messages received from the ground.
    pub handler_port_max: u16,
    /// Optional list of ports used by downlink endpoints that send messages to the ground.
    /// Each port in the list will be used by one downlink endpoint.
    pub downlink_ports: Option<Vec<u16>>,
    /// Timeout for the completion of GraphQL operations within message handlers (in milliseconds).
    pub timeout: u64,
    /// IP address of the ground gateway.
    pub ground_ip: String,
    /// Specifies the port to which the ground gateway is bound.
    pub ground_port: Option<u16>,
    /// Satellite's IP address.
    pub satellite_ip: String,
}

// Implementation of `Default` trait for `CommsConfig`.
impl Default for CommsConfig {
    fn default() -> Self {
        CommsConfig {
            handler_port_min: DEFAULT_HANDLER_START,
            handler_port_max: DEFAULT_HANDLER_END,
            downlink_ports: None,
            ground_port: None,
            timeout: DEFAULT_TIMEOUT,
            ground_ip: DEFAULT_GROUND_IP.to_string(),
            satellite_ip: DEFAULT_SATELLITE_IP.to_string(),
        }
    }
}

impl CommsConfig {
    /// Builds a new configuration for a specific `comms-service`.
    pub fn new(service_config: kubos_system::Config) -> Self {
        let config = service_config.get("comms").and_then(|raw| raw.try_into().ok());
        config.unwrap_or_default()
    }
}
