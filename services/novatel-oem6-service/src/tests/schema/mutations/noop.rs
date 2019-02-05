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
fn noop_good() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    let mut output = LOG_RESPONSE_GOOD.to_vec();
    output.extend_from_slice(&VERSION_LOG);
    mock.read.set_output(output);

    let service = service_new!(mock);

    let query = r#"mutation {
            noop {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "noop": {
                "errors": "",
                "success": true
            }
    });

    test!(service, query, expected);
}

#[test]
fn noop_no_response() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            noop {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "noop": {
               "errors": "Failed to get command response",
                "success": false
            }
    });

    test!(service, query, expected);
}

#[test]
fn noop_no_log() {
    let mut mock = MockStream::default();

    mock.write.set_input(LOG_VERSION_COMMAND.to_vec());

    mock.read.set_output(LOG_RESPONSE_GOOD.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            noop {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "noop": {
                "errors": "Failed to receive version info - timed out waiting on channel",
                "success": false
            }
    });

    test!(service, query, expected);
}
