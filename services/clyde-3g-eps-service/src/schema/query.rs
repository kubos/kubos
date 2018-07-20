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

use juniper::FieldResult;
use models::*;
use schema::Context;

pub struct Telemetry;

graphql_object!(Telemetry: Context as "telemetry" |&self| {
    field motherboard() -> FieldResult<motherboard_telemetry::Telemetry>
        as "Motherboard Telemetry"
    {
        Ok(motherboard_telemetry::Telemetry {})
    }

    field daughterboard() -> FieldResult<daughterboard_telemetry::Telemetry>
        as "Daughterboard Telemetry"
    {
        Ok(daughterboard_telemetry::Telemetry {})
    }

    field reset() -> FieldResult<reset_telemetry::Telemetry>
        as "Reset Telemetry"
    {
        Ok(reset_telemetry::Telemetry {})
    }

    field watchdog_period(&executor) -> FieldResult<i32>
        as "Current watchdog period in minutes"
    {
        Ok(i32::from(executor.context().subsystem().get_comms_watchdog_period()?))
    }

    // Get the version information for the EPS
    // motherboard and daughterboard (if accesssible)
    //
    // {
    //     Data: {
    //         motherboard: VersionData {
    //             revision: i32,
    //             firmware_number: i32
    //         },
    //         daughterboard: VersionData {
    //             revision: i32,
    //             firmware_number: i32
    //         }
    //     }
    // }
    field version(&executor) -> FieldResult<version::Data>
        as "Hardware version information"
    {
        Ok(executor.context().subsystem().get_version()?)
    }
});

pub struct Root;

/// Base GraphQL query
graphql_object!(Root: Context as "Query" |&self| {

    // Test query to verify service is running without
    // attempting to communicate with hardware
    //
    // {
    //    ping: "pong"
    // }
    field ping() -> FieldResult<String>
        as "Test service query"
    {
        Ok(String::from("pong"))
    }

    // Get the last mutation run
    //
    // {
    //    ack: subsystem::Mutations
    // }
    field ack(&executor) -> FieldResult<subsystem::Mutations>
        as "Last run mutation"
    {
        Ok(executor.context().subsystem().get_last_mutation())
    }

    // Get all errors encountered since the last time
    // this field was queried
    //
    // {
    //    errors: [String]
    // }
    field errors(&executor) -> FieldResult<Vec<String>>
        as "Last errors encountered"
    {
        Ok(executor.context().subsystem().get_errors()?)
    }

    // Get telemetry from the EPS
    //
    // {
    //     telemetry {
    //         version {
    //             motherboard {
    //                 revision: i32,
    //                 firmwareVersion: i32
    //             },
    //             daughterboard {
    //                 revision: i32,
    //                 firmwareVersion: i32
    //             }
    //         },
    //         watchdogPeriod: i32,

    //     }
    // }
    field telemetry(&executor) -> FieldResult<Telemetry>
    {
        Ok(Telemetry {})
    }
});
