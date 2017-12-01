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

use model::Subsystem;
use juniper::Context as JuniperContext;

/// Context used to pass global data into Juniper queries
pub struct Context {
    pub subsystem: Subsystem,
}

impl JuniperContext for Context {}

impl Context {
    /// Give us a reference to subsystem for passing
    /// along the Juniper chain
    pub fn get_subsystem(&self) -> &Subsystem {
        &self.subsystem
    }
}

/// GraphQL model for Subsystem
graphql_object!(Subsystem: Context as "Subsystem" |&self| {
    description: "Handler subsystem"

    field power() -> bool as "Power state of subsystem" {
        self.power()
    }

    field uptime() -> i32 as "Uptime of subsystem" {
        self.uptime()
    }
});

pub struct QueryRoot;

/// GraphQL model for base query
graphql_object!(QueryRoot : Context as "Query" |&self| {
    field subsystem(&executor) -> Option<&Subsystem>
        as "Subsystem query"
    {
        Some(executor.context().get_subsystem())
    }
});
