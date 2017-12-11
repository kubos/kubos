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

use std::fmt;
use std::error;

/// Model for handler's subsystem
pub struct Subsystem;

#[derive(Debug)]
pub enum SubsystemError {
    IOFailure,
}

impl fmt::Display for SubsystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

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
    pub fn power(&self) -> Result<bool, SubsystemError> {
        println!("getting power");
        // Low level query here
        Ok(true)
        // If we had a failure retrieving power status...
        // Err(SubsystemError::IOFailure)
    }

    /// Uptime getter
    /// Code querying for new uptime value
    /// could be placed here
    pub fn uptime(&self) -> Result<i32, SubsystemError> {
        println!("getting uptime");
        // Low level query here
        Ok(100)
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
