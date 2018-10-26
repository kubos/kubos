/*
 * Copyright (C) 2018 Kubos Corporation
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

use juniper::FieldResult;
use kubos_app::RunLevel;
use kubos_service;
use objects::*;
use registry::AppRegistry;

type Context = kubos_service::Context<AppRegistry>;

///
pub struct QueryRoot;

/// Base GraphQL query model
graphql_object!(QueryRoot : Context as "Query" |&self| {
    field apps(&executor,
               uuid: Option<String>,
               name: Option<String>,
               version: Option<String>,
               active: Option<bool>)
        -> FieldResult<Vec<KAppRegistryEntry>> as "Kubos Apps Query"
    {
        let mut result: Vec<KAppRegistryEntry> = Vec::new();
        let entries = executor.context().subsystem().entries.borrow();
        let mut final_iter = entries.iter().filter(|ref e| {
            if uuid.is_some() && &e.app.uuid != uuid.as_ref().unwrap() {
                return false;
            }
            if name.is_some() && &e.app.metadata.name != name.as_ref().unwrap() {
                return false;
            }
            if version.is_some() && &e.app.metadata.version != version.as_ref().unwrap() {
                return false;
            }
            if active.is_some() && e.active_version != active.unwrap() {
                return false;
            }
            true
        });

        for entry in final_iter {
            result.push(KAppRegistryEntry(entry.clone()));
        }

        Ok(result)
    }
});

///
pub struct MutationRoot;

/// Base GraphQL mutation model
graphql_object!(MutationRoot : Context as "Mutation" |&self| {

    field register(&executor, path: String, uuid: Option<String>) -> FieldResult<RegisterResponse>
        as "Register App"
    {
        let registry = executor.context().subsystem();
        Ok(match registry.register(&path, uuid) {
            Ok(app) =>  RegisterResponse { success: true, errors: "".to_owned(), entry: Some(KAppRegistryEntry(app))},
            Err(error) => RegisterResponse {
                success: false,
                errors: error.to_string(),
                entry: None
            }
        })
    }

    field uninstall(&executor, uuid: String, version: String) -> FieldResult<GenericResponse>
        as "Uninstall App"
    {
        Ok(match executor.context().subsystem().uninstall(&uuid, &version) {
            Ok(v) => GenericResponse { success: true, errors: "".to_owned() },
            Err(error) => GenericResponse { success: false, errors: error.to_string() },
        })
    }

    field start_app(&executor, uuid: String, run_level: String, args: Option<Vec<String>>) -> FieldResult<StartResponse>
        as "Start App"
    {
        let run_level_o = {
            match run_level.as_ref() {
                "OnBoot" => RunLevel::OnBoot,
                _ => RunLevel::OnCommand
            }
        };

        Ok(match executor.context().subsystem().start_app(&uuid, run_level_o, args) {
            Ok(num) => StartResponse { success: true, errors: "".to_owned(), pid: Some(num as i32)},
            Err(error) => StartResponse { success: false, errors: error.to_string(), pid: None },
        })
    }
});
