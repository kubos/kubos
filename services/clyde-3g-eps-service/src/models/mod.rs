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

//! EPS system components

/// GenericResponse struct for use in queries or mutations without an explicit response
#[derive(GraphQLObject)]
pub struct MutationResponse {
    /// Any errors which occurred during query
    pub errors: String,
    /// Success or fail status of query
    pub success: bool,
}

pub mod board_status;
pub mod daughterboard_telemetry;
pub mod last_error;
pub mod motherboard_telemetry;
pub mod reset_telemetry;
pub mod subsystem;
pub mod version;
