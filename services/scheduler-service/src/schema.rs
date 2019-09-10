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
use crate::mode::*;
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

    // Returns information on the currently active mode
    // {
    //     activeMode: {
    //         name: String,
    //         path: String,
    //         lastRevised: String,
    //         schedules: [ScheduleConfigFile],
    //         active: Boolean
    //     }
    // }
    field active_mode(&executor) -> FieldResult<ScheduleMode> as "Active Mode"
    {
        Ok(get_active_mode(&executor.context().subsystem().scheduler_dir)?)
    }

    // Returns a list of information on currently available modes
    // {
    //     availableModes: [
    //         {
    //             name: String,
    //             path: String,
    //             lastRevised: String,
    //             schedules: [ScheduleConfigFile],
    //             active: Boolean
    //         }
    //     ]
    // }
    field available_modes(&executor, name: Option<String>) -> FieldResult<Vec<ScheduleMode>> as "Available Modes"
    {
        Ok(get_available_modes(&executor.context().subsystem().scheduler_dir, name)?)
    }
});

pub struct MutationRoot;

// Base GraphQL mutation model
graphql_object!(MutationRoot: Context as "Mutation" |&self| {

    // Creates a new mode
    //
    // mutation {
    //     createMode(name: String!): {
    //         errors: String,
    //         success: Boolean
    //    }
    // }
    field create_mode(&executor, name: String) -> FieldResult<GenericResponse> {
        Ok(match create_mode(&executor.context().subsystem().scheduler_dir, &name) {
            Ok(_) => {
                GenericResponse { success: true, errors: "".to_owned() }
            },
            Err(error) => GenericResponse { success: false, errors: error.to_string() }
        })
    }

    // Removes an existing mode
    //
    // mutation {
    //     removeMode(name: String!): {
    //         errors: String,
    //         success: Boolean
    //    }
    // }
    field remove_mode(&executor, name: String) -> FieldResult<GenericResponse> {
        Ok(match remove_mode(&executor.context().subsystem().scheduler_dir, &name) {
            Ok(_) => {
                GenericResponse { success: true, errors: "".to_owned() }
            },
            Err(error) => GenericResponse { success: false, errors: error.to_string() }
        })
    }

    // Activates a mode
    //
    // mutation {
    //     activateMode(name: String!): {
    //         errors: String,
    //         success: Boolean
    //    }
    // }
    field activate_mode(&executor, name: String) -> FieldResult<GenericResponse> {
        Ok(match activate_mode(&executor.context().subsystem().scheduler_dir, &name)
        .and_then(|_| executor.context().subsystem().stop())
        .and_then(|_| executor.context().subsystem().start()) {
            Ok(_) => {
                GenericResponse { success: true, errors: "".to_owned() }
            },
            Err(error) => GenericResponse { success: false, errors: error.to_string() }
        })
    }

    // Activates the safe mode
    //
    // mutation {
    //     safeMode(): {
    //         errors: String,
    //         success: Boolean
    //    }
    // }
    field safe_mode(&executor) -> FieldResult<GenericResponse> {
        Ok(match activate_mode(&executor.context().subsystem().scheduler_dir, "safe")
        .and_then(|_| executor.context().subsystem().stop())
        .and_then(|_| executor.context().subsystem().start()) {
            Ok(_) => {
                GenericResponse { success: true, errors: "".to_owned() }
            },
            Err(error) => GenericResponse { success: false, errors: error.to_string() }
        })
    }

    // Imports a new schedule config into a mode
    //
    // mutation {
    //     importConfig(path: String!, name: String!, mode: String!): {
    //         errors: String,
    //         success: Boolean
    //    }
    // }
    field import_config(&executor, path: String, name: String, mode: String) -> FieldResult<GenericResponse> {
        Ok(match import_config(&executor.context().subsystem().scheduler_dir, &path, &name, &mode) {
            Ok(_) => GenericResponse { success: true, errors: "".to_owned() },
            Err(error) => GenericResponse { success: false, errors: error.to_string() }
        })
    }

    // Removes a schedule config from a mode
    //
    // mutation {
    //     removeConfig(name: String!, mode:String!): {
    //         errors: String,
    //         success: Boolean
    //    }
    // }
    field remove_config(&executor, name: String, mode: String) -> FieldResult<GenericResponse> {
        Ok(match remove_config(&executor.context().subsystem().scheduler_dir, &name, &mode) {
            Ok(_) => {
                GenericResponse { success: true, errors: "".to_owned() }
            },
            Err(error) => GenericResponse { success: false, errors: error.to_string() }
        })
    }
});
