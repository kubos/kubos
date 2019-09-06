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
use log::{error, info};
use serde::Deserialize;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};

pub struct DirectUdp {
    db: Arc<Mutex<Database>>,
}

#[derive(Debug, Deserialize)]
struct DataPoint {
    timestamp: Option<f64>,
    subsystem: String,
    parameter: String,
    value: String,
}

impl DirectUdp {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        DirectUdp { db }
    }

    pub fn start(&self, url: String) {
        let socket = UdpSocket::bind(url.parse::<SocketAddr>().unwrap_or_else(|err| {
            error!(
                "Couldn't start direct UDP connection. Failed to parse {}: {:?}",
                url, err
            );
            panic!()
        }))
        .unwrap_or_else(|err| {
            error!(
                "Couldn't start direct UDP connection. Failed to bind {}: {:?}",
                url, err
            );
            panic!()
        });

        info!("Direct UDP listening on: {}", socket.local_addr().unwrap());

        loop {
            // Wait for an incoming message
            let mut buf = [0; 4096];
            let (size, _peer) = socket
                .recv_from(&mut buf)
                .map_err(|err| format!("Failed to receive a message: {}", err))
                .unwrap();

            if let Ok(val) = serde_json::from_slice::<DataPoint>(&buf[0..(size)]) {
                if let Err(err) = self.process(&val) {
                    error!("Error {:?} storing message {:?}", err, val);
                }
            } else if let Ok(vec) = serde_json::from_slice::<Vec<DataPoint>>(&buf[0..(size)]) {
                for val in vec.iter() {
                    if let Err(err) = self.process(&val) {
                        error!("Error {:?} storing message {:?}", err, val);
                    }
                }
            } else {
                error!(
                    "Couldn't deserialize JSON object or object array from {:?}",
                    String::from_utf8_lossy(&buf[0..(size)].to_vec())
                );
            }
        }
    }

    fn process(&self, message: &DataPoint) -> Result<(), String> {
        if let Some(time) = message.timestamp {
            self.db
                .lock()
                .map_err(|err| {
                    error!("udp - Failed to get lock on database: {}", err);
                    format!("{}", err)
                })?
                .insert(time, &message.subsystem, &message.parameter, &message.value)
                .map_err(|err| {
                    error!("udp - Failed to get lock on database: {}", err);
                    format!("{}", err)
                })?;
        } else {
            self.db
                .lock()
                .map_err(|err| {
                    error!("udp - Failed to get lock on database: {}", err);
                    format!("{}", err)
                })?
                .insert_systime(&message.subsystem, &message.parameter, &message.value)
                .map_err(|err| {
                    error!("udp - Failed to get lock on database: {}", err);
                    format!("{}", err)
                })?;
        }

        Ok(())
    }
}
