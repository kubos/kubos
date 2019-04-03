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
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use std::net::Ipv4Addr;
use std::net::UdpSocket;
use std::str::FromStr;
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::time::Duration;
use util::*;

// Tests sending a packet from the ground to a service through a handler
// No response is sent from the service
#[test]
fn uplink_to_service_no_response() {
    let sat_ip = "127.0.0.3";
    let ground_ip = "127.0.0.2";
    let ground_port = 15001;
    let downlink_port = 15002;
    let service_port = 15005;
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

    // Pretend to be the ground and provide a packet
    // for the comms service to read from the radio
    mock_comms.lock().unwrap().push_read(&ground_packet);

    // Setup & start HTTP server
    let barrier = Arc::new(Barrier::new(2));
    let recv_data: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![]));
    let thread_data = recv_data.clone();
    spawn_http_server(
        vec![],
        thread_data,
        &format!("{}:{}", sat_ip, service_port),
        barrier.clone(),
    );

    // Start communication service.
    CommsService::start(controls, &telem).unwrap();

    // Let the wheels turn
    barrier.wait();

    // Retrieve the message for the service from shared buffer
    let rx_data = recv_data.lock().unwrap().to_owned();

    assert_eq!(rx_data, payload);
}

// Tests sending a packet from the ground to a service through a handler
// Service sends back a response via the message handler
#[test]
fn uplink_to_service_with_handler_response() {
    let sat_ip = "127.0.0.5";
    let ground_ip = "127.0.0.6";
    let ground_port = 16001;
    let downlink_port = 16002;
    let service_port = 16005;
    let config = comms_config(sat_ip, ground_ip, ground_port, downlink_port);
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

    let ground_packet = build_packet(
        &payload,
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

    // Setup & start HTTP server
    let barrier = Arc::new(Barrier::new(2));
    let recv_data: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![]));
    let thread_data = recv_data.clone();

    spawn_http_server(
        resp_payload.clone(),
        thread_data,
        &format!("{}:{}", sat_ip, service_port),
        barrier.clone(),
    );

    // Start communication service.
    CommsService::start(controls, &telem).unwrap();

    // Let the wheels turn
    barrier.wait();

    // Retrieve the message for the service from shared buffer
    let rx_data = recv_data.lock().unwrap().to_owned();

    assert_eq!(rx_data, payload);

    // Let the wheels turn
    thread::sleep(Duration::from_millis(200));

    // Pretend to be the ground and read the
    // packet which was written to the radio
    let data = mock_comms.lock().unwrap().pop_write().unwrap();
    let packet = UdpPacket::new(&data).unwrap();

    assert_eq!(packet.payload().to_vec(), resp_payload);
    assert_eq!(packet.get_destination(), ground_port);
}

// Tests sending a packet from the ground to a service through a handler
// Service sends back a response via the downlink port
#[test]
fn uplink_to_service_with_downlink_response() {
    let sat_ip = "127.0.0.7";
    let ground_ip = "127.0.0.8";
    let ground_port = 17001;
    let downlink_port = 17002;
    let service_port = 17005;
    let config = comms_config(sat_ip, ground_ip, ground_port, downlink_port);
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

    let ground_packet = build_packet(
        &payload,
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

    // Setup & start HTTP server
    let barrier = Arc::new(Barrier::new(2));
    let recv_data: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![]));
    spawn_http_server(
        vec![],
        recv_data.clone(),
        &format!("{}:{}", sat_ip, service_port),
        barrier.clone(),
    );

    // Start communication service.
    CommsService::start(controls, &telem).unwrap();

    // Let the wheels turn
    barrier.wait();

    // Retrieve the message for the service from shared buffer
    let rx_data = recv_data.lock().unwrap().to_owned();

    assert_eq!(rx_data, payload);

    let downlink_writer = UdpSocket::bind((sat_ip, 0)).unwrap();

    // Let the wheels turn
    thread::sleep(Duration::from_millis(200));

    // Send packet to comm service's downlink port
    downlink_writer
        .send_to(&resp_payload, (sat_ip, downlink_port))
        .unwrap();

    // Let the wheels turn
    thread::sleep(Duration::from_millis(10));

    // Pretend to be the ground and read the
    // packet which was written to the radio
    let data = mock_comms.lock().unwrap().pop_write().unwrap();
    let packet = UdpPacket::new(&data).unwrap();

    assert_eq!(packet.payload().to_vec(), resp_payload);
    assert_eq!(packet.get_destination(), ground_port);
}
