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

use cbor_protocol;
use kubos_telemetry_db::Database;
use serde_cbor::Value;
use std::sync::{Arc, Mutex};

pub struct DirectUdp {
    db: Arc<Mutex<Database>>,
}

impl DirectUdp {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        DirectUdp { db }
    }

    pub fn start(&self) {
        //let addr = self.config.hosturl().parse::<SocketAddr>().unwrap();
        let host = "0.0.0.0:8000";

        let socket = cbor_protocol::Protocol::new(host.to_owned());

        loop {
            // Wait for an incoming message
            let (peer, message) = socket
                .recv_message_peer()
                .expect("Failed to receive a message");

            if let Some(msg) = message {
                // Go process the request
                let res = self.process(msg);

                println!("Result: {:?}", res);

                // And then send the response back
                //let _amt = socket.send_to(&res.as_bytes(), &peer);
                //println!("[{}] -> [{}] {}", socket.local_addr().unwrap(), peer, &res);
            }
        }
    }

    fn process(&self, message: Value) -> Result<(), String> {
        println!("Received: {:?}", message);

        let data = match message {
            Value::Array(val) => val.to_owned(),
            _ => return Err("Unable to parse message: Data not an array".to_owned()),
        };

        let len = data.len();

        let mut pieces = data.iter();

        let timestamp = match len {
            3 => None,
            4 => match pieces.next().ok_or("Failed to read timestamp param")? {
                Value::U64(val) => Some(*val as i32),
                Value::I64(val) => Some(*val as i32),
                _ => return Err("Unable to parse message: Invalid timestamp param".to_owned()),
            },
            other => return Err(format!("Incorrect number of parameters: {}", other)),
        };

        let subsystem = match pieces
            .next()
            .ok_or(format!("Unable to parse message: No subsystem param"))?
        {
            Value::String(val) => val,
            _ => return Err("Unable to parse message: Invalid subsystem param".to_owned()),
        };

        let param = match pieces
            .next()
            .ok_or(format!("Unable to parse message: No parameter param"))?
        {
            Value::String(val) => val,
            _ => return Err("Unable to parse message: Invalid parameter param".to_owned()),
        };

        let value = match pieces
            .next()
            .ok_or(format!("Unable to parse message: No value param"))?
        {
            Value::String(val) => val,
            _ => return Err("Unable to parse message: Invalid value param".to_owned()),
        };

        println!(
            "Inserting: {:?}, {}, {}, {}",
            timestamp, subsystem, param, value
        );

        if let Some(time) = timestamp {
            self.db
                .lock()
                .map_err(|err| format!("{}", err))?
                .insert(time, subsystem, param, value)
                .map_err(|err| format!("{}", err))?;
        } else {
            self.db
                .lock()
                .map_err(|err| format!("{}", err))?
                .insert_systime(subsystem, param, value)
                .map_err(|err| format!("{}", err))?;
        }

        Ok(())
    }
}
