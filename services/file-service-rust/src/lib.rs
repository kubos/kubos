extern crate file_protocol;
extern crate kubos_system;
#[macro_use]
extern crate log;
extern crate cbor_protocol;
extern crate simplelog;

use kubos_system::Config as ServiceConfig;
use simplelog::*;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

use file_protocol::{FileProtocol, Role, State};

// We need this in this lib.rs file so we can build integration tests

pub fn recv_loop(config: ServiceConfig) -> Result<(), String> {
    let c_protocol = cbor_protocol::Protocol::new(config.hosturl());

    loop {
        let (source, first_message) = c_protocol.recv_message_peer()?;

        thread::spawn(move || {
            let mut state = State::Holding;

            let f_protocol =
                FileProtocol::new(String::from("127.0.0.1"), source.port(), Role::Server);

            if let Some(msg) = first_message {
                let (_, n_state) = f_protocol.on_message(msg, &state, None).unwrap();
                state = n_state;
            }
            loop {
                
                let message = match f_protocol.recv(None) {
                    Ok(message) => (message),
                    Err(e) => {
                        // Probably should check the type of error...
                        // For now we'll assume its just no msg received
                        match &state {
                            State::Receiving(channel, hash, path, mode) => {
                                match f_protocol.local_export(&hash, &path, *mode) {
                                    Ok(_) => {
                                        // new_state = State::Holding;
                                        f_protocol.send_success(*channel);
                                        return;
                                    }
                                    Err(e) => {
                                        f_protocol.send_failure(*channel, &e);
                                        break;
                                    }
                                }
                            }
                            _ => {}
                        }
                        continue;
                    }
                };
                if let Some(msg) = message.clone() {
                    let (_, state) = f_protocol.on_message(msg, &state, None).unwrap();
                }
            }
        });
    }
}
