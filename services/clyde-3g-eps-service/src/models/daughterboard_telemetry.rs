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

use clyde_3g_eps_api::DaughterboardTelemetry::Type as DaughterboardTelemetryType;

macro_rules! make_telemetry {
    (
        $($type: ident,)+
    ) => {
        #[derive(GraphQLEnum)]
        pub enum Type {
            $($type,)+
        }

        impl Into<DaughterboardTelemetryType> for Type {
            fn into(self) -> DaughterboardTelemetryType {
                match self {
                    $(Type::$type => DaughterboardTelemetryType::$type,)+
                }
            }
        }
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
