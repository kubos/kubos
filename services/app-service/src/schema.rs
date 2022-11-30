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

use crate::monitor::MonitorEntry;
use crate::objects::*;
use crate::registry::AppRegistry;
use juniper::FieldResult;

type Context = kubos_service::Context<AppRegistry>;

///
pub struct QueryRoot;

// Base GraphQL query model
graphql_object!(QueryRoot : Context as "Query" |&self| {
    // Test query to verify service is running without
    // attempting to execute an actual logic
    //
    // {
    //    ping: "pong"
    // }
    field ping() -> FieldResult<String>
        as "Test service query"
    {
        Ok(String::from("pong"))
    }

    field registered_apps(&executor,
               name: Option<String>,
               version: Option<String>,
               active: Option<bool>)
        -> FieldResult<Vec<KAppRegistryEntry>> as "Kubos Apps Query"
    {
        let entries = executor.context().subsystem().entries.lock()?;
        let result = entries.iter().filter(|e| {
            if name.is_some() && &e.app.name != name.as_ref().unwrap() {
                return false;
            }
            if version.is_some() && &e.app.version != version.as_ref().unwrap() {
                return false;
            }
            if active.is_some() && e.active_version != active.unwrap() {
                return false;
            }
            true
        }).map(|entry| KAppRegistryEntry(entry.clone())).collect();

        Ok(result)
    }

    field app_status(&executor,
        name: Option<String>,
        version: Option<String>,
        running: Option<bool>)
        -> FieldResult<Vec<MonitorEntry>> as "App Status Query"
    {
        let entries = executor.context().subsystem().monitoring.lock()?;
        let result = entries.iter().filter(|e| {
            if name.is_some() && &e.name != name.as_ref().unwrap() {
                return false;
            }
            if version.is_some() && &e.version != version.as_ref().unwrap() {
                return false;
            }
            if running.is_some() && &e.running != running.as_ref().unwrap() {
                return false;
            }
            true
        }).map(|entry_ref| (*entry_ref).clone()).collect();

        Ok(result)
    }

});

///
pub struct MutationRoot;

// Base GraphQL mutation model
graphql_object!(MutationRoot : Context as "Mutation" |&self| {

    field register(&executor, path: String) -> FieldResult<RegisterResponse>
        as "Register App"
    {
        let registry = executor.context().subsystem();
        Ok(match registry.register(&path) {
            Ok(app) =>  RegisterResponse { success: true, errors: "".to_owned(), entry: Some(KAppRegistryEntry(app))},
            Err(error) => RegisterResponse {
                success: false,
                errors: error.to_string(),
                entry: None
            }
        })
    }

    field uninstall(&executor, name: String, version: Option<String>) -> FieldResult<GenericResponse>
        as "Uninstall App"
    {
        if let Some(val) = version {
            Ok(match executor.context().subsystem().uninstall(&name, &val) {
                Ok(v) => GenericResponse { success: true, errors: "".to_owned() },
                Err(error) => GenericResponse { success: false, errors: error.to_string() },
            })
        } else {
            Ok(match executor.context().subsystem().uninstall_all(&name) {
                Ok(v) => GenericResponse { success: true, errors: "".to_owned() },
                Err(error) => GenericResponse { success: false, errors: error.to_string() },
            })
        }
    }

    field set_version(&executor, name: String, version: String) -> FieldResult<GenericResponse>
        as "Set App Active Version"
    {
        Ok(match executor.context().subsystem().set_version(&name, &version) {
            Ok(v) => GenericResponse { success: true, errors: "".to_owned() },
            Err(error) => GenericResponse { success: false, errors: error.to_string() },
        })
    }

    field start_app(&executor, name: String, config: Option<String>, args: Option<Vec<String>>) -> FieldResult<StartResponse>
        as "Start App"
    {
        Ok(match executor.context().subsystem().start_app(&name, config, args) {
            Ok(pid) => StartResponse { success: true, errors: "".to_owned(), pid},
            Err(error) => StartResponse { success: false, errors: error.to_string(), pid: None },
        })
    }

    field kill_app(&executor, name: String, signal: Option<i32>) -> FieldResult<GenericResponse>
        as "Kill Running App"
    {
        Ok(match executor.context().subsystem().kill_app(&name, signal) {
            Ok(pid) => GenericResponse { success: true, errors: "".to_owned() },
            Err(error) => GenericResponse { success: false, errors: error.to_string() },
        })
    }
});
