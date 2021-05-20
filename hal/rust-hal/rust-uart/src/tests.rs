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
use crate::mock::*;
use std::time::Duration;

#[test]
fn mock_test() {
    let mock = MockStream::default();

    let connection = Connection {
        stream: Box::new(mock),
    };

    let packet: [u8; 40] = [0; 40];

    assert_eq!(
        connection.write(&packet).unwrap_err(),
        UartError::GenericError
    );
}

#[test]
fn test_write_error_io() {
    let mut mock = MockStream::default();

    mock.write
        .set_result(Err(UartError::from(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "Operation timed out",
        ))));

    let connection = Connection {
        stream: Box::new(mock),
    };

    let packet: [u8; 40] = [0; 40];

    assert_eq!(
        connection.write(&packet).unwrap_err(),
        UartError::IoError {
            cause: std::io::ErrorKind::TimedOut,
            description: "Operation timed out".to_owned(),
        }
    );
}

#[test]
fn test_write_error_serial() {
    let mut mock = MockStream::default();

    mock.write
        .set_result(Err(UartError::from(serial::Error::new(
            serial::ErrorKind::NoDevice,
            "Device unavailable",
        ))));

    let connection = Connection {
        stream: Box::new(mock),
    };

    let packet: [u8; 40] = [0; 40];

    assert_eq!(
        connection.write(&packet).unwrap_err(),
        UartError::SerialError {
            cause: serial::ErrorKind::NoDevice,
            description: "Device unavailable".to_owned(),
        }
    );
}

#[test]
#[should_panic]
fn test_write_bad_input() {
    let mut mock = MockStream::default();

    mock.write.set_input([0, 1, 2, 3].to_vec());

    let connection = Connection {
        stream: Box::new(mock),
    };

    let packet: [u8; 40] = [0; 40];

    // This will fail under the covers because the passed input argument
    // doesn't match what we said we were expecting
    let _result = connection.write(&packet);
}

#[test]
fn test_write_good() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![0, 1, 2, 3]);

    let connection = Connection {
        stream: Box::new(mock),
    };

    let packet: [u8; 4] = [0, 1, 2, 3];

    assert_eq!(connection.write(&packet), Ok(()));
}

#[test]
fn test_read_error_io() {
    let mut mock = MockStream::default();

    mock.read
        .set_result(Err(UartError::from(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "Operation timed out",
        ))));

    let connection = Connection {
        stream: Box::new(mock),
    };

    assert_eq!(
        connection.read(5, Duration::new(0, 0)).unwrap_err(),
        UartError::IoError {
            cause: std::io::ErrorKind::TimedOut,
            description: "Operation timed out".to_owned(),
        }
    );
}

#[test]
fn test_read_error_serial() {
    let mut mock = MockStream::default();

    mock.read.set_result(Err(UartError::from(serial::Error::new(
        serial::ErrorKind::NoDevice,
        "Device unavailable",
    ))));

    let connection = Connection {
        stream: Box::new(mock),
    };

    assert_eq!(
        connection.read(5, Duration::new(0, 0)).unwrap_err(),
        UartError::SerialError {
            cause: serial::ErrorKind::NoDevice,
            description: "Device unavailable".to_owned(),
        }
    );
}

#[test]
fn test_read_bad_len() {
    let mut mock = MockStream::default();

    let expected = vec![0, 1, 2, 3, 4, 5];

    mock.read.set_output(expected.clone());

    let connection = Connection {
        stream: Box::new(mock),
    };

    // Explicitly setting the read to be one byte short
    let result = connection.read(5, Duration::new(0, 0));

    assert!(result.is_ok());
    assert_ne!(result.unwrap(), expected);
}

#[test]
fn test_read_bad_multi() {
    let mut mock = MockStream::default();

    let expected = vec![0, 1, 2, 3, 4, 5];

    mock.read.set_output(expected);

    let connection = Connection {
        stream: Box::new(mock),
    };

    assert_eq!(
        connection.read(3, Duration::new(0, 0)).unwrap(),
        vec![0, 1, 2]
    );
    assert_eq!(
        connection.read(4, Duration::new(0, 0)).unwrap_err(),
        UartError::IoError {
            cause: std::io::ErrorKind::TimedOut,
            description: "Operation timed out".to_owned(),
        }
    );
}

#[test]
fn test_read_good_single() {
    let mut mock = MockStream::default();

    let expected = vec![0, 1, 2, 3, 4, 5];

    mock.read.set_output(expected.clone());

    let connection = Connection {
        stream: Box::new(mock),
    };

    assert_eq!(connection.read(6, Duration::new(0, 0)).unwrap(), expected);
}

#[test]
fn test_read_good_multi() {
    let mut mock = MockStream::default();

    let expected = vec![0, 1, 2, 3, 4, 5];

    mock.read.set_output(expected);

    let connection = Connection {
        stream: Box::new(mock),
    };

    assert_eq!(
        connection.read(3, Duration::new(0, 0)).unwrap(),
        vec![0, 1, 2]
    );
    assert_eq!(
        connection.read(3, Duration::new(0, 0)).unwrap(),
        vec![3, 4, 5]
    );
}
