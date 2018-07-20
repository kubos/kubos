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

use clyde_3g_eps_api::{Version, VersionInfo};

#[derive(Clone, Debug, GraphQLObject)]
pub struct VersionData {
    pub revision: i32,
    pub firmware_number: i32,
}

#[derive(Clone, Debug, GraphQLObject)]
#[graphql(name = "data")]
pub struct Data {
    pub motherboard: VersionData,
    pub daughterboard: Option<VersionData>,
}

impl Into<VersionData> for Version {
    fn into(self) -> VersionData {
        VersionData {
            revision: i32::from(self.revision),
            firmware_number: i32::from(self.firmware_number),
        }
    }
}

impl Into<Data> for VersionInfo {
    fn into(self) -> Data {
        Data {
            motherboard: self.motherboard.into(),
            daughterboard: self.daughterboard.map(|d| d.into()),
        }
    }
}
