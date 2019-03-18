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

extern crate comms_service;
extern crate pnet;
#[macro_use]
extern crate failure;

mod util;

use comms_service::*;
use std::net::Ipv4Addr;
use std::net::UdpSocket;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use util::*;

// Tests sending a packet from the ground to a service through a handler
// No response is sent from the service
#[test]
fn ground_to_service_no_response() {
    let sat_ip = "127.0.0.3";
    let ground_ip = "127.0.0.2";
    let ground_port = 16001;
    let downlink_port = 16002;
    let service_port = 16005;
    let config = comms_config(sat_ip, ground_ip, ground_port, downlink_port);
    let mock_comms = Arc::new(Mutex::new(MockComms::new()));
    let payload = vec![0, 1, 4, 5];

    // Control block to configure communication service.
    let controls = CommsControlBlock::new(
        Some(Arc::new(read)),
        vec![Arc::new(write)],
        mock_comms.clone(),
        mock_comms.clone(),
        config,
    )
    .unwrap();

    // Initialize new `CommsTelemetry` object.
    let telem = Arc::new(Mutex::new(CommsTelemetry::default()));

    let ground_packet = build_packet(
        &payload,
        ground_port,
        service_port,
        12,
        Ipv4Addr::from_str(sat_ip).unwrap(),
        Ipv4Addr::from_str(ground_ip).unwrap(),
    )
    .unwrap();

    // Insert data from ground into mock_comms read buffer
    if let Ok(comms) = mock_comms.lock() {
        comms.read_data(&ground_packet);
    }

    // Setup service listener
    let service_listener = UdpSocket::bind((sat_ip, service_port)).unwrap();

    // Start communication service.
    CommsService::start(controls, &telem).unwrap();

    thread::sleep(Duration::from_millis(100));

    let mut buf = [0; 4096];
    let (size, _) = service_listener.recv_from(&mut buf).unwrap();
    let recv_data = buf[0..size].to_vec();

    assert_eq!(recv_data, payload);
}
