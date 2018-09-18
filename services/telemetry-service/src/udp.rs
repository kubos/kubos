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

use kubos_telemetry_db::Database;
use serde_json::{self, Value};
use std::sync::{Arc, Mutex};
use std::net::{SocketAddr, UdpSocket};

pub struct DirectUdp {
    db: Arc<Mutex<Database>>,
}

impl DirectUdp {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        DirectUdp { db }
    }

    pub fn start(&self, url: String) {
        let socket = UdpSocket::bind(url.parse::<SocketAddr>().unwrap()).unwrap();
        println!("Direct UDP listening on: {}", socket.local_addr().unwrap());

        loop {
            // Wait for an incoming message
            let mut buf = [0; 4096];
            let (size, peer) = socket
                .recv_from(&mut buf)
                .map_err(|err| format!("Failed to receive a message: {}", err))
                .unwrap();

            if let Ok(msg) = serde_json::from_slice(&buf[0..(size)]) {
                // Go process the request
                let res = self.process(msg);
            }
        }
    }

    fn process(&self, message: Value) -> Result<(), String> {
        let timestamp = serde_json::from_value::<i32>(message["timestamp"].clone()).ok();

        let subsystem = serde_json::from_value::<String>(message["subsystem"].clone())
            .map_err(|err| format!("Failed to parse subsystem parameter: {}", err))?;

        let param = serde_json::from_value::<String>(message["parameter"].clone())
            .map_err(|err| format!("Failed to parse parameter parameter: {}", err))?;

        let value = serde_json::from_value::<String>(message["value"].clone())
            .map_err(|err| format!("Failed to parse value parameter: {}", err))?;

        if let Some(time) = timestamp {
            self.db
                .lock()
                .map_err(|err| format!("{}", err))?
                .insert(time, &subsystem, &param, &value)
                .map_err(|err| format!("{}", err))?;
        } else {
            self.db
                .lock()
                .map_err(|err| format!("{}", err))?
                .insert_systime(&subsystem, &param, &value)
                .map_err(|err| format!("{}", err))?;
        }

        Ok(())
    }
}
