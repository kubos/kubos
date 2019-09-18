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

use failure::Error;
use isis_ants_api::{AntsTelemetry, DeployStatus, IAntS, KANTSAnt};
use juniper::FieldResult;
use kubos_service::{process_errors, push_err, run};
use std::sync::{Arc, Mutex, RwLock};

/// Common response fields structure for requests
/// which don't return any specific data
#[derive(GraphQLObject)]
pub struct GenericResponse {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
}

/// Return field for 'ack' query
///
/// Indicates last mutation executed by the service
#[derive(GraphQLEnum, Clone, Copy)]
pub enum AckCommand {
    /// No mutations have been executed
    None,
    /// No-Op
    Noop,
    /// System power state was changed
    ControlPower,
    /// System configuration was updated
    ConfigureHardware,
    /// A hardware test was performed
    TestHardware,
    /// A raw command was passed through to the system
    IssueRawCommand,
    /// Arm state was changed
    Arm,
    /// Antenna/s were deployed
    Deploy,
}

/// Return field for 'armStatus' query
#[derive(GraphQLEnum, Eq, PartialEq, Debug)]
pub enum ArmStatus {
    /// System is armed
    Armed,
    /// System is disarmed
    Disarmed,
}

/// Input field for 'arm' mutation
#[derive(GraphQLEnum)]
pub enum ArmState {
    /// Arm the system
    Arm,
    /// Disarm the system
    Disarm,
}

/// Response fields for 'arm' mutation
pub type ArmResponse = GenericResponse;

/// Input field for 'configureHardware' mutation
///
/// Sets which AntS microcontroller will be used to issue
/// commands to the antennas
#[derive(GraphQLEnum, Clone, Copy, Eq, PartialEq, Debug)]
pub enum ConfigureController {
    /// Primary controller should be used
    Primary,
    /// Secondary controller should be used
    Secondary,
}

/// Response fields for 'configureHardware' mutation
#[derive(GraphQLObject)]
pub struct ConfigureHardwareResponse {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
    /// Controller which is being used to issue commands to the antennas
    pub config: ConfigureController,
}

/// Input field for 'controlPower' mutation and
/// response field for 'power' query
#[derive(GraphQLEnum, Clone, Eq, PartialEq, Debug)]
pub enum PowerState {
    /// System is on
    On,
    /// System is off or unavailable
    Off,
    /// System will be reset
    Reset,
}

/// Response fields for 'power' query
pub struct GetPowerResponse {
    /// Current power status
    pub state: PowerState,
    /// System uptime, in seconds
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
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
    /// Current power status
    pub power: PowerState,
}

/// Enum for 'deployStatus' response field of 'deploymentStatus' query
#[derive(GraphQLEnum, Clone, Debug, PartialEq)]
pub enum DeploymentStatus {
    /// All antennas have been successfully deployed
    Deployed,
    /// At least one antenna is currently being deployed
    InProgress,
    /// Some, but not all, antennas have been deployed.
    /// No errors are present
    Partial,
    /// All antennas are stowed (not deployed)
    Stowed,
    /// At least one antenna encountered an error during its last
    /// deployment attempt
    Error,
}

/// Response fields for 'deploymentStatus' query
pub struct GetDeployResponse {
    /// Overall deployment status
    pub status: DeploymentStatus,
    /// Full deployment status
    pub details: DeployStatus,
}

graphql_object!(GetDeployResponse: () |&self| {
    field status() -> FieldResult<DeploymentStatus> {
        Ok(self.status.clone())
    }

    field sys_burn_active() -> FieldResult<bool> {
        Ok(self.details.sys_burn_active)
    }

    field sys_ignore_deploy() -> FieldResult<bool> {
        Ok(self.details.sys_ignore_deploy)
    }

    field sys_armed() -> FieldResult<bool> {
        Ok(self.details.sys_armed)
    }

    field ant_1_not_deployed() -> FieldResult<bool> {
        Ok(self.details.ant_1_not_deployed)
    }

    field ant_1_stopped_time() -> FieldResult<bool> {
        Ok(self.details.ant_1_stopped_time)
    }

    field ant_1_active() -> FieldResult<bool> {
        Ok(self.details.ant_1_active)
    }

    field ant_2_not_deployed() -> FieldResult<bool> {
        Ok(self.details.ant_2_not_deployed)
    }

    field ant_2_stopped_time() -> FieldResult<bool> {
        Ok(self.details.ant_2_stopped_time)
    }

    field ant_2_active() -> FieldResult<bool> {
        Ok(self.details.ant_2_active)
    }

    field ant_3_not_deployed() -> FieldResult<bool> {
        Ok(self.details.ant_3_not_deployed)
    }

    field ant_3_stopped_time() -> FieldResult<bool> {
        Ok(self.details.ant_3_stopped_time)
    }

    field ant_3_active() -> FieldResult<bool> {
        Ok(self.details.ant_3_active)
    }

    field ant_4_not_deployed() -> FieldResult<bool> {
        Ok(self.details.ant_4_not_deployed)
    }

    field ant_4_stopped_time() -> FieldResult<bool> {
        Ok(self.details.ant_4_stopped_time)
    }

    field ant_4_active() -> FieldResult<bool> {
        Ok(self.details.ant_4_active)
    }

});

/// Input field for 'deploy' mutation
#[derive(GraphQLEnum)]
pub enum DeployType {
    /// Deploy all antennas sequentially
    All,
    /// Deploy antenna 1
    Antenna1,
    /// Deploy antenna 2
    Antenna2,
    /// Deploy antenna 3
    Antenna3,
    /// Deploy antenna 4
    Antenna4,
}

/// Response fields for 'deploy' mutation
pub type DeployResponse = GenericResponse;

/// Response fields for 'noop' mutation
pub type NoopResponse = GenericResponse;

