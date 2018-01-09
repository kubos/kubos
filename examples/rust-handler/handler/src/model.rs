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

extern crate extern_lib;

use std::io::{Error, ErrorKind};

/// Model for power mutations
pub struct SetPower {
    pub power: bool,
}

/// Model for uptime mutations
pub struct ResetUptime {
    pub uptime: i32,
}

/// Model for thermometer mutations
pub struct CalibrateThermometer {
    pub temperature: i32,
}

/// Model for handler's subsystem
pub struct Subsystem;

impl Subsystem {
    /// Creates new Subsystem structure instance
    /// Code initializing subsystems communications
    /// would likely be placed here
    pub fn new() -> Subsystem {
        println!("getting new subsystem data");
        // Here we call into an external C based function
        extern_lib::k_init_device();
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
    pub fn set_power(&self, _power: bool) -> Result<SetPower, Error> {
        println!("Setting power state");
        // Send command to device here
        if _power {
            Ok(SetPower { power: true })
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
        Ok(111001)
    }

    /// Uptime reset function
    pub fn reset_uptime(&self) -> Result<ResetUptime, Error> {
        println!("Resetting uptime");
        // Send command to device here
        Ok(ResetUptime { uptime: 0 })
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
    pub fn calibrate_thermometer(&self) -> Result<CalibrateThermometer, Error> {
        println!("calibrating thermometer");
        Ok(CalibrateThermometer { temperature: 98 })
    }
}

/// Overriding the destructor
impl Drop for Subsystem {
    /// Here is where we would clean up
    /// any subsystem communications stuff
    fn drop(&mut self) {
        println!("Destructing subsystem");
        extern_lib::k_terminate_device();
    }
}
