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
extern crate juniper_iron;
extern crate iron;
extern crate mount;
extern crate logger;

use iron::prelude::*;
use juniper_iron::{GraphQLHandler, GraphiQLHandler};

mod model;
mod schema;
use std::env;

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
    let graphql_endpoint =
        GraphQLHandler::new(context_factory, schema::QueryRoot, schema::MutationRoot);

    let graphiql_endpoint = GraphiQLHandler::new("/grapihql");

    let mut mount = mount::Mount::new();
    mount.mount("/", graphql_endpoint);
    mount.mount("/graphiql", graphiql_endpoint);

    let (logger_before, logger_after) = logger::Logger::new(None);

    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let host = env::var("LISTEN").unwrap_or("0.0.0.0:8080".to_owned());
    println!("GraphQL server started on {}", host);
    Iron::new(chain).http(host.as_str()).unwrap();
}
