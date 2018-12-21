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

use super::*;
use model::*;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tests::test_data::*;

#[test]
fn read_good() {
    let mut mock = MockStream::default();
    let data = Arc::new(ReadData::new());

    mock.read.set_output(RAW_READ.to_vec());

    service_new_with_read!(mock, data);

    // Give it a sec to work
    thread::sleep(Duration::from_millis(500));

    // Make sure persistent data matches expected values

    assert_eq!(data.std_telem.lock().unwrap().clone(), STD);
    assert_eq!(data.irehs_telem.lock().unwrap().clone(), IREHS);
    assert_eq!(data.imu.lock().unwrap().clone(), IMU);
    // Only check the rotating variables which will have been updated
    // with the single standard telemetry message
    assert_eq!(
        data.rotating.lock().unwrap().clone().sc_vel_eci,
        ROTATING.sc_vel_eci
    );
}

#[test]
fn read_panic() {
    let mut mock = MockStream::default();
    let data = Arc::new(ReadData::new());

    mock.read.set_result(Err(UartError::GenericError));

    let service = service_new_with_read!(mock, data);

    // Give it a sec to work
    thread::sleep(Duration::from_millis(500));

    let query = r#"{
            errors
        }"#;

    let expected = json!({
            "errors": ["Read thread panicked. Service restart required."]
    });

    let expected = json!({
                "data": expected,
                "errors": ""
        }).to_string();

    assert_eq!(service.process(&query.to_owned()), expected);
}
