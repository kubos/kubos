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

impl Response {
    pub fn new(msg: Vec<u8>) -> Option<Self> {
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

#[derive(Clone, Debug, PartialEq)]
pub enum ResponseID {
    Ok = 1,
    LogInvalid = 2,
    OutOfResources = 3,
    PacketNotVerified = 4,
    CommandFailed = 5,
    InvalidID = 6,
    InvalidField = 7,
    InvalidChecksum = 8,
    MissingField = 9,
    ArrayOverflow = 10,
    ErrorField = 11,
    InvalidTrigger = 14,
    AuthcodeOverflow = 15,
    InvalidDate = 16,
    InvalidAuthcode = 17,
    NoModel = 18,
    InvalidModel = 19,
    InvalidChannel = 20,
    InvalidRate = 21,
    NoMask = 22,
    LockedChannels = 23,
    InvalidTime = 24,
    InvalidPort = 25,
    InvalidMessage = 26,
    InvalidPRN = 27,
    PRNNotLocked = 28,
    PRNLockoutOverflow = 29,
    PRNAlreadyLocked = 30,
    Timeout = 31,
    UnknownPort = 33,
    BadHex = 34,
    InvalidBaud = 35,
    ModelInvalidMessage = 36,
    RequiresFailMode = 40,
    InvalidOffset = 41,
    MessageOverflow = 78,
    PreciseTimeAlreadyKnown = 84,
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
