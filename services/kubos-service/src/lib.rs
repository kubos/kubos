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

use std::env;
use std::net::{SocketAddr, UdpSocket};

use juniper::{GraphQLType, RootNode};

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
            Ok((val, _errs)) => {
                // Should do something with _errs
                return serde_json::to_string(&val).unwrap();
            }
            Err(_e) => {
                "Error running query".to_string()
                // Could also do this to retain the juniper error
                // return serde_json::to_string(&e).unwrap(),
            }
        }
    }

    pub fn start(&self) {
        let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
        let addr = addr.parse::<SocketAddr>().unwrap();

        let socket = UdpSocket::bind(&addr).unwrap();
        println!("Listening on: {}", socket.local_addr().unwrap());
        let mut buf = [0; 128];
        let mut to_send: Option<(usize, SocketAddr)> = None;
        loop {
            if let Some((size, peer)) = to_send {
                let query_string = String::from_utf8(buf[0..(size - 1)].to_vec()).unwrap();
                let res = self.process(query_string);
                let _amt = socket.send_to(&res.as_bytes(), &peer);
            }

            to_send = Some(socket.recv_from(&mut buf).unwrap());
        }
    }
}
