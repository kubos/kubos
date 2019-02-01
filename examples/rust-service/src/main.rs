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

use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use warp::{http::Response, log, Filter};
use juniper::RootNode;
use std::sync::{Arc, Mutex};
use std::sync::RwLock;

fn main() {

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(format!(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
            ))
    });

    print!("Listening on 127.0.0.1:8080");
    
    let context = Context {
        subsystem: Subsystem::new(),
        storage: Arc::new(RwLock::new(HashMap::new()))
    };

    let state = warp::any().map(move || context.clone());
    let graphql_filter = juniper_warp::make_graphql_filter(RootNode::new(QueryRoot, MutationRoot), state.boxed());

    warp::serve(
        warp::get2()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql"))
            .or(homepage)
            .or(warp::path("graphql").and(graphql_filter)),
    )
    .run(([0, 0, 0, 0], 8080));
}

/*
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
    )
    .start();
}
*/