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

use crate::model::{GeoRecordResponse, StateOfHealthResponse, Subsystem};
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
    //     "data":{
    //                "ping": "pong"
    //     },
    //     "errors": ""
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
    //                "errors" : ["A UDP header was unable to be correctly parsed"]
    //            },
    //     "errors" : ""
    // }
    field errors(&executor) -> FieldResult<Vec<String>>
    {
        Ok(executor.context().subsystem().errors()?)
    }

    // Request current modem health information
    //
    // Query
    //
    // {
    //     modemHealth {
    //         resetCount,
    //         currentTime,
    //         currentRssi,
    //         connectionStatus,
    //         globalstarGateway,
    //         lastContactTime,
    //         lastAttemptTime,
    //         callAttemptsSinceReset,
    //         successfulConnectsSinceReset,
    //         averageConnectionDuration,
    //         connectionDurationStdDev
    //     }
    // }
    //
    // Response
    //
    // {
    //     "data":{
    //         "modemHealth": {
    //             "resetCount": 1,
    //             "currentTime": 1025,
    //             "currentRssi": 3,
    //             "connectionStatus": 1,
    //             "globalstarGateway": 45,
    //             "lastContactTime": 500,
    //             "lastAttemptTime": 10,
    //             "callAttemptsSinceReset": 4,
    //             "successfulConnectsSinceReset": 25,
    //             "averageConnectionDuration": 120,
    //             "connectionDurationStdDev": 10,
    //         }
    //     },
    //     "errors" : ""
    // }
    field modem_health(&executor) -> FieldResult<StateOfHealthResponse>
    {
        Ok(executor.context().subsystem().modem_health()?)
    }

    // Request current geolocation data
    //
    // Query
    //
    // {
    //     geolocation {
    //         lon,
    //         lat,
    //         time,
    //         maxError
    //     }
    // }
    //
    // Response
    //
    // {
    //    "data": {
    //         "geolocation": {
    //             "lat": 22202.22,
    //             "lon": 3333.2,
    //             "time": 1818119191,
    //             "maxError": 100
    //         }
     //    },
    //     "errors" : ""
    // }
    field geolocation(&executor) -> FieldResult<GeoRecordResponse>
    {
        Ok(executor.context().subsystem().geolocation()?)
    }

    // Request number of files in the downlink queue
    //
    // Query
    //
    // {
    //     downlink_queue_count
    // }
    //
    // Response
    //
    // {
    //     "data":{
    //                "downlink_queue_count" : 10
    //            },
    //     "errors" : ""
    // }
    field downlink_queue_count(&executor) -> FieldResult<i32>
    {
        Ok(executor.context().subsystem().file_queue_count()?)
    }

    // Queries the modem's 'is_alive' status
    //
    // Query
    //
    // {
    //     alive
    // }
    //
    // Response
    //
    // {
    //     "data":{
    //                "alive": true
    //            },
    //     "errors" : ""
    // }
    field alive(&executor) -> FieldResult<bool>
    {
        Ok(executor.context().subsystem().get_alive()?)
    }
});

pub struct MutationRoot;

// Base GraphQL mutation model
graphql_object!(MutationRoot: Context as "Mutation" |&self| {
    // Execute a trivial command against the system
    //
    // Mutation
    //
    // mutation {
    //     noop
    // }
    //
    // Response
    //
    // {
    //     "data":{
    //                "noop": true
    //            },
    //     "errors" : ""
    // }
    field noop(&executor) -> FieldResult<bool>
    {
        Ok(executor.context().subsystem().get_alive()?)
    }
});
