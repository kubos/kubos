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

mod best_xyz;
mod rxstatusevent;
mod version;

pub use self::best_xyz::*;
pub use self::rxstatusevent::*;
pub use self::version::*;
use super::*;

/// Supported log messages
#[derive(Clone, Debug, PartialEq)]
pub enum Log {
    /// Best available position and velocity in ECEF coordinates
    BestXYZ(BestXYZLog),
    /// Event and/or error message
    RxStatusEvent(RxStatusEventLog),
    /// System version information
    Version(VersionLog),
}

impl Log {
    /// Convert a raw data buffer into a useable struct
    pub fn new(
        id: MessageID,
        recv_status: ReceiverStatusFlags,
        time_status: u8,
        week: u16,
        ms: i32,
        raw: Vec<u8>,
    ) -> Option<Log> {
        match id {
            MessageID::BestXYZ => {
                BestXYZLog::new(recv_status, time_status, week, ms, &raw).map(Log::BestXYZ)
            }
            MessageID::RxStatusEvent => {
                RxStatusEventLog::new(recv_status, time_status, week, ms, &raw)
                    .map(Log::RxStatusEvent)
            }
            MessageID::Version => {
                VersionLog::new(recv_status, time_status, week, ms, raw).map(Log::Version)
            }
            _ => None,
        }
    }
}
