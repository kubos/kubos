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
fn control_power_good() {
    let mut mock = mock_new!();
    mock.state = true;
    
    let service = service_new!(mock);

    let query = r#"mutation {
            controlPower(state: RESET) {
                errors,
                power,
                success
            }
        }"#;

    let expected = json!({
            "controlPower": {
                "errors": "",
                "power": "RESET",
                "success": true
            }
    });

    test!(service, query, expected);
}

#[test]
fn control_power_bad() {
    let mut mock = mock_new!();
    mock.state = false;

    let service = service_new!(mock);

    let query = r#"mutation {
            controlPower(state: RESET) {
                errors,
                power,
                success
            }
        }"#;

    let expected = json!({
            "controlPower": {
                "errors": "Configuration error",
                "power": "RESET",
                "success": false
            }
    });

    test!(service, query, expected);
}

#[test]
fn control_power_invalid() {
    let mock = mock_new!();
    let service = service_new!(mock);

    let query = r#"mutation {
            controlPower(state: ON) {
                errors,
                power,
                success
            }
        }"#;

    let expected = json!({
            "controlPower": {
                "errors": "Invalid power state",
                "power": "ON",
                "success": false
            }
    });

    test!(service, query, expected);
}
