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
fn mutation_errors_empty() {
    let mut mock = mock_new!();
    mock.state = false;

    let service = service_new!(mock);

    let query = r#"mutation {
            errors
        }"#;

    let expected = json!({
            "errors": []
    });

    test!(service, query, expected);
}

#[test]
fn mutation_errors_single() {
    let mut mock = mock_new!();
    mock.state = false;

    let service = service_new!(mock);

    let noop = r#"mutation {
            noop {
                success
            }
        }"#;

    request!(service, noop);

    let query = r#"mutation {
            errors
        }"#;

    let expected = json!({
            "errors": ["watchdog_kick (services/isis-ants-service/src/model.rs:364): Configuration error"]
    });

    test!(service, query, expected);
}

#[test]
fn mutation_errors_multiple() {
    let mut mock = mock_new!();
    mock.state = false;

    let service = service_new!(mock);

    let noop = r#"mutation {
            noop {
                success
            }
        }"#;

    request!(service, noop);
    request!(service, noop);

    let query = r#"mutation {
            errors
        }"#;

    let expected = json!({
            "errors": ["watchdog_kick (services/isis-ants-service/src/model.rs:364): Configuration error", "watchdog_kick (services/isis-ants-service/src/model.rs:364): Configuration error"]
    });

    test!(service, query, expected);
}
