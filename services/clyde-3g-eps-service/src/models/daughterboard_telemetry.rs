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

//! Data returned by `daughterboardTelemetry` telemetry query

use crate::schema::Context;
use clyde_3g_eps_api::DaughterboardTelemetry::Type as DaughterboardTelemetryType;
use juniper::FieldResult;

/// Daughterboard telemetry structure
pub struct Telemetry;

macro_rules! make_telemetry {
    (
        $($type: ident,)+
    ) => {
        /// Daughterboard telemetry values
        ///
        /// See Table 11-8 in the EPS' User Manual for more information
        #[derive(Clone, Hash, Debug, Eq, GraphQLEnum, PartialEq)]
        pub enum Type {
            $(
                /// $type
                $type,
            )+
        }

        impl From<Type> for DaughterboardTelemetryType {
            fn from(t: Type) -> Self {
                match t {
                    $(Type::$type => Self::$type,)+
                }
            }
        }

        graphql_object!(Telemetry: Context as "daughterboard" |&self| {
            $(
                field $type(&executor) -> FieldResult<f64>
                {
                    Ok(executor.context().subsystem().get_daughterboard_telemetry(Type::$type)? as f64)
                }
            )+
        });
    }
}

make_telemetry!(
    VoltageFeedingBcr4,
    CurrentBcr4Sa4a,
    CurrentBcr4Sa4b,
    ArrayTempSa4a,
    ArrayTempSa4b,
    SunDetectorSa4a,
    SunDetectorSa4b,
    VoltageFeedingBcr5,
    CurrentBcr5Sa5a,
    CurrentBcr5Sa5b,
    ArrayTempSa5a,
    ArrayTempSa5b,
    SunDetectorSa5a,
    SunDetectorSa5b,
    VoltageFeedingBcr6,
    CurrentBcr6Sa6a,
    CurrentBcr6Sa6b,
    ArrayTempSa6a,
    ArrayTempSa6b,
    SunDetectorSa6a,
    SunDetectorSa6b,
    VoltageFeedingBcr7,
    CurrentBcr7Sa7a,
    CurrentBcr7Sa7b,
    ArrayTempSa7a,
    ArrayTempSa7b,
    SunDetectorSa7a,
    SunDetectorSa7b,
    VoltageFeedingBcr8,
    CurrentBcr8Sa8a,
    CurrentBcr8Sa8b,
    ArrayTempSa8a,
    ArrayTempSa8b,
    SunDetectorSa8a,
    SunDetectorSa8b,
    VoltageFeedingBcr9,
    CurrentBcr9Sa9a,
    CurrentBcr9Sa9b,
    ArrayTempSa9a,
    ArrayTempSa9b,
    SunDetectorSa9a,
    SunDetectorSa9b,
    BoardTemperature,
);
