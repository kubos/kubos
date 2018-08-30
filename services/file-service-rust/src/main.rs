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

extern crate cbor_protocol;
extern crate file_protocol;
extern crate file_service_rust;
extern crate kubos_system;
#[macro_use]
extern crate log;
extern crate simplelog;
use std::fs::File;

use file_service_rust::*;
use kubos_system::Config as ServiceConfig;
use simplelog::*;

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
        // Should this log path come from the config file?
        File::create("/var/log/kubos/file-transfer-service.log").unwrap(),
    ));

    match CombinedLogger::init(loggers) {
        Err(e) => panic!("Logging failed to start {:?}", e),
        _ => {}
    }

    let config = ServiceConfig::new("file-transfer-service");

    info!("Starting file transfer service");

    match recv_loop(config) {
        Ok(()) => warn!("Service listener loop exited successfully?"),
        Err(err) => error!("Service listener exited early: {}", err),
    }
}
