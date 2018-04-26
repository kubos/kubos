/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#![allow(unused_imports)]

use kubos_service::{Config, Service};
use super::*;
use model::*;
use schema::*;
use objects::*;
use std::cell::{Cell, RefCell};
//use std::io::{Error, ErrorKind};
use std::sync::{Arc, Mutex};
use serde_json;
use tests::test_data::*;

use juniper::{execute, RootNode, Value, Variables};
use std::collections::HashMap;

macro_rules! wrap {
    ($result:ident) => {{
            json!({
                    "msg": serde_json::to_string(&$result).unwrap(),
                    "errs": ""}).to_string()
        }}
}

macro_rules! service_new {
    ($mock:ident) => {{
            Service::new(
            Config::new("mai400-service"),
            Subsystem {
                mai: MAI400 { conn: Connection { stream: Box::new($mock) } },
                last_cmd: Cell::new(AckCommand::None),
                errors: RefCell::new(vec![]),
                persistent: Arc::new(ReadData {
                        std_telem: Mutex::new(STD),
                        irehs_telem: Mutex::new(IREHS),
                        imu: Mutex::new(IMU),
                        rotating: Mutex::new(ROTATING),
                }),
            },
            QueryRoot,
            MutationRoot,
        )
        }}
}

#[test]
fn ping() {
    let mock = mock_new!();

    let service = service_new!(mock);

    let query = r#"{
            ping
        }"#;

    let expected = json!({
            "ping": "pong"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn ack_default() {
    let mock = mock_new!();

    let service = service_new!(mock);

    let query = r#"{
            ack
        }"#;

    let expected = json!({
            "ack": "NONE"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn ack_noop() {
    let mock = mock_new!();

    let service = service_new!(mock);

    let noop = r#"mutation {
            noop {
                success
            }
        }"#;

    service.process(noop.to_owned());

    let query = r#"{
            ack
        }"#;

    let expected = json!({
            "ack": "NOOP"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn errors_empty() {
    let mock = mock_new!();

    let service = service_new!(mock);

    let query = r#"{
            errors
        }"#;

    let expected = json!({
            "errors": []
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn errors_single() {
    let mock = mock_new!();

    let service = service_new!(mock);

    let noop = r#"mutation {
            noop {
                success
            }
        }"#;

    service.process(noop.to_owned());

    let query = r#"{
            errors
        }"#;

    let expected = json!({
            "errors": ["Noop: Unable to communicate with MAI400"]
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn errors_multiple() {
    let mock = mock_new!();

    let service = service_new!(mock);

    let noop = r#"mutation {
            noop {
                success
            }
        }"#;

    service.process(noop.to_owned());
    service.process(noop.to_owned());

    let query = r#"{
            errors
        }"#;

    let expected = json!({
            "errors": ["Noop: Unable to communicate with MAI400", "Noop: Unable to communicate with MAI400"]
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn errors_clear_after_query() {
    let mock = mock_new!();

    let service = service_new!(mock);

    let noop = r#"mutation {
            noop {
                success
            }
        }"#;

    service.process(noop.to_owned());
    service.process(noop.to_owned());

    let query = r#"{
            errors
        }"#;

    service.process(query.to_owned());

    let expected = json!({
            "errors": []
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

//TODO: HOW DO I TEST POWER ON???????? It involves mucking with the read thread...
#[test]
fn power_off() {
    let mock = mock_new!();

    let service = service_new!(mock);

    let query = r#"{
            power{
                state,
                uptime
            }
        }"#;

    let expected = json!({
            "power": {
                "state": "OFF",
                "uptime": 0
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn config() {
    let mock = mock_new!();

    let service = service_new!(mock);

    let query = r#"{
            config
        }"#;

    let expected = json!({
            "config": "Not Implemented"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

// TODO: telemetry

// TODO: testResults

#[test]
fn mode() {
    let mock = mock_new!();

    let service = service_new!(mock);

    let query = r#"{
            mode
        }"#;

    let expected = json!({
            "mode": "TEST_MODE"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn orientation() {
    let mock = mock_new!();

    let service = service_new!(mock);

    let query = r#"{
            orientation
        }"#;

    let expected = json!({
            "orientation": "Not Implemented"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn spin() {
    let mock = mock_new!();

    let service = service_new!(mock);

    let query = r#"{
            spin {
                x,
                y,
                z
            }
        }"#;

    let expected = json!({
            "spin": {
                "x": -100000.0,
                "y": -100000.0,
                "z": -100000.0
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

//TODO: errors (mutation)

//TODO: noop good
#[test]
fn noop_fail() {
    let mock = mock_new!();

    let service = service_new!(mock);

    let query = r#"mutation {
            noop {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "noop": {
                "errors": "Unable to communicate with MAI400",
                "success": false
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn control_power_good() {
    let mock = mock_new!();

    mock.write.return_value_for(
        (REQUEST_RESET.to_vec()),
        Ok(()),
    );
    mock.write.return_value_for(
        (CONFIRM_RESET.to_vec()),
        Ok(()),
    );

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

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn configure_hardware() {
    let mock = mock_new!();

    let service = service_new!(mock);

    let query = r#"mutation {
            configureHardware
        }"#;

    let expected = json!({
            "configureHardware": "Not Implemented"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

//TODO: testHardware(integration)

#[test]
fn test_hardware_hardware() {
    let mock = mock_new!();

    let service = service_new!(mock);

    let query = r#"mutation {
            testHardware(test: HARDWARE) {
                ... on HardwareTestResults {
                    data,
                    errors,
                    success
                }
            }
        }"#;

    let expected = json!({
            "testHardware": {
                "data": "",
                "errors": "Not Implemented",
                "success": true
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

//TODO: issueRawCommand

#[test]
fn set_mode() {
    let mock = mock_new!();

    mock.write.return_value_for(
        (SET_MODE_AQUISITION.to_vec()),
        Ok(()),
    );

    let service = service_new!(mock);

    let query = r#"mutation {
            setMode(mode: RATE_NULLING, qbiCmd: [2, 3, 4, 5]) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "setMode": {
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn update_gps() {
    let mock = mock_new!();

    mock.write.return_value_for((SET_GPS_TIME.to_vec()), Ok(()));

    let service = service_new!(mock);

    let query = r#"mutation {
            update(gpsTime: 1198800018) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "update": {
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn update_rv() {
    let mock = mock_new!();

    mock.write.return_value_for((SET_RV.to_vec()), Ok(()));

    let service = service_new!(mock);

    let query = r#"mutation {
            update(rv: {eciPos: [1.1, 2.2, 3.3], eciVel: [4.4, 5.5, 6.6], timeEpoch: 1198800018}) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "update": {
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}
