//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Kubos API for interacting with [ISIS Antenna Systems](https://www.isispace.nl/product-category/products/antenna-systems/)
//!
//! All work is done against an instantiated [`AntS`] struct.
//!
//! # Examples
//!
//! ```
//! use isis_ants_api::*;
//! use std::thread::sleep;
//! use std::time::Duration;
//!
//! # fn func() -> AntSResult<()> {
//! // Create a new AntS connection
//! let ants = AntS::new("/dev/i2c-0", 0x31, 0x32, 4, 10).unwrap();
//!
//! // Configure it to run commands against the secondary controller
//! ants.configure(KANTSController::Secondary)?;
//!
//! // Prepare the system for deployment
//! ants.arm()?;
//!
//! // Auto-deploy the antennas with a five second timeout for each
//! ants.auto_deploy(5)?;
//!
//! // Give deployment a moment to run
//! sleep(Duration::from_secs(5));
//!
//! // Get the current deployment status
//! let deploy = ants.get_deploy()?;
//! println!("Antenna 1 deployed: {}", !deploy.ant_1_not_deployed);
//! println!("Antenna 2 deployment active: {}", deploy.ant_2_active);
//! # Ok(())
//! # }
//! ```
//!
//! [`AntS`]: struct.AntS.html

#![deny(missing_docs)]

pub use crate::ants::*;
pub use crate::parse::{AntsTelemetry, DeployStatus, KANTSAnt, KANTSController};

mod ants;
#[cfg_attr(feature = "nos3", allow(dead_code))]
mod ffi;
#[cfg_attr(feature = "nos3", allow(dead_code))]
mod parse;
