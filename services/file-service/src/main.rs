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

extern crate cbor_protocol;
extern crate file_protocol;
extern crate file_service;
extern crate kubos_system;
#[macro_use]
extern crate log;
extern crate simplelog;

use file_service::*;
use kubos_system::Config as ServiceConfig;
use simplelog::*;
use std::fs::File;

fn main() {
    let mut loggers: Vec<Box<SharedLogger>> = vec![];
    if let Some(l) = TermLogger::new(LevelFilter::Info, Config::default()) {
        loggers.push(l);
    }

    // This will panic if the log file fails to open
    // But I think that is correct
    loggers.push(WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        // TODO: Making log file directory configurable
        File::create("/var/log/kubos-file-transfer-service.log").unwrap(),
    ));

    match CombinedLogger::init(loggers) {
        Err(e) => panic!("Logging failed to start {:?}", e),
        _ => {}
    }

    let config = ServiceConfig::new("file-transfer-service");

    info!("Starting file transfer service at {}", config.hosturl());

    match recv_loop(config) {
        Ok(()) => warn!("Service listener loop exited successfully?"),
        Err(err) => error!("Service listener exited early: {}", err),
    }
}
