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

use serde_json;

use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use std::{thread, thread::JoinHandle};

static UP_SQL: &'static str = r"CREATE TABLE telemetry (
timestamp INTEGER NOT NULL,
subsystem VARCHAR(255) NOT NULL,
parameter VARCHAR(255) NOT NULL,
value DOUBLE NOT NULL,
PRIMARY KEY (timestamp, subsystem, parameter));";

static DOWN_SQL: &'static str = r"DROP TABLE telemetry;";

fn setup_db(sql: Option<&str>) {
    Command::new("sqlite3")
        .arg("test.db")
        .arg(DOWN_SQL)
        .output()
        .expect("Sql cmd failed");

    Command::new("sqlite3")
        .arg("test.db")
        .arg(UP_SQL)
        .output()
        .expect("Sql cmd failed");

    if let Some(sql) = sql {
        Command::new("sqlite3")
            .arg("test.db")
            .arg(sql)
            .output()
            .expect("Sql cmd failed");
    }
}

fn start_telemetry() -> (JoinHandle<()>, Sender<bool>) {
    let mut telem_path = env::current_exe().unwrap();
    telem_path.pop();
    telem_path.set_file_name("telemetry-service");

    let (tx, rx): (Sender<bool>, Receiver<bool>) = channel();
    let telem_thread = thread::spawn(move || {
        println!("Current working dir {:?}", env::current_dir());
        let mut telem_proc = Command::new(telem_path)
            .arg("-c")
            .arg("tests/config.toml")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let mut run = true;
        while run {
            thread::sleep(Duration::from_millis(100));
            if let Ok(_) = rx.try_recv() {
                telem_proc.kill().unwrap();
                run = false;
            }
        }
    });

    // Give the process a bit to actually start
    thread::sleep(Duration::from_millis(100));
    return (telem_thread, tx);
}

pub fn setup(sql: Option<&str>) -> (JoinHandle<()>, Sender<bool>) {
    setup_db(sql);

    return start_telemetry();
}

pub fn teardown(handle: JoinHandle<()>, sender: Sender<bool>) {
    sender.send(true).unwrap();
    handle.join().unwrap();
}

pub fn do_query(query: &str) -> serde_json::Value {
    let remote_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8111);
    let local_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8110);

    let socket = UdpSocket::bind(local_addr).expect("couldn't bind to address");
    socket
        .send_to(&query.as_bytes(), &remote_addr)
        .expect("couldn't send message");
    socket.set_read_timeout(Some(Duration::new(10, 0))).unwrap();

    let mut buf = [0; 1024];
    match socket.recv_from(&mut buf) {
        Ok((amt, _)) => serde_json::from_slice(&buf[0..amt]).unwrap(),
        Err(e) => panic!("recv function failed: {:?}", e),
    }
}
