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

extern crate cbor_protocol;
extern crate kubos_system;
extern crate shell_protocol;
#[macro_use]
extern crate log;
extern crate channel_protocol;
extern crate failure;
extern crate serde_cbor;
extern crate simplelog;

use kubos_system::Config as ServiceConfig;
use shell_protocol::{ProtocolError, ShellProtocol};
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// We need this in this lib.rs file so we can build integration tests
pub fn recv_loop(config: ServiceConfig) -> Result<(), failure::Error> {
    // Get and bind our UDP listening socket
    let host = config.hosturl();

    // Extract our local IP address so we can spawn child sockets later
    let mut host_parts = host.split(':').map(|val| val.to_owned());
    let host_ip = host_parts.next().unwrap();

    let c_protocol = cbor_protocol::Protocol::new(host.clone(), 4096);

    let timeout = config
        .get("timeout")
        .and_then(|val| {
            val.as_integer()
                .and_then(|num| Some(Duration::from_secs(num as u64)))
        }).unwrap_or(Duration::from_secs(2));

    // Setup map of channel IDs to thread channels
    let raw_threads: HashMap<u32, Sender<serde_cbor::Value>> = HashMap::new();
    // Create thread sharable wrapper
    let threads = Arc::new(Mutex::new(raw_threads));

    loop {
        // Listen on UDP port
        let (source, first_message) = match c_protocol.recv_message_peer() {
            Ok((source, first_message)) => (source, first_message),
            Err(e) => {
                warn!("Failed to receive message: {}", e);
                continue;
            }
        };

        let host_ref = host_ip.clone();
        let timeout_ref = timeout.clone();

        let channel_id = match channel_protocol::parse_channel_id(&first_message) {
            Ok(channel_id) => channel_id,
            Err(e) => {
                warn!("Error parsing channel ID: {:?}", e);
                continue;
            }
        };

        if !threads.lock().unwrap().contains_key(&channel_id) {
            let (sender, receiver): (
                Sender<serde_cbor::Value>,
                Receiver<serde_cbor::Value>,
            ) = mpsc::channel();
            threads.lock().unwrap().insert(channel_id, sender.clone());
            // Break the processing work off into its own thread so we can
            // listen for requests from other clients
            let shared_threads = threads.clone();
            thread::spawn(move || {
                let s_protocol = ShellProtocol::new(&host_ref, &format!("{}", source));

                // Listen, process, and react to the remaining messages in the
                // requested operation
                match s_protocol.message_engine(
                    |d| match receiver.recv_timeout(d) {
                        Ok(v) => Ok(v),
                        Err(RecvTimeoutError::Timeout) => Err(ProtocolError::ReceiveTimeout),
                        Err(e) => Err(ProtocolError::ReceiveError {
                            err: format!("Error {:?}", e),
                        }),
                    },
                    timeout_ref,
                ) {
                    Err(e) => warn!("Encountered errors while processing transaction: {}", e),
                    _ => {}
                }

                // Remove ourselves from threads list if we are finished
                shared_threads.lock().unwrap().remove(&channel_id);
            });
        }

        if let Some(sender) = threads.lock().unwrap().get(&channel_id) {
            match sender.send(first_message) {
                Err(e) => warn!("Error when sending to channel {}: {:?}", channel_id, e),
                _ => {}
            };
        }

        if !threads.lock().unwrap().contains_key(&channel_id) {
            warn!("No sender found for {}", channel_id);
            threads.lock().unwrap().remove(&channel_id);
        }
    }
}
