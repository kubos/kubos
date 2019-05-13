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
    let downlink_port = 16002;
    let config = comms_config(sat_ip, downlink_port);
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

    let downlink_writer = UdpSocket::bind((sat_ip, 0)).unwrap();

    // Let the wheels turn
    thread::sleep(Duration::from_millis(10));

    // Send packet to comm service's downlink port
    downlink_writer
        .send_to(&payload, (sat_ip, downlink_port))
        .unwrap();

    // Let the wheels turn
    thread::sleep(Duration::from_millis(10));

    // Pretend to be the ground and read the
    // packet which was written to the radio
    let data = mock_comms.lock().unwrap().pop_write().unwrap();
    let packet = SpacePacket::parse(&data).unwrap();

    assert_eq!(packet.destination(), 0);
    assert_eq!(packet.payload().to_vec(), payload);
}
