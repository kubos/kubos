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

#[macro_use]
extern crate failure;
extern crate tempfile;

mod util;

use comms_service::*;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use util::*;
use utils::testing::TestService;

// Tests sending a ping from the ground to an instance of the actual
// monitor service and reading back the response.
#[test]
fn query_monitor_service() {
    let sat_ip = "127.0.0.5";
    let ground_ip = "127.0.0.6";
    let ground_port = 15001;
    let downlink_port = 15002;
    let service_port = 15005;
    let config = comms_config(sat_ip, ground_ip, ground_port, downlink_port);
    let mock_comms = Arc::new(Mutex::new(MockComms::new()));
    let query = "{\"query\":\"{ping}\"}".as_bytes();
    let response = "{\"data\":{\"ping\":\"pong\"}}".as_bytes();

    // Start up monitor service
    let monitor_service = TestService::new("monitor-service", sat_ip, service_port);
    monitor_service.build();
    monitor_service.spawn();

    thread::sleep(Duration::from_millis(1000));

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
        &query,
        ground_port,
        service_port,
        12,
        Ipv4Addr::from_str(sat_ip).unwrap(),
        Ipv4Addr::from_str(ground_ip).unwrap(),
    )
    .unwrap();

    // Pretend to be the ground and provide a packet
    // for the comms service to read from the radio
    mock_comms.lock().unwrap().push_read(&ground_packet);

    // Start communication service.
    CommsService::start(controls, &telem).unwrap();

    // Let the wheels turn
    thread::sleep(Duration::from_millis(500));

    // Pretend to be the ground and read the
    // packet which was written to the radio
    let data = mock_comms.lock().unwrap().pop_write().unwrap();
    let packet = UdpPacket::new(&data).unwrap();

    assert_eq!(packet.payload().to_vec(), response);
    assert_eq!(packet.get_destination(), ground_port);
}
