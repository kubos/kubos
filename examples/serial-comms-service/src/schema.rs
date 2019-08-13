//
// Copyright (C) 2019 Kubos Corporation
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

//!
//! GraphQL schema for exposing communications service
//! telemetry information.
//!

use crate::model::Subsystem;
use juniper::FieldResult;

type Context = kubos_service::Context<Subsystem>;

pub struct QueryRoot;

graphql_object!(QueryRoot: Context as "Query" |&self| {
    // Test query to verify service is running without attempting
    // to communicate with the underlying subsystem
    //
    // Query
    //
    // {
    //     ping
    // }
    //
    // Response
    //
    // {
    //     ping: "pong"
    // }
    field ping() -> FieldResult<String>
    {
        Ok(String::from("pong"))
    }

    // Request number of bad uplink packets
    //
    // Query
    //
    // {
    //     failedPacketsUp
    // }
    //
    // Response
    //
    // {
    //     "data":{
    //                "failedPacketsUp" : 0
    //            },
    //     "errors" : ""
    // }
    field failed_packets_up(&executor) -> FieldResult<i32>
    {
        Ok(executor.context().subsystem().failed_packets_up()?)
    }

    // Request number of bad downlink packets
    //
    // Query
    //
    // {
    //     failedPacketsDown
    // }
    //
    // Response
    //
    // {
    //     "data":{
    //                "failedPacketsDown" : 0
    //            },
    //     "errors" : ""
    // }
    field failed_packets_down(&executor) -> FieldResult<i32>
    {
        Ok(executor.context().subsystem().failed_packets_down()?)
    }

    // Request number of packets successfully uplinked
    //
    // Query
    //
    // {
    //     packetsUp
    // }
    //
    // Response
    //
    // {
    //     "data":{
    //                "packetsUp" : 0
    //            },
    //     "errors" : ""
    // }
    field packets_up(&executor) -> FieldResult<i32>
    {
        Ok(executor.context().subsystem().packets_up()?)
    }

    // Request number of packets successfully downlinked
    //
    // Query
    //
    // {
    //     packetsDown
    // }
    //
    // Response
    //
    // {
    //     "data":{
    //                "packetsDown" : 10
    //            },
    //     "errors" : ""
    // }
    field packets_down(&executor) -> FieldResult<i32>
    {
        Ok(executor.context().subsystem().packets_down()?)
    }

    // Request errors that have occured
    //
    // Query
    //
    // {
    //     errors
    // }
    //
    // Response
    //
    // {
    //     "data":{
    //                "errors" : ""
    //            },
    //     "errors" : ""
    // }
    field errors(&executor) -> FieldResult<Vec<String>>
    {
        Ok(executor.context().subsystem().errors()?)
    }
});

pub struct MutationRoot;

// Base GraphQL mutation model
graphql_object!(MutationRoot: Context as "Mutation" |&self| {

});
