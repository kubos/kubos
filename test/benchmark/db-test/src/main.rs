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

extern crate kubos_system;
extern crate kubos_telemetry_db;
extern crate rand;
#[macro_use]
extern crate serde_json;
extern crate time;

use kubos_system::Config;
use kubos_telemetry_db::Database;
use rand::{thread_rng, Rng};
use serde_json::{ser, Value};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::thread;
use std::time::Duration;
use time::PreciseTime;

const ITERATIONS: i64 = 100;

fn db_test(config: &Config) {
    let db_path = config
        .get("database")
        .expect("No database path found in config file");
    let db_path = db_path.as_str().unwrap_or("");

    let db = Database::new(&db_path);
    db.setup();

    let mut times: Vec<i64> = Vec::new();

    for _ in 0..ITERATIONS {
        let mut rng = thread_rng();
        let timestamp = rng.gen_range(0, ::std::i32::MAX);

        let start = PreciseTime::now();
        if db.insert(timestamp, "db-test", "parameter", "value")
            .is_ok()
        {
            times.push(start.to(PreciseTime::now()).num_microseconds().unwrap());
        }
    }

    let num_entries = times.len() as i64;
    let sum: i64 = times.iter().sum();

    let average = sum / num_entries;

    println!(
        "Average insert time after {} runs: {} us",
        num_entries, average
    );
}

fn graphql_test(config: &Config) {
    let mut times: Vec<i64> = Vec::new();

    for _ in 0..ITERATIONS {
        let mut rng = thread_rng();
        let timestamp = rng.gen_range(0, ::std::i32::MAX);

        let mutation = format!(
            r#"mutation {{
            insert(timestamp: {}, subsystem: "db-test", parameter: "voltage", value: "4.0") {{
                success,
                errors
            }}
        }}"#,
            timestamp
        );

        let remote_addr = config.hosturl();
        let local_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);

        let socket = UdpSocket::bind(local_addr).expect("Couldn't bind to address");

        let start = PreciseTime::now();
        socket
            .send_to(&mutation.as_bytes(), &remote_addr)
            .expect("Couldn't send message");
        socket.set_read_timeout(Some(Duration::new(1, 0))).unwrap();

        let mut buf = [0; 1024];
        match socket.recv_from(&mut buf) {
            Ok(_) => times.push(start.to(PreciseTime::now()).num_microseconds().unwrap()),
            Err(e) => panic!("recv function failed: {:?}", e),
        }
    }

    let num_entries = times.len() as i64;
    let sum: i64 = times.iter().sum();

    let average = sum / num_entries;

    println!(
        "Average mutation time after {} runs: {} us",
        num_entries, average
    );
}

fn direct_udp_test(config: &Config) {
    let mut times: Vec<i64> = Vec::new();

    for _ in 0..ITERATIONS {
        let remote_addr = config.hosturl();
        let local_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);

        let socket = UdpSocket::bind(local_addr).expect("Couldn't bind to address");

        let message = json!({
            "timestamp": 1,
            "subsystem": "db-test",
            "parameter": "voltage",
            "value": "3.3"
        });

        let start = PreciseTime::now();

        socket
            .send_to(&ser::to_vec(&message).unwrap(), remote_addr)
            .unwrap();

        times.push(start.to(PreciseTime::now()).num_microseconds().unwrap())
    }

    let num_entries = times.len() as i64;
    let sum: i64 = times.iter().sum();

    let average = sum / num_entries;

    println!(
        "Average UDP send time after {} runs: {} us",
        num_entries, average
    );
}

fn test_cleanup(config: &Config) {
    let mutation = r#"mutation {
            delete(subsystem: "db-test") {
                success,
                errors,
                entriesDeleted
            }
        }"#;

    let remote_addr = config.hosturl();
    let local_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);

    let socket = UdpSocket::bind(local_addr).expect("Couldn't bind to address");

    socket
        .send_to(&mutation.as_bytes(), &remote_addr)
        .expect("Couldn't send message");
    socket.set_read_timeout(Some(Duration::new(1, 0))).unwrap();

    let mut buf = [0; 1024];
    match socket.recv_from(&mut buf) {
        Ok((amt, _)) => {
            let mut v: serde_json::Value = serde_json::from_slice(&buf[0..(amt)]).unwrap();
            println!("{}", serde_json::to_string_pretty(&v).unwrap());
            match v.get("msg") {
                Some(message) => {
                    let success =
                        serde_json::from_value::<bool>(message["success"].clone()).unwrap();

                    let errors =
                        serde_json::from_value::<String>(message["errors"].clone()).unwrap();

                    let entries_deleted =
                        serde_json::from_value::<i64>(message["entriesDeleted"].clone()).unwrap();

                    println!(
                        "Delete operation: {} {} {}",
                        success, errors, entries_deleted
                    );
                }
                None => println!("Failed to process delete response"),
            }
        }
        Err(e) => panic!("recv function failed: {:?}", e),
    }
}

fn main() {
    let config = Config::new("telemetry-service");
    /*
    db_test(&config);

    // This sleep likely isn't necessary, but I'd like to make extra sure nothing about a test
    // lingers to affect the next one
    thread::sleep(Duration::new(1, 0));

    graphql_test(&config);

    thread::sleep(Duration::new(1, 0));

    direct_udp_test(&config);
    */

    test_cleanup(&config);
}