/// Input field for 'testHardware' mutation
#[derive(GraphQLEnum)]
pub enum TestType {
    /// Integration (non-invasive) test
    Integration,
    /// Hardware (invasive) test
    Hardware,
}

/// Enum for the 'testHardware' mutation response union
pub enum TestResults {
    /// Integration test results
    Integration(IntegrationTestResults),
    /// Hardware test results
    Hardware(HardwareTestResults),
}

// Response union for 'testHardware' mutation
graphql_union!(TestResults: () where Scalar = <S> |&self| {
    instance_resolvers: |&_| {
        &IntegrationTestResults => match *self { TestResults::Integration(ref i) => Some(i), _ => None},
        &HardwareTestResults => match *self { TestResults::Hardware(ref h) => Some(h), _ => None},
    }
});

/// Response fields for 'testHardware(test: INTEGRATION)' mutation
#[derive(GraphQLObject)]
pub struct IntegrationTestResults {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
    /// Nominal telemetry
    pub telemetry_nominal: TelemetryNominal,
    /// Debug telemetry
    pub telemetry_debug: TelemetryDebug,
}

/// Response fields for 'testHardware(test: HARDWARE)' mutation
#[derive(GraphQLObject)]
pub struct HardwareTestResults {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
    /// Test results
    pub data: String,
}

/// Response fields for 'issueRawCommand' mutation
#[derive(GraphQLObject)]
pub struct RawCommandResponse {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
    /// Command response from system
    pub response: String,
}

/// Response fields for 'telemetry' query
#[derive(GraphQLObject)]
pub struct Telemetry {
    /// Nominal telemetry
    pub nominal: TelemetryNominal,
    /// Debug telemetry
    pub debug: TelemetryDebug,
}

/// Nominal telemetry data
#[derive(Debug, Default, PartialEq)]
pub struct TelemetryNominal(pub AntsTelemetry);

graphql_object!(TelemetryNominal: () where Scalar = <S> |&self| {
    field raw_temp() -> i32 {
        i32::from(self.0.raw_temp)
    }

    field uptime() -> i32 {
        self.0.uptime as i32
    }

    field sys_burn_active() -> bool {
        self.0.deploy_status.sys_burn_active
    }

    field sys_ignore_deploy() -> bool {
        self.0.deploy_status.sys_ignore_deploy
    }

    field sys_armed() -> bool {
        self.0.deploy_status.sys_armed
    }

    field ant_1_not_deployed() -> bool {
        self.0.deploy_status.ant_1_not_deployed
    }

    field ant_1_stopped_time() -> bool {
        self.0.deploy_status.ant_1_stopped_time
    }

    field ant_1_active() -> bool {
        self.0.deploy_status.ant_1_active
    }

    field ant_2_not_deployed() -> bool {
        self.0.deploy_status.ant_2_not_deployed
    }

    field ant_2_stopped_time() -> bool {
        self.0.deploy_status.ant_2_stopped_time
    }

    field ant_2_active() -> bool {
        self.0.deploy_status.ant_2_active
    }

    field ant_3_not_deployed() -> bool {
        self.0.deploy_status.ant_3_not_deployed
    }

    field ant_3_stopped_time() -> bool {
        self.0.deploy_status.ant_3_stopped_time
    }

    field ant_3_active() -> bool {
        self.0.deploy_status.ant_3_active
    }

    field ant_4_not_deployed() -> bool {
        self.0.deploy_status.ant_4_not_deployed
    }

    field ant_4_stopped_time() -> bool {
        self.0.deploy_status.ant_4_stopped_time
    }

    field ant_4_active() -> bool {
        self.0.deploy_status.ant_4_active
    }

});

/// Debug telemetry data
#[derive(Debug, Default, PartialEq)]
pub struct TelemetryDebug {
    /// Antenna 1 status
    pub ant1: AntennaStats,
    /// Antenna 2 status
    pub ant2: AntennaStats,
    /// Antenna 3 status
    pub ant3: AntennaStats,
    /// Antenna 4 status
    pub ant4: AntennaStats,
}

/// Antenna status data
#[derive(Debug, Default, PartialEq)]
pub struct AntennaStats {
    /// Number of times deployment has been attempted without success
    pub act_count: u8,
    /// Cummulative amount of time, in 50ms steps, which has been spent deploying the antenna
    pub act_time: u16,
}

/// Get status data for a particular antenna
pub fn get_stats(
    ants_ref: &Arc<Mutex<Box<dyn IAntS>>>,
    errors: &Arc<RwLock<Vec<String>>>,
    antenna: KANTSAnt,
) -> AntennaStats {
    let ants = ants_ref.lock().unwrap();

    AntennaStats {
        act_count: run!(ants.get_activation_count(antenna.clone()), errors).unwrap_or_default(),
        act_time: run!(ants.get_activation_time(antenna), errors).unwrap_or_default(),
    }
}

graphql_object!(TelemetryDebug: () where Scalar = <S> |&self| {
    field ant_1_activation_count() -> i32 {
        i32::from(self.ant1.act_count)
    }

    field ant_1_activation_time() -> i32 {
        i32::from(self.ant1.act_time)
    }

    field ant_2_activation_count() -> i32 {
        i32::from(self.ant2.act_count)
    }

    field ant_2_activation_time() -> i32 {
        i32::from(self.ant2.act_time)
    }

    field ant_3_activation_count() -> i32 {
        i32::from(self.ant3.act_count)
    }

    field ant_3_activation_time() -> i32 {
        i32::from(self.ant3.act_time)
    }

    field ant_4_activation_count() -> i32 {
        i32::from(self.ant4.act_count)
    }

    field ant_4_activation_time() -> i32 {
        i32::from(self.ant4.act_time)
    }
});
