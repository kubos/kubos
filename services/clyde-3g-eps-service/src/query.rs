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
use kubos_service;
use models::subsystem::Subsystem;
use models::*;

/// Move this elsewhere?
pub type Context = kubos_service::Context<Subsystem>;

pub struct Root;

/// Base GraphQL query
graphql_object!(Root: Context as "Query" |&self| {

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

    field ack(&executor) -> FieldResult<subsystem::Mutations>
    {
        Ok(executor.context().subsystem().last_mutation.get())
    }

    field version(&executor) -> FieldResult<version::Data>
    {
        Ok(executor.context().subsystem().get_version()?)
    }

    // Get current reset telemetry information for the system
    //
    // {
    //    resetTelemetry {
    //      brownOut: i32,
    //      automaticSoftware: i32,
    //      manual: i32,
    //      watchdog: i32,
    //   }
    // }
    field reset_telemetry(&executor, telem_type: reset_telemetry::Type) -> FieldResult<reset_telemetry::Data>
    {
        Ok(executor.context().subsystem().get_reset_telemetry(telem_type)?)
    }

    field motherboard_telemetry(&executor, telem_type: motherboard_telemetry::Type) -> FieldResult<f64>
    {
        Ok(f64::from(executor.context().subsystem().get_motherboard_telemetry(telem_type)?))
    }

    field daughterboard_telemetry(&executor, telem_type: daughterboard_telemetry::Type) -> FieldResult<f64>
    {
        Ok(f64::from(executor.context().subsystem().get_daughterboard_telemetry(telem_type)?))
    }

    field watchdog_period(&executor) -> FieldResult<i32>
    {
        Ok(i32::from(executor.context().subsystem().get_comms_watchdog_period()?))
    }
});
