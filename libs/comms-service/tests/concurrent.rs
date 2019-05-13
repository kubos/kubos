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

mod util;

use comms_service::*;
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::time::Duration;
use util::*;

// Tests sending concurrent packets from the ground to a service through a handler
// Service sends back a response via the message handler
#[test]
fn concurrent_uplinks_to_service_with_handler_response() {
    let sat_ip = "127.0.0.9";
    let downlink_port = 18002;
    let service_port = 18005;
    let config = comms_config(sat_ip, downlink_port);
    let mock_comms = Arc::new(Mutex::new(MockComms::new()));
    let payload = vec![0, 1, 4, 5];
    let resp_payload = vec![9, 8, 7, 6];

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

    let num_tests = 10;
    let barrier = Arc::new(Barrier::new(11));
    let mut recv_data_list: Vec<Arc<Mutex<Vec<u8>>>> = vec![];
    for i in 0..(num_tests) {
        let ground_packet =
            SpacePacket::build(i, LinkType::GraphQL, (service_port + i) as u16, &payload).unwrap();

        // Pretend to be the ground and provide a packet
        // for the comms service to read from the radio
        mock_comms
            .lock()
            .unwrap()
            .push_read(&ground_packet.to_bytes().unwrap());

        // Setup & start HTTP server
        let recv_data: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![]));

        spawn_http_server(
            resp_payload.clone(),
            recv_data.clone(),
            &format!("{}:{}", sat_ip, service_port + i),
            barrier.clone(),
        );

        recv_data_list.push(recv_data);

        thread::sleep(Duration::from_millis(10));
    }

    // Start communication service.
    CommsService::start(controls, &telem).unwrap();

    // Wait until HTTP servers are ready
    barrier.wait();

    for _ in 0..(num_tests) {
        let recv_data = recv_data_list.pop().unwrap();
        // Retrieve the message for the service from shared buffer
        let rx_data = recv_data.lock().unwrap().to_owned();

        assert_eq!(rx_data, payload);
    }

    thread::sleep(Duration::from_millis(100));

    for _ in 0..(num_tests) {
        // Pretend to be the ground and read the
        // packet which was written to the radio
        let data = mock_comms.lock().unwrap().pop_write().unwrap();
        let packet = SpacePacket::parse(&data).unwrap();

        assert_eq!(packet.payload().to_vec(), resp_payload);
        assert_eq!(packet.destination(), 0);
    }
}

// Tests sending concurrent packets from the ground to a service through a handler
// Service sends back a response via the message handler
#[test]
fn too_many_concurrent_uplinks_to_service_with_handler_response() {
    let sat_ip = "127.0.0.11";
    let downlink_port = 19002;
    let service_port = 19005;
    let config = comms_config(sat_ip, downlink_port);
    let mock_comms = Arc::new(Mutex::new(MockComms::new()));
    let payload = vec![0, 1, 4, 5];
    let resp_payload = vec![9, 8, 7, 6];

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
    let barrier = Arc::new(Barrier::new(11));
    let num_tests = 15;
    let mut recv_data_list: Vec<Arc<Mutex<Vec<u8>>>> = vec![];
    for i in 0..(num_tests) {
        let ground_packet =
            SpacePacket::build(i, LinkType::GraphQL, (service_port + i) as u16, &payload).unwrap();

        // Pretend to be the ground and provide a packet
        // for the comms service to read from the radio
        mock_comms
            .lock()
            .unwrap()
            .push_read(&ground_packet.to_bytes().unwrap());

        // Setup & start HTTP server
        let recv_data: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![]));
        spawn_http_server(
            resp_payload.clone(),
            recv_data.clone(),
            &format!("{}:{}", sat_ip, service_port + i),
            barrier.clone(),
        );

        recv_data_list.push(recv_data);

        thread::sleep(Duration::from_millis(10));
    }

    // Start communication service.
    CommsService::start(controls, &telem).unwrap();

    // Wait for HTTP server to get ready
    barrier.wait();

    let mut num_rx_correct = 0;
    let mut num_rx_empty = 0;
    for _ in 0..(num_tests) {
        let recv_data = recv_data_list.pop().unwrap();
        // Retrieve the message for the service from shared buffer
        let rx_data = recv_data.lock().unwrap().to_owned();

        if rx_data == payload {
            num_rx_correct += 1;
        } else {
            num_rx_empty += 1;
        }
    }
    assert_eq!(num_rx_correct, 10);
    assert_eq!(num_rx_empty, 5);

    // Let the wheels turn
    thread::sleep(Duration::from_millis(100));

    let mut num_packet_correct = 0;
    let mut num_packet_empty = 0;
    for _ in 0..(num_tests) {
        // Pretend to be the ground and read the
        // packet which was written to the radio
        let data = mock_comms.lock().unwrap().pop_write();
        if let Some(data) = data {
            let packet = SpacePacket::parse(&data).unwrap();
            assert_eq!(packet.payload().to_vec(), resp_payload);
            assert_eq!(packet.destination(), 0);
            num_packet_correct += 1;
        } else {
            num_packet_empty += 1;
        }
    }
    assert_eq!(num_packet_correct, 10);
    assert_eq!(num_packet_empty, 5);
}
