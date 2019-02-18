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
fn deploy_good_default() {
    let mut mock = mock_new!();
    mock.state = true;

    let service = service_new!(mock);

    let query = r#"mutation {
            deploy(time: 5) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "deploy": {
                "errors": "",
                "success": true
            }
    });

    test!(service, query, expected);
}

#[test]
fn deploy_good_all() {
    let mut mock = mock_new!();
    mock.state = true;

    let service = service_new!(mock);

    let query = r#"mutation {
            deploy(ant: ALL, time: 5) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "deploy": {
                "errors": "",
                "success": true
            }
    });

    test!(service, query, expected);
}

#[test]
fn deploy_good_ant1() {
    let mut mock = mock_new!();
    mock.state = true;

    let service = service_new!(mock);

    let query = r#"mutation {
            deploy(ant: ANTENNA1, time: 5) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "deploy": {
                "errors": "",
                "success": true
            }
    });

    test!(service, query, expected);
}

#[test]
fn deploy_good_ant1_override() {
    let mut mock = mock_new!();
    mock.state = true;

    let service = service_new!(mock);

    let query = r#"mutation {
            deploy(ant: ANTENNA1, force: true, time: 5) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "deploy": {
                "errors": "",
                "success": true
            }
    });

    test!(service, query, expected);
}

#[test]
fn deploy_good_ant2() {
    let mut mock = mock_new!();
    mock.state = true;

    let service = service_new!(mock);

    let query = r#"mutation {
            deploy(ant: ANTENNA2, time: 5) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "deploy": {
                "errors": "",
                "success": true
            }
    });

    test!(service, query, expected);
}

#[test]
fn deploy_good_ant2_override() {
    let mut mock = mock_new!();
    mock.state = true;

    let service = service_new!(mock);

    let query = r#"mutation {
            deploy(ant: ANTENNA2, force: true, time: 5) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "deploy": {
                "errors": "",
                "success": true
            }
    });

    test!(service, query, expected);
}

#[test]
fn deploy_good_ant3() {
    let mut mock = mock_new!();
    mock.state = true;

    let service = service_new!(mock);

    let query = r#"mutation {
            deploy(ant: ANTENNA3, time: 5) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "deploy": {
                "errors": "",
                "success": true
            }
    });

    test!(service, query, expected);
}

#[test]
fn deploy_good_ant3_override() {
    let mut mock = mock_new!();
    mock.state = true;

    let service = service_new!(mock);

    let query = r#"mutation {
            deploy(ant: ANTENNA3, force: true, time: 5) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "deploy": {
                "errors": "",
                "success": true
            }
    });

    test!(service, query, expected);
}

#[test]
fn deploy_good_ant4() {
    let mut mock = mock_new!();
    mock.state = true;

    let service = service_new!(mock);

    let query = r#"mutation {
            deploy(ant: ANTENNA4, time: 5) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "deploy": {
                "errors": "",
                "success": true
            }
    });

    test!(service, query, expected);
}

#[test]
fn deploy_good_ant4_override() {
    let mut mock = mock_new!();
    mock.state = true;

    let service = service_new!(mock);

    let query = r#"mutation {
            deploy(ant: ANTENNA4, force: true, time: 5) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "deploy": {
                "errors": "",
                "success": true
            }
    });

    test!(service, query, expected);
}

#[test]
fn deploy_bad() {
    let mut mock = mock_new!();
    mock.state = false;

    let service = service_new!(mock);

    let query = r#"mutation {
            deploy(ant: ANTENNA1, time: 5) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "deploy": {
                "errors": "Configuration error",
                "success": false
            }
    });

    test!(service, query, expected);
}
