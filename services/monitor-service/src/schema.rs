//
// Copyright (C) 2018 Kubos Corporation
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

use juniper::{self, FieldError, FieldResult};
use kubos_service;

use crate::meminfo;
use crate::objects::*;
use crate::process;

type Context = kubos_service::Context<()>;

pub struct QueryRoot;

// Base GraphQL query model
graphql_object!(QueryRoot: Context as "Query" |&self| {
    field ping() -> FieldResult<String>
    {
        Ok(String::from("pong"))
    }

    field mem_info(&executor) -> FieldResult<MemInfoResponse> {

        meminfo::MemInfo::from_proc()
            .map(|info| MemInfoResponse { info })
            .map_err(|err| FieldError::new(err, juniper::Value::null()))
    }

    field ps(&executor, pids: Option<Vec<i32>>) -> FieldResult<Vec<PSResponse>>
    {
        let pids_vec: Vec<i32> = match pids {
            Some(vec) => vec,
            None => process::running_pids()?
        };

        Ok(pids_vec.into_iter().map(PSResponse::new).collect())
    }
});

pub struct MutationRoot;

// Base GraphQL mutation model
graphql_object!(MutationRoot: Context as "Mutation" |&self| {
    field noop(&executor) -> FieldResult<()>
    {
        Ok(())
    }
});
