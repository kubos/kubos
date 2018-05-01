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
//! All work is done against an instantiated [`MAI400`] struct.
//!
//! # Examples
//!
//! TODO
//!
//! [`MAI400`]: struct.MAI400.html

#![allow(missing_docs)]
#![allow(unused_imports)]
//Need a higher recursion limit for nom when parsing larger (>60 bytes) structures
#![recursion_limit = "256"]

#[macro_use]
extern crate bitflags;
extern crate byteorder;
extern crate crc16;
#[cfg(test)]
#[macro_use]
extern crate double;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;
extern crate rust_uart;
extern crate serial;

mod crc32;
mod oem6;
mod messages;
//#[cfg(test)]
//mod tests;

pub use oem6::*;
pub use messages::rx::*;
