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

use double;
use mai400_api::*;
use std::sync::mpsc::channel;

#[macro_export]
macro_rules! mock_new {
    () => {
        MockStream::new(
            Err(MAIError::GenericError.into()),
            Err(MAIError::GenericError.into()),
        )
    };
}

mock_trait_no_default!(
    pub MockStream,
    write(Vec<u8>) -> MAIResult<()>,
    read() -> MAIResult<Vec<u8>>
);

impl Stream for MockStream {
    mock_method!(write(&self, data: &[u8]) -> MAIResult<()>, self, {
            self.write.call(data.to_vec())
        });

    mock_method!(read(&self) -> MAIResult<Vec<u8>>);
}

#[macro_export]
macro_rules! service_new {
    ($mock:ident) => {{
        let (sender, receiver) = channel();

        // We don't actually want to do anything with this thread, the channel
        // sender just needs to live through the lifetime of each test
        thread::spawn(move || {let _send = sender; thread::sleep(Duration::from_secs(2))});

        Service::new(
            Config::new("mai400-service"),
            Subsystem {
                mai: MAI400 {
                    conn: Connection {
                        stream: Box::new($mock),
                    },
                },
                last_cmd: Cell::new(AckCommand::None),
                errors: RefCell::new(vec![]),
                persistent: Arc::new(ReadData {
                    std_telem: Mutex::new(STD),
                    irehs_telem: Mutex::new(IREHS),
                    imu: Mutex::new(IMU),
                    rotating: Mutex::new(ROTATING),
                }),
                receiver,
            },
            QueryRoot,
            MutationRoot,
        )
    }};
}

#[test]
fn mock_test() {
    let mock = mock_new!();

    let connection = Connection {
        stream: Box::new(mock),
    };

    let packet: [u8; 40] = [0; 40];

    assert_eq!(
        connection.write(&packet).unwrap_err(),
        MAIError::GenericError
    );
}

mod read;
mod schema;
mod test_data;
