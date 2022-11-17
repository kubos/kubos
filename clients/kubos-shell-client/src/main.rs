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

#![deny(warnings)]
use channel_protocol::{ChannelProtocol, ProtocolError};
use clap::{value_t, App, AppSettings, Arg, SubCommand};
use failure::{bail, Error};
use std::collections::HashMap;
use std::io::{self, Write};
use std::time::Duration;

fn start_session(channel_proto: &ChannelProtocol) -> Result<(), Error> {
    let channel_id = channel_protocol::generate_channel();

    println!("Starting shell session -> {}", channel_id);

    channel_proto.send(&shell_protocol::messages::spawn::to_cbor(
        channel_id, "/bin/sh", None,
    )?)?;
    run_shell(channel_proto, channel_id)?;
    Ok(())
}

fn run_command(channel_proto: &ChannelProtocol, command: &str) -> Result<(), Error> {
    let channel_id = channel_protocol::generate_channel();
    let command = format!("{}\n", command);

    channel_proto.send(&shell_protocol::messages::spawn::to_cbor(
        channel_id, "/bin/sh", None,
    )?)?;

    channel_proto.send(&shell_protocol::messages::stdin::to_cbor(
        channel_id,
        Some(&command),
    )?)?;

    loop {
        let recvd_data = match channel_proto.recv_message(Some(Duration::from_millis(100))) {
            Ok(data) => data,
            Err(ProtocolError::ReceiveTimeout) => break,
            Err(e) => {
                eprintln!("Recv Error: {}", e);
                break;
            }
        };
        let parsed_msg = match shell_protocol::messages::parse_message(&recvd_data) {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Msg Parse Error: {}", e);
                break;
            }
        };

        match parsed_msg {
            shell_protocol::messages::Message::Exit { .. } => {
                break;
            }
            shell_protocol::messages::Message::Stdout {
                data: Some(data), ..
            } => {
                print!("{}", data);
            }
            shell_protocol::messages::Message::Stderr {
                data: Some(data), ..
            } => {
                print!("{}", data);
            }
            shell_protocol::messages::Message::Error { message, .. } => {
                eprintln!("Shell Protocol Error: {}", message);
                break;
            }
            _ => {}
        }
    }

    channel_proto.send(&shell_protocol::messages::kill::to_cbor(channel_id, None)?)?;

    Ok(())
}

fn list_sessions(channel_proto: &ChannelProtocol) -> Result<(), Error> {
    channel_proto.send(&shell_protocol::messages::list::to_cbor(
        channel_protocol::generate_channel(),
        None,
    )?)?;

    let parsed_msg = shell_protocol::messages::parse_message(
        &channel_proto.recv_message(Some(Duration::from_millis(100)))?,
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

            if process_list.is_empty() {
                println!("\tNo active sessions");
            } else {
                for (channel_id, (path, pid)) in process_list.iter() {
                    println!("\t{}\t{{ path = '{}', pid = {} }}", channel_id, path, pid);
                }
            }
        }
        _ => bail!("Shell service is not responding correctly".to_owned()),
    }

    Ok(())
}

fn kill_session(
    channel_proto: &ChannelProtocol,
    channel_id: u32,
    signal: Option<u32>,
) -> Result<(), Error> {
    channel_proto.send(&shell_protocol::messages::kill::to_cbor(
        channel_id, signal,
    )?)?;
    Ok(())
}

fn run_shell(channel_proto: &ChannelProtocol, channel_id: u32) -> Result<(), Error> {
    println!("Press enter to send input to the shell session");
    println!("Press Control-D to detach from the session");
    loop {
        let mut input = String::new();
        print!(" $ ");
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    return Ok(());
                }

                channel_proto.send(&shell_protocol::messages::stdin::to_cbor(
                    channel_id,
                    Some(&input),
                )?)?;

                while let Ok(m) = channel_proto.recv_message(Some(Duration::from_millis(100))) {
                    match shell_protocol::messages::parse_message(&m) {
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
                        Ok(shell_protocol::messages::Message::Error { message, .. }) => {
                            eprintln!("Error received from service: {}", message);
                            return Ok(());
                        }
                        _ => {}
                    }
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
                        .help("Channel ID of shell session to join")
                        .short("c")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("kill")
                .about("Kills an existing shell session")
                .arg(
                    Arg::with_name("channel_id")
                        .help("Channel ID of shell session to kill")
                        .short("c")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("signal")
                        .help("Signal to send to shell session")
                        .short("s")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs a single remote command")
                .arg(
                    Arg::with_name("command")
                        .help("Remote command to execute")
                        .short("c")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .arg(
            Arg::with_name("service_ip")
                .help("IP address of remote shell service")
                .short("i")
                .takes_value(true)
                .default_value("0.0.0.0"),
        )
        .arg(
            Arg::with_name("service_port")
                .help("Port number of remote shell service")
                .short("p")
                .takes_value(true)
                .default_value(shell_protocol::PORT),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .get_matches();

    let ip = args.value_of("service_ip").unwrap();
    let port = args.value_of("service_port").unwrap();
    let remote = format!("{}:{}", ip, port);
    let channel_proto =
        channel_protocol::ChannelProtocol::new("0.0.0.0", &remote, shell_protocol::CHUNK_SIZE);

    println!("Starting shell client -> {}", remote);

    match args.subcommand_name() {
        Some("start") => start_session(&channel_proto),
        Some("run") => {
            let command = if let Some(command_matches) = args.subcommand_matches("run") {
                command_matches.value_of("command").unwrap()
            } else {
                bail!("No arguments found for run");
            };

            println!("Running remote command: '{}'", &command);
            run_command(&channel_proto, command)
        }
        Some("list") => {
            println!("Fetching existing shell sessions:");
            list_sessions(&channel_proto)
        }
        Some("join") => {
            let channel_id = if let Some(kill_args) = args.subcommand_matches("join") {
                value_t!(kill_args, "channel_id", u32).unwrap_or_else(|e| e.exit())
            } else {
                bail!("No arguments found for join");
            };

            println!("Joining existing shell session: {}", channel_id);
            run_shell(&channel_proto, channel_id)
        }
        Some("kill") => {
            let channel_id = if let Some(kill_args) = args.subcommand_matches("kill") {
                value_t!(kill_args, "channel_id", u32).unwrap_or_else(|e| e.exit())
            } else {
                bail!("No arguments found for kill");
            };

            let signal = if let Some(kill_args) = args.subcommand_matches("kill") {
                if let Ok(s) = value_t!(kill_args, "signal", u32) {
                    if s > 0 && s < 35 {
                        Some(s)
                    } else {
                        bail!("Invalid signal specified");
                    }
                } else {
                    None
                }
            } else {
                None
            };

            println!(
                "Killing existing shell session {} with signal {}",
                channel_id,
                signal.unwrap_or(9)
            );
            kill_session(&channel_proto, channel_id, signal)
        }
        _ => panic!("Invalid command"),
    }
}
