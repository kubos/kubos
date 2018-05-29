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
