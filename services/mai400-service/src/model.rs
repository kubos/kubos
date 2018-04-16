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
use mai400_api::*;
use std::cell::RefCell;
use std::io::Error;
//use std::str;

use objects::*;

pub struct Subsystem {
    pub mai: MAI400,
    pub errors: RefCell<Vec<String>>,
}

impl Subsystem {
    pub fn new(bus: String) -> Subsystem {
        println!("New Subsystem");

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

    pub fn get_config(&self) -> Result<Config, Error> {
        unimplemented!();
    }

    pub fn get_power(&self) -> Result<GetPowerResponse, Error> {
        unimplemented!();
    }

    pub fn get_telemetry(&self) -> Result<Telemetry, Error> {
        unimplemented!();
    }

    pub fn get_test_results(&self) -> Result<IntegrationTestResults, Error> {
        unimplemented!();
    }

    pub fn get_mode(&self) -> Result<Mode, Error> {
        unimplemented!();
    }

    pub fn get_orientation(&self) -> Result<Orientation, Error> {
        unimplemented!();
    }

    pub fn get_spin(&self) -> Result<Spin, Error> {
        unimplemented!();
    }

    // Mutations

    pub fn passthrough(&self, command: String) -> Result<GenericResponse, Error> {
        let result = run!(self.mai.passthrough(command.as_bytes()), self.errors);

        Ok(GenericResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }

    pub fn noop(&self) -> Result<NoopResponse, Error> {
        unimplemented!();
    }
}
