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

use super::*;
use nom::*;

/// Event/error log message
#[derive(Clone, Default, Debug, PartialEq)]
pub struct RxStatusEventLog {
    /// Current status of receiver
    pub recv_status: ReceiverStatusFlags,
    /// Validity of the time information
    pub time_status: u8,
    /// GPS reference week
    pub week: u16,
    /// Milliseconds into GPS reference week
    pub ms: i32,
    /// Status word which generated the event
    pub word: u32,
    /// Location of the bit in the status word
    pub bit: u32,
    /// Event type
    pub event: u32,
    /// Text description of event/error
    pub description: String,
}

impl RxStatusEventLog {
    /// Convert a raw data buffer into a useable struct
    pub fn new(
        recv_status: ReceiverStatusFlags,
        time_status: u8,
        week: u16,
        ms: i32,
        raw: &[u8],
    ) -> Option<Self> {
        let mut log = match parse_rxstatusevent(&raw) {
            Ok(conv) => conv.1,
            _ => return None,
        };

        log.recv_status = recv_status;
        log.time_status = time_status;
        log.week = week;
        log.ms = ms;

        Some(log)
    }
}

named!(parse_rxstatusevent(&[u8]) -> RxStatusEventLog,
    do_parse!(
        word: le_u32 >>
        bit: le_u32 >>
        event: le_u32 >>
        description: take!(32) >>
        (RxStatusEventLog {
            recv_status: ReceiverStatusFlags::empty(),
            time_status: 0,
            week: 0,
            ms: 0,
            word,
            bit,
            event,
            description: String::from_utf8_lossy(description)
                            .trim_end_matches('\u{0}').to_owned(),
            }
        )
    )
);
