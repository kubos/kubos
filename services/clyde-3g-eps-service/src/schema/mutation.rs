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

//! Service mutations

use crate::models::subsystem::Mutations;
use crate::models::MutationResponse;
use crate::schema::Context;
use juniper::FieldResult;

/// Top-level mutation root structure
pub struct Root;

/// Base GraphQL mutation model
graphql_object!(Root: Context as "Mutation" |&self| {

    // TODO: No-op. Reset watchdog

    field manual_reset(&executor) -> FieldResult<MutationResponse>
        as "Perform manual reset of EPS board"
    {
        executor.context().subsystem().set_last_mutation(Mutations::ManualReset);
        Ok(executor.context().subsystem().manual_reset()?)
    }

    field reset_watchdog(&executor) -> FieldResult<MutationResponse>
        as "Reset/kick communications watchdog"
    {
        executor.context().subsystem().set_last_mutation(Mutations::ResetWatchdog);
        Ok(executor.context().subsystem().reset_watchdog()?)
    }

    field set_watchdog_period(&executor, period: i32) -> FieldResult<MutationResponse>
        as "Set watchdog period (in minutes)"
    {
        executor.context().subsystem().set_last_mutation(Mutations::SetWatchdogPeriod);
        Ok(executor.context().subsystem().set_watchdog_period(period as u8)?)
    }

    field issue_raw_command(&executor, command: i32, data: Vec<i32>) -> FieldResult<MutationResponse>
        as "Issue raw command to EPS"
    {
        executor.context().subsystem().set_last_mutation(Mutations::RawCommand);
        let data_u8 = data.iter().map(|x| *x as u8).collect();
        Ok(executor.context().subsystem().raw_command(command as u8, data_u8)?)
    }
});
