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
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use util::*;

// Testing sending a packet to the downlink port
// and checking if it arrives in the MockComms write queue
#[test]
fn downlink_to_ground() {
    let sat_ip = "127.0.0.3";
    let ground_ip = "127.0.0.2";
    let ground_port = 16001;
    let downlink_port = 16002;
    let service_port = 16005;
    let config = comms_config(sat_ip, ground_ip, ground_port, downlink_port);
    let mock_comms = Arc::new(Mutex::new(MockComms::new()));
    let payload = vec![5, 4, 3, 2];

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

    // Start communication service.
    CommsService::start(controls, &telem).unwrap();

    let downlink_writer = UdpSocket::bind((ground_ip, 0)).unwrap();

    thread::sleep(Duration::from_millis(1));

    downlink_writer
        .send_to(&payload, (sat_ip, downlink_port))
        .unwrap();

    thread::sleep(Duration::from_millis(1));

    let comms = mock_comms.lock().unwrap();
    let data = comms.write_buff.borrow_mut().pop().unwrap();
    let packet = UdpPacket::new(&data).unwrap();

    assert_eq!(packet.get_destination(), ground_port);
    assert_eq!(packet.payload().to_vec(), payload);
}
