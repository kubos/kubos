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

//! A simple API to make standalone Rust applications with high-level hooks
//! for mission life-cycle management
//!
//! # Examples
//!
//! ```ignore
//! use failure::{bail, Error};
//! use kubos_app::*;
//! use std::time::Duration;
//!
//! fn main() -> Result<(), Error> {
//!     logging_setup!("my-app")?;
//!
//!     let request = r#"mutation {
//!         power(state: ON) {
//!             success
//!         }
//!     }"#;
//!
//!     match query(&ServiceConfig::new("radio-service")?, request, Some(Duration::from_secs(1))) {
//!         Err(error) => bail!("Failed to communicate with radio service: {}", error),
//!         Ok(data) => {
//!             if let Some(success) = data.get("power")
//!                 .and_then(|power| power.get("success"))
//!             {
//!                 match success.as_bool() {
//!                     Some(true) => println!("Successfully turned on radio"),
//!                     Some(false) => eprintln!("Failed to turn on radio"),
//!                     None => eprintln!("Failed to fetch radio power state")
//!                 }
//!             } else {
//!                 bail!("Failed to fetch radio power state");
//!             }
//!         }
//!     }
//!     Ok(())
//! }
//! ```
//!

#![deny(missing_docs)]
#![deny(warnings)]

#[cfg(test)]
#[macro_use]
extern crate juniper;

mod framework;
mod query;
#[cfg(test)]
mod tests;

pub use crate::framework::*;
pub use crate::query::query;
pub use kubos_system::Config as ServiceConfig;
