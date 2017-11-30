#[macro_use] extern crate juniper;

extern crate iron;
extern crate mount;
extern crate logger;
extern crate persistent;

use std::env;

use iron::prelude::*;
use juniper::iron_handlers::{GraphQLHandler, GraphiQLHandler};
use juniper::EmptyMutation;
use persistent::{State};
use iron::typemap::Key;
use std::ops::{Deref, DerefMut};

mod model;
mod schema;

#[derive(Copy, Clone)]
pub struct SubsystemData {
    pub state : bool
}
impl Key for SubsystemData { type Value = SubsystemData; }

/*
A context object is used in Juniper to provide out-of-band access to global
data when resolving fields. We use it here to pass a database connection
to the Query and Mutation types.

Since this function is called once for every request, it will create a
database connection per request. A more realistic solution would be to use
the "r2d2" crate for connection pooling, and the "persistent" crate to pass
data into Iron requests.
*/
fn context_factory(req: &mut Request) -> schema::Context {
    let rwlock = req.get::<State<SubsystemData>>().unwrap();
    let data : bool;
    {
        let reader = rwlock.read().unwrap();
        let _d = reader.deref();
        data = (*_d).state;
    }
    {
        let mut writer = rwlock.write().unwrap();
        let mut _d = writer.deref_mut();
        (*_d).state = !data;
    }
    schema::Context {
        subsystem : model::Subsystem {
            power : data
        }
    }
}

fn main() {
    let graphql_endpoint = GraphQLHandler::new(
        context_factory,
        schema::QueryRoot,
        EmptyMutation::<schema::Context>::new()
    );

    let graphiql_endpoint = GraphiQLHandler::new("/graphql");

    let mut mount = mount::Mount::new();
    mount.mount("/", graphiql_endpoint);
    mount.mount("/graphql", graphql_endpoint);

    let (logger_before, logger_after) = logger::Logger::new(None);

    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let d = SubsystemData {
        state : false
    };
    chain.link(State::<SubsystemData>::both(d));

    let host = env::var("LISTEN").unwrap_or("0.0.0.0:8080".to_owned());
    println!("GraphQL server started on {}", host);
    Iron::new(chain).http(host.as_str()).unwrap();
}
