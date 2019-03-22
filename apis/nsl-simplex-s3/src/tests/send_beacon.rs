//
// Copyright (C) 2019 Kubos Corporation
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
fn test_ack() {
    let mut mock = MockStream::default();

    mock.write
        .set_input(vec![0x50, 0x50, 0x50, 0x01, 0x02, 0x03]);

    mock.read.set_output(ACK.to_vec());

    let simplex = mock_new!(mock);

    let msg = [1, 2, 3];

    assert!(simplex.send_beacon(&msg).is_ok());
}

#[test]
fn test_nak() {
    let mut mock = MockStream::default();

    mock.write
        .set_input(vec![0x50, 0x50, 0x50, 0x01, 0x02, 0x03]);

    mock.write.set_result(Ok(()));

    // The logic will attempt to send the message five times.
    // We want to NAK all of them
    let mut output = NAK.to_vec();
    output.extend_from_slice(NAK);
    output.extend_from_slice(NAK);
    output.extend_from_slice(NAK);
    output.extend_from_slice(NAK);
    output.extend_from_slice(NAK);

    mock.read.set_output(output);

    let simplex = mock_new!(mock);

    let msg = [1, 2, 3];

    assert_eq!(
        format!("{}", simplex.send_beacon(&msg).unwrap_err()),
        "Failed to send message"
    );
}

#[test]
fn test_nak_ack() {
    let mut mock = MockStream::default();

    mock.write
        .set_input(vec![0x50, 0x50, 0x50, 0x01, 0x02, 0x03]);

    mock.write.set_result(Ok(()));

    // Let's pretend that a byte got dropped the first time we tried to write
    let mut output = NAK.to_vec();
    output.extend_from_slice(ACK);

    mock.read.set_output(output);

    let simplex = mock_new!(mock);

    let msg = [1, 2, 3];

    assert!(simplex.send_beacon(&msg).is_ok());
}

#[test]
fn test_busy() {
    let mut mock = MockStream::default();

    mock.write.set_result(Ok(()));
    mock.read
        .set_result(Err(UartError::from(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "Operation timed out",
        ))));

    let simplex = mock_new!(mock);

    let msg = [1, 2, 3];

    assert_eq!(
        format!("{}", simplex.send_beacon(&msg).unwrap_err()),
        "IO Error: Operation timed out"
    );
}
