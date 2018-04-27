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

/// Module for receiving and processing the IREHS telemetry message
pub mod irehs;
/// Module for receiving and processing the raw IMU telemetry message
pub mod raw_imu;
/// Module for extracting and saving the rotating variables from the standard telemetry messages
pub mod rotating;
/// Module for receiving and processing the standard telemetry message
pub mod std_telem;

pub use self::irehs::*;
pub use self::raw_imu::*;
pub use self::rotating::*;
pub use self::std_telem::*;

/// Sync word for raw IMU and IREHS telemetry packets
pub const AUX_SYNC: u16 = 0xEA91;
