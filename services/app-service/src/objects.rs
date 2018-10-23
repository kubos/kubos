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

use app_entry;
use juniper::FieldResult;

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

graphql_object!(KApp: () as "App" |&self| {
    description: "Kubos Application"

    field uuid() -> FieldResult<&String>
        as "UUID"
    {
        Ok(&(self.0.uuid))
    }

    field name() -> FieldResult<&String>
        as "Name"
    {
        Ok(&self.0.metadata.name)
    }

    field version() -> FieldResult<&String>
        as "Version"
    {
        Ok(&self.0.metadata.version)
    }

    field author() -> FieldResult<&String>
        as "Author"
    {
        Ok(&self.0.metadata.author)
    }

    field pid() -> FieldResult<i32>
        as "Process ID"
    {
        Ok(self.0.pid as i32)
    }

    field path() -> FieldResult<&String>
        as "Absolute Path"
    {
        Ok(&self.0.path)
    }
});

pub struct KAppRegistryEntry(pub app_entry::AppRegistryEntry);

graphql_object!(KAppRegistryEntry: () as "AppRegistryEntry" |&self| {
    field app() -> FieldResult<KApp>
        as "App"
    {
        Ok(KApp(self.0.app.clone()))
    }

    field active() -> FieldResult<bool>
        as "Active"
    {
        Ok(self.0.active_version)
    }
});
