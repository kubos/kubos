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

extern crate channel_protocol;
#[macro_use]
extern crate clap;
extern crate shell_protocol;
#[macro_use]
extern crate failure;

use channel_protocol::ChannelProtocol;
use clap::{App, AppSettings, Arg, SubCommand};
use failure::Error;
use std::collections::HashMap;
use std::io::{self, Write};
use std::time::Duration;

fn start_session(channel_proto: ChannelProtocol, remote: &str) -> Result<(), Error> {
    let channel_id = channel_protocol::generate_channel();
    channel_proto.send(shell_protocol::messages::spawn::to_cbor(
        channel_id,
        &"/bin/bash".to_owned(),
        None,
    )?)?;
    run_shell(channel_proto, remote, channel_id)?;
    Ok(())
}

fn list_sessions(channel_proto: ChannelProtocol) -> Result<(), Error> {
    channel_proto.send(shell_protocol::messages::list::to_cbor(
        channel_protocol::generate_channel(),
        None,
    )?)?;

    let parsed_msg = shell_protocol::messages::parse_message(
        channel_proto.recv_message(Some(Duration::from_millis(100)))?,
    )?;

    match parsed_msg {
        shell_protocol::messages::Message::List {
            channel_id: _channel_id,
            process_list,
        } => {
            let process_list = match process_list {
                Some(l) => l,
                None => HashMap::<u32, (String, u32)>::new(),
            };

            for (channel_id, (path, pid)) in process_list.iter() {
                println!("{}\t{{ path = '{}', pid = {} }}", channel_id, path, pid);
            }
        }
        _ => bail!("Shell service is not responding correctly".to_owned()),
    }

    Ok(())
}

fn kill_session(
    channel_proto: ChannelProtocol,
    channel_id: u32,
    signal: Option<u32>,
) -> Result<(), Error> {
    channel_proto.send(shell_protocol::messages::kill::to_cbor(channel_id, signal)?)?;
    Ok(())
}

fn run_shell(
    channel_proto: ChannelProtocol,
    remote_addr: &str,
    channel_id: u32,
) -> Result<(), Error> {
    loop {
        let mut input = String::new();
        print!("{}>$ ", remote_addr);
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    return Ok(());
                }

                channel_proto.send(shell_protocol::messages::stdin::to_cbor(
                    channel_id,
                    Some(&input),
                )?)?;

                let mut received_msg = false;
                loop {
                    match channel_proto.recv_message(Some(Duration::from_millis(100))) {
                        Ok(m) => {
                            received_msg = true;
                            match shell_protocol::messages::parse_message(m) {
                                Ok(shell_protocol::messages::Message::Stdout {
                                    channel_id: _channel_id,
                                    data: Some(data),
                                }) => print!("{}", data),
                                Ok(shell_protocol::messages::Message::Stderr {
                                    channel_id: _channel_id,
                                    data: Some(data),
                                }) => eprint!("{}", data),
                                Ok(shell_protocol::messages::Message::Exit { .. }) => {
                                    return Ok(());
                                }
                                _ => {}
                            }
                        }
                        _ => break,
                    }
                }

                if !received_msg {
                    bail!("No message received from shell service");
                }
            }
            Err(err) => bail!("Error encountered: {}", err),
        }
    }
}

fn main() -> Result<(), failure::Error> {
    let args = App::new("Shell client")
        .subcommand(SubCommand::with_name("start").about("Starts new shell session"))
        .subcommand(SubCommand::with_name("list").about("Lists existing shell sessions"))
        .subcommand(
            SubCommand::with_name("join")
                .about("Joins an existing shell session")
                .arg(
                    Arg::with_name("channel_id")
                        .short("c")
                        .takes_value(true)
                        .required(true),
                ),
        ).subcommand(
            SubCommand::with_name("kill")
                .about("Kills an existing shell session")
                .arg(
                    Arg::with_name("channel_id")
                        .short("c")
                        .takes_value(true)
                        .required(true),
                ).arg(Arg::with_name("signal").short("s").takes_value(true)),
        ).arg(
            Arg::with_name("service_ip")
                .short("i")
                .takes_value(true)
                .default_value("0.0.0.0"),
        ).arg(
            Arg::with_name("service_port")
                .short("p")
                .takes_value(true)
                .default_value(shell_protocol::PORT),
        ).setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    let ip = args.value_of("service_ip").unwrap();
    let port = args.value_of("service_port").unwrap();
    let remote = format!("{}:{}", ip, port);
    let channel_proto =
        channel_protocol::ChannelProtocol::new("0.0.0.0", &remote, shell_protocol::CHUNK_SIZE);

    match match args.subcommand_name() {
        Some("start") => {
            println!("starting new shell session");
            start_session(channel_proto, &remote)
        }
        Some("list") => {
            println!("listing existing shell sessions");
            list_sessions(channel_proto)
        }
        Some("join") => {
            let channel_id = args
                .subcommand_matches("join")
                .unwrap()
                .value_of("channel_id")
                .unwrap();

            println!("Joining existing shell session: {}", channel_id);
            let channel_id = channel_id.parse::<u32>()?;
            run_shell(channel_proto, &remote, channel_id)
        }
        Some("kill") => {
            let channel_id = args
                .subcommand_matches("kill")
                .unwrap()
                .value_of("channel_id")
                .unwrap();

            let signal: Option<u32> = args
                .subcommand_matches("kill")
                .unwrap()
                .value_of("signal")
                .map(|s| {
                    s.parse::<u32>()
                        .map_err(|e| panic!("Failed to parse signal: {}", e))
                        .unwrap()
                });

            println!("Sending signal {:?} to session {}", signal, channel_id);
            let channel_id = channel_id.parse::<u32>()?;
            kill_session(channel_proto, channel_id, signal)
        }
        _ => panic!("Invalid command"),
    } {
        Ok(_) => Ok(()),
        Err(e) => bail!("Error encountered: {}", e),
    }
}
