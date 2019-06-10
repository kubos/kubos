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

#[macro_use]
extern crate prettytable;

use kubos_system::Config;
use kubos_telemetry_db::{Database, Entry};
use prettytable::{Row, Table};
use rand::{thread_rng, Rng};
use serde_json::{json, ser};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::thread;
use std::time::Duration;
use time::PreciseTime;

const ITERATIONS: i64 = 1000;

fn db_insert_test(config: &Config) -> Row {
    let db_path = config
        .get("database")
        .expect("No database path found in config file");
    let db_path = db_path.as_str().unwrap();

    let db = Database::new(&db_path);
    db.setup();

    let mut times: Vec<i64> = Vec::new();

    for _ in 0..ITERATIONS {
        let timestamp: f64 = thread_rng().gen_range(0.0, 99999999999999999.9);

        let start = PreciseTime::now();
        if db
            .insert(timestamp, "db-test", "parameter", "value")
            .is_ok()
        {
            times.push(start.to(PreciseTime::now()).num_microseconds().unwrap());
        }
    }

    let num_entries = times.len() as i64;
    let sum: i64 = times.iter().sum();

    let average = sum / num_entries;

    row!["DB API insert (local)", average, sum]
}

fn db_insert_bulk_test(config: &Config) -> Row {
    let db_path = config
        .get("database")
        .expect("No database path found in config file");
    let db_path = db_path.as_str().unwrap();

    let db = Database::new(&db_path);
    db.setup();

    let mut entries: Vec<Entry> = Vec::new();

    for _ in 0..ITERATIONS {
        let timestamp: f64 = thread_rng().gen_range(0.0, 99999999999999999.9);

        entries.push(Entry {
            timestamp,
            subsystem: "db-test".to_string(),
            parameter: "parameter".to_string(),
            value: "value".to_string(),
        });
    }

    let start = PreciseTime::now();
    let end = match db.insert_bulk(entries) {
        Ok(_) => start.to(PreciseTime::now()).num_microseconds().unwrap(),
        Err(e) => panic!("insert_bulk function failed: {:?}", e),
    };

    row!["DB API insert_bulk (local)", end / ITERATIONS, end]
}

fn graphql_insert_test(config: &Config) -> Row {
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

        let start = PreciseTime::now();

        let client = reqwest::Client::builder().build().unwrap();

        let uri = format!("http://{}", config.hosturl());

        let mut map = ::std::collections::HashMap::new();
        map.insert("query", mutation);

        match client.post(&uri).json(&map).send() {
            Ok(_) => times.push(start.to(PreciseTime::now()).num_microseconds().unwrap()),
            Err(e) => panic!("recv function failed: {:?}", e),
        }
    }

    let num_entries = times.len() as i64;
    let sum: i64 = times.iter().sum();

    let average = sum / num_entries;

    row!["GQL insert (remote)", average, sum]
}

fn graphql_insert_bulk_test(config: &Config) -> Row {
    let mut bulk_entries = String::from("[");
    for i in 0..ITERATIONS {
        let mut rng = thread_rng();
        let timestamp = rng.gen_range(0, ::std::i32::MAX);
        let next = if i < ITERATIONS - 1 { "," } else { "]" };

        let entry = format!(
            r#"{{ timestamp: {}, subsystem: "db-test", parameter: "voltage", value: "5.0" }}{}"#,
            timestamp, next
        );
        bulk_entries.push_str(&entry);
    }

    let mutation = format!(
        r#"
        mutation {{
            insertBulk(entries: {}) {{
                success,
                errors
            }}
        }}"#,
        bulk_entries
    );

    let start = PreciseTime::now();
    let client = reqwest::Client::builder().build().unwrap();
    let uri = format!("http://{}", config.hosturl());
    let mut map = ::std::collections::HashMap::new();
    map.insert("query", mutation);

    let end = match client.post(&uri).json(&map).send() {
        Ok(_) => start.to(PreciseTime::now()).num_microseconds().unwrap(),
        Err(e) => panic!("recv function failed: {:?}", e),
    };

    row!["GQL insertBulk (remote)", end / ITERATIONS, end]
}

fn direct_udp_test(config: &Config) -> Row {
    let mut times: Vec<i64> = Vec::new();

    let port = config.get("direct_port").unwrap();

    let host = config.hosturl().to_owned();
    let ip: Vec<&str> = host.split(':').collect();

    let remote_addr = format!("{}:{}", ip[0], port);

    let local_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);

    let socket = UdpSocket::bind(local_addr).expect("Couldn't bind to address");

    let message = json!({
        "subsystem": "db-test",
        "parameter": "voltage",
        "value": "3.3"
    });

    for _ in 0..ITERATIONS {
        let start = PreciseTime::now();

        socket
            .send_to(&ser::to_vec(&message).unwrap(), &remote_addr)
            .unwrap();

        times.push(start.to(PreciseTime::now()).num_microseconds().unwrap());

        thread::sleep(Duration::from_millis(2));
    }

    let num_entries = times.len() as i64;
    let sum: i64 = times.iter().sum();

    let average = sum / num_entries;

    row!["UDP insert (remote)", average, sum]
}

fn test_cleanup(config: &Config) {
    let mutation = r#"mutation {
            delete(subsystem: "db-test") {
                success,
                errors,
                entriesDeleted
            }
        }"#;

    let client = reqwest::Client::builder().build().unwrap();

    let uri = format!("http://{}", config.hosturl());

    let mut map = ::std::collections::HashMap::new();
    map.insert("query", mutation);

    let result: serde_json::Value = client.post(&uri).json(&map).send().unwrap().json().unwrap();

    match result.get("data").and_then(|msg| msg.get("delete")) {
        Some(message) => {
            let success = serde_json::from_value::<bool>(message["success"].clone()).unwrap();

            let errors = serde_json::from_value::<String>(message["errors"].clone()).unwrap();

            let entries_deleted =
                serde_json::from_value::<i64>(message["entriesDeleted"].clone()).unwrap();

            if success {
                println!("Cleaned up {} test entries", entries_deleted);
            } else {
                eprintln!("Failed to deleted test entries: {}", errors);
            }
        }
        None => eprintln!("Failed to process delete response"),
    }
}

fn main() {
    let config = Config::new("telemetry-service");
    let mut table = Table::new();

    table.add_row(row!["NAME", "Avg us per-entry", "Total us"]);
    table.add_row(db_insert_test(&config));
    table.add_row(db_insert_bulk_test(&config));

    // This sleep likely isn't necessary, but I'd like to make extra sure nothing about a test
    // lingers to affect the next one
    thread::sleep(Duration::new(1, 0));

    table.add_row(graphql_insert_test(&config));

    thread::sleep(Duration::new(1, 0));

    table.add_row(graphql_insert_bulk_test(&config));

    thread::sleep(Duration::new(1, 0));

    table.add_row(direct_udp_test(&config));

    thread::sleep(Duration::new(1, 0));

    test_cleanup(&config);

    table.printstd();
}
