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

extern crate nom;

use std::cell::RefCell;
use nom::IResult;

/// The signature of parse functions used in Connection read calls.
pub type ParseFn<T> = fn(input: &[u8]) -> IResult<&[u8], T>;

/// Connections expect a struct instance with this trait to represent streams.
pub trait Stream {
    /// Write raw bytes to the stream.
    fn write(&self, data: &[u8]) -> Result<(), String>;
    /// Read raw bytes from the stream.
    fn read(&self) -> Result<Vec<u8>, String>;
}

/// A connection is like a stream, but allowed parsed reads with properly buffered
/// input data.
pub struct Connection {
    stream: Box<Stream>,
    buffer: RefCell<Vec<u8>>,
}

impl Connection {
    /// Convenience constructor to create connection from stream.
    pub fn new(stream: Box<Stream>) -> Connection {
        Connection {
            stream,
            buffer: RefCell::new(Vec::new()),
        }
    }

    /// Write out raw bytes to the underlying stream.
    pub fn write(&self, data: &[u8]) -> Result<(), String> {
        self.stream.write(data)
    }

    /// Read the next object using provided parser.
    pub fn read<T>(&self, parse: ParseFn<T>) -> Result<T, String> {
        let mut buffer = self.buffer.borrow_mut();
        loop {
            let copy = buffer.clone();
            let res = parse(&copy);
            if let Ok((extra, value)) = res {
                buffer.clear();
                buffer.extend_from_slice(extra);
                return Ok(value);
            }
            if let Err(nom::Err::Incomplete(_)) = res {
                let more = self.stream.read()?;
                buffer.extend_from_slice(&more);
                continue;
            }
            return Err("Parse Error".to_string());
        }
    }
}
