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

/// Common response fields structure for requests
/// which don't return any specific data
#[derive(GraphQLObject)]
pub struct GenericResponse {
    pub errors: String,
    pub success: bool,
}

/// Return field for 'ack' query
///
/// Indicates last mutation executed by the service
// Future work: Actually implement this. Rust lifetimes are hard...
#[derive(GraphQLEnum, Clone, Copy)]
pub enum AckCommand {
    None,
    Noop,
    ControlPower,
    ConfigureHardware,
    TestHardware,
    IssueRawCommand,
    Arm,
    Deploy,
}

/// Response fields for 'configureHardware' mutation
#[derive(GraphQLObject)]
pub struct ConfigureHardwareResponse {
    pub errors: String,
    pub success: bool,
    pub config: ConfigureController,
}

/// Input field for 'controlPower' mutation and
/// response field for 'power' query
#[derive(GraphQLEnum, Clone, Eq, PartialEq, Debug)]
pub enum PowerState {
    On,
    Off,
    Reset,
}

/// Response fields for 'power' query
pub struct GetPowerResponse {
    pub state: PowerState,
    pub uptime: u32,
}

graphql_object!(GetPowerResponse: () |&self| {
    field state() -> FieldResult<PowerState> {
        Ok(self.state.clone())
    }
    
    field uptime() -> FieldResult<i32> {
        Ok(self.uptime as i32)
    }
});

/// Response fields for 'controlPower' mutation
#[derive(GraphQLObject)]
pub struct ControlPowerResponse {
    pub errors: String,
    pub success: bool,
    pub power: PowerState,
}

/// Response fields for 'noop' mutation
pub type NoopResponse = GenericResponse;

/// Input field for 'testHardware' mutation
///
/// Indicates which test should be run against the AntS device
#[derive(GraphQLEnum)]
pub enum TestType {
    Integration,
    Hardware,
}

/// Enum for the 'testHardware' mutation response union
pub enum TestResults {
    Integration(IntegrationTestResults),
    Hardware(HardwareTestResults),
}

/// Response union for 'testHardware' mutation
graphql_union!(TestResults: () |&self| {
    instance_resolvers: |&_| {
        &IntegrationTestResults => match *self { TestResults::Integration(ref i) => Some(i), _ => None},
        &HardwareTestResults => match *self { TestResults::Hardware(ref h) => Some(h), _ => None},
    }
});

/// Response fields for 'testHardware(test: INTEGRATION)' mutation
#[derive(GraphQLObject)]
pub struct IntegrationTestResults {
    pub errors: String,
    pub success: bool,
    pub telemetry_nominal: TelemetryNominal,
    pub telemetry_debug: TelemetryDebug,
}

/// Response fields for 'testHardware(test: HARDWARE)' mutation
#[derive(GraphQLObject)]
pub struct HardwareTestResults {
    pub errors: String,
    pub success: bool,
    pub data: String,
}

/// Response fields for 'issueRawCommand' mutation
#[derive(GraphQLObject)]
pub struct RawCommandResponse {
    pub errors: String,
    pub success: bool,
    pub response: String,
}

/// Input field for 'telemetry' query
///
/// Indicates which type of telemetry data should be fetched
#[derive(GraphQLEnum)]
pub enum TelemetryType {
    Nominal,
    Debug,
}

/// Enum for 'telemetry' query response union
pub enum Telemetry {
    Nominal(TelemetryNominal),
    Debug(TelemetryDebug),
}

/// Response union for 'telemetry' query
graphql_union!(Telemetry: () |&self| {
    description: "Test"
    instance_resolvers: |&_| {
        &TelemetryNominal => match *self { Telemetry::Nominal(ref n) => Some(n), _ => None},
        &TelemetryDebug => match *self { Telemetry::Debug(ref d) => Some(d), _ => None},
    }
});

/// Response fields for 'telemetry(telem: NOMINAL)' query
#[derive(Debug, Default, PartialEq)]
pub struct TelemetryNominal(pub StandardTelemetry);

graphql_object!(TelemetryNominal: () |&self| {
 

});

/// Response fields for 'telemetry(telem: DEBUG)' query
#[derive(Debug, Default, PartialEq)]
pub struct TelemetryDebug {
    pub irehs: IREHSTelemetry,
    pub raw_imu: RawIMU,
    pub config: ConfigInfo,
}

graphql_object!(TelemetryDebug: () |&self| {

});
