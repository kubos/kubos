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
extern crate juniper;
extern crate kubos_service;

use juniper::{FieldError, FieldResult};
use kubos_app::registry::{self, AppRegistry, RunLevel};

type Context = kubos_service::Context<AppRegistry>;

pub struct KApp(pub registry::App);

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

    field pid() -> FieldResult<f64>
        as "Process ID"
    {
        Ok(f64::from(self.0.pid))
    }

    field path() -> FieldResult<&String>
        as "Absolute Path"
    {
        Ok(&self.0.path)
    }
});

pub struct KAppRegistryEntry(pub registry::AppRegistryEntry);

graphql_object!(KAppRegistryEntry: () as "AppRegistryEntry" |&self| {
    field app() -> FieldResult<KApp>
        as "App"
    {
        Ok(KApp(self.0.app.clone()))
    }

    field active() -> FieldResult<bool>
        as "Active"
    {
        Ok(self.0.active)
    }

    field run_level() -> FieldResult<String>
        as "Run Level"
    {
        Ok(String::from(format!("{}", self.0.run_level)))
    }
});


///
pub struct QueryRoot;

/// Base GraphQL query model
graphql_object!(QueryRoot : Context as "Query" |&self| {
    field apps(&executor) -> FieldResult<Vec<KAppRegistryEntry>>
        as "Kubos Apps Query"
    {
        let mut entries: Vec<KAppRegistryEntry> = Vec::new();
        for entry in executor.context().subsystem().entries.borrow().iter() {
            entries.push(KAppRegistryEntry(entry.clone()));
        }
        Ok(entries)
    }
});

///
pub struct MutationRoot;

/// Base GraphQL mutation model
graphql_object!(MutationRoot : Context as "Mutation" |&self| {

    field register(&executor, path: String) -> FieldResult<KAppRegistryEntry>
        as "Register App"
    {
        let registry = executor.context().subsystem();
        match registry.register(&path) {
            Ok(entry) => Ok(KAppRegistryEntry(entry)),
            Err(e) => {
                println!("Register error: {}", e);
                Err(FieldError::new(e, juniper::Value::null()))
            }
        }
    }

    field uninstall(&executor, uuid: String, version: String) -> FieldResult<bool>
        as "Uninstall App"
    {
        match executor.context().subsystem().uninstall(&uuid, &version) {
            Ok(v) => Ok(v),
            Err(msg) => {
                println!("{}", msg);
                Err(FieldError::new(msg, juniper::Value::null()))
            }
        }
    }

    field start_app(&executor, uuid: String, run_level: String) -> FieldResult<f64>
        as "Start App"
    {
        let run_level_o = {
            match run_level.as_ref() {
                "OnBoot" => RunLevel::OnBoot,
                _ => RunLevel::OnCommand
            }
        };

        match executor.context().subsystem().start_app(&uuid, run_level_o) {
            Ok(pid) => Ok(f64::from(pid)),
            Err(err) => Err(FieldError::new(err, juniper::Value::null()))
        }
    }
});
