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
use rust_uart::mock::*;
use rust_uart::*;
use std::sync::mpsc::sync_channel;
use std::sync::{Arc, Mutex};
use std::thread;

/// A macro for mocking a new OEM6 device instance.
#[macro_export]
macro_rules! mock_new {
    ($mock:ident) => {{
        let (log_send, log_recv) = sync_channel(5);
        let (response_send, response_recv) = sync_channel(5);
        let (response_abbrv_send, response_abbrv_recv) = sync_channel(5);

        let conn = Arc::new(Mutex::new(Connection {
            stream: Box::new($mock),
        }));
        let rx_conn = conn.clone();

        thread::spawn(move || {
            read_thread(&rx_conn, &log_send, &response_send, &response_abbrv_send)
        });

        OEM6 {
            conn,
            log_recv: Arc::new(Mutex::new(log_recv)),
            response_recv: Arc::new(Mutex::new(response_recv)),
            response_abbrv_recv: Arc::new(Mutex::new(response_abbrv_recv)),
        }
    }};
}

// Simple test to make sure our mock objects are working as expected
#[test]
fn mock_test() {
    let mock = MockStream::default();

    let oem = mock_new!(mock);

    let packet: [u8; 40] = [0; 40];

    assert_eq!(
        oem.conn.lock().unwrap().write(&packet).unwrap_err(),
        UartError::GenericError
    );
}

#[test]
fn test_passthrough_bad() {
    let mock = MockStream::default();

    let oem = mock_new!(mock);

    let packet: [u8; 40] = [0; 40];

    assert_eq!(
        oem.passthrough(&packet).unwrap_err(),
        OEMError::UartError {
            cause: UartError::GenericError,
        }
    );
}

#[test]
fn test_passthrough_good() {
    let mut mock = MockStream::default();

    let packet: [u8; 4] = [0, 1, 2, 3];
    mock.write.set_input(packet.to_vec());

    let oem = mock_new!(mock);

    assert_eq!(oem.passthrough(&packet), Ok(()));
}

mod errors;
mod position;
mod unlog;
mod version;
