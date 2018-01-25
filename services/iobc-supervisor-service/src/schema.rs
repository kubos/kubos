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

extern crate kubos_hal_iobc;

use model::{Supervisor, SupervisorVersion};
use juniper::Context as JuniperContext;
use juniper::FieldResult;

/// Context used to pass global data into Juniper queries
pub struct Context {
    pub supervisor: Supervisor,
}

impl JuniperContext for Context {}

impl Context {
    /// Give us a reference to subsystem for passing
    /// along the Juniper chain
    pub fn get_supervisor(&self) -> &Supervisor {
        &self.supervisor
    }
}

/// GraphQL model annotations for SupervisorVersion
graphql_object!(SupervisorVersion: Context as "SupervisorVersion" |&self| {
    description: "Supervisor Version Information"

    field dummy() -> FieldResult<i32> as "Dummy bit" {
        Ok(self.dummy()?)
    }

    field spi_command_status() -> FieldResult<i32>
        as "Spi Command Status"
    {
        Ok(self.spi_command_status()?)
    }

    field compile_information() -> FieldResult<Vec<i32>>
        as "Compile information"
    {
        Ok(self.compile_information()?)
    }
});


/// GraphQL model for Subsystem
graphql_object!(Supervisor: Context as "Supervisor" |&self| {
    description: "iOBC Supervisor"

    field version() -> FieldResult<SupervisorVersion>
        as "Supervisor version information"
    {
        Ok(self.version()?)
    }
});

pub struct QueryRoot;

/// Base GraphQL query model
graphql_object!(QueryRoot : Context as "Query" |&self| {
    field supervisor(&executor) -> FieldResult<&Supervisor>
        as "Supervisor query"
    {
        Ok(executor.context().get_supervisor())
    }
});

pub struct MutationRoot;

/// Base GraphQL mutation model
graphql_object!(MutationRoot : Context as "Mutation" |&self| {

    field reset(&executor) -> FieldResult<()>
        as "Reset supervisor"
    {
        Ok(kubos_hal_iobc::supervisor_reset()?)
    }

    field emergency_reset(&executor) -> FieldResult<()>
        as "Supervisor emergency reset"
    {
        Ok(kubos_hal_iobc::supervisor_emergency_reset()?)
    }

    field powercycle(&executor) -> FieldResult<()>
        as "Supervisor powercycle"
    {
        Ok(kubos_hal_iobc::supervisor_powercycle()?)
    }

});
