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

use super::*;

#[test]
fn query_errors_empty() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_bad_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let query = r#"{
            errors
        }"#;

    let expected = json!({
            "errors": []
    });

    test!(service, query, expected);
}

#[test]
fn query_errors_single() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_bad_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let reset = r#"mutation {
        resetWatchdog {
            success
        }
    }"#;

    request!(service, reset);

    let query = r#"{
            errors
        }"#;

    // The resulting error containg a line number, which changes every time this crate changes.
    // We'll verify this result by parts in order to avoid having to update the test repeatedly
    let result: Vec<u8> = request!(service, query)
        .body()
        .iter()
        .map(|entry| *entry)
        .collect();
    let result: serde_json::Value =
        serde_json::from_str(::std::str::from_utf8(&result).unwrap()).unwrap();

    let data = &result["data"]["errors"][0].as_str().unwrap();

    assert!(data
        .contains("reset_comms_watchdog (services/clyde-3g-eps-service/src/models/subsystem.rs"));
    assert!(data.ends_with("Generic Error"));
}

#[test]
fn query_errors_multiple() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_bad_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let reset = r#"mutation {
            resetWatchdog {
                success
            }
        }"#;

    request!(service, reset);
    request!(service, reset);

    let query = r#"{
            errors
        }"#;

    let result: Vec<u8> = request!(service, query)
        .body()
        .iter()
        .map(|entry| *entry)
        .collect();
    let result: serde_json::Value =
        serde_json::from_str(::std::str::from_utf8(&result).unwrap()).unwrap();

    for index in 0..2 {
        let data = &result["data"]["errors"][index].as_str().unwrap();

        assert!(data.contains(
            "reset_comms_watchdog (services/clyde-3g-eps-service/src/models/subsystem.rs"
        ));
        assert!(data.ends_with("Generic Error"));
    }
}

#[test]
fn query_errors_clear_after_query() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_bad_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let reset = r#"mutation {
            resetWatchdog {
                success
            }
        }"#;

    request!(service, reset);

    let query = r#"{
            errors
        }"#;

    request!(service, query);

    let expected = json!({
            "errors": []
    });

    test!(service, query, expected);
}
