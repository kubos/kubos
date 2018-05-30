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
use std::thread;

#[test]
fn get_lock_info_default() {
    let mut mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"{
            lockInfo {
                position,
                time {
                    ms,
                    week
                },
                velocity
            }
        }"#;

    let expected = json!({
            "lockInfo": {
                "position": [0.0, 0.0, 0.0],
                "time": {
                    "ms": 0,
                    "week": 0,
                },
                "velocity": [0.0, 0.0, 0.0],
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn get_lock_info_no_lock() {
    let mut mock = MockStream::default();

    mock.read.set_output(POSITION_LOG_NO_LOCK.to_vec());

    let service = service_new!(mock);

    thread::sleep(SETUP_DELAY);

    let query = r#"{
            lockInfo {
                position,
                time {
                    ms,
                    week
                },
                velocity
            }
        }"#;

    let expected = json!({
            "lockInfo": {
                "position": [0.0, 0.0, 0.0],
                "time": {
                    "ms": 0,
                    "week": 0,
                },
                "velocity": [0.0, 0.0, 0.0],
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn get_lock_info_good() {
    let mut mock = MockStream::default();

    mock.read.set_output(POSITION_LOG_GOOD.to_vec());

    let service = service_new!(mock);

    thread::sleep(SETUP_DELAY);

    let query = r#"{
            lockInfo {
                position,
                time {
                    ms,
                    week
                },
                velocity
            }
        }"#;

    let expected = json!({
            "lockInfo": {
                "position": [1.1, 2.2, 3.3],
                "time": {
                    "ms": 164195000,
                	"week": 3025
                },
                "velocity": [4.4, 5.5, 6.6],
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn get_lock_info_nolock_after_good() {
    let mut mock = MockStream::default();

    let mut output = POSITION_LOG_GOOD.to_vec();
    output.extend_from_slice(&POSITION_LOG_NO_LOCK);
    mock.read.set_output(output);

    let service = service_new!(mock);

    thread::sleep(SETUP_DELAY);

    let query = r#"{
            lockInfo {
                position,
                time {
                    ms,
                    week
                },
                velocity
            }
        }"#;

    let expected = json!({
            "lockInfo": {
                "position": [1.1, 2.2, 3.3],
                "time": {
                    "ms": 164195000,
                	"week": 3025
                },
                "velocity": [4.4, 5.5, 6.6],
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}
