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
extern crate log;
extern crate simplelog;

use kubos_system::Config as ServiceConfig;
use std::thread;

use file_protocol::{messages, FileProtocol, State};

// We need this in this lib.rs file so we can build integration tests

pub fn recv_loop(config: ServiceConfig) -> Result<(), String> {
    let c_protocol = cbor_protocol::Protocol::new(config.hosturl());

    loop {
        let (source, first_message) = c_protocol.recv_message_peer()?;

        thread::spawn(move || {
            let mut state = State::Holding;

            let f_protocol = FileProtocol::new(String::from("127.0.0.1"), source.port());

            if let Some(msg) = first_message {
                if let Ok(new_state) = f_protocol.on_message(msg, state.clone()) {
                    state = new_state;
                }
            }
            loop {
                let message = match f_protocol.recv(None) {
                    Ok(message) => (message),
                    Err(_e) => {
                        // Probably should check the type of error...
                        // For now we'll assume its just no msg received
                        match state.clone() {
                            State::Receiving {
                                channel_id,
                                hash,
                                path,
                                mode,
                            } => match f_protocol.local_export(&hash, &path, mode) {
                                Ok(_) => {
                                    f_protocol
                                        .send(messages::success(channel_id).unwrap())
                                        .unwrap();
                                    return;
                                }
                                Err(e) => {
                                    f_protocol
                                        .send(messages::failure(channel_id, &e).unwrap())
                                        .unwrap();
                                    break;
                                }
                            },
                            _ => {}
                        }
                        continue;
                    }
                };
                if let Some(msg) = message.clone() {
                    if let Ok(new_state) = f_protocol.on_message(msg, state.clone()) {
                        state = new_state;
                    }
                }
            }
        });
    }
}
