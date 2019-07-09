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
// Example client "radio" over UART program
//
// Wraps the user's data in a SpacePacket and then sends it over the UART port
// The example communication service, `uart-comms-service`, should be running and listening for
// these messages.
// The service will forward the message on to the requested destination port and then return the
// response once the request has completed.
//
// Note: Currently this client can only be used to send/receive GraphQL requests
//
// Packets can be additionally encapsulated using the KISS protocol to simulate additional
// radio-specific framing. Resulting packet is `KISS<UDP<payload>>`.

mod comms;
mod kiss;

use clap::{App, Arg};
use comms_service::{LinkPacket, PayloadType, SpacePacket};
use failure::{bail, Error};
use std::fs::File;
use std::io::Read;
use std::time::Duration;

// Return type for this service.
type ClientResult<T> = Result<T, Error>;

fn main() -> ClientResult<()> {
    let args = App::new("UART Comms Client")
        .arg(
            Arg::with_name("bus")
                .help("Serial Device")
                .short("b")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("port")
                .help("Destination port")
                .short("p")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("file")
                .help("File containing data to send")
                .short("f")
                .takes_value(true)
                .conflicts_with("listen"),
        )
        .arg(
            Arg::with_name("data")
                .help("Data to send")
                .required_unless_one(&["file", "listen"])
                .conflicts_with("file")
                .conflicts_with("listen"),
        )
        .arg(
            Arg::with_name("kiss")
                .help("Enable KISS framing")
                .short("k"),
        )
        .get_matches();

    let bus = args.value_of("bus").unwrap();
    let dest_port = args.value_of("port").unwrap().parse()?;

    let query = if let Some(file) = args.value_of("file") {
        let mut raw = String::new();
        File::open(file).and_then(|mut f| f.read_to_string(&mut raw))?;
        raw
    } else {
        args.value_of("data").unwrap().to_string()
    };
    
    let mut map = ::std::collections::HashMap::new();
    map.insert("query", query);

    // Build SpacePacket around payload message
    let packet = SpacePacket::build(0, PayloadType::GraphQL, dest_port, &serde_json::to_vec(&map)?)
        .and_then(|packet| packet.to_bytes())?;

    let packet = if args.is_present("kiss") {
        // Add KISS framing
        kiss::encode(&packet)
    } else {
        packet
    };

    let mut conn = comms::serial_init(bus)?;

    // Write our request out through the "radio"
    comms::write(&mut conn, &packet)?;

    // Get our response
    let msg = comms::read(&mut conn)?;

    let msg = if args.is_present("kiss") {
        // Parse the KISS frame
        let (frame, _, _) = kiss::decode(&msg)?;
        frame
    } else {
        msg
    };
    
    // Parse the returned SpacePacket
    let response = String::from_utf8(SpacePacket::parse(&msg)?.payload())?;

    println!("Response: {}", response);

    Ok(())
}
