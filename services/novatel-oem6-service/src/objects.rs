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
use novatel_oem6_api::{Component, ReceiverStatusFlags};

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

/// Response fields for `lockStatus` query
#[derive(Clone)]
pub struct LockStatus {
    pub time_status: u8, // Validity of time
    pub time: OEMTime,   // Timestamp from last BestXYZ log message received
    pub position_status: u32,
    pub position_type: u32,
    pub velocity_status: u32,
    pub velocity_type: u32,
}

impl Default for LockStatus {
    fn default() -> LockStatus {
        LockStatus {
            time_status: 20, // Unknown
            time: OEMTime { week: 0, ms: 0 },
            position_status: 1, // Insufficient Observations
            position_type: 0,   // None
            velocity_status: 1, // Insufficient Observations
            velocity_type: 0,   // None
        }
    }
}

/// Time structure for `lockStatus` and `lockInfo` response fields
#[derive(Clone, Default, GraphQLObject)]
pub struct OEMTime {
    pub week: i32,
    pub ms: i32,
}

/// Enum for the `positionStatus` and `velocityStatus` response fields
/// of the `lockStatus` query
#[derive(GraphQLEnum, Debug)]
pub enum SolutionStatus {
    SolComputed,
    InsufficientObservations,
    NoConvergence,
    Singularity,
    CovarianceTraceExceeded,
    TestDistanceExceeded,
    ColdStart,
    HeightVelocityExceeded,
    VarianceExceeded,
    ResidualsTooLarge,
    IntegrityWarning,
    Pending,
    InvalidFix,
    Unauthorized,
    KubosInvalid,
}

impl From<u32> for SolutionStatus {
    fn from(t: u32) -> SolutionStatus {
        match t {
            0 => SolutionStatus::SolComputed,
            1 => SolutionStatus::InsufficientObservations,
            2 => SolutionStatus::NoConvergence,
            3 => SolutionStatus::Singularity,
            4 => SolutionStatus::CovarianceTraceExceeded,
            5 => SolutionStatus::TestDistanceExceeded,
            6 => SolutionStatus::ColdStart,
            7 => SolutionStatus::HeightVelocityExceeded,
            8 => SolutionStatus::VarianceExceeded,
            9 => SolutionStatus::ResidualsTooLarge,
            13 => SolutionStatus::IntegrityWarning,
            18 => SolutionStatus::Pending,
            19 => SolutionStatus::InvalidFix,
            20 => SolutionStatus::Unauthorized,
            _ => SolutionStatus::KubosInvalid,
        }
    }
}

/// Enum for the `positionType` and `velocityType` response fields
/// of the `lockStatus` query
#[derive(GraphQLEnum, Debug)]
pub enum PosVelType {
    None,
    FixedPos,
    FixedHeight,
    DopplerVelocity,
    Single,
    PSRDiff,
    WAAS,
    Propagated,
    Omnistar,
    L1Float,
    IonoFreeFloat,
    NarrowFloat,
    L1Integer,
    NarrowInteger,
    OmnistarHP,
    OmnistarXP,
    PPPConverging,
    PPP,
    Operational,
    Warning,
    OutOfBounds,
    PPPBasicConverging,
    PPPBasic,
    KubosInvalid,
}

impl From<u32> for PosVelType {
    fn from(t: u32) -> PosVelType {
        match t {
            0 => PosVelType::None,
            1 => PosVelType::FixedPos,
            2 => PosVelType::FixedHeight,
            8 => PosVelType::DopplerVelocity,
            16 => PosVelType::Single,
            17 => PosVelType::PSRDiff,
            18 => PosVelType::WAAS,
            19 => PosVelType::Propagated,
            20 => PosVelType::Omnistar,
            32 => PosVelType::L1Float,
            33 => PosVelType::IonoFreeFloat,
            34 => PosVelType::NarrowFloat,
            48 => PosVelType::L1Integer,
            50 => PosVelType::NarrowInteger,
            64 => PosVelType::OmnistarHP,
            65 => PosVelType::OmnistarXP,
            68 => PosVelType::PPPConverging,
            69 => PosVelType::PPP,
            70 => PosVelType::Operational,
            71 => PosVelType::Warning,
            72 => PosVelType::OutOfBounds,
            77 => PosVelType::PPPBasicConverging,
            78 => PosVelType::PPPBasic,
            _ => PosVelType::KubosInvalid,
        }
    }
}

/// Enum for the `TimeStatus` response field of the `lockStatus` query
#[derive(GraphQLEnum, Debug)]
pub enum RefTimeStatus {
    Unknown,
    Approximate,
    CoarseAdjusting,
    Coarse,
    CoarseSteering,
    FreeWheeling,
    FineAdjusting,
    Fine,
    FineBackupSteering,
    FineSteering,
    SatTime,
    KubosInvalid,
}

impl From<u8> for RefTimeStatus {
    fn from(t: u8) -> RefTimeStatus {
        match t {
            20 => RefTimeStatus::Unknown,
            60 => RefTimeStatus::Approximate,
            80 => RefTimeStatus::CoarseAdjusting,
            100 => RefTimeStatus::Coarse,
            120 => RefTimeStatus::CoarseSteering,
            130 => RefTimeStatus::FreeWheeling,
            140 => RefTimeStatus::FineAdjusting,
            160 => RefTimeStatus::Fine,
            170 => RefTimeStatus::FineBackupSteering,
            180 => RefTimeStatus::FineSteering,
            200 => RefTimeStatus::SatTime,
            _ => RefTimeStatus::KubosInvalid,
        }
    }
}

graphql_object!(LockStatus: () | &self | {

    field time_status() -> FieldResult<RefTimeStatus> {
        Ok(self.time_status.into())
    }

    field time() -> FieldResult<OEMTime> {
        Ok(self.time.clone())
    }

    field position_status() -> FieldResult<SolutionStatus> {
        Ok(self.position_status.into())
    }

    field position_type() -> FieldResult<PosVelType> {
        Ok(self.position_type.into())
    }

    field velocity_status() -> FieldResult<SolutionStatus> {
        Ok(self.velocity_status.into())
    }

    field velocity_type() -> FieldResult<PosVelType> {
        Ok(self.velocity_type.into())
    }
});

/// Current system lock information. Used in the response fields of
/// the `lockInfo` query
#[derive(Clone, Default)]
pub struct LockInfo {
    pub time: OEMTime,      // Timestamp when other fields were last updated
    pub position: [f64; 3], // Last known good position
    pub velocity: [f64; 3], // Last known good velocity
}

graphql_object!(LockInfo: () | &self | {
    field time() -> FieldResult<OEMTime> {
        Ok(self.time.clone())
    }

    field position() -> FieldResult<Vec<f64>> {
        Ok(self.position.to_vec())
    }

    field velocity() -> FieldResult<Vec<f64>> {
        Ok(self.velocity.to_vec())
    }
});

/// Response field for 'power' query
#[derive(GraphQLEnum, Clone, Eq, PartialEq, Debug)]
pub enum PowerState {
    On,
    Off,
}

/// Response fields for 'power' query
#[derive(GraphQLObject)]
pub struct GetPowerResponse {
    pub state: PowerState,
    pub uptime: i32,
}

pub struct SystemStatus {
    pub status: ReceiverStatusFlags,
    pub errors: Vec<String>,
}

graphql_object!(SystemStatus: () | &self | {
    field status() -> FieldResult<Vec<String>> {
        Ok(self.status.to_vec())
    }
    
    field errors() -> FieldResult<Vec<String>> {
        Ok(self.errors.clone())
    }
});

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
