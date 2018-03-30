//
// Copyright (C) 2017 Kubos Corporation
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

#[macro_use]
extern crate juniper;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate futures;
extern crate tokio;

use std::{env, io};
use std::net::SocketAddr;

use tokio::prelude::*;
use tokio::net::UdpSocket;

struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}


mod model;
mod schema;

/// A context object is used in Juniper to provide out-of-band access to global
/// data when resolving fields. We will use it here to provide a Subsystem structure
/// with recently fetched data.
///
/// Since this function is called once for every request, it will fetch new
/// data with each request.
// fn context_factory(_: &mut Request) -> schema::Context {
//     schema::Context { subsystem: model::Subsystem::new() }
// }

use juniper::{EmptyMutation, RootNode};

type Schema = RootNode<'static, schema::QueryRoot, schema::MutationRoot>;

// fn main() {
//     // let graphql_endpoint =
//     //     GraphQLHandler::new(context_factory, schema::QueryRoot, schema::MutationRoot);

//     // let graphiql_endpoint = GraphiQLHandler::new("/graphiql");

//     // let mut mount = mount::Mount::new();
//     // mount.mount("/", graphql_endpoint);
//     // mount.mount("/graphiql", graphiql_endpoint);

//     // let (logger_before, logger_after) = logger::Logger::new(None);

//     // let mut chain = Chain::new(mount);
//     // chain.link_before(logger_before);
//     // chain.link_after(logger_after);

//     // let host = env::var("LISTEN").unwrap_or("0.0.0.0:8080".to_owned());
//     // println!("GraphQL server started on {}", host);
//     // Iron::new(chain).http(host.as_str()).unwrap();
//     let q = schema::QueryRoot;
//     let m = schema::MutationRoot;
//     let query = r#"
//         {
//           subsystem { power }
//         }"#;

//     let s = Schema::new(
//         schema::QueryRoot,
//         schema::MutationRoot
//     );

//     let res = juniper::execute(query, None, &s, &juniper::Variables::new(), &schema::Context {
//         subsystem: model::Subsystem::new()
//     });

//     println!("{:?}", serde_json::to_string(&res));
// }


impl Future for Server {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        loop {
            // First we check to see if there's a message we need to echo back.
            // If so then we try to send it back to the original source, waiting
            // until it's writable and we're able to do so.
            if let Some((size, peer)) = self.to_send {
                let mut query_string = String::from_utf8(self.buf[0..size-1].to_vec()).unwrap();
                let clean = query_string.replace(char::is_whitespace, "");
                // let query_string = r#"{subsystem{power}}"#;
                println!("Running\n{}", clean);
                let s = Schema::new(schema::QueryRoot, schema::MutationRoot);
                println!("Executing...");

                let mut amt = 0;
                match juniper::execute(&clean, None, &s, &juniper::Variables::new(), &schema::Context{subsystem: model::Subsystem::new()}) {
                    Ok(res) => amt = try_ready!(self.socket.poll_send_to(serde_json::to_string(&res).unwrap().as_bytes(), &peer)),
                    Err(e) => amt = try_ready!(self.socket.poll_send_to(serde_json::to_string(&e).unwrap().as_bytes(), &peer)),
                };
                println!("Sending back...");
                
                // let amt = try_ready!(self.socket.poll_send_to(&self.buf[..size], &peer));
                println!("Echoed {}/{} bytes to {}", amt, size, peer);
                self.to_send = None;
            }

            // If we're here then `to_send` is `None`, so we take a look for the
            // next message we're going to echo back.
            self.to_send = Some(try_ready!(self.socket.poll_recv_from(&mut self.buf)));
        }
    }
}

fn main() {
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    let socket = UdpSocket::bind(&addr).unwrap();
    println!("Listening on: {}", socket.local_addr().unwrap());

    let server = Server {
        socket: socket,
        buf: vec![0; 1024],
        to_send: None,
    };

    // This starts the server task.
    //
    // `map_err` handles the error by logging it and maps the future to a type
    // that can be spawned.
    //
    // `tokio::run` spanws the task on the Tokio runtime and starts running.
    tokio::run(server.map_err(|e| println!("server error = {:?}", e)));
}
