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
extern crate file_protocol;
extern crate kubos_system;
#[macro_use]
extern crate log;
extern crate serde_cbor;
extern crate simplelog;

use file_protocol::{FileProtocol, State};
use kubos_system::Config as ServiceConfig;
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

// We need this in this lib.rs file so we can build integration tests
pub fn recv_loop(config: ServiceConfig) -> Result<(), String> {
    // Get and bind our UDP listening socket
    let host = config.hosturl();
    let c_protocol = cbor_protocol::Protocol::new(host.clone());

    // Extract our local IP address so we can spawn child sockets later
    let mut host_parts = host.split(':').map(|val| val.to_owned());
    let host_ip = host_parts.next().unwrap();

    // Get the storage directory prefix that we'll be using for our
    // temporary/intermediate storage location
    let prefix = match config.get("storage_dir") {
        Some(val) => val.as_str().and_then(|str| Some(str.to_owned())),
        None => None,
    };

    let timeout = config
        .get("timeout")
        .and_then(|val| {
            val.as_integer()
                .and_then(|num| Some(Duration::from_secs(num as u64)))
        })
        .unwrap_or(Duration::from_secs(2));
    // Setup maps of channel_id -> tx channel
    let mut threads: HashMap<u64, Sender<Option<serde_cbor::Value>>> = HashMap::new();

    loop {
        // Listen on UDP port
        let (source, first_message) = c_protocol.recv_message_peer()?;
        info!("msg from {:?}", source);

        let prefix_ref = prefix.clone();
        let host_ref = host_ip.clone();
        let timeout_ref = timeout.clone();

        let channel_id = match cbor_protocol::peek_channel_id(&first_message) {
            Ok(Some(channel_id)) => channel_id,
            Ok(None) => 0,
            Err(e) => {
                warn!("Error parsing channel id: {:?}", e);
                continue;
            }
        };

        if !threads.contains_key(&channel_id) {
            println!("create new thread {}", channel_id);
            let (tx, rx): (
                Sender<Option<serde_cbor::Value>>,
                Receiver<Option<serde_cbor::Value>>,
            ) = mpsc::channel();
            threads.insert(channel_id, tx.clone());
            // Break the processing work off into its own thread so we can
            // listen for requests from other clients
            thread::spawn(move || {
                let mut state = State::Holding {
                    count: 0,
                    prev_state: Box::new(State::Done),
                };

                let my_channel = channel_id;

                let receiver = rx;

                // Set up the file system processor with the reply socket information
                let f_protocol = FileProtocol::new(&host_ref, &format!("{}", source), prefix_ref);

                // // Process that first message that we got
                // if let Some(msg) = first_message {
                //     if let Ok(new_state) = f_protocol.process_message(msg, state.clone()) {
                //         state = new_state;
                //     }
                // }

                // Listen, process, and react to the remaining messages in the
                // requested operation
                match f_protocol.message_engine(
                    |d| match receiver.recv_timeout(d) {
                        Ok(v) => Ok(v),
                        Err(RecvTimeoutError) => Ok(None),
                        Err(e) => Err(Some(format!("Error {:?}", e))),
                    },
                    timeout_ref,
                    state.clone(),
                ) {
                    Err(e) => warn!("Encountered errors while processing transaction: {}", e),
                    _ => {}
                }

                info!("thread {} done", my_channel);
            });
        }

        if let Some(sender) = threads.get(&channel_id) {
            info!("send msg {:?} to channel {}", first_message, channel_id);
            sender.send(first_message);
        } else {
            warn!("no sender found for {}", channel_id);
        }
    }
}

// Break the processing work off into its own thread so we can
// // listen for requests from other clients
// thread::spawn(move || {
//     let mut state = State::Holding {
//         count: 0,
//         prev_state: Box::new(State::Done),
//     };

//     // Set up the file system processor with the reply socket information
//     let f_protocol = FileProtocol::new(&host_ref, &format!("{}", source), prefix_ref);

//     // Process that first message that we got
//     if let Some(msg) = first_message {
//         if let Ok(new_state) = f_protocol.process_message(msg, state.clone()) {
//             state = new_state;
//         }
//     }

//     // Listen, process, and react to the remaining messages in the
//     // requested operation
//     match f_protocol.message_engine(
//         |d| f_protocol.recv(Some(d)),
//         Duration::from_secs(2),
//         state,
//     ) {
//         Err(e) => warn!("Encountered errors while processing transaction: {}", e),
//         _ => {}
//     }
// });
