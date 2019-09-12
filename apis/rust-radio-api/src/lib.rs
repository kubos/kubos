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

//! A high level interface for interacting with radios

#![deny(missing_docs)]
#![deny(warnings)]

use failure::{bail, Error, Fail};
use nom::IResult;
use std::cell::RefCell;

/// Common Error for Radio Actions
#[derive(Debug, Fail)]
pub enum RadioError {
    #[fail(display = "Parse error: {}", message)]
    /// There was a problem parsing the result data
    ParseError {
        /// The message from original error
        message: String,
    },
    /// No data available
    #[fail(display = "No data available")]
    NoData,
}

fn nom_to_radio_error<T>(err: nom::Err<&[u8]>) -> Result<(&[u8], T), RadioError> {
    Err(match err {
        nom::Err::Error(nom::simple_errors::Context::Code(_, e)) => RadioError::ParseError {
            message: e.description().to_string(),
        },
        nom::Err::Failure(nom::simple_errors::Context::Code(_, e)) => RadioError::ParseError {
            message: e.description().to_string(),
        },
        nom::Err::Incomplete(_) => RadioError::ParseError {
            message: "Incomplete data".to_string(),
        },
    })
}

/// Custom error type for radio operations.
pub type RadioResult<T> = Result<T, Error>;

/// The signature of parse functions used in Connection read calls.
pub type ParseFn<T> = fn(input: &[u8]) -> IResult<&[u8], T>;

/// Connections expect a struct instance with this trait to represent streams.
pub trait Stream {
    /// Write raw bytes to the stream.
    fn write(&self, data: &[u8]) -> RadioResult<()>;
    /// Read raw bytes from the stream.
    fn read(&self) -> RadioResult<Vec<u8>>;
}

/// A connection is like a stream, but allowed parsed reads with properly buffered
/// input data.
pub struct Connection {
    stream: Box<dyn Stream + Send>,
    buffer: RefCell<Vec<u8>>,
}

impl Connection {
    /// Convenience constructor to create connection from stream.
    pub fn new(stream: Box<dyn Stream + Send>) -> Connection {
        Connection {
            stream,
            buffer: RefCell::new(Vec::new()),
        }
    }

    /// Write out raw bytes to the underlying stream.
    pub fn write(&self, data: &[u8]) -> RadioResult<()> {
        self.stream.write(data)
    }

    /// Read the next object using provided parser.
    pub fn read<T>(&self, parse: ParseFn<T>) -> RadioResult<T> {
        let mut buffer = self.buffer.borrow_mut();
        for _ in 0..5 {
            let copy = buffer.clone();
            let res = parse(&copy);

            if let Err(nom::Err::Incomplete(_)) = res {
                let more = self.stream.read()?;
                buffer.extend_from_slice(&more);
                continue;
            }

            let (extra, value) = res.or_else(nom_to_radio_error)?;
            buffer.clear();
            buffer.extend_from_slice(extra);
            return Ok(value);
        }
        bail!("Incomplete parsing".to_owned())
    }
}
