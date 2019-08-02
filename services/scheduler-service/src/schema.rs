/*
 * Copyright (C) 2019 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//!
//! GraphQL schema for scheduler service's public interface
//!

use crate::file::*;
use crate::scheduler::Scheduler;
use juniper::FieldResult;
use kubos_service;
use serde::Deserialize;

type Context = kubos_service::Context<Scheduler>;

// Generic GraphQL Response
#[derive(Debug, Deserialize, GraphQLObject)]
pub struct GenericResponse {
    pub success: bool,
    pub errors: String,
}

pub struct QueryRoot;

// Base GraphQL query model
graphql_object!(QueryRoot: Context as "Query" |&self| {

    // Test query to verify service is running without attempting
    // to communicate with the underlying subsystem
    //
    // {
    //     ping: "pong"
    // }
    field ping() -> FieldResult<String>
    {
        Ok(String::from("pong"))
    }

    // Returns information on the currently active schedule file
    // {
    //     activeSchedule: {
    //         contents: String,
    //         path: String,
    //         name: String,
    //         timeImported: String,
    //         active: Boolean
    //     }
    // }
    field active_schedule(&executor) -> FieldResult<Option<ScheduleFile>> as "Current Schedule File"
    {
        Ok(get_active_schedule(&executor.context().subsystem().scheduler_dir))
    }

    // Returns a list of information on currently available schedules
    // {
    //     availableSchedules: [
    //         {
    //             contents: String,
    //             path: String,
    //             name: String,
    //             timeImported: String,
    //             active: Boolean
    //         }
    //     ]
    // }
    field available_schedules(&executor, name: Option<String>) -> FieldResult<Vec<ScheduleFile>> as "Available Schedule Files"
    {
        Ok(get_available_schedules(&executor.context().subsystem().scheduler_dir, name)?)
    }
});

pub struct MutationRoot;

// Base GraphQL mutation model
graphql_object!(MutationRoot: Context as "Mutation" |&self| {

    // Imports a new schedule
    //
    // mutation {
    //     import(path: String!, name: String!): {
    //         errors: String,
    //         success: Boolean
    //    }
    // }
    // TODO: Maybe change to import?
    field import(&executor, path: String, name: String) -> FieldResult<GenericResponse> {
        Ok(match import_schedule(&executor.context().subsystem().scheduler_dir, &path, &name) {
            Ok(_) => GenericResponse { success: true, errors: "".to_owned() },
            Err(error) => GenericResponse { success: false, errors: error.to_string() }
        })
    }

    // Activates a schedule
    //
    // mutation {
    //     activate(name: String!): {
    //         errors: String,
    //         success: Boolean
    //    }
    // }
    field activate(&executor, name: String) -> FieldResult<GenericResponse> {
        Ok(match activate_schedule(&executor.context().subsystem().scheduler_dir, &name)
        .and_then(|_| executor.context().subsystem().stop())
        .and_then(|_| executor.context().subsystem().start()) {
            Ok(_) => {
                GenericResponse { success: true, errors: "".to_owned() }
            },
            Err(error) => GenericResponse { success: false, errors: error.to_string() }
        })
    }

    // Removes a schedule
    //
    // mutation {
    //     remove(name: String!): {
    //         errors: String,
    //         success: Boolean
    //    }
    // }
    field remove(&executor, name: String) -> FieldResult<GenericResponse> {
        Ok(match remove_schedule(&executor.context().subsystem().scheduler_dir, &name) {
            Ok(_) => {
                GenericResponse { success: true, errors: "".to_owned() }
            },
            Err(error) => GenericResponse { success: false, errors: error.to_string() }
        })
    }
});
