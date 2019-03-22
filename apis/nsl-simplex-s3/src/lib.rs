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

#[cfg(test)]
mod tests;

use failure::{bail, Error};
use log::*;
use rust_uart::Connection;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const ACK: &[u8] = &[0xAA, 0x05, 0x00];
const NAK: &[u8] = &[0xAA, 0x05, 0xFF];
const RESP_LEN: usize = 3;

#[derive(Clone)]
pub struct SimplexS3 {
    /// Device connection structure
    pub conn: Arc<Mutex<Connection>>,
}

impl SimplexS3 {
    pub fn new(bus: &str) -> Result<SimplexS3, Error> {
        let settings = serial::PortSettings {
            baud_rate: serial::Baud38400,
            char_size: serial::Bits8,
            parity: serial::ParityNone,
            stop_bits: serial::Stop1,
            flow_control: serial::FlowNone,
        };

        let conn = Arc::new(Mutex::new(Connection::from_path(
            bus,
            settings,
            Duration::from_millis(60),
        )?));

        Ok(SimplexS3 { conn })
    }

    pub fn send_beacon(&self, msg: &[u8]) -> Result<(), Error> {
        if msg.len() > 35 {
            bail!("Packet too large");
        }

        let mut packet = vec![0x50, 0x50, 0x50];
        packet.extend_from_slice(msg);

        // TODO: Check for busy signal

        let conn = self
            .conn
            .lock()
            .or_else(|err| bail!("Mutex is poisoned: {:?}", err))?;

        for _ in 0..5 {
            conn.write(&packet)?;

            let response = conn.read(RESP_LEN, Duration::from_millis(10))?;

            match response.as_slice() {
                ACK => {
                    info!("Sent beacon");
                    return Ok(());
                }
                NAK => continue,
                other => warn!("Unknown resp from simplex: {:?}", other),
            }
        }

        bail!("Failed to send message");
    }
}
