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

#![deny(warnings)]

use kubos_system::Config as ServiceConfig;
use log::{error, info, warn};
use shell_service::*;
use syslog::Facility;

fn main() {
    syslog::init(
        Facility::LOG_DAEMON,
        log::LevelFilter::Debug,
        Some("kubos-shell-service"),
    )
    .unwrap();

    let config = ServiceConfig::new("shell-service")
        .map_err(|err| {
            error!("Failed to load service config: {:?}", err);
            err
        })
        .unwrap();

    let hosturl = config
        .hosturl()
        .ok_or_else(|| {
            error!("Failed to load service URL");
            failure::format_err!("Failed to load service URL")
        })
        .unwrap();

    info!("Starting shell service at {}", hosturl);

    match recv_loop(&config) {
        Ok(()) => warn!("Service listener loop exited successfully?"),
        Err(err) => error!("Service listener exited early: {}", err),
    }
}
