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

use config::Config;
use std::net::{SocketAddr, UdpSocket};
use serde_json::to_string;
use juniper::{execute, Context as JuniperContext, GraphQLType, RootNode, Variables};

pub struct Context<T> {
    subsystem: T,
}

impl<T> JuniperContext for Context<T> {}

impl<T> Context<T> {
    pub fn get_subsystem(&self) -> &T {
        &self.subsystem
    }
}

pub struct KubosService<'a, Query, Mutation, S>
where
    Query: GraphQLType<Context = Context<S>> + Send + Sync + 'static,
    Mutation: GraphQLType<Context = Context<S>> + Send + Sync + 'static,
{
    pub config: Config,
    root_node: RootNode<'a, Query, Mutation>,
    context: Context<S>,
}

impl<'a, Query, Mutation, S> KubosService<'a, Query, Mutation, S>
where
    Query: GraphQLType<Context = Context<S>, TypeInfo = ()> + Send + Sync + 'static,
    Mutation: GraphQLType<Context = Context<S>, TypeInfo = ()> + Send + Sync + 'static,
{
    /// Build a new Kubos
    pub fn new(name: &str, subsystem: S, query: Query, mutation: Mutation) -> Self {
        KubosService {
            config: Config::new(&name),
            root_node: RootNode::new(query, mutation),
            context: Context {
                subsystem: subsystem,
            },
        }
    }

    pub fn process(&self, query: String) -> String {
        match execute(
            &query,
            None,
            &self.root_node,
            &Variables::new(),
            &self.context,
        ) {
            Ok((val, _errs)) => {
                // Should do something with _errs
                return to_string(&val).unwrap();
            }
            Err(_e) => {
                "Error running query".to_string()
                // Could also do this to retain the juniper error
                // return serde_json::to_string(&e).unwrap(),
            }
        }
    }

    pub fn start(&self) {
        let addr = self.config.hosturl().parse::<SocketAddr>().unwrap();

        let socket = UdpSocket::bind(&addr).unwrap();
        println!("Listening on: {}", socket.local_addr().unwrap());
        let mut buf = [0; 128];
        let mut to_send: Option<(usize, SocketAddr)> = None;
        loop {
            if let Some((size, peer)) = to_send {
                let query_string = String::from_utf8(buf[0..(size)].to_vec()).unwrap();
                let res = self.process(query_string);
                let _amt = socket.send_to(&res.as_bytes(), &peer);
            }

            to_send = Some(socket.recv_from(&mut buf).unwrap());
        }
    }
}
