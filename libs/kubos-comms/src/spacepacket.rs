//
// Copyright (C) 2019 Kubos Corporation
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

//! Packet Definition for SpacePacket

use crate::packet::{LinkPacket, PayloadType};
use crate::CommsResult;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

#[derive(Eq, Debug, PartialEq)]
struct PrimaryHeader {
    /// Packet Version Number - 3 bits
    version: u8,
    /// Packet Type - 1 bit
    packet_type: u8,
    /// Secondary Header Flag - 1 bit
    sec_header_flag: u8,
    /// Application Process ID - 11 bits
    app_proc_id: u16,
    /// Sequence Flags - 2 bits
    sequence_flags: u8,
    /// Packet Sequence Count or Packet Name - 14 bits
    sequence_count: u16,
    /// Packet Data Length - 2 bytes
    data_length: u16,
}

#[derive(Eq, Debug, PartialEq)]
struct SecondaryHeader {
    /// Command ID from MT - 64 bits
    command_id: u64,
    /// Destination service port - 16 bits
    destination_port: u16,
}

/// Structure used to implement SpacePacket version of LinkPacket
#[derive(Eq, Debug, PartialEq)]
pub struct SpacePacket {
    primary_header: PrimaryHeader,
    secondary_header: SecondaryHeader,
    payload: Vec<u8>,
}

impl LinkPacket for SpacePacket {
    fn build(
        command_id: u64,
        payload_type: PayloadType,
        destination_port: u16,
        payload: &[u8],
    ) -> CommsResult<Box<Self>> {
        Ok(Box::new(SpacePacket {
            primary_header: PrimaryHeader {
                version: 0,
                packet_type: 0,
                sec_header_flag: 0,
                app_proc_id: u16::from(payload_type),
                sequence_flags: 0,
                sequence_count: 0,
                data_length: (payload.len() + 10) as u16,
            },
            secondary_header: SecondaryHeader {
                command_id,
                destination_port,
            },
            payload: payload.to_vec(),
        }))
    }

    fn parse(raw: &[u8]) -> CommsResult<Box<Self>> {
        let mut reader = Cursor::new(raw.to_vec());

        let header_0 = reader.read_u16::<BigEndian>()?;
        let version = ((header_0 & 0xE000) >> 13) as u8;
        let packet_type = ((header_0 & 0x1000) >> 12) as u8;
        let sec_header_flag = ((header_0 & 0x800) >> 11) as u8;
        let app_proc_id = (header_0 & 0x7FF) as u16;

        let header_1 = reader.read_u16::<BigEndian>()?;
        let sequence_flags = ((header_1 & 0xC000) >> 14) as u8;
        let sequence_count = (header_1 & 0x3FFF) as u16;

        let data_length = reader.read_u16::<BigEndian>()?;
        let command_id = reader.read_u64::<BigEndian>()?;
        let destination_port = reader.read_u16::<BigEndian>()?;
        let pos = reader.position() as usize;
        let payload = raw[pos..].to_vec();
        Ok(Box::new(SpacePacket {
            primary_header: PrimaryHeader {
                version,
                packet_type,
                sec_header_flag,
                app_proc_id,
                sequence_flags,
                sequence_count,
                data_length,
            },
            secondary_header: SecondaryHeader {
                command_id,
                destination_port,
            },
            payload,
        }))
    }

    fn to_bytes(&self) -> CommsResult<Vec<u8>> {
        let mut bytes = vec![];

        let header_0: u16 = (self.primary_header.app_proc_id) as u16
            | u16::from(self.primary_header.sec_header_flag) << 11
            | u16::from(self.primary_header.packet_type) << 12
            | u16::from(self.primary_header.version) << 13;

        let header_1 = (self.primary_header.sequence_count as u16)
            | u16::from(self.primary_header.sequence_flags) << 14;

        let header_2 = self.primary_header.data_length;

        bytes.write_u16::<BigEndian>(header_0)?;
        bytes.write_u16::<BigEndian>(header_1)?;
        bytes.write_u16::<BigEndian>(header_2)?;
        bytes.write_u64::<BigEndian>(self.secondary_header.command_id)?;
        bytes.write_u16::<BigEndian>(self.secondary_header.destination_port)?;
        bytes.append(&mut self.payload.clone());

        Ok(bytes)
    }

    fn command_id(&self) -> u64 {
        self.secondary_header.command_id
    }

    fn payload(&self) -> Vec<u8> {
        self.payload.clone()
    }

    fn payload_type(&self) -> PayloadType {
        PayloadType::from(self.primary_header.app_proc_id)
    }

    fn destination(&self) -> u16 {
        self.secondary_header.destination_port
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn do_build_parse() {
        let packet =
            SpacePacket::build(1294, PayloadType::GraphQL, 15001, &[5, 4, 3, 2, 1]).unwrap();
        println!("packet {:?}", packet);

        let raw = packet.to_bytes();
        println!("bytes {:?}", raw);

        let parsed = SpacePacket::parse(&raw.unwrap());
        println!("parsed {:?}", parsed);

        assert_eq!(packet, parsed.unwrap());
    }

    #[test]
    fn parse_python_spacepacket() {
        let raw = b"\x00\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00o\x05\xdcquery";
        let parsed = SpacePacket::parse(raw).unwrap();
        dbg!(parsed);
    }
}
