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

#[macro_use]
extern crate failure;

use std::io;

#[derive(Debug, Display, Eq, Fail, PartialEq)]
#[display(fmt = "Eps Error")]
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
