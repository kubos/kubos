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
extern crate isis_ants_api;
extern crate iron;
#[macro_use]
extern crate juniper;
extern crate juniper_iron;
extern crate logger;
extern crate mount;
#[macro_use]
extern crate serde_json;

use serde_json::{Value, Error};

use std::fs::File;
use std::io::prelude::*;

use iron::prelude::*;
use iron::typemap::Key;
use juniper_iron::{GraphQLHandler, GraphiQLHandler};

mod model;
mod schema;
use std::env;

//TODO: use me
#[derive(Copy, Clone)]
pub struct LastCmd;
impl Key for LastCmd {
    type Value = model::AckCommand;
}

/// A context object is used in Juniper to provide out-of-band access to global
/// data when resolving fields. We will use it here to provide a Subsystem structure
/// with recently fetched data.
///
/// Since this function is called once for every request, it will fetch new
/// data with each request.
fn context_factory(_: &mut Request) -> schema::Context {
    schema::Context { subsystem: model::Subsystem::new() }
}

fn main() {

    let default = json!({
                    "isis-ants-service": {
                        "addr": "0.0.0.0",
                        "port": "8080"
                    }
                });

    let mut raw = String::new();

    //TODO: decide on official location for services' configuration file
    let config: Value =
        match File::open("sys-config.txt")
            .map(|mut f| f.read_to_string(&mut raw))
            .and_then(|_x| serde_json::from_str(&raw).map_err(|err| err.into())) {
            Ok(v) => v,
            _ => {
                println!("Failed to get configuration. Using default {}", default);
                default
            }
        };

    let host = config["isis-ants-service"]["addr"].to_string();
    let port = config["isis-ants-service"]["port"].to_string();

    let addr = format!("{}:{}", host.trim_matches('"'), port.trim_matches('"'));

    let graphql_endpoint =
        GraphQLHandler::new(context_factory, schema::QueryRoot, schema::MutationRoot);

    let graphiql_endpoint = GraphiQLHandler::new("/");

    let mut mount = mount::Mount::new();
    mount.mount("/", graphql_endpoint);
    mount.mount("/graphiql", graphiql_endpoint);

    let (logger_before, logger_after) = logger::Logger::new(None);

    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let host = env::var("LISTEN").unwrap_or(addr.to_owned());
    println!("GraphQL server started on {}", host);
    Iron::new(chain).http(host.as_str()).unwrap();
}
