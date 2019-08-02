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
use log::info;

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
    field active_schedule() -> FieldResult<Schedule> as "Current Schedule"
    {
        Ok(Schedule {
            contents: "{json_blob}".to_owned(),
            path: "/home/system/etc/schedules/operational.json".to_owned(),
            name: "Operational".to_owned(),
            time_registered: "2019-08-02 14:45:45".to_owned(),
            active: true
        })
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
    field registered_schedules() -> FieldResult<Vec<Schedule>> as "Registered Schedules"
    {
        Ok(vec![Schedule {
            contents: "{json_blob}".to_owned(),
            path: "/home/system/etc/schedules/operational.json".to_owned(),
            name: "Operational".to_owned(),
            time_registered: "2019-08-02 14:45:45".to_owned(),
            active: true
        }, Schedule {
            contents: "{json_blob}".to_owned(),
            path: "/home/system/etc/schedules/safemode.json".to_owned(),
            name: "Safemode".to_owned(),
            time_registered: "2019-08-01 14:00:45".to_owned(),
            active: false
        }])
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
                info!("Activated schedule {}", name);
                GenericResponse { success: true, errors: "".to_owned() }
            },
            Err(error) => GenericResponse { success: false, errors: error.to_string() }
        })
    }
});
