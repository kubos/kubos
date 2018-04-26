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

//! Kubos API for interacting with [Adcole Maryland Aerospace MAI-400 ADACS](https://www.cubesatshop.com/product/mai-400-adacs/)
//!
//! All work is done against an instantiated [`MAI400`] struct.
//!
//! # Examples
//! //TODO: No longer valid
//!
//! ```
//! use mai400_api::*;
//!
//! # fn func() -> MAIResult<()> {
//! // Create a new MAI connection
//! let connection = Connection::new("/dev/ttyS5".to_owned());
//! let mai = MAI400::new(connection);
//!
//! // Set the GPS time to Jan 01, 2018
//! mai.set_gps_time(1198800018)?;
//!
//! // Pull the updated time out of the next standard telemetry message
//! let (std, _imu, _irehs) = mai.get_message()?;
//!
//! if let Some(telem) = std {
//!     println!("Current GPS time: {}", telem.gps_time);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! [`MAI400`]: struct.MAI400.html

#![deny(missing_docs)]
//Need a higher recursion limit for nom when parsing larger (>60 bytes) structures
#![recursion_limit="256"]

#[macro_use]
extern crate bitflags;
extern crate byteorder;
extern crate crc16;
#[cfg(test)]
#[macro_use]
extern crate double;
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
pub use messages::rx::*;
pub use serial_comm::{Connection, Stream};
