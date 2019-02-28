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

//! Kubos API for interacting with [NovAtel OEM6 High Precision GNSS Receivers](https://www.novatel.com/products/gnss-receivers/oem-receiver-boards/oem6-receivers/)
//!
//! All work is done against an instantiated [`OEM6`] struct.
//!
//! More information about the device and it's behavior can be found in the following guides:
//! - [OEM6 Family Firmware Reference Manual](https://www.novatel.com/assets/documents/manuals/om-20000129.pdf)
//! - [OEM6 Family Installation and Operation User Manual](https://www.novatel.com/assets/Documents/Manuals/om-20000128.pdf)
//!
//! # Examples
//!
//! ```
//! use novatel_oem6_api::*;
//! use std::thread;
//! use std::sync::mpsc::sync_channel;
//!
//! # fn func() -> OEMResult<()> {
//! // Create communication channels to be used between the read thread and the main thread
//! let (log_send, log_recv) = sync_channel(5);
//! let (response_send, response_recv) = sync_channel(5);
//!
//! // Create the main connection to the device
//! let oem = OEM6::new("/dev/ttyS5", BaudRate::Baud9600, log_recv, response_recv).unwrap();
//!
//! // Clone the connection mutex for the read thread
//! let rx_conn = oem.conn.clone();
//!
//! // Start up a read thread to consume messages from the device
//! thread::spawn(move || read_thread(&rx_conn, &log_send, &response_send));
//!
//! // Request that the device send position information once per second
//! oem.request_position(1.0, 0.0, false)?;
//!
//! // Continually read the log messages
//! loop {
//!     let entry = oem.get_log()?;
//!
//!     match entry {
//!         Log::BestXYZ(log) => {
//!             println!("Best XYZ Data:");
//!             println!("    Position: {:?}", log.position);
//!             println!("    Velocity: {:?}", log.velocity);
//!         }
//!         _ => {},
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! [`OEM6`]: struct.OEM6.html

#![deny(missing_docs)]
//Need a higher recursion limit for nom when parsing larger (>60 bytes) structures
#![recursion_limit = "256"]

mod crc32;
mod messages;
mod oem6;
#[cfg(test)]
mod tests;

pub use crate::messages::commands::ResponseID;
pub use crate::messages::logs::*;
pub use crate::messages::MessageID;
pub use crate::messages::ReceiverStatusFlags;
pub use crate::oem6::*;
pub use rust_uart::{mock, Connection, UartError};
pub use serial::BaudRate;
