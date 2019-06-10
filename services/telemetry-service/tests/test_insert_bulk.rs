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
mod utils;

use crate::utils::*;
use serde_json::json;
use tempfile::TempDir;

macro_rules! test_mutation_query_results {
    ($port_offset: expr, $mutation:expr, $mutation_expected:tt, $query:expr, $query_expected:tt) => {
        let db_dir = TempDir::new().unwrap();
        let db_path = db_dir.path().join("test.db");

        let db = db_path.to_str().unwrap();
        let port: u16 = 8110 + $port_offset;
        let udp: u16 = 8210 + $port_offset;

        let (handle, sender) = setup(Some(db), Some(port), Some(udp), None);
        let mutation_expected = json!($mutation_expected);
        let mutation_result = do_query(Some(port), $mutation);
        let query_expected = json!($query_expected);

        let query_result = do_query(Some(port), $query);
        teardown(handle, sender);

        assert_eq!(mutation_result, mutation_expected);
        assert_eq!(query_result, query_expected);
    };
}

#[test]
fn test_insert_bulk() {
    test_mutation_query_results!(1,
    r#"mutation {
        insertBulk(
            entries: [
                { subsystem: "test2", parameter: "voltage", value: "4.0" },
                { subsystem: "test2", parameter: "amps", value: "0.3" },
                { subsystem: "test2", parameter: "cpu", value: "85.1" }
            ])
        {
            success,
            errors
        }
    }"#,
    {
        "data": {
            "insertBulk": {
                "errors": "",
                "success": true
            }
        }
    },
    r#"{
        telemetry(subsystem: "test2") {
            subsystem,
            parameter,
            value
        }
    }"#,
    {
        "data": {
            "telemetry": [{
                "subsystem": "test2",
                "parameter": "voltage",
                "value": "4.0"
            }, {
                "subsystem": "test2",
                "parameter": "cpu",
                "value": "85.1"
            }, {
                "subsystem": "test2",
                "parameter": "amps",
                "value": "0.3"
            }]
        }
    });
}

#[test]
fn test_insert_bulk_timestamp() {
    test_mutation_query_results!(2,
    r#"mutation {
        insertBulk(
            timestamp: 100,
            entries: [
                { subsystem: "test2", parameter: "voltage", value: "4.0" },
                { subsystem: "test2", parameter: "amps", value: "0.3" },
                { timestamp: 199, subsystem: "test2", parameter: "cpu", value: "85.1" }
            ])
        {
            success,
            errors
        }
    }"#,
    {
        "data": {
            "insertBulk": {
                "errors": "",
                "success": true
            }
        }
    },
    r#"{
        telemetry(subsystem: "test2") {
            timestamp,
            subsystem,
            parameter,
            value
        }
    }"#,
    {
        "data": {
            "telemetry": [{
                "timestamp": 199.0,
                "subsystem": "test2",
                "parameter": "cpu",
                "value": "85.1"
            }, {
                "timestamp": 100.0,
                "subsystem": "test2",
                "parameter": "voltage",
                "value": "4.0"
            }, {
                "timestamp": 100.0,
                "subsystem": "test2",
                "parameter": "amps",
                "value": "0.3"
            }]
        }
    });
}
