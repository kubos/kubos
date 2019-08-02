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

use crate::objects::{GenericResponse, Schedule};
use crate::scheduler::Scheduler;
use juniper::FieldResult;
use kubos_service;

type Context = kubos_service::Context<Scheduler>;

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

    // Returns information on the currently active schedule
    // {
    //     activeSchedule: {
    //         contents: String,
    //         path: String,
    //         name: String,
    //         timeRegistered: String,
    //         active: Boolean
    //     }
    // }
    field active_schedule(&executor) -> FieldResult<Option<Schedule>> as "Current Schedule"
    {
        Ok(executor.context().subsystem().get_active_schedule())
    }

    // Returns a list of information on currently registered schedules
    // {
    //     registeredSchedules: [
    //         {
    //             contents: String,
    //             path: String,
    //             name: String,
    //             timeRegistered: String,
    //             active: Boolean
    //         }
    //     ]
    // }
    field registered_schedules(&executor, name: Option<String>) -> FieldResult<Vec<Schedule>> as "Registered Schedules"
    {
        Ok(executor.context().subsystem().get_registered_schedules(name)?)
    }
});

pub struct MutationRoot;

// Base GraphQL mutation model
graphql_object!(MutationRoot: Context as "Mutation" |&self| {

    // Registers a new schedule
    //
    // mutation {
    //     register(path: String!, name: String!): {
    //         errors: String,
    //         success: Boolean
    //    }
    // }
    field register(&executor, path: String, name: String) -> FieldResult<GenericResponse> {
        Ok(match executor.context().subsystem().register_schedule(&path, &name) {
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
        Ok(match executor.context().subsystem().activate_schedule(&name) {
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
        Ok(match executor.context().subsystem().remove_schedule(&name) {
            Ok(_) => {
                GenericResponse { success: true, errors: "".to_owned() }
            },
            Err(error) => GenericResponse { success: false, errors: error.to_string() }
        })
    }
});
