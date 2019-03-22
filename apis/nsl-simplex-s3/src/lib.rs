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

use failure::Error;

const ACK: [u8; 3] = [0xAA, 0x05, 0x00];
const NAK: [u8; 3] = [0xAA, 0x05, 0xFF];
const RESP_LEN: u8 = 3;

pub struct SimplexS3 {
    /// Device connection structure
    pub conn: Arc<Mutex<Connection>>,
}

impl SimplexS3 {
    pub fn new(
        bus: &str,
        baud_rate: serial::BaudRate,
        log_recv: Receiver<(Header, Vec<u8>)>,
        response_recv: Receiver<(Header, Vec<u8>)>,
    ) -> OEMResult<OEM6> {
        let settings = serial::PortSettings {
            baud_rate,
            char_size: CHAR_SIZE,
            parity: PARITY,
            stop_bits: STOP_BITS,
            flow_control: FLOW_CONTROL,
        };

        let conn = Arc::new(Mutex::new(Connection::from_path(bus, settings, TIMEOUT)?));

        Ok(SimplexS3 {
            conn
        })
    }
    
    pub fn send_beacon(&self, msg: &[u8]) -> Result<(), Error> {
        
        if msg.len() > 35 {
            bail!("Packet too large");
        }
        
        let mut packet = vec![ 0x50, 0x50, 0x50 ];
        packet.push(msg);
        
        // TODO: Check for busy signal
        
        let conn = self.conn.lock()?;
        
        for _ in 0..5 {
        
            conn.write(packet)?;
            
            let response = conn.read(RESP_LEN, Duration::from_millis(10))?;
            
            match response {
                ACK => {
                    info!("Sent beacon");
                    return Ok(());
                }
                NAK => continue,
                other => warn!("Unknown resp from simplex: {:?}", other)
            }
        }
        
        bail!("Failed to send message");
    }
}

