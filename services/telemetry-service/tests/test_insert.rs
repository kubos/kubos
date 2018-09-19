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

#[test]
fn test_insert_auto_timestamp() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8111;

    let (handle, sender) = setup(Some(db), Some(port), None);

    let mutation = r#"mutation {
            insert(timestamp: 5, subsystem: "test2", parameter: "voltage", value: "4.0") {
                success,
                errors
            }
        }"#;
    let mutation_expected = json!({
            "errs": "",
            "msg": {
                "insert": {
                    "errors": "",
                    "success": true
                }
            }
        });
    let mutation_result = do_query(Some(port), mutation);

    let query = r#"{
            telemetry(subsystem: "test2", parameter: "voltage") {
                timestamp,
                subsystem,
                parameter,
                value
            }
        }"#;
    let query_expected = json!({
            "errs": "",
            "msg": {
                "telemetry": [{
                    "timestamp": 5,
                    "subsystem": "test2",
                    "parameter": "voltage",
                    "value": "4.0"
                }]
            }
        });
    let query_result = do_query(Some(port), query);

    teardown(handle, sender);

    assert_eq!(mutation_result, mutation_expected);
    assert_eq!(query_result, query_expected);
}

#[test]
fn test_insert_custom_timestamp() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8112;

    let (handle, sender) = setup(Some(db), Some(port), None);

    let mutation = r#"mutation {
            insert(timestamp: 5, subsystem: "test2", parameter: "voltage", value: "4.0") {
                success,
                errors
            }
        }"#;
    let mutation_expected = json!({
            "errs": "",
            "msg": {
                "insert": {
                    "errors": "",
                    "success": true
                }
            }
        });
    let mutation_result = do_query(Some(port), mutation);

    let query = r#"{
            telemetry(subsystem: "test2", parameter: "voltage") {
                timestamp,
                subsystem,
                parameter,
                value
            }
        }"#;
    let query_expected = json!({
            "errs": "",
            "msg": {
                "telemetry": [{
                    "timestamp": 5,
                    "subsystem": "test2",
                    "parameter": "voltage",
                    "value": "4.0"
                }]
            }
        });
    let query_result = do_query(Some(port), query);

    teardown(handle, sender);

    assert_eq!(mutation_result, mutation_expected);
    assert_eq!(query_result, query_expected);
}

#[test]
fn test_insert_multi_auto() {
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.db");

    let db = db_path.to_str().unwrap();
    let port = 8113;

    let (handle, sender) = setup(Some(db), Some(port), None);

    let mutation_expected = json!({
            "errs": "",
            "msg": {
                "insert": {
                    "errors": "",
                    "success": true
                }
            }
        });

    // It currently takes more than 1ms for each round-trip GraphQL request,
    // so we're safe to fire these off one after another
    for num in 0..5 {
        let mutation = format!(
            r#"mutation {{
            insert(subsystem: "eps", parameter: "voltage", value: "4.{}") {{
                success,
                errors
            }}
        }}"#,
            num
        );

        let mutation_result = do_query(Some(port), &mutation);
        if mutation_result != mutation_expected {
            teardown(handle, sender);
            panic!();
        }
    }

    let query = r#"{
            telemetry(subsystem: "eps", parameter: "voltage") {
                subsystem,
                parameter,
                value
            }
        }"#;
    let query_expected = json!({
            "errs": "",
            "msg": {
                "telemetry": [
                {
                    "subsystem": "eps",
                    "parameter": "voltage",
                    "value": "4.4"
                },
                {
                    "subsystem": "eps",
                    "parameter": "voltage",
                    "value": "4.3"
                },
                {
                    "subsystem": "eps",
                    "parameter": "voltage",
                    "value": "4.2"
                },
                {
                    "subsystem": "eps",
                    "parameter": "voltage",
                    "value": "4.1"
                },
                {
                    "subsystem": "eps",
                    "parameter": "voltage",
                    "value": "4.0"
                },
                ]
            }
        });
    let query_result = do_query(Some(port), query);

    teardown(handle, sender);

    assert_eq!(query_result, query_expected);
}
