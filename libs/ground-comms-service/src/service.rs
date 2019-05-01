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

use crate::config::*;
use crate::errors::CommsResult;
use log::info;
use std::fmt::Debug;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Type definition for a "read" function pointer.
pub type ReadFn<T> = Fn(&T) -> CommsResult<Vec<u8>> + Send + Sync + 'static;
/// Type definition for a "write" function pointer.
pub type WriteFn<T> = Fn(&T, &[u8]) -> CommsResult<()> + Send + Sync + 'static;

pub trait CommsConnection {
    /// Type definition for a "read" function pointer.
    fn read(&self) -> CommsResult<Vec<u8>>;

    /// Type definition for a "write" function pointer.
    fn write(&self, data: &[u8]) -> CommsResult<()>;
}

/// Struct that holds configuration data to allow users to set up a Ground Communication Service.
#[derive(Clone)]
pub struct CommsControlBlock<T: Clone + CommsConnection, U: Clone + CommsConnection> {
    /// Connection to gateway
    pub gateway_conn: Arc<Mutex<T>>,
    /// Connection to radio
    pub radio_conn: Arc<Mutex<U>>,
    /// Timeout for the completion of send/receive operations in ms
    pub timeout: u64,
    /// IP address of the ground comms service.
    pub ground_ip: Ipv4Addr,
    /// IP address of the ground gateway.
    pub gateway_ip: Ipv4Addr,
    /// Specifies the port to which the ground gateway is bound.
    pub gateway_port: u16,
    /// Specifies the port on which the ground comms service listens.
    pub ground_port: u16,
}

impl<T: CommsConnection + Clone + Debug, U: Clone + CommsConnection + Debug> Debug
    for CommsControlBlock<T, U>
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(
            f,
            "CommsControlBlock {{ gateway_conn: {:?}, radio_conn: {:?},
            ground: {}:{}, gateway: {}:{} }}",
            self.gateway_conn,
            self.radio_conn,
            self.ground_ip,
            self.ground_port,
            self.gateway_ip,
            self.gateway_port
        )
    }
}

impl<T: Clone + CommsConnection, U: Clone + CommsConnection> CommsControlBlock<T, U> {
    /// Creates a new instance of the CommsControlBlock
    pub fn new(
        gateway_conn: Arc<Mutex<T>>,
        radio_conn: Arc<Mutex<U>>,
        config: CommsConfig,
    ) -> CommsResult<Self> {
        Ok(CommsControlBlock {
            gateway_conn,
            radio_conn,
            timeout: config.timeout.unwrap_or(DEFAULT_TIMEOUT),
            ground_ip: Ipv4Addr::from_str(&config.ground_ip)?,
            gateway_ip: Ipv4Addr::from_str(&config.gateway_ip)?,
            ground_port: config.ground_port,
            gateway_port: config.gateway_port,
        })
    }
}

/// Struct that enables users to start the Communication Service.
pub struct CommsService;

impl CommsService {
    /// Starts an instance of the Communication Service and its associated background threads.
    pub fn start<
        T: CommsConnection + Debug + Clone + Send + 'static,
        U: CommsConnection + Debug + Clone + Send + 'static,
    >(
        control: CommsControlBlock<T, U>,
    ) -> CommsResult<()> {
        // Spawn a radio read thread
        let control_ref = control.clone();
        thread::spawn(move || comms_loop_thread(control_ref.radio_conn, control_ref.gateway_conn));

        // Spawn gateway read thread
        let control_ref = control.clone();
        thread::spawn(move || comms_loop_thread(control_ref.gateway_conn, control_ref.radio_conn));

        info!("Communication service started");
        Ok(())
    }
}

pub fn comms_loop_thread<
    T: CommsConnection + Debug + Clone + Send + 'static,
    U: CommsConnection + Debug + Clone + Send + 'static,
>(
    sender: Arc<Mutex<T>>,
    receiver: Arc<Mutex<U>>,
) {
    loop {
        let data: Option<Vec<u8>> = if let Ok(sender) = sender.lock() {
            // Attempt to read packet from the sender
            match sender.read() {
                Ok(bytes) => Some(bytes),
                Err(e) => {
                    warn!("Failed to read {:?}", e);
                    None
                }
            }
        } else {
            None
        };

        // Send packet to the receiver
        if let Some(data) = data {
            if let Ok(receiver) = receiver.lock() {
                info!("sending gateway -> radio {:?}", data);
                match receiver.write(&data) {
                    Ok(_) => {}
                    Err(e) => {
                        warn!("Failed to write {:?}", e);
                    }
                }
            }
        }

        thread::sleep(Duration::from_millis(10));
    }
}
