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
extern crate clap;
extern crate shell_protocol;
#[macro_use]
extern crate failure;

use channel_protocol::ChannelProtocol;
use clap::{App, Arg};
use failure::Error;
use std::collections::HashMap;
use std::io::{self, Write};
use std::time::Duration;

fn select_channel(channel_proto: &ChannelProtocol) -> Result<Option<u32>, Error> {
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
            let channels_list = process_list
                .iter()
                .map(|(channel_id, (_, _))| *channel_id)
                .collect::<Vec<u32>>();

            loop {
                println!("Choose an option:");
                println!("Press enter to start a new sh shell.");
                println!("Press Control-C to exit");
                println!("Or enter session ID to take over an existing session.");
                for (channel_id, (path, pid)) in process_list.iter() {
                    println!("{}\t{{ path = '{}', pid = {} }}", channel_id, path, pid);
                }

                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_n) => match input.as_ref() {
                        "\n" => return Ok(None),
                        s => match s.trim().parse::<u32>() {
                            Ok(n) => if channels_list.contains(&n) {
                                return Ok(Some(n));
                            },
                            _ => {}
                        },
                    },
                    _ => {}
                }
            }
        }
        _ => bail!("Shell service is not responding correctly".to_owned()),
    }
}

fn get_channel(channel_proto: &ChannelProtocol) -> Result<u32, Error> {
    let selected_channel = select_channel(&channel_proto)?;

    match selected_channel {
        Some(c) => Ok(c),
        None => {
            let channel_id = channel_protocol::generate_channel();
            channel_proto.send(shell_protocol::messages::spawn::to_cbor(
                channel_id,
                &"/bin/bash".to_owned(),
                None,
            )?)?;
            Ok(channel_id)
        }
    }
}

fn run_shell(
    channel_id: u32,
    remote_addr: &str,
    channel_proto: &ChannelProtocol,
) -> Result<(), Error> {
    loop {
        let mut input = String::new();
        print!("{}>", remote_addr);
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                channel_proto.send(shell_protocol::messages::stdin::to_cbor(
                    channel_id,
                    Some(&input),
                )?)?;

                loop {
                    match channel_proto.recv_message(Some(Duration::from_millis(100))) {
                        Ok(m) => match shell_protocol::messages::parse_message(m) {
                            Ok(shell_protocol::messages::Message::Stdout {
                                channel_id: _channel_id,
                                data: Some(data),
                            }) => print!("{}", data),
                            Ok(shell_protocol::messages::Message::Stderr {
                                channel_id: _channel_id,
                                data: Some(data),
                            }) => print!("{}", data),
                            Ok(shell_protocol::messages::Message::Exit { .. }) => {
                                return Ok(());
                            }
                            _ => {}
                        },
                        _ => break,
                    }
                }
            }
            Err(err) => bail!("Error encountered: {}", err),
        }
    }
}

fn main() {
    let args = App::new("Shell client")
        .arg(
            Arg::with_name("service_ip")
                .short("i")
                .takes_value(true)
                .default_value("0.0.0.0"),
        ).arg(
            Arg::with_name("service_port")
                .short("p")
                .takes_value(true)
                .default_value("8080"),
        ).get_matches();

    let ip = args.value_of("service_ip").unwrap();
    let port = args.value_of("service_port").unwrap();

    println!("Starting shell client -> {}:{}", ip, port);

    let channel_proto =
        channel_protocol::ChannelProtocol::new("0.0.0.0", &format!("{}:{}", ip, port), 4096);

    if let Ok(channel_id) = get_channel(&channel_proto) {
        match run_shell(channel_id, &format!("{}:{}", ip, port), &channel_proto) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error encountered: {}", e);
            }
        }
    }
}
