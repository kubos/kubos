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

use serde_json::{json, Value};
mod utils;
use crate::utils::*;
use tempfile::TempDir;

static SQL: &'static str = r"
insert into telemetry values(1000, 'eps', 'voltage', '3.3');
insert into telemetry values(1000, 'eps', 'voltage5', '3.5');
insert into telemetry values(1001, 'eps', 'voltage', '3.4');
insert into telemetry values(1000, 'eps', 'current', '100');
insert into telemetry values(1500, 'eps', 'current', '150');
";

#[test]
fn test_parameter() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8111;
    let udp = 8121;

    let (handle, sender) = setup(db, Some(port), Some(udp), Some(SQL));

    let res = do_query(
        Some(port),
        "{telemetry(parameter: \"voltage\"){parameter,value}}",
    );
    teardown(handle, sender);
    assert_eq!(
        res,
        json!({
            "data": {
                "telemetry": [
                    {"parameter":"voltage","value":"3.4"},
                    {"parameter":"voltage","value":"3.3"}
                ]
            }
        })
    );
}

#[test]
fn test_parameters_multiple() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8112;
    let udp = 8122;

    let (handle, sender) = setup(db, Some(port), Some(udp), Some(SQL));

    let res = do_query(
        Some(port),
        r#"{
        telemetry(parameters: ["voltage", "current"]) {
            parameter,
            value
        }
    }"#,
    );
    teardown(handle, sender);
    assert_eq!(
        res,
        json!({
            "data": {
                "telemetry": [
                    {"parameter":"current","value":"150"},
                    {"parameter":"voltage","value":"3.4"},
                    {"parameter":"voltage","value":"3.3"},
                    {"parameter":"current","value":"100"}
                ]
            }
        })
    );
}

#[test]
fn test_parameters_single() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8113;
    let udp = 8123;

    let (handle, sender) = setup(db, Some(port), Some(udp), Some(SQL));

    let res = do_query(
        Some(port),
        r#"{
        telemetry(parameters: ["voltage"]) {
            parameter,
            value
        }
    }"#,
    );
    teardown(handle, sender);
    assert_eq!(
        res,
        json!({
            "data": {
                "telemetry": [
                    {"parameter":"voltage","value":"3.4"},
                    {"parameter":"voltage","value":"3.3"}
                ]
            }
        })
    );
}

#[test]
fn test_conflict() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8114;
    let udp = 8124;

    let (handle, sender) = setup(db, Some(port), Some(udp), Some(SQL));

    let res = do_query(
        Some(port),
        r#"{
        telemetry(parameter: "voltage", parameters: ["current"]) {
            parameter,
            value
        }
    }"#,
    );
    teardown(handle, sender);
    assert_eq!(
        res,
        json!({
                "data": Value::Null,
                "errors": [{
                    "locations": [{"column": 9, "line": 2}],
                    "message": "The `parameter` and `parameters` input fields are mutually exclusive",
                    "path": ["telemetry"]
                }]
        })
    );
}
