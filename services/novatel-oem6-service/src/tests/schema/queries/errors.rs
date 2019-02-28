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
use serde_json::json;

#[test]
fn query_errors_empty() {
    let mut mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"{
            errors
        }"#;

    let expected = json!({
            "errors": []
    });

    test!(service, query, expected);
}

#[test]
fn query_errors_local_single() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    mock.read.set_output(LOG_RESPONSE_GOOD.to_vec());

    let service = service_new!(mock);

    let noop = r#"mutation {
            noop {
                success
            }
        }"#;

    request!(service, noop);

    let query = r#"{
            errors
        }"#;

    let expected = json!({
            "errors": ["Noop: Failed to receive version info - timed out waiting on channel"]
    });

    test!(service, query, expected);
}

#[test]
fn query_errors_local_multiple() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    let mut output = LOG_RESPONSE_GOOD.to_vec();
    output.extend_from_slice(&LOG_RESPONSE_GOOD);
    mock.read.set_output(output);

    let service = service_new!(mock);

    let noop = r#"mutation {
            noop {
                success
            }
        }"#;

    request!(service, noop);
    request!(service, noop);

    let query = r#"{
            errors
        }"#;

    let expected = json!({
            "errors": ["Noop: Failed to receive version info - timed out waiting on channel", "Noop: UART Error, Generic Error"]
    });

    test!(service, query, expected);
}

#[test]
fn query_errors_device_single() {
    let mut mock = MockStream::default();

    mock.read.set_output(ERROR_LOG.to_vec());

    let service = service_new!(mock);

    let query = r#"{
            errors
        }"#;

    let expected = json!({
            "errors": ["RxStatusEvent(1, 19, 1): No Valid Position Calculated"]
    });

    test!(service, query, expected);
}

#[test]
fn query_errors_device_multiple() {
    let mut mock = MockStream::default();

    let mut output = ERROR_LOG.to_vec();
    output.extend_from_slice(&ERROR_LOG);
    mock.read.set_output(output);

    let service = service_new!(mock);

    let query = r#"{
            errors
        }"#;

    let expected = json!({
            "errors": ["RxStatusEvent(1, 19, 1): No Valid Position Calculated", "RxStatusEvent(1, 19, 1): No Valid Position Calculated"]
    });

    test!(service, query, expected);
}

#[test]
fn query_errors_mixed() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    let mut output = LOG_RESPONSE_GOOD.to_vec();
    output.extend_from_slice(&ERROR_LOG);
    mock.read.set_output(output);

    let service = service_new!(mock);

    let noop = r#"mutation {
            noop {
                success
            }
        }"#;

    request!(service, noop);

    let query = r#"{
            errors
        }"#;

    let expected = json!({
            "errors": ["Noop: Failed to receive version info - timed out waiting on channel", "RxStatusEvent(1, 19, 1): No Valid Position Calculated"]
    });

    test!(service, query, expected);
}

#[test]
fn query_errors_clear_after_query() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    mock.read.set_output(LOG_RESPONSE_GOOD.to_vec());

    let service = service_new!(mock);

    let noop = r#"mutation {
            noop {
                success
            }
        }"#;

    request!(service, noop);

    let query = r#"{
            errors
        }"#;

    request!(service, query);

    let expected = json!({
            "errors": []
    });

    test!(service, query, expected);
}
