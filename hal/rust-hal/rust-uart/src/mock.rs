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

//! Mock objects for use with unit tests
//!
//! Note: This module was created because the crate we previously used for mocking
//! functions, [Double](https://github.com/DonaldWhyte/double), was written specifically
//! for single-threaded code. Our APIs and services which utilize this crate are not
//! guaranteed to be single-threaded, so we made the decision to create our own mock objects.

use super::*;
use std::collections::VecDeque;
use std::io::Cursor;

/// Structure containing the input data to verify and/or result to return
/// when the MockStream's write function is called
pub struct WriteStruct {
    input: RefCell<VecDeque<Vec<u8>>>,
    result: UartResult<()>,
}

impl WriteStruct {
    /// Set the result to be returned for any write() calls
    ///
    /// Note: This will be ignored if set_input is also used
    ///
    /// # Arguments
    ///
    /// * result - The UartResult to return in future write() calls
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_uart::*;
    /// use rust_uart::mock::*;
    ///
    /// fn test_write_error() {
    ///     let mut mock = MockStream::default();
    ///
    ///     mock.write
    ///         .set_result(Err(UartError::GenericError.into()));
    ///
    ///     let connection = Connection {
    ///         stream: Box::new(mock),
    ///     };
    ///
    ///     let packet: [u8; 40] = [0; 40];
    ///
    ///     assert_eq!(
    ///         connection.write(&packet).unwrap_err(),
    ///         UartError::GenericError
    ///     );
    /// }
    /// ```
    pub fn set_result(&mut self, result: UartResult<()>) {
        self.result = result;
    }

    /// Set the input to validate for any write() calls
    ///
    /// # Arguments
    ///
    /// * input - The input data expected from write() calls
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_uart::*;
    /// use rust_uart::mock::*;
    ///
    /// fn test_write_good() {
    ///     let mut mock = MockStream::default();
    ///
    ///     mock.write.set_input(vec![0, 1, 2, 3]);
    ///
    ///     let connection = Connection {
    ///         stream: Box::new(mock),
    ///     };
    ///
    ///     let packet: [u8; 4] = [0, 1, 2, 3];
    ///
    ///     assert_eq!(connection.write(&packet), Ok(()));
    /// }
    /// ```
    pub fn set_input(&mut self, input: Vec<u8>) {
        self.input.borrow_mut().push_back(input)
    }
}

/// Structure containing the output data or result to return
/// when the MockStream's read function is called
pub struct ReadStruct {
    output: Option<RefCell<Cursor<Vec<u8>>>>,
    result: UartResult<Vec<u8>>,
}

impl ReadStruct {
    /// Set the result to be returned for any read() calls
    ///
    /// Note: This will be ignored if set_output is also used
    ///
    /// # Arguments
    ///
    /// * result - The UartResult to return in future read() calls
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_uart::*;
    /// use rust_uart::mock::*;
    /// use std::time::Duration;
    ///
    /// fn test_read_error_io() {
    ///     let mut mock = MockStream::default();
    ///
    ///     mock.read
    ///         .set_result(Err(UartError::GenericError.into()));
    ///
    ///     let connection = Connection {
    ///         stream: Box::new(mock),
    ///     };
    ///
    ///     assert_eq!(
    ///         connection.read(5, Duration::new(0, 0)).unwrap_err(),
    ///         UartError::GenericError
    ///     );
    /// }
    /// ```
    pub fn set_result(&mut self, result: UartResult<Vec<u8>>) {
        self.result = result;
    }

    /// Set the output data
    ///
    /// # Arguments
    ///
    /// * output - The output buffer which future read() calls should
    ///            retrieve data from
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_uart::*;
    /// use rust_uart::mock::*;
    /// use std::time::Duration;
    ///
    /// fn test_read_good_multi() {
    ///     let mut mock = MockStream::default();
    ///
    ///     let expected = vec![0, 1, 2, 3, 4, 5];
    ///
    ///     mock.read.set_output(expected.clone());
    ///
    ///     let connection = Connection {
    ///         stream: Box::new(mock),
    ///     };
    ///
    ///     assert_eq!(
    ///         connection.read(3, Duration::new(0, 0)).unwrap(),
    ///         vec![0, 1, 2]
    ///     );
    ///     assert_eq!(
    ///         connection.read(3, Duration::new(0, 0)).unwrap(),
    ///         vec![3, 4, 5]
    ///     );
    /// }
    /// ```
    pub fn set_output(&mut self, output: Vec<u8>) {
        self.output = Some(RefCell::new(Cursor::new(output)))
    }
}

/// Mock object for simulating a UART data stream
pub struct MockStream {
    /// Information to use when write() calls are made
    pub write: WriteStruct,
    /// Information to use when read() calls are made
    pub read: ReadStruct,
}

impl Default for MockStream {
    fn default() -> Self {
        MockStream {
            write: WriteStruct {
                result: Err(UartError::GenericError.into()),
                input: RefCell::new(VecDeque::new()),
            },
            read: ReadStruct {
                result: Err(UartError::GenericError.into()),
                output: None,
            },
        }
    }
}

impl Stream for MockStream {
    fn write(&self, data: &[u8]) -> UartResult<()> {
        if self.write.input.borrow_mut().is_empty() {
            self.write.result.clone()
        } else {
            let input = self.write.input.borrow_mut().pop_front().unwrap();
            if input.is_empty() {
                self.write.result.clone()
            } else {
                //Verify input matches data
                assert_eq!(input.as_slice(), data);
                Ok(())
            }
        }
    }

    fn read(&self, len: usize, _timeout: Duration) -> UartResult<Vec<u8>> {
        if let Some(ref output) = self.read.output {
            let mut response: Vec<u8> = vec![0; len];

            match output.borrow_mut().read_exact(response.as_mut_slice()) {
                Ok(_) => Ok(response),
                Err(_) => {
                    // Our buffer will throw an EOF error when it's empty,
                    // but, in reality, our UART stream would throw a timeout
                    // error when it was unable to read the requested number of
                    // bytes within the timeout period
                    Err(UartError::from(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "Operation timed out",
                    )))
                }
            }
        } else {
            self.read.result.clone()
        }
    }
}
