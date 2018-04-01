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

use juniper::{Context, GraphQLType, RootNode};

struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}

impl Future for Server {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        loop {
            // if let Some((size, peer)) = self.to_send {
            //     let query_string =
            //         String::from_utf8(self.buf[0..(size - 1)].to_vec()).unwrap();

            //     let res = self.process(query_string);
            //     println!("{:?}", res);
            //     let amt = try_ready!(self.socket.poll_send_to(&res.as_bytes(), &peer));
            // }

            if let Some((size, peer)) = self.to_send {
                let amt = try_ready!(self.socket.poll_send_to(&self.buf, &peer));
            }

            self.to_send = Some(try_ready!(self.socket.poll_recv_from(&mut self.buf)));
        }
    }
}

pub struct KubosService<'a, CtxFactory, Query, Mutation, CtxT>
where
    CtxFactory: Fn() -> CtxT + Send + Sync + 'static,
    CtxT: 'static,
    Query: GraphQLType<Context = CtxT> + Send + Sync + 'static,
    Mutation: GraphQLType<Context = CtxT> + Send + Sync + 'static,
{
    context_factory: CtxFactory,
    root_node: RootNode<'a, Query, Mutation>,
}

impl<'a, CtxFactory, Query, Mutation, CtxT> KubosService<'a, CtxFactory, Query, Mutation, CtxT>
where
    CtxFactory: Fn() -> CtxT + Send + Sync + 'static,
    CtxT: 'static,
    Query: GraphQLType<Context = CtxT, TypeInfo = ()> + Send + Sync + 'static,
    Mutation: GraphQLType<Context = CtxT, TypeInfo = ()> + Send + Sync + 'static,
{
    /// Build a new Kubos
    pub fn new(context_factory: CtxFactory, query: Query, mutation: Mutation) -> Self {
        KubosService {
            context_factory: context_factory,
            root_node: RootNode::new(query, mutation),
        }
    }

    pub fn process(&self, query: String) -> String {
        match juniper::execute(
            &query,
            None,
            &self.root_node,
            &juniper::Variables::new(),
            &((self.context_factory)()),
        ) {
            Ok((val, errs)) => return serde_json::to_string(&val).unwrap(),
            Err(e) => return serde_json::to_string(&e).unwrap(),
        }
    }

    pub fn start(&self) {
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
}

// pub fn start(p: fn(String) -> String) {
//     impl Future for Server {
//         type Item = ();
//         type Error = io::Error;

//         fn poll(&mut self) -> Poll<(), io::Error> {
//             loop {
//                 if let Some((size, peer)) = self.to_send {
//                     let query_string = String::from_utf8(self.buf[0..(size - 1)].to_vec()).unwrap();

//                     let res = (self.process)(query_string);
//                     println!("{:?}", res);
//                     let amt = try_ready!(self.socket.poll_send_to(&res.as_bytes(), &peer));
//                 }

//                 self.to_send = Some(try_ready!(self.socket.poll_recv_from(&mut self.buf)));
//             }
//         }
//     }

//     let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
//     let addr = addr.parse::<SocketAddr>().unwrap();

//     let socket = UdpSocket::bind(&addr).unwrap();
//     println!("Listening on: {}", socket.local_addr().unwrap());

//     let server = Server {
//         socket: socket,
//         buf: vec![0; 1024],
//         to_send: None,
//         process: p,
//     };

//     // This starts the server task.
//     //
//     // `map_err` handles the error by logging it and maps the future to a type
//     // that can be spawned.
//     //
//     // `tokio::run` spanws the task on the Tokio runtime and starts running.
//     tokio::run(server.map_err(|e| println!("server error = {:?}", e)));
// }
