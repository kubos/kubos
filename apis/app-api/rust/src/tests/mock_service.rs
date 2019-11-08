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

use juniper::{FieldError, FieldResult, Value};
use kubos_service;

#[derive(Clone)]
pub struct Subsystem;
type Context = kubos_service::Context<Subsystem>;

pub struct QueryRoot;

// Base GraphQL query model
graphql_object!(QueryRoot: Context as "Query" |&self| {
    field ping(fail = false: bool) -> FieldResult<String>
    {
        if fail {
            Err(FieldError::new("Query failed", Value::null()))
        } else {
            Ok(String::from("query"))
        }
    }
});

pub struct MutationRoot;

// Base GraphQL mutation model
graphql_object!(MutationRoot: Context as "Mutation" |&self| {
    field ping() -> FieldResult<String>
        {
            Ok(String::from("mutation"))
        }
});
