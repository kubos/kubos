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
use std::sync::mpsc::TryRecvError;

fn sleep_read(_: ()) -> MAIResult<Vec<u8>> {
    thread::sleep(Duration::from_millis(250));
    Ok(RAW_READ.to_vec())
}

#[test]
fn read_good() {
    let data = Arc::new(ReadData::new());

    let data_ref = data.clone();

    let (sender, _) = channel();

    thread::spawn(move || {
        let mock = mock_new!();

        mock.read.use_fn(sleep_read);

        let mai = MAI400 {
            conn: Connection {
                stream: Box::new(mock),
            },
        };
        read_thread(mai, data_ref, sender)
    });

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
fn read_bad() {
    let data = Arc::new(ReadData::new());

    let data_ref = data.clone();

    let (sender, receiver) = channel();

    thread::spawn(move || {
        let mock = mock_new!();

        mock.read.return_value(Err(MAIError::SerialError {
            cause: "some serial error".to_owned(),
        }));

        let mai = MAI400 {
            conn: Connection {
                stream: Box::new(mock),
            },
        };
        read_thread(mai, data_ref, sender)
    });

    // Give it a sec to work
    thread::sleep(Duration::from_millis(500));

    let result = receiver.try_recv();

    assert_eq!(
        result,
        Ok(
            "SerialError: some serial error. Read thread bailing. Service restart required."
                .to_owned()
        )
    );
}

fn panic_read(_: ()) -> MAIResult<Vec<u8>> {
    panic!();
}

#[test]
fn read_panic() {
    let data = Arc::new(ReadData::new());

    let data_ref = data.clone();

    let (sender, receiver) = channel();

    thread::spawn(move || {
        let mock = mock_new!();

        mock.read.use_fn(panic_read);

        mock.read.return_value(Err(MAIError::SerialError {
            cause: "some serial error".to_owned(),
        }));

        let mai = MAI400 {
            conn: Connection {
                stream: Box::new(mock),
            },
        };
        read_thread(mai, data_ref, sender)
    });

    thread::sleep(Duration::from_millis(500));

    let result = receiver.try_recv();

    assert_eq!(result, Err(TryRecvError::Disconnected));
}
