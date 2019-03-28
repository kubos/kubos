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

//! Data returned by `reset` telemetry query

use crate::schema::Context;
use clyde_3g_eps_api::ResetTelemetry::Data as ResetTelemetryData;
use clyde_3g_eps_api::ResetTelemetry::Type as ResetTelemetryType;
use juniper::FieldResult;

/// Reset count telemetry structure used by each reset type
#[derive(Clone, Debug, GraphQLObject)]
pub struct Data {
    /// Motherboard reset count
    pub motherboard: i32,
    /// Daughterboard reset count
    pub daughterboard: Option<i32>,
}

impl Into<Data> for ResetTelemetryData {
    fn into(self) -> Data {
        Data {
            motherboard: i32::from(self.motherboard),
            daughterboard: self.daughterboard.map(i32::from),
        }
    }
}

/// Reset types
#[derive(Clone, Debug, Eq, Hash, GraphQLEnum, PartialEq)]
pub enum Type {
    /// Brown-out reset
    BrownOut,
    /// Reset automatically triggered by the EPS when it experiences a malfunction
    AutomaticSoftware,
    /// Manual reset
    Manual,
    /// Reset triggered by the I2C watchdog
    Watchdog,
}

impl Into<ResetTelemetryType> for Type {
    fn into(self) -> ResetTelemetryType {
        match self {
            Type::BrownOut => ResetTelemetryType::BrownOut,
            Type::AutomaticSoftware => ResetTelemetryType::AutomaticSoftware,
            Type::Manual => ResetTelemetryType::Manual,
            Type::Watchdog => ResetTelemetryType::Watchdog,
        }
    }
}

/// High-level reset telemetry structure
pub struct Telemetry;

graphql_object!(Telemetry: Context as "ResetTelemetry" |&self| {
    field brown_out(&executor) -> FieldResult<Data>
    {
        Ok(executor.context().subsystem().get_reset_telemetry(Type::BrownOut)?)
    }

    field automatic_software(&executor) -> FieldResult<Data>
    {
        Ok(executor.context().subsystem().get_reset_telemetry(Type::AutomaticSoftware)?)
    }

    field manual(&executor) -> FieldResult<Data>
    {
        Ok(executor.context().subsystem().get_reset_telemetry(Type::Manual)?)
    }

    field watchdog(&executor) -> FieldResult<Data>
    {
        Ok(executor.context().subsystem().get_reset_telemetry(Type::Watchdog)?)
    }
});
