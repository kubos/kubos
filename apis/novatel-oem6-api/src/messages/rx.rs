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

use byteorder::{LittleEndian, ReadBytesExt};
use nom::*;
use std::io::Cursor;
use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Response {
    pub hdr: Header,
    pub resp_id: ResponseID,
    pub resp_string: String,
}

impl Response {
    /// Constructor. Converts a raw data array received from the OEM6 into a usable structure
    pub fn new(msg: Vec<u8>) -> Option<Self> {
        // Convert the raw data to an official struct
        match parse_response(&msg) {
            Ok(conv) => {
                let mut resp = conv.1;

                for elem in conv.0 {
                    resp.resp_string.push(*elem as char);
                }

                Some(resp)
            }
            _ => None,
        }
    }
}

named!(parse_response(&[u8]) -> Response,
    do_parse!(
        sync1: le_u8 >>
        sync2: le_u8 >>
        sync3: le_u8 >>
        hdr_len: le_u8 >>
        msg_id: le_u16 >>
        msg_type: le_u8 >>
        port_addr: le_u8 >>
        msg_len: le_u16 >>
        seq: le_u16 >>
        idle_time: le_u8 >>
        time_status: le_u8 >>
        week: le_u16 >>
        ms: le_i32 >>
        recv_status: le_u32 >>
        le_u16 >>
        recv_ver: le_u16 >>
        resp_id: le_u32 >>
        (Response {
                hdr: Header {
                    sync: [sync1, sync2, sync3],
                    hdr_len,
                    msg_id,
                    msg_type,
                    port_addr,
                    msg_len,
                    seq,
                    idle_time,
                    time_status,
                    week,
                    ms,
                    recv_status,
                    recv_ver
                },
                resp_id: resp_id.into(),
                resp_string: "".to_owned()
        })
    )
);

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
