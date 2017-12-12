//
// Copyright (C) 2017 Kubos Corporation
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

use std::io::{Error, ErrorKind};

/// Model for handler's subsystem
pub struct Subsystem;

impl Subsystem {
    /// Creates new Subsystem structure instance
    /// Code initializing subsystems communications
    /// would likely be placed here
    pub fn new() -> Subsystem {
        println!("getting new subsystem data");
        Subsystem {}
    }

    /// Power status getter
    /// Code querying for new power value
    /// could be placed here
    pub fn power(&self) -> Result<bool, Error> {
        println!("getting power");
        // Low level query here
        Ok(true)
    }

    /// Power state setter
    /// Here we would call into the low level
    /// device function
    pub fn set_power(&self, _power: bool) -> Result<bool, Error> {
        println!("Setting power state");
        // Send command to device here
        if _power {
            Ok(true)
        } else {
            Err(Error::new(
                ErrorKind::PermissionDenied,
                "I'm sorry Dave, I afraid I can't do that",
            ))
        }
    }

    /// Uptime getter
    /// Code querying for new uptime value
    /// could be placed here
    pub fn uptime(&self) -> Result<i32, Error> {
        println!("getting uptime");
        // Low level query here
        Ok(100)
    }

    /// Uptime reset function
    pub fn reset_uptime(&self) -> Result<bool, Error> {
        println!("Resetting uptime");
        // Send command to device here
        Ok(true)
    }

    /// Temperature getter
    /// Demonstrates returning an error condition
    pub fn temperature(&self) -> Result<i32, Error> {
        println!("getting temperature");
        // Low level query here
        Err(Error::new(
            ErrorKind::TimedOut,
            "Failed to retrieve temperature",
        ))
    }

    /// Temperature calibration
    /// Demonstrates a mutation with error condition
    pub fn calibrate_thermometer(&self) -> Result<bool, Error> {
        println!("calibrating thermometer");
        Err(Error::new(
            ErrorKind::NotFound,
            "Failed to find thermometer",
        ))
    }
}

/// Overriding the destructor
impl Drop for Subsystem {
    /// Here is where we would clean up
    /// any subsystem communications stuff
    fn drop(&mut self) {
        println!("Destructing subsystem");
    }
}
