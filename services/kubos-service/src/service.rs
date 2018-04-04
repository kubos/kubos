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

use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::cell::RefCell;

use config::Config;
use serde_json::{to_string, Value};
use juniper::{execute, Context as JuniperContext, GraphQLType, RootNode, Variables};

/// Context struct used by a service to provide:
/// - Juniper context
/// - Subsystem access
/// - Persistent storage
pub struct Context<T> {
    subsystem: T,
    storage: RefCell<HashMap<String, String>>,
}

impl<T> JuniperContext for Context<T> {}

impl<T> Context<T> {
    /// Returns a reference to the context's subsystem instance
    pub fn subsystem(&self) -> &T {
        &self.subsystem
    }

    /// Attempts to get a value from the context's storage
    ///
    /// # Arguments
    ///
    /// `name` - Key to search for in storage
    pub fn get(&self, name: &str) -> String {
        let stor = self.storage.borrow();
        match stor.get(&name.to_string()) {
            Some(s) => s.clone(),
            None => "".to_string(),
        }
    }

    /// Sets a value in the context's storage
    ///
    /// # Arguments
    ///
    /// `key` - Key to store value under
    /// `value` - Value to store
    pub fn set(&self, key: &str, value: &str) {
        let mut stor = self.storage.borrow_mut();
        stor.insert(key.to_string(), value.to_string());
    }

    /// Clears a single key/value from storage
    ///
    /// # Arguments
    ///
    /// `key` - Key to clear (along with corresponding value)
    pub fn clear(&self, name: &String) {
        let mut storage = self.storage.borrow_mut();
        storage.remove(name);
    }

    /// Clears all key/value pairs from storage
    pub fn clear_all(&self) {
        self.storage.borrow_mut().clear();
    }
}

/// This structure represents a hardware service.
/// Specifically the functionality provided by this struct
/// exists to provide
/// - A GraphQL interface over UDP
/// - A means of exposing a subsystem to GraphQL queries
/// - A means of persistence throughout GraphQL queries
///
/// # Examples
///
/// Creating and starting a service:
/// ```
/// use kubos_service::KubosService;
///
/// let sub = model::Subsystem::new();
/// KubosService::new(
///     "example-service",
///     sub,
///     schema::QueryRoot,
///     schema::MutationRoot,
/// ).start();
/// ```
pub struct KubosService<'a, Query, Mutation, S>
where
    Query: GraphQLType<Context = Context<S>> + Send + Sync + 'static,
    Mutation: GraphQLType<Context = Context<S>> + Send + Sync + 'static,
{
    config: Config,
    root_node: RootNode<'a, Query, Mutation>,
    context: Context<S>,
}

impl<'a, Query, Mutation, S> KubosService<'a, Query, Mutation, S>
where
    Query: GraphQLType<Context = Context<S>, TypeInfo = ()> + Send + Sync + 'static,
    Mutation: GraphQLType<Context = Context<S>, TypeInfo = ()> + Send + Sync + 'static,
{
    /// Creates a new service instance
    ///
    /// # Arguments
    ///
    /// `name` - The name of the service. This is used to find the appropriate config information
    /// `subsystem` - An instance of the subsystem struct. This one instance will be used by all queries.
    /// `query` - The root query struct holding all other GraphQL queries.
    /// `mutation` - The root mutation struct holding all other GraphQL mutations.
    pub fn new(name: &str, subsystem: S, query: Query, mutation: Mutation) -> Self {
        KubosService {
            config: Config::new(&name),
            root_node: RootNode::new(query, mutation),
            context: Context {
                subsystem: subsystem,
                storage: RefCell::new(HashMap::new()),
            },
        }
    }

    /// Starts the service's GraphQL/UDP server
    ///
    /// # Panics
    ///
    /// The UDP interface will panic if the ip address and port provided
    /// cannot be bound (like if they are already in use).
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

    /// Returns the service's configuration information
    /// in the `serde_json::Value` format.
    /// This will contain the ip/port if provided, along with any other
    /// configuration information found in the config file.
    pub fn config(&self) -> Value {
        return self.config.raw.clone();
    }

    fn process(&self, query: String) -> String {
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
}
