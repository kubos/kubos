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

pub mod commands;
pub mod logs;

pub use self::commands::*;
pub use self::logs::*;
use bitflags::bitflags;
use byteorder::{LittleEndian, WriteBytesExt};
use nom::*;

pub const SYNC: [u8; 3] = [0xAA, 0x44, 0x12];
pub const HDR_LEN: u8 = 28;

/// Supported message types
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MessageID {
    /// Log command message
    Log = 1,
    /// Unlog command message
    Unlog = 36,
    /// Unlog all command message
    UnlogAll = 38,
    /// Version data log
    Version = 37,
    /// RX status event data log
    RxStatusEvent = 94,
    /// Best XYZ position/velocity data log
    BestXYZ = 241,
    /// Catch-all value for received messages with an unknown ID
    Unknown,
}

impl Default for MessageID {
    fn default() -> MessageID {
        MessageID::Unknown
    }
}

impl From<u16> for MessageID {
    fn from(t: u16) -> MessageID {
        match t {
            1 => MessageID::Log,
            36 => MessageID::Unlog,
            37 => MessageID::Version,
            38 => MessageID::UnlogAll,
            94 => MessageID::RxStatusEvent,
            241 => MessageID::BestXYZ,
            _ => MessageID::Unknown,
        }
    }
}

/// Common header structure for all messages
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Header {
    pub sync: [u8; 3],
    pub hdr_len: u8,
    pub msg_id: MessageID,
    pub msg_type: u8,
    pub port_addr: u8,
    pub msg_len: u16,
    pub seq: u16,
    pub idle_time: u8,
    pub time_status: u8,
    pub week: u16,
    pub ms: i32,
    pub recv_status: ReceiverStatusFlags,
    pub recv_ver: u16,
}

impl Header {
    fn new(msg_id: MessageID, msg_len: u16) -> Self {
        Header {
            sync: SYNC,
            hdr_len: HDR_LEN,
            msg_id,
            msg_type: 0, // Measurement source = Primary antenna, Format = Binary, Response bit = Original message.
            port_addr: Port::ThisPort as u8,
            msg_len,
            seq: 0,         // Always zero. We're only sending the one message
            idle_time: 0,   // Ignored for TX
            time_status: 0, // Ignored for TX
            week: 0,        // Ignored for TX
            ms: 0,          // Ignored for TX
            recv_status: ReceiverStatusFlags::empty(), // Ignored for TX
            recv_ver: 0,    // Ignored for TX
        }
    }

    pub fn parse(raw: &[u8]) -> Option<Self> {
        match parse_header(raw) {
            Ok(conv) => Some(conv.1),
            _ => None,
        }
    }
}

impl Message for Header {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = SYNC.to_vec();

        vec.push(self.hdr_len);
        vec.write_u16::<LittleEndian>(self.msg_id as u16).unwrap();
        vec.push(self.msg_type);
        vec.push(self.port_addr);
        vec.write_u16::<LittleEndian>(self.msg_len).unwrap();
        vec.write_u16::<LittleEndian>(self.seq).unwrap();
        vec.push(self.idle_time);
        vec.push(self.time_status);
        vec.write_u16::<LittleEndian>(self.week).unwrap();
        vec.write_i32::<LittleEndian>(self.ms).unwrap();
        vec.write_u32::<LittleEndian>(self.recv_status.bits())
            .unwrap();
        vec.push(0);
        vec.push(0);
        vec.write_u16::<LittleEndian>(self.recv_ver).unwrap();

        vec
    }
}

/// Device communication ports
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Port {
    COM1 = 32,
    ThisPort = 192,
}

named!(parse_header(&[u8]) -> Header,
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
        (Header {
                sync: [sync1, sync2, sync3],
                hdr_len,
                msg_id: msg_id.into(),
                msg_type,
                port_addr,
                msg_len,
                seq,
                idle_time,
                time_status,
                week,
                ms,
                recv_status: ReceiverStatusFlags::from_bits_truncate(recv_status),
                recv_ver
        })
    )
);

bitflags! {
    /// Receiver status flags
    #[derive(Default)]
    pub struct ReceiverStatusFlags: u32 {
        /// System has encountered an error
        const ERROR_PRESENT = 0x0000_0001;
        /// Temperature warning
        const TEMPERATURE_WARNING = 0x0000_0002;
        /// Voltage supply warning
        const VOLTAGE_SUPPLY_WARNING = 0x0000_0004;
        /// Antenna not powered
        const ANTENNA_NOT_POWERED = 0x0000_0008;
        /// LNA failure encountered
        const LNA_FAILURE = 0x0000_0010;
        /// Antenna open
        const ANTENNA_OPEN = 0x0000_0020;
        /// Antenna shortened
        const ANTENNA_SHORTENED = 0x0000_0040;
        /// CPU overloaded
        const CPU_OVERLOAD = 0x0000_0080;
        /// The COM1 buffer has been overrun
        const COM1_BUFFER_OVERRUN = 0x0000_0100;
        /// The COM2 buffer has been overrun
        const COM2_BUFFER_OVERRUN = 0x0000_0200;
        /// The COM3 buffer has been overrun
        const COM3_BUFFER_OVERRUN = 0x0000_0400;
        /// Link overrun
        const LINK_OVERRUN = 0x0000_0800;
        /// Auxilliary transmit overrun
        const AUX_TRANSMIT_OVERRUN = 0x0000_2000;
        /// AGC out of range
        const AGC_OUT_OF_RANGE = 0x0000_4000;
        /// INS has been reset
        const INS_RESET = 0x0001_0000;
        /// GPS almanac invalid and/or UTC unknown
        const GPS_ALMANAC_INVALID = 0x0004_0000;
        /// Position solution is invalid
        const POSITION_SOLUTION_INVALID = 0x0008_0000;
        /// A fixed position is in place
        const POSITION_FIXED = 0x0010_0000;
        /// Clock steering is disabled
        const CLOCK_STEERING_DISABLED = 0x0020_0000;
        /// The clock model is invalid
        const CLOCK_MODEL_INVALID = 0x0040_0000;
        /// The external oscillator is locked
        const EXTERNAL_OSCILLATOR_LOCKED = 0x0080_0000;
        /// Software resource warning
        const SOFTWARE_RESOURCE_WARNING = 0x0100_0000;
        /// An auxilliary 3 status event has occurred
        const AUX3_STATUS_EVENT = 0x2000_0000;
        /// An auxilliary 2 status event has occurred
        const AUX2_STATUS_EVENT = 0x4000_0000;
        /// An auxilliary 1 status event has occurred
        const AUX1_STATUS_EVENT = 0x8000_0000;
    }
}

impl ReceiverStatusFlags {
    /// Convert the flags byte into a vector containing the string representations
    /// of all flags present.
    ///
    /// # Examples
    ///
    /// ```
    /// use novatel_oem6_api::*;
    ///
    /// # fn func() -> OEMResult<()> {
    /// let flags = ReceiverStatusFlags::CLOCK_MODEL_INVALID |
    ///     ReceiverStatusFlags::POSITION_SOLUTION_INVALID;
    ///
    /// let conv = flags.to_vec();
    ///
    /// assert_eq!(conv, vec!["CLOCK_MODEL_INVALID", "POSITION_SOLUTION_INVALID"]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub fn to_vec(self) -> Vec<String> {
        format!("{:?}", self)
            .split(" | ")
            .map(|x| x.to_string())
            .collect()
    }
}
