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

// Ping all the services to make sure they're still running
//
// Log messages will be generated for each failing test.
// We assume that Monit is taking care of any reboots, so no further action will be taken

use super::*;

use failure::format_err;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;

use toml::Value;

pub fn ping_services() -> Result<u8, Error> {
    // Ping all the services to make sure they're still running

    let mut bad_count = 0;

    let services = get_services()?;

    for (name, config) in services.iter() {
        if ping(name, config).is_err() {
            bad_count += 1;
        }
    }

    Ok(bad_count)
}

fn get_services() -> Result<Vec<(String, ServiceConfig)>, Error> {
    let mut services = vec![];

    // Read the config.toml file
    let mut raw = String::new();
    let mut file = File::open(CONFIG_PATH)?;
    file.read_to_string(&mut raw)?;

    // Parse the config
    let data: Value = toml::from_str(&raw)?;
    let entries = data
        .as_table()
        .ok_or_else(|| format_err!("Failed to parse config.toml"))?;

    for (name, value) in entries {
        // If the config entry has an `addr` section, we should assume that it is a service
        if value.get("addr").is_some() {
            // Reassemble the TOML for this entry and then parse it
            let mut map = BTreeMap::new();
            map.insert(name, value);
            let config = ServiceConfig::new_from_str(name, &toml::to_string(&map)?)?;

            services.push((name.to_owned(), config));
        }
    }

    // Return a list of tuples with the name and config for each service
    Ok(services)
}

fn ping(service: &str, config: &ServiceConfig) -> Result<(), Error> {
    match query(config, "{ping}", Some(QUERY_TIMEOUT)) {
        Ok(data) => match data["ping"].as_str() {
            Some("pong") => Ok(()),
            other => {
                error!("Got bad result from {}: {:?}", service, other);
                bail!("Bad result");
            }
        },
        Err(err) => {
            error!("Failed to ping {}: {:?}", service, err);
            Err(err)
        }
    }
}
