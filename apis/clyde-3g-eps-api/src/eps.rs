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

use std::io;

use i2c_api::Stream;

use commands::*;

#[derive(Debug, Display, Eq, Fail, PartialEq)]
pub enum EpsError {
    #[display(fmt = "IO Error {}", cause)] IoError { cause: String },
    #[display(fmt = "Bad Data")] BadData,
}

impl From<io::Error> for EpsError {
    fn from(error: io::Error) -> Self {
        EpsError::IoError {
            cause: error.to_string(),
        }
    }
}

pub type EpsStatus<T> = Result<T, EpsError>;

pub struct Eps {
    connection: Box<Stream>,
}

impl Eps {
    pub fn new(connection: Box<Stream>) -> Self {
        Eps { connection }
    }

    pub fn get_board_status(&self) -> EpsStatus<Status> {
        Status::parse(&self.connection.transfer(Status::command())?)
    }

    pub fn get_checksum(&self) -> EpsStatus<Checksum> {
        Checksum::parse(&self.connection.transfer(Checksum::command())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error;

    struct MockConn;

    impl Stream for MockConn {
        fn write(&self, data: &[u8]) -> Result<(), Error> {
            Ok(())
        }

        fn read(&self, length: usize) -> Result<Vec<u8>, Error> {
            Ok(vec![])
        }
    }
}
