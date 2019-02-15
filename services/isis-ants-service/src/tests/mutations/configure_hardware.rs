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
fn configure_good_primary() {
    let mut mock = mock_new!();
    mock.state = true;

    let service = service_new!(mock);

    let query = r#"mutation {
            configureHardware(config: PRIMARY) {
                config,
                errors,
                success
            }
        }"#;

    let expected = json!({
            "configureHardware": {
                "config": "PRIMARY",
                "errors": "",
                "success": true
            }
    });

    test!(service, query, expected);
}

#[test]
fn configure_good_secondary() {
    let mut mock = mock_new!();
    mock.state = true;

    let service = service_new!(mock);

    let query = r#"mutation {
            configureHardware(config: SECONDARY) {
                config,
                errors,
                success
            }
        }"#;

    let expected = json!({
            "configureHardware": {
                "config": "SECONDARY",
                "errors": "",
                "success": true
            }
    });

    test!(service, query, expected);
}

#[test]
fn configure_bad() {
    let mut mock = mock_new!();
    mock.state = false;

    let service = service_new!(mock);

    let query = r#"mutation {
            configureHardware(config: SECONDARY) {
                config,
                errors,
                success
            }
        }"#;

    let expected = json!({
            "configureHardware": {
                "config": "SECONDARY",
                "errors": "Configuration error",
                "success": false
            }
    });

    test!(service, query, expected);
}
