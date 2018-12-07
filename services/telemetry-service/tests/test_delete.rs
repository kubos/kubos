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
extern crate serde_json;
extern crate tempfile;

mod utils;

use tempfile::TempDir;
use utils::*;

static SQL: &'static str = r"
insert into telemetry values(1000, 'eps', 'voltage', '3.3');
insert into telemetry values(1001, 'mcu', 'voltage', '3.4');
insert into telemetry values(1002, 'gps', 'voltage', '3.2');
insert into telemetry values(1003, 'eps', 'current', '3.1');
insert into telemetry values(1004, 'mcu', 'current', '3.0');
insert into telemetry values(1005, 'gps', 'current', '2.9');
insert into telemetry values(1006, 'eps', 'voltage', '2.8');
insert into telemetry values(1007, 'mcu', 'voltage', '2.7');
insert into telemetry values(1008, 'gps', 'voltage', '2.6');
insert into telemetry values(1009, 'eps', 'current', '2.5');
insert into telemetry values(1010, 'mcu', 'current', '2.4');
";

#[test]
fn test_delete_ge() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8112;
    let udp = 8122;

    let (handle, sender) = setup(Some(db), Some(port), Some(udp), Some(SQL));

    let mutation = r#"mutation {
            delete(timestampGe: 1004) {
                success,
                errors,
                entriesDeleted
            }
        }"#;

    let mutation_expected = json!({
            "errors": "",
            "data": {
                "delete": {
                    "entriesDeleted": 7,
                    "errors": "",
                    "success": true
                }
            }
        });
    let mutation_result = do_query(Some(port), mutation);

    let query = r#"{
            telemetry {
                timestamp,
                subsystem,
                parameter,
                value
            }
        }"#;
    let query_expected = json!({
            "errors": "",
            "data": {
                "telemetry": [
                    {
                    "timestamp": 1003.0,
                    "subsystem": "eps",
                    "parameter": "current",
                    "value": "3.1"
                    },
                    {
                    "timestamp": 1002.0,
                    "subsystem": "gps",
                    "parameter": "voltage",
                    "value": "3.2"
                    },
                    {
                    "timestamp": 1001.0,
                    "subsystem": "mcu",
                    "parameter": "voltage",
                    "value": "3.4"
                    },
                    {
                    "timestamp": 1000.0,
                    "subsystem": "eps",
                    "parameter": "voltage",
                    "value": "3.3"
                    },
                ]
            }
        });
    let query_result = do_query(Some(port), query);

    teardown(handle, sender);

    assert_eq!(mutation_result, mutation_expected);
    assert_eq!(query_result, query_expected);
}

#[test]
fn test_delete_le() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8113;
    let udp = 8123;

    let (handle, sender) = setup(Some(db), Some(port), Some(udp), Some(SQL));

    let mutation = r#"mutation {
            delete(timestampLe: 1008) {
                success,
                errors,
                entriesDeleted
            }
        }"#;

    let mutation_expected = json!({
            "errors": "",
            "data": {
                "delete": {
                    "entriesDeleted": 9,
                    "errors": "",
                    "success": true
                }
            }
        });
    let mutation_result = do_query(Some(port), mutation);

    let query = r#"{
            telemetry {
                timestamp,
                subsystem,
                parameter,
                value
            }
        }"#;
    let query_expected = json!({
            "errors": "",
            "data": {
                "telemetry": [
                    {
                    "timestamp": 1010.0,
                    "subsystem": "mcu",
                    "parameter": "current",
                    "value": "2.4"
                    },
                    {
                    "timestamp": 1009.0,
                    "subsystem": "eps",
                    "parameter": "current",
                    "value": "2.5"
                    },
                ]
            }
        });
    let query_result = do_query(Some(port), query);

    teardown(handle, sender);

    assert_eq!(mutation_result, mutation_expected);
    assert_eq!(query_result, query_expected);
}

