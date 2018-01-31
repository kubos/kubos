//
// Copyright (C) 2017 Kubos Corporation
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

use libc::uint8_t;

pub const LENGTH_TELEMETRY_HOUSEKEEPING: usize = 37;
pub const LENGTH_TELEMETRY_GET_VERSION: usize = 34;
pub const LENGTH_COMPILE_INFORMATION: usize = 19;
pub const SUPERVISOR_NUMBER_OF_ADC_CHANNELS: usize = 10;

/// There isn't a big need to replicate the C-union/struct representation
/// of the data only to have it moved into a Rust struct. So we will just
/// use an array of uint8_t to capture the data and later move it into
/// a proper Rust struct.
#[repr(C)]
pub struct supervisor_version(pub [uint8_t; LENGTH_TELEMETRY_GET_VERSION]);

#[repr(C)]
pub struct supervisor_housekeeping(pub [uint8_t; LENGTH_TELEMETRY_HOUSEKEEPING]);

/// Bring in C functions from kubos-hal-iobc
extern "C" {
    pub fn supervisor_emergency_reset() -> bool;
    pub fn supervisor_reset() -> bool;
    pub fn supervisor_powercycle() -> bool;
    pub fn supervisor_get_version(version: *mut supervisor_version) -> bool;
    pub fn supervisor_get_housekeeping(housekeeping: *mut supervisor_housekeeping) -> bool;
}
