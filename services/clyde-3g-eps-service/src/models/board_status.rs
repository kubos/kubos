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

use clyde_3g_eps_api::{BoardStatus, StatusCode};

#[derive(Clone, Debug, GraphQLEnum)]
pub enum Status {
    NoStatus,
    LastCommandFailed,
    WatchdogError,
    BadCommandData,
    BadCommandChannel,
    ErrorReadingEeprom,
    PowerOnReset,
    BrownOutReset,
}

#[derive(Clone, Debug, GraphQLObject)]
pub struct Data {
    pub motherboard: Status,
    pub daughterboard: Option<Status>,
}

fn to_status(status_code: StatusCode) -> Status {
    if status_code.contains(StatusCode::LAST_COMMAND_FAILED) {
        Status::LastCommandFailed
    } else if status_code.contains(StatusCode::WATCHDOG_ERROR) {
        Status::WatchdogError
    } else if status_code.contains(StatusCode::BAD_COMMAND_DATA) {
        Status::BadCommandData
    } else if status_code.contains(StatusCode::BAD_COMMAND_CHANNEL) {
        Status::BadCommandChannel
    } else if status_code.contains(StatusCode::ERROR_READING_EEPROM) {
        Status::ErrorReadingEeprom
    } else if status_code.contains(StatusCode::POWER_ON_RESET) {
        Status::PowerOnReset
    } else if status_code.contains(StatusCode::BROWN_OUT_RESET) {
        Status::BrownOutReset
    } else {
        Status::NoStatus
    }
}

impl Into<Data> for BoardStatus {
    fn into(self) -> Data {
        Data {
            motherboard: to_status(self.motherboard),
            daughterboard: self.daughterboard.map(|d| to_status(d)),
        }
    }
}
