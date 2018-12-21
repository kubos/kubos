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
fn configure_hardware_bad_single() {
    let mut mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"mutation {
            configureHardware(config: [{option: LOG_ERROR_DATA}]) {
                config,
                errors,
                success
            }
        }"#;

    let expected = json!({
            "configureHardware": {
                "config": "LogErrorData(Hold: false)",
                "errors": "LogErrorData: UART Error, Generic Error",
                "success": false
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}

#[test]
fn configure_hardware_bad_multi() {
    let mut mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"mutation {
            configureHardware(config: [{option: LOG_ERROR_DATA}, {option: LOG_POSITION_DATA}]) {
                config,
                errors,
                success
            }
        }"#;

    let expected = json!({
            "configureHardware": {
                "config": "LogErrorData(Hold: false), LogPositionData(Hold: false)",
                "errors": "LogErrorData: UART Error, Generic Error. LogPositionData: UART Error, Generic Error",
                "success": false
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}

#[test]
fn configure_hardware_no_response() {
    let mut mock = MockStream::default();

    mock.write.set_result(Ok(()));

    let service = service_new!(mock);

    let query = r#"mutation {
            configureHardware(config: [{option: LOG_ERROR_DATA}]) {
                config,
                errors,
                success
            }
        }"#;

    let expected = json!({
            "configureHardware": {
                "config": "LogErrorData(Hold: false)",
                "errors": "LogErrorData: Failed to get command response",
                "success": false
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}

#[test]
fn configure_hardware_log_errors_default() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![
        0xAA, 0x44, 0x12, 0x1C, 0x1, 0x0, 0x0, 0xC0, 0x20, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20, 0x0, 0x0, 0x0, 0x5E, 0x0, 0x0,
        0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xB7, 0xAF, 0xFE, 0x9B,
    ]);

    mock.read.set_output(LOG_RESPONSE_GOOD.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            configureHardware(config: [{option: LOG_ERROR_DATA}]) {
                config,
                errors,
                success
            }
        }"#;

    let expected = json!({
            "configureHardware": {
                "config": "LogErrorData(Hold: false)",
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}

#[test]
fn configure_hardware_log_errors_no_hold() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![
        0xAA, 0x44, 0x12, 0x1C, 0x1, 0x0, 0x0, 0xC0, 0x20, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20, 0x0, 0x0, 0x0, 0x5E, 0x0, 0x0,
        0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xB7, 0xAF, 0xFE, 0x9B,
    ]);

    mock.read.set_output(LOG_RESPONSE_GOOD.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            configureHardware(config: [{option: LOG_ERROR_DATA, hold: false}]) {
                config,
                errors,
                success
            }
        }"#;

    let expected = json!({
            "configureHardware": {
                "config": "LogErrorData(Hold: false)",
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}

#[test]
fn configure_hardware_log_errors_hold() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![
        0xAA, 0x44, 0x12, 0x1C, 0x1, 0x0, 0x0, 0xC0, 0x20, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20, 0x0, 0x0, 0x0, 0x5E, 0x0, 0x0,
        0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0xD2, 0xC8, 0x42, 0x23,
    ]);

    mock.read.set_output(LOG_RESPONSE_GOOD.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            configureHardware(config: [{option: LOG_ERROR_DATA, hold: true}]) {
                config,
                errors,
                success
            }
        }"#;

    let expected = json!({
            "configureHardware": {
                "config": "LogErrorData(Hold: true)",
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}

#[test]
fn configure_hardware_log_errors_no_defaults() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![
        0xAA, 0x44, 0x12, 0x1C, 0x1, 0x0, 0x0, 0xC0, 0x20, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20, 0x0, 0x0, 0x0, 0x5E, 0x0, 0x0,
        0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0xD2, 0xC8, 0x42, 0x23,
    ]);

    mock.read.set_output(LOG_RESPONSE_GOOD.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            configureHardware(config: [{option: LOG_ERROR_DATA, hold: true, interval: 1.0, offset: 0.5}]) {
                config,
                errors,
                success
            }
        }"#;

    let expected = json!({
            "configureHardware": {
                "config": "LogErrorData(Hold: true): 1+0.5sec",
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}

#[test]
fn configure_hardware_log_position_default() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![
        0xAA, 0x44, 0x12, 0x1C, 0x1, 0x0, 0x0, 0xC0, 0x20, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20, 0x0, 0x0, 0x0, 0xF1, 0x0, 0x0,
        0x0, 0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x54, 0x62, 0x3A, 0x96,
    ]);

    mock.read.set_output(LOG_RESPONSE_GOOD.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            configureHardware(config: [{option: LOG_POSITION_DATA}]) {
                config,
                errors,
                success
            }
        }"#;

    let expected = json!({
            "configureHardware": {
                "config": "LogPositionData(Hold: false)",
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}

#[test]
fn configure_hardware_log_position_no_defaults() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![
        0xAA, 0x44, 0x12, 0x1C, 0x1, 0x0, 0x0, 0xC0, 0x20, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20, 0x0, 0x0, 0x0, 0xF1, 0x0, 0x0,
        0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xF0, 0x3F, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0xE0, 0x3F, 0x1, 0x0, 0x0, 0x0, 0x9, 0x7, 0x9B, 0xE2,
    ]);

    mock.read.set_output(LOG_RESPONSE_GOOD.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            configureHardware(config: [{option: LOG_POSITION_DATA, hold: true, interval: 1.0, offset: 0.5}]) {
                config,
                errors,
                success
            }
        }"#;

    let expected = json!({
            "configureHardware": {
                "config": "LogPositionData(Hold: true): 1+0.5sec",
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}

#[test]
fn configure_hardware_unlog_all_no_hold() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![
        0xAA, 0x44, 0x12, 0x1C, 0x26, 0x0, 0x0, 0xC0, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0xD5, 0xEA, 0xAF, 0x8D,
    ]);

    mock.read.set_output(UNLOG_ALL_RESPONSE_GOOD.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            configureHardware(config: [{option: UNLOG_ALL, hold: false}]) {
                config,
                errors,
                success
            }
        }"#;

    let expected = json!({
            "configureHardware": {
                "config": "UnlogAll(Hold: false)",
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}

#[test]
fn configure_hardware_unlog_all_hold() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![
        0xAA, 0x44, 0x12, 0x1C, 0x26, 0x0, 0x0, 0xC0, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0,
        0x0, 0xB0, 0x8D, 0x13, 0x35,
    ]);

    mock.read.set_output(UNLOG_ALL_RESPONSE_GOOD.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            configureHardware(config: [{option: UNLOG_ALL, hold: true}]) {
                config,
                errors,
                success
            }
        }"#;

    let expected = json!({
            "configureHardware": {
                "config": "UnlogAll(Hold: true)",
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}

#[test]
fn configure_hardware_unlog_errors() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![
        0xAA, 0x44, 0x12, 0x1C, 0x24, 0x0, 0x0, 0xC0, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20, 0x0, 0x0, 0x0, 0x5E, 0x0, 0x0,
        0x0, 0x51, 0x9F, 0xB8, 0x9E,
    ]);

    mock.read.set_output(UNLOG_RESPONSE_GOOD.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            configureHardware(config: [{option: UNLOG_ERROR_DATA}]) {
                config,
                errors,
                success
            }
        }"#;

    let expected = json!({
            "configureHardware": {
                "config": "UnlogErrorData(Hold: false)",
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}

#[test]
fn configure_hardware_unlog_position() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![
        0xAA, 0x44, 0x12, 0x1C, 0x24, 0x0, 0x0, 0xC0, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20, 0x0, 0x0, 0x0, 0xF1, 0x0, 0x0,
        0x0, 0x2, 0x96, 0xB0, 0x8B,
    ]);

    mock.read.set_output(UNLOG_RESPONSE_GOOD.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            configureHardware(config: [{option: UNLOG_POSITION_DATA}]) {
                config,
                errors,
                success
            }
        }"#;

    let expected = json!({
            "configureHardware": {
                "config": "UnlogPositionData(Hold: false)",
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(&query.to_owned()), wrap!(expected));
}
