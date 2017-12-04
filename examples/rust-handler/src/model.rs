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


/// Model for subsystem function status
pub struct Status {
    status: bool,
}

impl Status {
    pub fn status(&self) -> bool {
        self.status
    }
}

/// Model for handler's subsystem
pub struct Subsystem {
    power: bool,
    uptime: i32,
    status: Status,
}

impl Subsystem {
    /// Creates new Subsystem structure instance
    /// Code querying for new subsystem data could
    /// be used here to populate structure
    pub fn new() -> Subsystem {
        println!("getting new subsystem data");
        Subsystem {
            power: true,
            uptime: 100,
            status: Status { status: true },
        }
    }

    /// Power status getter
    /// Code querying for new power value
    /// could be placed here
    pub fn power(&self) -> bool {
        println!("getting power");
        self.power
    }

    /// Power state setter
    /// Here we would call into the low level
    /// device function
    pub fn set_power(&self, _power: bool) -> &Status {
        println!("Setting power state");
        &self.status
    }

    /// Uptime getter
    /// Code querying for new uptime value
    /// could be placed here
    pub fn uptime(&self) -> i32 {
        println!("getting uptime");
        self.uptime
    }
}
