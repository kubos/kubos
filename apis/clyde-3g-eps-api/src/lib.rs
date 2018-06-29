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

#![deny(missing_docs)]
#![deny(warnings)]

//! Low level interface for interacting with a ClydeSpace 3G EPS

#[macro_use]
extern crate bitflags;
extern crate eps_api;
#[macro_use]
extern crate failure;
extern crate nom;
extern crate rust_i2c;

mod commands;
mod eps;
mod telemetry;

pub use commands::version::{Version, VersionInfo};
pub use eps::Eps;
pub use telemetry::daughterboard as DaughterboardTelemetry;
pub use telemetry::motherboard as MotherboardTelemetry;
pub use telemetry::reset as ResetTelemetry;