#[test]
fn test_delete_range() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8114;
    let udp = 8124;

    let (handle, sender) = setup(Some(db), Some(port), Some(udp), Some(SQL));

    let mutation = r#"mutation {
            delete(timestampGe: 1001, timestampLe: 1009) {
                success,
                errors,
                entriesDeleted
            }
        }"#;

    let mutation_expected = json!({
            "errors": "",
            "data": {
                "delete": {
                    "entriesDeleted": 9,
                    "errors": "",
                    "success": true
                }
            }
        });
    let mutation_result = do_query(Some(port), mutation);

    let query = r#"{
            telemetry {
                timestamp,
                subsystem,
                parameter,
                value
            }
        }"#;
    let query_expected = json!({
            "errors": "",
            "data": {
                "telemetry": [
                    {
                    "timestamp": 1010.0,
                    "subsystem": "mcu",
                    "parameter": "current",
                    "value": "2.4"
                    },
                    {
                    "timestamp": 1000.0,
                    "subsystem": "eps",
                    "parameter": "voltage",
                    "value": "3.3"
                    },
                ]
            }
        });
    let query_result = do_query(Some(port), query);

    teardown(handle, sender);

    assert_eq!(mutation_result, mutation_expected);
    assert_eq!(query_result, query_expected);
}

#[test]
fn test_delete_subsystem() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8115;
    let udp = 8125;

    let (handle, sender) = setup(Some(db), Some(port), Some(udp), Some(SQL));

    let mutation = r#"mutation {
            delete(subsystem: "eps") {
                success,
                errors,
                entriesDeleted
            }
        }"#;

    let mutation_expected = json!({
            "errors": "",
            "data": {
                "delete": {
                    "entriesDeleted": 4,
                    "errors": "",
                    "success": true
                }
            }
        });
    let mutation_result = do_query(Some(port), mutation);

    let query = r#"{
            telemetry {
                timestamp,
                subsystem,
            }
        }"#;
    let query_expected = json!({
            "errors": "",
            "data": {
                "telemetry": [
                    {
                    "timestamp": 1010.0,
                    "subsystem": "mcu",
                    },
                    {
                    "timestamp": 1008.0,
                    "subsystem": "gps",
                    },
                    {
                    "timestamp": 1007.0,
                    "subsystem": "mcu",
                    },
                    {
                    "timestamp": 1005.0,
                    "subsystem": "gps",
                    },
                    {
                    "timestamp": 1004.0,
                    "subsystem": "mcu",
                    },
                    {
                    "timestamp": 1002.0,
                    "subsystem": "gps",
                    },
                    {
                    "timestamp": 1001.0,
                    "subsystem": "mcu",
                    },
                ]
            }
        });
    let query_result = do_query(Some(port), query);

    teardown(handle, sender);

    assert_eq!(mutation_result, mutation_expected);
    assert_eq!(query_result, query_expected);
}

#[test]
fn test_delete_parameter() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8116;
    let udp = 8126;

    let (handle, sender) = setup(Some(db), Some(port), Some(udp), Some(SQL));

    let mutation = r#"mutation {
            delete(parameter: "voltage") {
                success,
                errors,
                entriesDeleted
            }
        }"#;

    let mutation_expected = json!({
            "errors": "",
            "data": {
                "delete": {
                    "entriesDeleted": 6,
                    "errors": "",
                    "success": true
                }
            }
        });
    let mutation_result = do_query(Some(port), mutation);

    let query = r#"{
            telemetry {
                timestamp,
                subsystem,
            }
        }"#;
    let query_expected = json!({
            "errors": "",
            "data": {
                "telemetry": [
                    {
                    "timestamp": 1010.0,
                    "subsystem": "mcu",
                    },
                    {
                    "timestamp": 1009.0,
                    "subsystem": "eps",
                    },
                    {
                    "timestamp": 1005.0,
                    "subsystem": "gps",
                    },
                    {
                    "timestamp": 1004.0,
                    "subsystem": "mcu",
                    },
                    {
                    "timestamp": 1003.0,
                    "subsystem": "eps",
                    },
                ]
            }
        });
    let query_result = do_query(Some(port), query);

    teardown(handle, sender);

    assert_eq!(mutation_result, mutation_expected);
    assert_eq!(query_result, query_expected);
}
