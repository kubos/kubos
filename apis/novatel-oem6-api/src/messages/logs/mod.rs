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

#![allow(dead_code)]

use byteorder::{LittleEndian, WriteBytesExt};
use nom::*;
use super::*;

pub mod version;
pub mod best_xyz;

pub use self::version::*;
pub use self::best_xyz::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Log {
    BestXYZ(BestXYZLog),
    Version(VersionLog),
    //TODO: RXSTATUSEVENTA (to use with UNLOG)
    //TODO: RXSTATUSEVENTB (to actually report error events)
}

impl Log {
    pub fn new(id: MessageID, raw: Vec<u8>) -> Option<Log> {
        match id {
            MessageID::BestXYZ => match BestXYZLog::new(raw) {
                Some(log) => Some(Log::BestXYZ(log)),
                _ => None,
            },
            MessageID::Version => match VersionLog::new(raw) {
                Some(log) => Some(Log::Version(log)),
                _ => None,
            },
            _ => None,
        }
    }
}
