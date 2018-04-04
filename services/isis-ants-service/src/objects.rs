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

use isis_ants_api::{AntsTelemetry, DeployStatus};
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

/// Return field for 'armStatus' query
#[derive(GraphQLEnum)]
pub enum ArmStatus {
    Armed,
    Disarmed,
}

/// Input field for 'arm' mutation
#[derive(GraphQLEnum)]
pub enum ArmState {
    Arm,
    Disarm,
}

/// Response fields for 'arm' mutation
pub type ArmResponse = GenericResponse;

/// Input field for 'configureHardware' mutation
///
/// Sets which AntS microcontroller will be used to issue
/// commands to the antennas
#[derive(GraphQLEnum, Clone, Eq, PartialEq, Debug)]
pub enum ConfigureController {
    Primary,
    Secondary,
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

/// Enum for 'deployStatus' response field of 'deploymentStatus' query
#[derive(GraphQLEnum, Clone)]
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
    pub status: DeploymentStatus,
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
///
/// Indicates which antenna should be deployed.
/// `All` indicates that all antennas should be sequentially deployed
#[derive(GraphQLEnum)]
pub enum DeployType {
    All,
    Antenna1,
    Antenna2,
    Antenna3,
    Antenna4,
}

/// Response fields for 'deploy' mutation
pub type DeployResponse = GenericResponse;

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
#[derive(Default)]
pub struct TelemetryNominal(pub AntsTelemetry);

graphql_object!(TelemetryNominal: () |&self| {
    field raw_temp() -> FieldResult<i32> {
        Ok(self.0.raw_temp as i32)
    }
    
    field uptime() -> FieldResult<i32> {
        Ok(self.0.uptime as i32)
    }
    
    field sys_burn_active() -> FieldResult<bool> {
        Ok(self.0.deploy_status.sys_burn_active)
    }

    field sys_ignore_deploy() -> FieldResult<bool> {
        Ok(self.0.deploy_status.sys_ignore_deploy)
    }

    field sys_armed() -> FieldResult<bool> {
        Ok(self.0.deploy_status.sys_armed)
    }

    field ant_1_not_deployed() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_1_not_deployed)
    }

    field ant_1_stopped_time() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_1_stopped_time)
    }

    field ant_1_active() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_1_active)
    }

    field ant_2_not_deployed() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_2_not_deployed)
    }

    field ant_2_stopped_time() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_2_stopped_time)
    }

    field ant_2_active() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_2_active)
    }

    field ant_3_not_deployed() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_3_not_deployed)
    }

    field ant_3_stopped_time() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_3_stopped_time)
    }

    field ant_3_active() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_3_active)
    }

    field ant_4_not_deployed() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_4_not_deployed)
    }

    field ant_4_stopped_time() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_4_stopped_time)
    }

    field ant_4_active() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_4_active)
    }

});

/// Response fields for 'telemetry(telem: DEBUG)' query
#[derive(Default)]
pub struct TelemetryDebug {
    pub ant1: AntennaStats,
    pub ant2: AntennaStats,
    pub ant3: AntennaStats,
    pub ant4: AntennaStats,
}

#[derive(Default)]
pub struct AntennaStats {
    pub act_count: u8,
    pub act_time: u16,
}

graphql_object!(TelemetryDebug: () |&self| {
    field ant_1_activation_count() -> FieldResult<i32> {
        Ok(self.ant1.act_count as i32)
    }
    
    field ant_1_activation_time() -> FieldResult<i32> {
        Ok(self.ant1.act_time as i32)
    }
    
    field ant_2_activation_count() -> FieldResult<i32> {
        Ok(self.ant2.act_count as i32)
    }
    
    field ant_2_activation_time() -> FieldResult<i32> {
        Ok(self.ant2.act_time as i32)
    }
    
    field ant_3_activation_count() -> FieldResult<i32> {
        Ok(self.ant3.act_count as i32)
    }
    
    field ant_3_activation_time() -> FieldResult<i32> {
        Ok(self.ant3.act_time as i32)
    }
    
    field ant_4_activation_count() -> FieldResult<i32> {
        Ok(self.ant4.act_count as i32)
    }
    
    field ant_4_activation_time() -> FieldResult<i32> {
        Ok(self.ant4.act_time as i32)
    }
});
