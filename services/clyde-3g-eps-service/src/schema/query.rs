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

pub struct Root;

/// Base GraphQL query
graphql_object!(Root: Context as "Query" |&self| {
    field ping() -> FieldResult<String>
        as "Test service query"
    {
        Ok(String::from("pong"))
    }

    field ack(&executor) -> FieldResult<subsystem::Mutations>
        as "Last run mutation"
    {
        Ok(executor.context().subsystem().last_mutation.get())
    }

    field version(&executor) -> FieldResult<version::Data>
        as "Hardware version information"
    {
        Ok(executor.context().subsystem().get_version()?)
    }

    field reset_telemetry(&executor, telem_type: reset_telemetry::Type) -> FieldResult<reset_telemetry::Data>
        as "Telemetry data regarding the number of resets"
    {
        Ok(executor.context().subsystem().get_reset_telemetry(telem_type)?)
    }

    field motherboard_telemetry(&executor, telem_type: motherboard_telemetry::Type) -> FieldResult<f64>
        as "Telemetry data from motherboard"
    {
        Ok(f64::from(executor.context().subsystem().get_motherboard_telemetry(telem_type)?))
    }

    field daughterboard_telemetry(&executor, telem_type: daughterboard_telemetry::Type) -> FieldResult<f64>
        as "Telemetry data from daughterboard"
    {
        Ok(f64::from(executor.context().subsystem().get_daughterboard_telemetry(telem_type)?))
    }

    field watchdog_period(&executor) -> FieldResult<i32>
        as "Current watchdog period in minutes"
    {
        Ok(i32::from(executor.context().subsystem().get_comms_watchdog_period()?))
    }
});
