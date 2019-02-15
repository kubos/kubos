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

mod model;
mod schema;

use crate::model::Subsystem;
use crate::schema::{MutationRoot, QueryRoot};
use kubos_service::{Config, Service, Context};
use syslog::Facility;

/*
fn main() {
    
    print!("Listening on 127.0.0.1:8080");
    
    let context = Context {
        subsystem: Subsystem::new(),
        storage: Arc::new(RwLock::new(HashMap::new()))
    };

    // Make the subsystem and other persistent data available to all endpoints
    let context = warp::any().map(move || context.clone());
    
    
    let graphql_filter = juniper_warp::make_graphql_filter(RootNode::new(QueryRoot, MutationRoot), context.boxed());

    warp::serve(
        // If the path ends in "graphiql" process the request using the graphiql interface
       warp::path("graphiql").and(juniper_warp::graphiql_filter("/graphql"))
            // Otherwise, just process the request as normal GraphQL
            .or(graphql_filter),
    )
    .run(([192, 168, 33, 10], 8080));
}
*/

fn main() {
    syslog::init(
        Facility::LOG_DAEMON,
        log::LevelFilter::Debug,
        Some("example-service"),
    )
    .unwrap();
    
    
    Service::new(
        Config::new("example-service"),
        Subsystem::new(),
        QueryRoot,
        MutationRoot,
    ).start();
}
