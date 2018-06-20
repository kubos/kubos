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

use config::Config;
use juniper::{execute, Context as JuniperContext, GraphQLType, RootNode, Variables};
use serde_json;
use std::cell::RefCell;
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::os::unix::io::AsRawFd;

const FIONREAD: u16 = 0x541B;
ioctl!(bad read udp_bytes_available with FIONREAD; usize);

/// GenericResponse struct for use in queries or mutations without an explicit response
#[derive(GraphQLObject)]
pub struct MutationResponse {
    /// Any errors which occurred during query
    pub errors: String,
    /// Success or fail status of query
    pub success: bool,
}

/// Context struct used by a service to provide Juniper context,
/// subsystem access and persistent storage.
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
///
/// Specifically the functionality provided by this struct
/// exists to provide a GraphQL interface over UDP, a means
/// of exposing a subsystem to GraphQL queries and means
/// for persistence throughout GraphQL queries.
///
/// ### Examples
///
/// # Creating and starting a service.
/// ```rust,ignore
/// use kubos_service::Service;
///
/// let sub = model::Subsystem::new();
/// Service::new(
///     "example-service",
///     sub,
///     schema::QueryRoot,
///     schema::MutationRoot,
/// ).start();
/// ```
pub struct Service<'a, Query, Mutation, S>
where
    Query: GraphQLType<Context = Context<S>> + Send + Sync + 'static,
    Mutation: GraphQLType<Context = Context<S>> + Send + Sync + 'static,
{
    config: Config,
    root_node: RootNode<'a, Query, Mutation>,
    context: Context<S>,
}

impl<'a, Query, Mutation, S> Service<'a, Query, Mutation, S>
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
    pub fn new(config: Config, subsystem: S, query: Query, mutation: Mutation) -> Self {
        Service {
            config: config,
            root_node: RootNode::new(query, mutation),
            context: Context {
                subsystem: subsystem,
                storage: RefCell::new(HashMap::new()),
            },
        }
    }

    /// Starts the service's GraphQL/UDP server. This function runs
    /// without return.
    ///
    /// # Panics
    ///
    /// The UDP interface will panic if the ip address and port provided
    /// cannot be bound (like if they are already in use).
    pub fn start(&self) {
        let addr = self.config.hosturl().parse::<SocketAddr>().unwrap();

        let socket = UdpSocket::bind(&addr).unwrap();
        println!("Listening on: {}", socket.local_addr().unwrap());

        loop {
            // Wait for an incoming message
            let mut buf: [u8; 1] = [0];
            socket.peek_from(&mut buf).unwrap();

            // Get the message size
            let mut len: usize = 0;
            unsafe {
                udp_bytes_available(socket.as_raw_fd(), &mut len).unwrap();
            }

            // Read it into a correctly sized buffer
            let mut buf = vec![0u8; len].into_boxed_slice();

            // Go process the request
            let (size, peer) = socket.recv_from(&mut buf).unwrap();
            let query_string = String::from_utf8(buf[0..(size)].to_vec()).unwrap();
            let res = self.process(query_string);

            // And then send the response back
            let _amt = socket.send_to(&res.as_bytes(), &peer);
        }
    }

    /// Processes a GraphQL query
    pub fn process(&self, query: String) -> String {
        match execute(
            &query,
            None,
            &self.root_node,
            &Variables::new(),
            &self.context,
        ) {
            Ok((val, errs)) => {
                let errs_msg: String = errs.into_iter()
                    .map(|x| serde_json::to_string(&x).unwrap())
                    .collect();

                json!({
                    "msg": val,
                    "errs": errs_msg})
                    .to_string()
            }
            Err(e) => serde_json::to_string(&e).unwrap(),
        }
    }
}
