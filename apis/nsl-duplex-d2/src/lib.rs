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

//! Device level API for interacting with the
//! [NSL EyeStar-D2 Duplex radio](https://nearspacelaunch.com/product/eyestar-d2/)

#![deny(missing_docs)]

extern crate chrono;
extern crate crc16;
extern crate radio_api;
extern crate serial;

#[macro_use]
extern crate nom;

mod duplex_d2;
mod messages;
mod serial_comm;

pub use duplex_d2::DuplexD2;
pub use messages::File;
pub use messages::GeoRecord;
pub use messages::StateOfHealth;
pub use serial_comm::serial_connection;
