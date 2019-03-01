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

use clyde_3g_eps_api::MotherboardTelemetry::Type as MotherboardTelemetryType;
use juniper::FieldResult;
use crate::schema::Context;

pub struct Telemetry;

macro_rules! make_telemetry {
    (
        $($type: ident,)+
    ) => {
        #[derive(Clone, Debug, Hash, Eq, GraphQLEnum, PartialEq)]
        pub enum Type {
            $($type,)+
        }

        impl Into<MotherboardTelemetryType> for Type {
            fn into(self) -> MotherboardTelemetryType {
                match self {
                    $(Type::$type => MotherboardTelemetryType::$type,)+
                }
            }
        }

        graphql_object!(Telemetry: Context as "MotherboardTelemetry" |&self| {
            $(
                field $type(&executor) -> FieldResult<f64>
                {
                    Ok(f64::from(executor.context().subsystem().get_motherboard_telemetry(Type::$type)?))
                }
            )+
        });
    }
}

make_telemetry!(
    VoltageFeedingBcr1,
    CurrentBcr1Sa1a,
    CurrentBcr1Sa1b,
    ArrayTempSa1a,
    ArrayTempSa1b,
    SunDetectorSa1a,
    SunDetectorSa1b,
    VoltageFeedingBcr2,
    CurrentBcr2Sa2a,
    CurrentBcr2Sa2b,
    ArrayTempSa2a,
    ArrayTempSa2b,
    SunDetectorSa2a,
    SunDetectorSa2b,
    VoltageFeedingBcr3,
    CurrentBcr3Sa3a,
    CurrentBcr3Sa3b,
    ArrayTempSa3a,
    ArrayTempSa3b,
    SunDetectorSa3a,
    SunDetectorSa3b,
    BcrOutputCurrent,
    BcrOutputVoltage,
    CurrentDraw3V3,
    CurrentDraw5V,
    OutputCurrent12V,
    OutputVoltage12V,
    OutputCurrentBattery,
    OutputVoltageBattery,
    OutputCurrent5v,
    OutputVoltage5v,
    OutputCurrent33v,
    OutputVoltage33v,
    OutputVoltageSwitch1,
    OutputCurrentSwitch1,
    OutputVoltageSwitch2,
    OutputCurrentSwitch2,
    OutputVoltageSwitch3,
    OutputCurrentSwitch3,
    OutputVoltageSwitch4,
    OutputCurrentSwitch4,
    OutputVoltageSwitch5,
    OutputCurrentSwitch5,
    OutputVoltageSwitch6,
    OutputCurrentSwitch6,
    OutputVoltageSwitch7,
    OutputCurrentSwitch7,
    OutputVoltageSwitch8,
    OutputCurrentSwitch8,
    OutputVoltageSwitch9,
    OutputCurrentSwitch9,
    OutputVoltageSwitch10,
    OutputCurrentSwitch10,
    BoardTemperature,
);
