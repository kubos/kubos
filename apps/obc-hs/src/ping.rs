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

pub fn ping_services() -> Result<u8, Error> {
    // Ping all the services to make sure they're still running

    let mut bad_count = 0;

    // Core services:
    if ping("app-service").is_err() {
        bad_count += 1;
    }
    if ping("monitor-service").is_err() {
        bad_count += 1;
    }
    if ping("telemetry-service").is_err() {
        bad_count += 1;
    }

    // Hardware services:
    //
    // Add an entry for each hardware service present in the system.

    // For example:
    // if ping("pumpkin-mcu-service").is_err() {
    //     bad_count += 1;
    // }

    Ok(bad_count)
}

fn ping(service: &str) -> Result<(), Error> {
    let config = ServiceConfig::new(service);
    match query(&config, "{ping}", Some(QUERY_TIMEOUT)) {
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
