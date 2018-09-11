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

mod utils;
use utils::*;

#[test]
fn tests() {
    let (handle, sender) = setup(None);

    // Test 1: Insert without specifying a timestamp (it will be auto-generated)
    let auto_timestamp_mutation = r#"mutation {
            insert(subsystem: "test1", parameter: "voltage", value: "4.0") {
                success,
                errors
            }
        }"#;
    let auto_timestamp_mutation_expected = json!({
            "errs": "",
            "msg": {
                "insert": {
                    "errors": "",
                    "success": true
                }
            }
        });
    let auto_timestamp_mutation_result = do_query(auto_timestamp_mutation);

    let auto_timestamp_query = r#"{
            telemetry(subsystem: "test1", parameter: "voltage") {
                subsystem,
                parameter,
                value
            }
        }"#;
    let auto_timestamp_query_expected = json!({
            "errs": "",
            "msg": {
                "telemetry": [{
                    "subsystem": "test1",
                    "parameter": "voltage",
                    "value": "4.0"
                }]
            }
        });
    let auto_timestamp_query_result = do_query(auto_timestamp_query);

    // Test 2: Insert with a custom timestamp
    let custom_timestamp_mutation = r#"mutation {
            insert(timestamp: 5, subsystem: "test2", parameter: "voltage", value: "4.0") {
                success,
                errors
            }
        }"#;
    let custom_timestamp_mutation_expected = json!({
            "errs": "",
            "msg": {
                "insert": {
                    "errors": "",
                    "success": true
                }
            }
        });
    let custom_timestamp_mutation_result = do_query(custom_timestamp_mutation);

    let custom_timestamp_query = r#"{
            telemetry(subsystem: "test2", parameter: "voltage") {
                timestamp,
                subsystem,
                parameter,
                value
            }
        }"#;
    let custom_timestamp_query_expected = json!({
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
    let custom_timestamp_query_result = do_query(custom_timestamp_query);

    teardown(handle, sender);
    // Test 1 Verification
    assert_eq!(
        auto_timestamp_mutation_result,
        auto_timestamp_mutation_expected
    );
    assert_eq!(auto_timestamp_query_result, auto_timestamp_query_expected);
    // Test 2 Verification
    assert_eq!(
        custom_timestamp_mutation_result,
        custom_timestamp_mutation_expected
    );
    assert_eq!(
        custom_timestamp_query_result,
        custom_timestamp_query_expected
    );
}
