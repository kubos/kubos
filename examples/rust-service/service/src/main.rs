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

extern crate kubos_service;

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
fn context_factory() -> schema::Context {
    schema::Context { subsystem: model::Subsystem::new() }
}

use juniper::{EmptyMutation, RootNode};

type Schema = RootNode<'static, schema::QueryRoot, schema::MutationRoot>;

fn process(query: String) -> String {
    let s = Schema::new(schema::QueryRoot, schema::MutationRoot);
    match juniper::execute(
        &query,
        None,
        &s,
        &juniper::Variables::new(),
        &schema::Context {
            subsystem: model::Subsystem::new(),
        },
    ) {
        Ok((val, errs)) => return serde_json::to_string(&val).unwrap(),
        Err(e) => return serde_json::to_string(&e).unwrap()
    }
}

fn main() {
    //kubos_service::new_start(schema::QueryRoot, schema::MutationRoot, schema::Context {subsystem: model::Subsystem::new()});
    //kubos_service::start(process);
    let s = kubos_service::KubosService::new(context_factory, schema::QueryRoot, schema::MutationRoot);
    s.start();
}
