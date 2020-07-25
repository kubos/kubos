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

use juniper::{Context as JuniperContext, GraphQLType, RootNode};
use kubos_system::Config;
use log::{error, info};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use juniper::{execute, Variables};
use serde_json::json;
use std::net::UdpSocket;

/// Context struct used by a service to provide Juniper context,
/// subsystem access and persistent storage.
#[derive(Clone)]
pub struct Context<T> {
    ///
    pub subsystem: T,
    ///
    pub storage: Arc<RwLock<HashMap<String, String>>>,
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
        let stor = self.storage.read().unwrap();
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
        let mut stor = self.storage.write().unwrap();
        stor.insert(key.to_string(), value.to_string());
    }

    /// Clears a single key/value from storage
    ///
    /// # Arguments
    ///
    /// `key` - Key to clear (along with corresponding value)
    pub fn clear(&self, name: &str) {
        let mut storage = self.storage.write().unwrap();
        storage.remove(name);
    }

    /// Clears all key/value pairs from storage
    pub fn clear_all(&self) {
        self.storage.write().unwrap().clear();
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
    S: Send + Sync + 'static,
    Query: GraphQLType<Context = Context<S>, TypeInfo = ()> + Send + Sync + 'static,
    Mutation: GraphQLType<Context = Context<S>, TypeInfo = ()> + Send + Sync + 'static,
{
    config: Config,
    context: Context<S>,
    root_node: RootNode<'a, Query, Mutation>,
}

impl<'a, Query, Mutation, S> Service<'a, Query, Mutation, S>
where
    S: Send + Sync + 'static,
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
        let root_node = RootNode::new(query, mutation);
        let context = Context {
            subsystem,
            storage: Arc::new(RwLock::new(HashMap::new())),
        };

        Service {
            config,
            context,
            root_node,
        }
    }

    /// Starts the service's GraphQL/UDP server. This function runs
    /// without return.
    ///
    /// # Panics
    ///
    /// The UDP interface will panic if the ip address and port provided
    /// cannot be bound (like if they are already in use), or if for some reason the socket fails
    /// to receive a message.
    pub fn start(self) {
        let hosturl = self
            .config
            .hosturl()
            .ok_or_else(|| {
                error!("Failed to load service URL");
                "Failed to load service URL"
            })
            .unwrap();

        let addr = hosturl
            .parse::<SocketAddr>()
            .map_err(|err| {
                error!("Failed to parse SocketAddr: {:?}", err);
                err
            })
            .unwrap();

        let socket = UdpSocket::bind(&addr).unwrap();
        info!("Listening on: {}", addr);

        let mut buf = [0; 4096];
        loop {
            if let Ok((size, peer)) = socket.recv_from(&mut buf) {
                if let Ok(query) = String::from_utf8(buf[0..size].to_vec()) {
                    let resp = match execute(
                        &query,
                        None,
                        &self.root_node,
                        &Variables::new(),
                        &self.context,
                    ) {
                        Ok((val, errs)) => {
                            let errs_msg: String = errs
                                .into_iter()
                                .map(|x| serde_json::to_string(&x).unwrap())
                                .collect();

                            json!({
                                "data": val,
                                "errors": errs_msg})
                            .to_string()
                        }
                        Err(e) => json!({ "errors": e }).to_string(),
                    };
                    if let Err(e) = socket.send_to(&resp.as_bytes(), &peer) {
                        error!("Failed to send udp response: {:?}", e);
                    };
                }
            }
        }
    }
}
