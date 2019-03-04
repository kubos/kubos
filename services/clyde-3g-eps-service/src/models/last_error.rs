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

//! Data returned by `lastEpsError` query

use clyde_3g_eps_api::{ErrorCode, LastError};

/// Error variants which can be returned by the EPS
#[derive(Clone, Debug, GraphQLEnum)]
pub enum Error {
    /// No errors were encountered
    None,
    /// CRC does not match data
    BadCRC,
    /// Unknown command received
    UnknownCommand,
    /// Supplied data incorrect when processing command
    CommandDataIncorrect,
    /// Selected channel does not exist
    ChannelDoesNotExist,
    /// Selected channel is currently inactive
    ChannelInactive,
    /// A reset had to occur
    ResetOccurred,
    /// There was an error with teh ADC acquisition
    BadADCAcquisition,
    /// Reading from EEPROM generated an error
    FailReadingEEPROM,
    /// Generic warning about an error on the internal SPI bus
    InternalSPIError,
    /// Catch all for future error values
    UnknownError,
}

/// Last command status for the EPS
#[derive(Clone, Debug, GraphQLObject)]
pub struct Data {
    /// Last command status for the motherboard
    pub motherboard: Error,
    /// Last command status for the daughterboard
    pub daughterboard: Option<Error>,
}

fn to_error(error_code: ErrorCode) -> Error {
    match error_code {
        ErrorCode::None => Error::None,
        ErrorCode::UnknownCommand => Error::UnknownCommand,
        ErrorCode::CommandDataIncorrect => Error::CommandDataIncorrect,
        ErrorCode::ChannelDoesNotExist => Error::ChannelDoesNotExist,
        ErrorCode::ChannelInactive => Error::ChannelInactive,
        ErrorCode::BadCRC => Error::BadCRC,
        ErrorCode::ResetOccurred => Error::ResetOccurred,
        ErrorCode::BadADCAcquisition => Error::BadADCAcquisition,
        ErrorCode::FailReadingEEPROM => Error::FailReadingEEPROM,
        ErrorCode::InternalSPIError => Error::InternalSPIError,
        ErrorCode::UnknownError => Error::UnknownError,
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
