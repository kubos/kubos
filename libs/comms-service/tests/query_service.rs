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
use std::fs::File;
use std::io::Write;
use std::net::Ipv4Addr;
use std::process;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tempfile::tempdir;
use util::*;

// Tests sending a packet from the ground to a service through a handler
// No response is sent from the service
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
    let service_handle_rx = spawn_monitor_service();

    thread::sleep(Duration::from_millis(500));

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

    // Kill off the service process
    let mut service_handle = service_handle_rx.recv().unwrap();
    service_handle.kill().unwrap();
}

fn spawn_monitor_service() -> Receiver<process::Child> {
    let (sender, receiver) = channel();

    let config = r#"[monitor-service.addr]
ip = "127.0.0.5"
port = 15005"#;

    thread::spawn(move || {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.toml");
        let mut file = File::create(file_path.clone()).unwrap();
        writeln!(file, "{}", config).unwrap();

        let child = Command::new("cargo")
            .arg("run")
            .arg("--package")
            .arg("monitor-service")
            .arg("--")
            .arg("-c")
            .arg(file_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to start");

        sender.send(child).unwrap();

        thread::sleep(Duration::from_secs(2));
    });

    receiver
}
