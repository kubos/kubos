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
use failure::{bail, Error};
use serde_derive::Deserialize;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use toml;
use toml::Value;

/// The default configuration file path
pub static DEFAULT_PATH: &str = "/home/system/etc/config.toml";

#[derive(Clone, Debug, Deserialize)]
/// A simple address consisting of an IP address and port number
pub struct Address {
    ip: String,
    port: u16,
}

impl Address {
    /// Returns the IP portion of this address
    pub fn ip(&self) -> &str {
        &self.ip
    }

    /// Returns the port of this address
    pub fn port(&self) -> u16 {
        self.port
    }
}

/// KubOS config used by either Apps or Services. KubOS config files use the TOML format, and can
/// may contain multiple named Categories. Typically each category corresponds to an App or Service
/// name. This allows one config file to store configuration for multiple Apps / Services at a
/// time.
///
/// Example KubOS config files for a Service called `my-service` with an IP/port binding
/// ```toml
/// [my-service]
/// my-property = "value"
///
/// [my-service.addr]
/// ip = 0.0.0.0
/// port = 8181
/// ```
///
#[derive(Clone, Debug)]
pub struct Config {
    addr: Option<Address>,
    raw: Value,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            addr: None,
            raw: Value::String("".to_string()),
        }
    }
}

impl Config {
    /// Creates and parses configuration data from the system configuration
    /// file or the path passed as the '-c' or '--config' option to this
    /// executable.
    ///
    /// # Arguments
    /// `name` - Category name used as a key in the config file
    pub fn new(name: &str) -> Result<Self, Error> {
        Self::new_from_path(name, get_config_path()?)
    }

    /// Creates and parses configuration data from the passed in configuration
    /// path.
    /// # Arguments
    /// `name` - Category name used as a key in the config file
    /// `path` - Path to configuration file
    pub fn new_from_path(name: &str, path: String) -> Result<Self, Error> {
        parse_config_file(name, path)
    }

    /// Creates and parses configuration data from the passed in configuration
    /// string.
    /// # Arguments
    /// `name` - Category name used as a key in the config
    /// `config` - Config data as a string
    pub fn new_from_str(name: &str, config: &str) -> Result<Self, Error> {
        parse_config_str(name, config)
    }

    /// Returns the configured hosturl string in the following
    /// format (using IPv4 addresses) - 0.0.0.0:0000
    pub fn hosturl(&self) -> Option<String> {
        if let Some(addr) = &self.addr {
            Some(format!("{}:{}", addr.ip(), addr.port()))
        } else {
            None
        }
    }

    /// Returns the category's configuration information
    /// in the `toml::Value` format.
    /// This will contain the ip/port if provided, along with any other
    /// configuration information found in the config file.
    ///
    /// ### Examples
    ///
    /// ```rust,no_run
    /// use kubos_system::Config;
    ///
    /// let config = Config::new("example-service").unwrap();
    /// let raw = config.raw();
    /// let bus = raw["bus"].as_str();
    /// ```
    pub fn raw(&self) -> toml::Value {
        self.raw.clone()
    }

    /// Performs a get on the raw config data
    ///
    /// # Arguments
    /// `key` - Key of value to get from config
    pub fn get(&self, key: &str) -> Option<toml::Value> {
        match self.raw.get(key) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
}

fn get_config_path() -> Result<String, Error> {
    // Manually check for a "-c {config-path}" command line argument specifying a custom config
    // file path to use.
    // Doing it this way so that entities which use this module (apps, services) can have any
    // number of additional command arguments
    let mut args = env::args();

    // Navigate to the "-c" option
    let config_arg_pos = args.position(|arg| arg == "-c");

    if let Some(_pos) = config_arg_pos {
        // The config path will be the arg immediately after "-c"
        match args.next() {
            Some(path) => Ok(path),
            None => bail!("The '-c' arg was specified, but no path value was provided"),
        }
    } else {
        // The "-c" arg wasn't specified, so we can go ahead with the default
        Ok(DEFAULT_PATH.to_string())
    }
}

fn get_file_data(path: String) -> Result<String, io::Error> {
    let mut contents = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_config_file(name: &str, path: String) -> Result<Config, Error> {
    let contents = get_file_data(path)?;
    parse_config_str(name, &contents)
}

fn parse_config_str(name: &str, contents: &str) -> Result<Config, Error> {
    let data: Value = toml::from_str(&contents)?;
    let mut config = Config::default();

    if let Some(data) = data.get(name) {
        if let Some(address) = data.get("addr") {
            config.addr = Some(address.clone().try_into()?);
        }
        config.raw = data.clone();
    } else {
        bail!("Failed to find {} in config", name);
    }

    Ok(config)
}
