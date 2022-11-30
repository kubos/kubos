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

//! Data returned by the `version` telemetry query

use clyde_3g_eps_api::{Version, VersionInfo};

/// Board version informaton
#[derive(Clone, Debug, GraphQLObject)]
pub struct VersionNum {
    /// Revision number of the firmware
    pub revision: i32,
    /// Firmware version number
    pub firmware_number: i32,
}

/// High-level version data structure
#[derive(Clone, Debug, GraphQLObject)]
pub struct VersionData {
    /// Motherboard version information
    pub motherboard: VersionNum,
    /// Daugtherboard version information
    pub daughterboard: Option<VersionNum>,
}

impl From<Version> for VersionNum {
    fn from(version: Version) -> Self {
        Self {
            revision: i32::from(version.revision),
            firmware_number: i32::from(version.firmware_number),
        }
    }
}

impl From<VersionInfo> for VersionData {
    fn from(info: VersionInfo) -> Self {
        Self {
            motherboard: info.motherboard.into(),
            daughterboard: info.daughterboard.map(|d| d.into()),
        }
    }
}
