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
use query::Context;

pub struct Root;

/// Base GraphQL mutation model
graphql_object!(Root: Context as "Mutation" |&self| {

    // Performs a manual reset of the EPS board
    field manual_reset(&executor) -> FieldResult<()>
    {
        Ok(executor.context().subsystem().manual_reset()?)
    }

    // Reset communications watchdog
    field reset_watchdog(&executor) -> FieldResult<()>
    {
        Ok(executor.context().subsystem().reset_watchdog()?)
    }

    // Set watchdog period
    field set_watchdog_period(&executor, period: i32) -> FieldResult<()>
    {
        Ok(executor.context().subsystem().set_watchdog_period(period as u8)?)
    }
});
