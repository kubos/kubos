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

extern crate isis_iobc_supervisor;

// Why create a new SupervisorVersion struct which just holds a SupervisorVersion?
// Because of E0117 (https://doc.rust-lang.org/error-index.html#E0117)
// Basically we can't implement the (external) GraphQL traits on
// isis_iobc_supervisor::SupervisorVersion because it is an external type.
// Same goes for SupervisorEnableStatus and SupervisorHousekeeping.
pub struct SupervisorVersion(pub isis_iobc_supervisor::SupervisorVersion);

pub struct SupervisorEnableStatus(pub isis_iobc_supervisor::SupervisorEnableStatus);

pub struct SupervisorHousekeeping(pub isis_iobc_supervisor::SupervisorHousekeeping);

pub struct Supervisor;

impl Supervisor {
    pub fn new() -> Supervisor {
        Supervisor {}
    }

    pub fn version(&self) -> Result<SupervisorVersion, String> {
        Ok(SupervisorVersion(
            isis_iobc_supervisor::supervisor_version()?
        ))
    }

    pub fn housekeeping(&self) -> Result<SupervisorHousekeeping, String> {
        Ok(SupervisorHousekeeping(
            isis_iobc_supervisor::supervisor_housekeeping()?,
        ))
    }
}
