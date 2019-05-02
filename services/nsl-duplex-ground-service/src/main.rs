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

// #![deny(warnings)]

#[macro_use]
extern crate serde_derive;
extern crate ground_comms_service;
extern crate reqwest;
extern crate serde_json;

#[macro_use]
extern crate failure;

use kubos_service::Config;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use ground_comms_service::{CommsConfig, GatewayComms};

mod nsl_http_api;

fn main() {
    let service_config = Config::new("nsl-duplex-ground-service");
    let comms_config = CommsConfig::new(service_config.clone()).unwrap();

    let nsl_user = service_config
        .get("nsl_user")
        .expect("No `nsl_user` parameter found")
        .as_str()
        .unwrap()
        .to_owned();
    let nsl_pass = service_config
        .get("nsl_password")
        .expect("No `nsl_password` parameter found")
        .as_str()
        .unwrap()
        .to_owned();
    let mission_id = service_config
        .get("mission_id")
        .expect("No `mission_id` parameter found")
        .as_integer()
        .unwrap() as u16;

    let gateway_conn = Arc::new(Mutex::new(GatewayComms::new(
        &comms_config.gateway_ip,
        comms_config.gateway_port,
        &comms_config.ground_ip,
        comms_config.ground_port,
    )));
    let radio_conn = nsl_http_api::RadioConn::new(mission_id);
    radio_conn.initialize(&nsl_user, &nsl_pass).unwrap();
    let radio_conn = Arc::new(Mutex::new(radio_conn));

    let controls =
        ground_comms_service::CommsControlBlock::new(gateway_conn, radio_conn, comms_config)
            .unwrap();

    println!("Starting service");
    ground_comms_service::CommsService::start(controls).unwrap();

    loop {
        thread::sleep(Duration::from_secs(5));
    }
}
