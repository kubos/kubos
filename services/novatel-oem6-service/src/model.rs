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

use novatel_oem6_api::*;
use std::cell::{Cell, RefCell};
use std::sync::mpsc::sync_channel;

use objects::*;

pub struct Subsystem {
    pub oem: OEM6,
    pub last_cmd: Cell<AckCommand>,
    pub errors: RefCell<Vec<String>>,
}

impl Subsystem {
    pub fn new(bus: &'static str) -> OEMResult<Subsystem> {
        let (_log_send, log_recv) = sync_channel(5);
        let (_response_send, response_recv) = sync_channel(5);

        let oem = OEM6::new(bus, BaudRate::Baud9600, log_recv, response_recv)?;

        // TODO: Spawn read thread. Will use `log_send` and `response_send`

        println!("Kubos OEM6 service started");

        Ok(Subsystem {
            oem,
            last_cmd: Cell::new(AckCommand::None),
            errors: RefCell::new(vec![]),
        })
    }
}
