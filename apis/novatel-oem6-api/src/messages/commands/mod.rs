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
use byteorder::LittleEndian;

mod log;
mod unlog;
mod unlog_all;

pub use self::log::*;
pub use self::unlog::*;
pub use self::unlog_all::*;

pub trait Message {
    fn serialize(&self) -> Vec<u8>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Response {
    pub resp_id: ResponseID,
    pub resp_string: String,
}

#[allow(dead_code)]
impl Response {
    pub fn new(msg: &[u8]) -> Option<Self> {
        match le_u32(&msg) {
            Ok(conv) => {
                let mut resp: Response = Response {
                    // Convert the bytes we just read into the response ID
                    resp_id: conv.1.into(),
                    resp_string: "".to_owned(),
                };

                // Add the actual response message
                resp.resp_string.push_str(&String::from_utf8_lossy(&conv.0));

                Some(resp)
            }
            _ => None,
        }
    }
}

/// Response values returned after sending a command to the device.
///
/// Error values will be returned as part of the [`OEMError::CommandError`] variant.
///
/// [`OEMError::CommandError`]: enum.OEMError.html#variant.CommandError
#[derive(Clone, Debug, PartialEq)]
pub enum ResponseID {
    /// Command was received correctly
    Ok = 1,
    /// Requested log type does not exist
    LogInvalid = 2,
    /// The request has exceeded some (unspecified) limit
    OutOfResources = 3,
    /// Data packet is not verified
    PacketNotVerified = 4,
    /// Command was attempted, but failed
    CommandFailed = 5,
    /// Input message ID is not valid
    InvalidID = 6,
    /// An input field was invalid. See the [`OEMError::CommandError`](variant.CommandError.html) variant's `description` field for
    /// information about the specific field that caused the error.
    InvalidField = 7,
    /// The checksum of the sent message was invalid
    InvalidChecksum = 8,
    /// A field is missing from the sent message
    MissingField = 9,
    /// An input field contains more array elements than allowed. See the [`OEMError::CommandError`](variant.CommandError.html)
    /// variant's `description` field for information about the specific field that caused the error.
    ArrayOverflow = 10,
    /// An input field's value is outside acceptable limits. See the [`OEMError::CommandError`](variant.CommandError.html) variant's
    /// `description` field for information about the specific field that caused the error.
    ErrorField = 11,
    /// The requested trigger type is invalid for the requested log type
    InvalidTrigger = 14,
    /// Too many authcodes are stored in the receiver. The receiver firmware must be reloaded
    AuthcodeOverflow = 15,
    /// This error is related to the inputting of authcodes. Indicates the date attached to the code is not valid
    InvalidDate = 16,
    /// The authcode entered is not valid
    InvalidAuthcode = 17,
    /// The model requested for removal does not exist
    NoModel = 18,
    /// The model attached to the authcode is not valid
    InvalidModel = 19,
    /// The selected channel is invalid
    InvalidChannel = 20,
    /// The requested rate is invalid
    InvalidRate = 21,
    /// The word has no mask for this type of log
    NoMask = 22,
    /// Channels are locked due to an error
    LockedChannels = 23,
    /// The injected time is invalid
    InvalidTime = 24,
    /// The COM/USB port is not supported
    InvalidPort = 25,
    /// The sent message is invalid
    InvalidMessage = 26,
    /// The PRN is invalid
    InvalidPRN = 27,
    /// The PRN is not locked out
    PRNNotLocked = 28,
    /// The PRN lockout list is full
    PRNLockoutOverflow = 29,
    /// The PRN is already locked out
    PRNAlreadyLocked = 30,
    /// Message timed out
    Timeout = 31,
    /// Unknown COM/USB port requested
    UnknownPort = 33,
    /// Hex string not formatted correctly
    BadHex = 34,
    /// The baud rate is invalid
    InvalidBaud = 35,
    /// The sent message is invalid for this model of receiver
    ModelInvalidMessage = 36,
    /// Command is only valid if NVM is in fail mode
    RequiresFailMode = 40,
    /// The offset is invalid
    InvalidOffset = 41,
    /// The maximum number of user messages has been reached
    MessageOverflow = 78,
    /// GPS precise time is already known
    PreciseTimeAlreadyKnown = 84,
    /// Catch-all value for unknown response IDs
    Unknown,
}

impl From<u32> for ResponseID {
    fn from(t: u32) -> ResponseID {
        match t {
            1 => ResponseID::Ok,
            2 => ResponseID::LogInvalid,
            3 => ResponseID::OutOfResources,
            4 => ResponseID::PacketNotVerified,
            5 => ResponseID::CommandFailed,
            6 => ResponseID::InvalidID,
            7 => ResponseID::InvalidField,
            8 => ResponseID::InvalidChecksum,
            9 => ResponseID::MissingField,
            10 => ResponseID::ArrayOverflow,
            11 => ResponseID::ErrorField,
            14 => ResponseID::InvalidTrigger,
            15 => ResponseID::AuthcodeOverflow,
            16 => ResponseID::InvalidDate,
            17 => ResponseID::InvalidAuthcode,
            18 => ResponseID::NoModel,
            19 => ResponseID::InvalidModel,
            20 => ResponseID::InvalidChannel,
            21 => ResponseID::InvalidRate,
            22 => ResponseID::NoMask,
            23 => ResponseID::LockedChannels,
            24 => ResponseID::InvalidTime,
            25 => ResponseID::InvalidPort,
            26 => ResponseID::InvalidMessage,
            27 => ResponseID::InvalidPRN,
            28 => ResponseID::PRNNotLocked,
            29 => ResponseID::PRNLockoutOverflow,
            30 => ResponseID::PRNAlreadyLocked,
            31 => ResponseID::Timeout,
            33 => ResponseID::UnknownPort,
            34 => ResponseID::BadHex,
            35 => ResponseID::InvalidBaud,
            36 => ResponseID::ModelInvalidMessage,
            40 => ResponseID::RequiresFailMode,
            41 => ResponseID::InvalidOffset,
            78 => ResponseID::MessageOverflow,
            84 => ResponseID::PreciseTimeAlreadyKnown,
            _ => ResponseID::Unknown,
        }
    }
}
