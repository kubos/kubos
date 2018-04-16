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

use failure::Fail;
use mai400_api::MAI400;
use std::cell::RefCell;
use std::io::Error;
use std::str;

use objects::*;

pub struct Subsystem {
    pub mai: MAI400,
    pub errors: RefCell<Vec<String>>,
}

impl Subsystem {
    pub fn new(bus: String) -> Subsystem {

        let connection = Connection::new(bus);
        let mai = MAI400::new(connection);

        let subsystem = Subsystem {
            mai,
            errors: RefCell::new(vec![]),
        };

        //TODO: stuff with receive thread

        subsystem
    }

    // Queries

    pub fn get_arm_status(&self) -> Result<ArmStatus, Error> {
        let (_errors, _success, deploy) = run!(self.ants.get_deploy(), self.errors);
        let armed = deploy.unwrap_or_default().sys_armed;

        let status = match armed {
            true => ArmStatus::Armed,
            false => ArmStatus::Disarmed,
        };

        Ok(status)
    }
}
