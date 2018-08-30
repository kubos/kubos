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
use std::time::Duration;

use file_protocol::{FileProtocol, State};

// We need this in this lib.rs file so we can build integration tests
pub fn recv_loop(config: ServiceConfig) -> Result<(), String> {
    let c_protocol = cbor_protocol::Protocol::new(config.hosturl());

    loop {
        let (source, first_message) = c_protocol.recv_message_peer()?;

        thread::spawn(move || {
            let mut state = State::Holding { count: 0 };

            let f_protocol = FileProtocol::new(String::from("127.0.0.1"), source.port());

            if let Some(msg) = first_message {
                if let Ok(new_state) = f_protocol.on_message(msg, state.clone()) {
                    state = new_state;
                }
            }
            f_protocol
                .message_engine(Duration::from_secs(2), state)
                .unwrap();
        });
    }
}
