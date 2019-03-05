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

//! Data returned by `boardStatus` telemetry query

use clyde_3g_eps_api::BoardStatus;

/// Board status flags
///
/// Returned structure contains stringified versions of the [StatusCode flags](../../../clyde_3g_eps_api/struct.StatusCode.html)
#[derive(Clone, Debug, GraphQLObject)]
pub struct BoardData {
    /// Status flags for the motherboard
    pub motherboard: Vec<String>,
    /// Status flags for the daughterboard
    pub daughterboard: Option<Vec<String>>,
}

impl Into<BoardData> for BoardStatus {
    fn into(self) -> BoardData {
        BoardData {
            motherboard: self.motherboard.to_vec(),
            daughterboard: self.daughterboard.map(|flags| flags.to_vec()),
        }
    }
}
