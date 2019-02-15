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
fn issue_raw_good_defaultresponse() {
    let mock = mock_new!();

    let service = service_new!(mock);

    let query = r#"mutation {
            issueRawCommand(command: \"C3C2\") {
                errors,
                response,
                success
            }
        }"#;

    let expected = json!({
            "issueRawCommand": {
                "errors": "",
                "response": "",
                "success": true
            }
    });

    test!(service, query, expected);
}

#[test]
fn issue_raw_good_noresponse() {
    let mock = mock_new!();

    let service = service_new!(mock);

    let query = r#"mutation {
            issueRawCommand(command: \"C3C2\", rxLen: 0) {
                errors,
                response,
                success
            }
        }"#;

    let expected = json!({
            "issueRawCommand": {
                "errors": "",
                "response": "",
                "success": true
            }
    });

    test!(service, query, expected);
}

#[test]
fn issue_raw_good_withresponse() {
    let mock = mock_new!();

    let service = service_new!(mock);

    let query = r#"mutation {
            issueRawCommand(command: \"C3C2\", rxLen: 2) {
                errors,
                response,
                success
            }
        }"#;

    let expected = json!({
            "issueRawCommand": {
                "errors": "",
                "response": "0001",
                "success": true
            }
    });

    test!(service, query, expected);
}

#[test]
fn issue_raw_bad() {
    let mut mock = mock_new!();
    mock.state = false;

    let service = service_new!(mock);

    let query = r#"mutation {
            issueRawCommand(command: \"C3C2\", rxLen: 2) {
                errors,
                response,
                success
            }
        }"#;

    let expected = json!({
            "issueRawCommand": {
                "errors": "Configuration error",
                "response": "",
                "success": false
            }
    });

    test!(service, query, expected);
}
