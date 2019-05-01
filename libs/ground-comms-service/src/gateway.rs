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

use std::net::UdpSocket;
use std::str;
use std::sync::Arc;
use std::time::Duration;

use crate::errors::*;
use crate::CommsConnection;

#[derive(Clone, Debug)]
pub struct GatewayComms {
    socket: Arc<UdpSocket>,
    gateway_ip: String,
    gateway_port: u16,
}

impl GatewayComms {
    pub fn new(
        gateway_ip: &str,
        gateway_port: u16,
        comms_ip: &str,
        listen_port: u16,
    ) -> GatewayComms {
        let socket = UdpSocket::bind((comms_ip, listen_port)).unwrap();
        socket
            .set_read_timeout(Some(Duration::from_millis(500)))
            .unwrap();
        socket
            .set_write_timeout(Some(Duration::from_millis(500)))
            .unwrap();
        let socket = Arc::new(socket);

        info!(
            "GatewayComms\nListening on {}:{}\nSending to {}:{}",
            comms_ip, listen_port, gateway_ip, gateway_port
        );

        GatewayComms {
            socket,
            gateway_ip: gateway_ip.to_owned(),
            gateway_port,
        }
    }
}

impl CommsConnection for GatewayComms {
    fn read(&self) -> CommsResult<Vec<u8>> {
        let mut buf = [0; 4096];
        match self.socket.recv_from(&mut buf) {
            Ok((size, _)) if size > 0 => Ok(buf[0..size].to_vec()),
            Ok(_) => bail!("No Gateway data read"),
            Err(e) => bail!(CommsServiceError::ReadFailed(e.to_string())),
        }
    }

    fn write(&self, data: &[u8]) -> CommsResult<()> {
        match self
            .socket
            .send_to(data, (self.gateway_ip.as_str(), self.gateway_port))
        {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Gateway write failed {:?}", e.to_string());
                bail!(CommsServiceError::WriteFailed(e.to_string()))
            }
        }
    }
}
