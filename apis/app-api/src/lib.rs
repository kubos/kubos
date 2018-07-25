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
#![deny(missing_docs)]
#![deny(warnings)]
#[macro_use]
extern crate failure;
#[cfg(test)]
#[macro_use]
extern crate juniper;
#[cfg(test)]
extern crate kubos_service;
#[cfg(not(test))]
extern crate serde_json;
#[cfg(test)]
#[macro_use]
extern crate serde_json;

mod framework;
mod query;
#[cfg(test)]
mod tests;

pub use framework::*;
pub use query::*;
