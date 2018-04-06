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

//#![deny(missing_docs)]

extern crate byteorder;
//extern crate chrono;
extern crate crc16;

#[cfg(test)]
#[macro_use]
extern crate double;
//extern crate radio_api;
extern crate serial;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;


mod mai400;
mod messages;
mod serial_comm;
#[cfg(test)]
mod tests;

pub use mai400::*;
pub use messages::*;
pub use serial_comm::*;
