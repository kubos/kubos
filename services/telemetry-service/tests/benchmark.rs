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

extern crate cbor_protocol;
#[macro_use]
extern crate criterion;
extern crate rand;
#[macro_use]
extern crate serde_json;
extern crate tempfile;

mod utils;

use criterion::{Criterion, Fun};
use rand::{thread_rng, Rng};
use serde_json::ser;
use std::net::UdpSocket;
use std::time::Duration;
use tempfile::TempDir;
use utils::*;

fn query(time: i64) {
    let mutation = format!(
        r#"
        mutation {{
            insert(timestamp: {}, subsystem: "gps", parameter: "voltage", value: "4.0") {{
                success,
                errors
            }}
        }}
        "#,
        time
    );

    do_query(None, &mutation);
}

fn direct(time: i64) {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let service = "0.0.0.0:8000";

    let message = json!({
            "timestamp": time,
            "subsystem": "eps",
            "parameter": "voltage",
            "value": "3.3"
    });
    socket
        .send_to(&ser::to_vec(&message).unwrap(), service)
        .unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();

    let (handle, sender) = setup(Some(db), None, Some(8000), None);

    c.bench_function("direct", |b| {
        b.iter_with_setup(
            || {
                let mut rng = thread_rng();
                rng.gen_range(0, ::std::i32::MAX) as i64
            },
            |time| direct(time),
        )
    });

    c.bench_function("query", |b| {
        b.iter_with_setup(
            || {
                let mut rng = thread_rng();
                rng.gen_range(0, ::std::i32::MAX) as i64
            },
            |time| query(time),
        )
    });

    teardown(handle, sender);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
