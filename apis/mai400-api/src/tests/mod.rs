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
use super::*;
use serial_comm::*;

macro_rules! mock_new {
    () => (
        MockStream::new(
            Err(MAIError::GenericError.into()),
            Err(MAIError::GenericError.into())
       )
    )
}

mock_trait_no_default!(
    MockStream,
    write(Vec<u8>) -> MAIResult<()>,
    read() -> MAIResult<Vec<u8>>
);

impl Stream for MockStream {
    mock_method!(write(&self, data: &[u8]) -> MAIResult<()>, self, {
            self.write.call(data.to_vec())
        });

    mock_method!(read(&self) -> MAIResult<Vec<u8>>);
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

mod tx;
mod rx;
mod rotating;
