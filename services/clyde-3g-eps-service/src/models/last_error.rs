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

use clyde_3g_eps_api::{ErrorCode, LastError};

#[derive(Clone, Debug, GraphQLEnum)]
pub enum Error {
    None,
    BadCrc,
    UnknownCommand,
    CommandDataIncorrect,
    ChannelDoesNotExist,
    ChannelInactive,
    ResetOccurred,
    BadAdcAcquisition,
    FailReadingEeprom,
    InternalSpiError,
}

#[derive(Clone, Debug, GraphQLObject)]
pub struct Data {
    pub motherboard: Error,
    pub daughterboard: Option<Error>,
}

fn to_error(error_code: ErrorCode) -> Error {
    if error_code.contains(ErrorCode::BAD_CRC) {
        Error::BadCrc
    } else if error_code.contains(ErrorCode::UNKNOWN_COMMAND) {
        Error::UnknownCommand
    } else if error_code.contains(ErrorCode::COMMAND_DATA_INCORRECT) {
        Error::CommandDataIncorrect
    } else if error_code.contains(ErrorCode::CHANNEL_DOES_NOT_EXIST) {
        Error::ChannelDoesNotExist
    } else if error_code.contains(ErrorCode::CHANNEL_INACTIVE) {
        Error::ChannelInactive
    } else if error_code.contains(ErrorCode::RESET_OCCURRED) {
        Error::ResetOccurred
    } else if error_code.contains(ErrorCode::BAD_ADC_ACQUISITION) {
        Error::BadAdcAcquisition
    } else if error_code.contains(ErrorCode::FAIL_READING_EEPROM) {
        Error::FailReadingEeprom
    } else if error_code.contains(ErrorCode::INTERNAL_SPI_ERROR) {
        Error::InternalSpiError
    } else {
        Error::None
    }
}

impl Into<Data> for LastError {
    fn into(self) -> Data {
        Data {
            motherboard: to_error(self.motherboard),
            daughterboard: self.daughterboard.map(to_error),
        }
    }
}
