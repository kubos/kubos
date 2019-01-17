//
// Copyright (C) 2019 Kubos Corporation
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

use comms_service::CommsTelemetry;
use std::sync::{Arc, Mutex};

pub struct Subsystem {
    telem: Arc<Mutex<CommsTelemetry>>,
}

impl Subsystem {
    pub fn new(telem: Arc<Mutex<CommsTelemetry>>) -> Subsystem {
        Subsystem { telem }
    }

    pub fn failed_packets_up(&self) -> Result<i32, String> {
        match self.telem.lock() {
            Ok(data) => Ok(data.failed_packets_up),
            Err(_) => Err("Failed to lock telemetry".to_owned()),
        }
    }

    pub fn failed_packets_down(&self) -> Result<i32, String> {
        match self.telem.lock() {
            Ok(data) => Ok(data.failed_packets_down),
            Err(_) => Err("Failed to lock telemetry".to_owned()),
        }
    }

    pub fn packets_up(&self) -> Result<i32, String> {
        match self.telem.lock() {
            Ok(data) => Ok(data.packets_up),
            Err(_) => Err("Failed to lock telemetry".to_owned()),
        }
    }

    pub fn packets_down(&self) -> Result<i32, String> {
        match self.telem.lock() {
            Ok(data) => Ok(data.packets_down),
            Err(_) => Err("Failed to lock telemetry".to_owned()),
        }
    }

    pub fn errors(&self) -> Result<Vec<String>, String> {
        match self.telem.lock() {
            Ok(data) => {
                println!("get errors {:?}", data.errors);
                Ok(data.errors.to_owned())
            }
            Err(_) => Err("Failed to lock telemetry".to_owned()),
        }
    }
}
