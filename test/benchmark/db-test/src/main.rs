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

use getopts::Options;

use kubos_system::{Config, DEFAULT_PATH};
use kubos_telemetry_db::{Database, Entry};
use rand::{thread_rng, Rng};
use serde_json::{json, ser};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::time::Duration;
use std::{env, thread};
use time::PreciseTime;

const DEFAULT_ITERATIONS: i64 = 1000;
const TEST_NAME_MAX_COLS: usize = 30;
const TEST_NUM_MAX_COLS: usize = 10;

fn pad(s: &str, cols: usize) -> String {
    if s.len() < cols {
        let mut s2 = String::from(s);
        s2.push_str(&" ".repeat(cols - s.len()));
        return s2;
    }
    s.to_string()
}

fn pad_name(name: &str) -> String {
    pad(name, TEST_NAME_MAX_COLS)
}

fn pad_num<T: ToString>(val: T) -> String {
    pad(&val.to_string(), TEST_NUM_MAX_COLS)
}

struct DbTest {
    iterations: i64,
    config: Config,
}

struct PerfResult {
    name: String,
    avg_us: i64,
    total_us: i64,
}

impl PerfResult {
    fn new(name: &str, avg_us: i64, total_us: i64) -> PerfResult {
        PerfResult {
            name: name.to_string(),
            avg_us,
            total_us,
        }
    }

    fn print(&self) {
        println!(
            "{} | {} | {}",
            pad_name(&self.name),
            pad_num(self.avg_us),
            pad_num(self.total_us)
        );
    }
}

impl DbTest {
    fn new(iterations: i64, config_path: String) -> DbTest {
        DbTest {
            iterations,
            config: Config::new_from_path("telemetry-service", config_path).unwrap(),
        }
    }

    fn db_insert_test(&self) -> PerfResult {
        let db_path = self
            .config
            .get("database")
            .expect("No database path found in config file");
        let db_path = db_path.as_str().unwrap();

        let db = Database::new(&db_path);
        db.setup();

        let mut times: Vec<i64> = Vec::new();

        for _ in 0..self.iterations {
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

        PerfResult::new("local_db_api_insert", average, sum)
    }

    fn db_insert_bulk_test(&self) -> PerfResult {
        let db_path = self
            .config
            .get("database")
            .expect("No database path found in config file");
        let db_path = db_path.as_str().unwrap();

        let db = Database::new(&db_path);
        db.setup();

        let mut entries: Vec<Entry> = Vec::new();

        for _ in 0..self.iterations {
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

        PerfResult::new("local_db_api_insert_bulk", end / self.iterations, end)
    }

    fn graphql_insert_test(&self) -> PerfResult {
        let mut times: Vec<i64> = Vec::new();

        for _ in 0..self.iterations {
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

            let uri = format!("http://{}", self.config.hosturl());

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

        PerfResult::new("remote_gql_insert", average, sum)
    }

    fn graphql_insert_bulk_test(&self) -> PerfResult {
        let mut bulk_entries = String::from("[");
        for i in 0..self.iterations {
            let mut rng = thread_rng();
            let timestamp = rng.gen_range(0, ::std::i32::MAX);
            let next = if i < self.iterations - 1 { "," } else { "]" };

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
        let uri = format!("http://{}", self.config.hosturl());
        let mut map = ::std::collections::HashMap::new();
        map.insert("query", mutation);

        let end = match client.post(&uri).json(&map).send() {
            Ok(_) => start.to(PreciseTime::now()).num_microseconds().unwrap(),
            Err(e) => panic!("recv function failed: {:?}", e),
        };

        PerfResult::new("remote_gql_insert_bulk", end / self.iterations, end)
    }

    fn direct_udp_test(&self) -> PerfResult {
        let mut times: Vec<i64> = Vec::new();

        let port = self.config.get("direct_port").unwrap();

        let host = self.config.hosturl().to_owned();
        let ip: Vec<&str> = host.split(':').collect();

        let remote_addr = format!("{}:{}", ip[0], port);

        let local_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);

        let socket = UdpSocket::bind(local_addr).expect("Couldn't bind to address");

        let message = json!({
            "subsystem": "db-test",
            "parameter": "voltage",
            "value": "3.3"
        });

        for _ in 0..self.iterations {
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

        PerfResult::new("remote_udp_insert", average, sum)
    }

    fn test_cleanup(&self) {
        let mutation = r#"mutation {
                delete(subsystem: "db-test") {
                    success,
                    errors,
                    entriesDeleted
                }
            }"#;

        let client = reqwest::Client::builder().build().unwrap();

        let uri = format!("http://{}", self.config.hosturl());

        let mut map = ::std::collections::HashMap::new();
        map.insert("query", mutation);

        let result: serde_json::Value =
            client.post(&uri).json(&map).send().unwrap().json().unwrap();

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
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt(
        "i",
        "iterations",
        &format!(
            "number of iterations (or entries) to insert. default is {}",
            DEFAULT_ITERATIONS
        ),
        "N",
    );
    opts.optopt("c", "config", "Path to config file", "CONFIG");

    let mut iterations = DEFAULT_ITERATIONS;
    let mut config = DEFAULT_PATH.to_string();

    if let Ok(matches) = opts.parse(&args[1..]) {
        iterations = matches
            .opt_str("i")
            .map(|iter| iter.parse::<i64>().unwrap_or(DEFAULT_ITERATIONS))
            .unwrap_or(DEFAULT_ITERATIONS);
        config = matches
            .opt_str("c")
            .unwrap_or_else(|| DEFAULT_PATH.to_string());
    };

    let db_test = DbTest::new(iterations, config);

    println!(
        "{} | {} | {}",
        pad_name("NAME"),
        pad_num("Avg (us)"),
        pad_num("Total (us)")
    );
    println!(
        "{}",
        "-".repeat(TEST_NAME_MAX_COLS + (TEST_NUM_MAX_COLS * 2) + 6)
    );

    db_test.db_insert_test().print();
    db_test.db_insert_bulk_test().print();

    // This sleep likely isn't necessary, but I'd like to make extra sure nothing about a test
    // lingers to affect the next one
    thread::sleep(Duration::new(1, 0));

    db_test.graphql_insert_test().print();

    thread::sleep(Duration::new(1, 0));

    db_test.graphql_insert_bulk_test().print();

    thread::sleep(Duration::new(1, 0));

    db_test.direct_udp_test().print();

    thread::sleep(Duration::new(1, 0));

    db_test.test_cleanup();
}
