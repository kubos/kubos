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

use crate::app_entry;

/// Common response fields structure for requests
/// which don't return any specific data
#[derive(GraphQLObject)]
pub struct GenericResponse {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
}

/// Response fields for the `register` mutation
#[derive(GraphQLObject)]
pub struct RegisterResponse {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
    /// The new registry entry created after successfully registration
    pub entry: Option<KAppRegistryEntry>,
}

/// Response fields for the `startApp` mutation
#[derive(GraphQLObject)]
pub struct StartResponse {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
    /// PID of the started process
    pub pid: Option<i32>,
}

pub struct KApp(pub app_entry::App);

graphql_object!(KApp: () as "App" where Scalar = <S> |&self| {
    description: "Kubos Application"

    field name() -> &String
        as "Name"
    {
        &self.0.name
    }

    field version() -> &String
        as "Version"
    {
        &self.0.version
    }

    field author() -> &String
        as "Author"
    {
        &self.0.author
    }

    field pid() -> i32
        as "Process ID"
    {
        self.0.pid as i32
    }

    field path() -> &String
        as "Absolute Path"
    {
        &self.0.path
    }
});

pub struct KAppRegistryEntry(pub app_entry::AppRegistryEntry);

graphql_object!(KAppRegistryEntry: () as "AppRegistryEntry" where Scalar = <S> |&self| {
    field app() -> KApp
        as "App"
    {
        KApp(self.0.app.clone())
    }

    field active() -> bool
        as "Active"
    {
        self.0.active_version
    }
});
