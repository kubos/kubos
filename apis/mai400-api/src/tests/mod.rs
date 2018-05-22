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

#[macro_export]
macro_rules! mock_new {
    ($mock:ident) => {{
        use std::sync::{Arc, Mutex};

        let conn = Arc::new(Mutex::new(Connection {
            stream: Box::new($mock),
        }));

        MAI400 { conn }
    }};
}

// Simple test to make sure our mock objects are working as expected
#[test]
fn mock_test() {
    let mock = MockStream::default();

    let mai = mock_new!(mock);

    let packet: [u8; 40] = [0; 40];

    assert_eq!(
        mai.conn.lock().unwrap().write(&packet).unwrap_err(),
        UartError::GenericError
    );
}

mod rotating;
mod rx;
mod tx;
