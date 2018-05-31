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
use novatel_oem6_api::Component;

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
#[derive(GraphQLEnum, Clone, Copy)]
pub enum AckCommand {
    None,
    Noop,
    ControlPower,
    ConfigureHardware,
    TestHardware,
    IssueRawCommand,
}

/// Input structure for 'configureHardware' mutation
#[derive(GraphQLInputObject)]
pub struct ConfigStruct {
    pub option: ConfigOption,
    #[graphql(default = "false")]
    pub hold: bool,
    #[graphql(default = "0.0")]
    pub interval: f64,
    #[graphql(default = "0.0")]
    pub offset: f64,
}

/// Input field for 'configureHardware' mutation
///
/// Indicates which configuration operation should be performed
#[derive(GraphQLEnum, Debug)]
pub enum ConfigOption {
    /// Configure system to output error data when errors or events occur
    LogErrorData,
    /// Configure system to output position data at a requested interval
    LogPositionData,
    /// Stop generation of all output data from device
    UnlogAll,
    /// Stop generation of error data from device
    UnlogErrorData,
    /// Stop generation of position data from device
    UnlogPositionData,
}

/// Response fields for 'configureHardware' mutation
#[derive(GraphQLObject, Clone)]
pub struct ConfigureHardwareResponse {
    pub config: String,
    pub errors: String,
    pub success: bool,
}

/// Input field for 'testHardware' mutation
///
/// Indicates which test should be run
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
        &IntegrationTestResults => match *self {
            TestResults::Integration(ref i) => Some(i),
            _ => None
        },
        &HardwareTestResults => match *self { TestResults::Hardware(ref h) => Some(h), _ => None},
    }
});

/// Response fields for 'testHardware(test: INTEGRATION)' mutation
#[derive(GraphQLObject)]
pub struct IntegrationTestResults {
    pub errors: String,
    pub success: bool,
    pub telemetry_debug: Option<VersionInfo>,
    /* TODO: Add telemetry_nominal */
}

/// Response fields for 'testHardware(test: HARDWARE)' mutation
#[derive(GraphQLObject)]
pub struct HardwareTestResults {
    pub errors: String,
    pub success: bool,
    pub data: String,
}

/// Version information about the device, returned as the
/// `telemetryDebug` response field
#[derive(GraphQLObject)]
pub struct VersionInfo {
    pub num_components: i32,
    pub components: Vec<VersionComponent>,
}

pub struct VersionComponent(pub Component);

graphql_object!(VersionComponent: () | &self | {
    field comp_type() -> FieldResult<i32> {
        Ok(self.0.comp_type as i32)
    }

    field model() -> FieldResult<String> {
        Ok(self.0.model.clone())
    }

    field serial_num() -> FieldResult<String> {
        Ok(self.0.serial_num.clone())
    }

    field hw_version() -> FieldResult<String> {
        Ok(self.0.hw_version.clone())
    }

    field sw_version() -> FieldResult<String> {
        Ok(self.0.sw_version.clone())
    }

    field boot_version() -> FieldResult<String> {
        Ok(self.0.boot_version.clone())
    }

    field compile_date() -> FieldResult<String> {
        Ok(self.0.compile_date.clone())
    }

    field compile_time() -> FieldResult<String> {
        Ok(self.0.compile_time.clone())
    }
});
