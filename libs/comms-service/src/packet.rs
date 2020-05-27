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

//! Link layer definitions used by the communications service

use crate::CommsResult;

/// Enum representing the different payload types handled
/// by the communications service
#[repr(u8)]
pub enum PayloadType {
    /// Packet intended for GraphQL request/response
    #[cfg(features = "graphql")]
    GraphQL,
    /// Packet intended for UDP passthrough
    UDP,
    /// Unknown type
    Unknown(u16),
}

impl From<u16> for PayloadType {
    fn from(num: u16) -> PayloadType {
        match num {
            #[cfg(features = "graphql")]
            0 => PayloadType::GraphQL,
            1 => PayloadType::UDP,
            other => PayloadType::Unknown(other),
        }
    }
}

impl From<PayloadType> for u16 {
    fn from(value: PayloadType) -> u16 {
        match value {
            #[cfg(features = "graphql")]
            PayloadType::GraphQL => 0,
            PayloadType::UDP => 1,
            PayloadType::Unknown(value) => value as u16,
        }
    }
}

/// Generic LinkPacket trait which defines the internal packet requirements
/// of the communications service.
pub trait LinkPacket {
    /// Parse packet from raw bytes
    fn parse(raw: &[u8]) -> CommsResult<Box<Self>>;
    /// Build packet from necessary parts
    fn build(
        command_id: u64,
        link_type: PayloadType,
        destination_port: u16,
        payload: &[u8],
    ) -> CommsResult<Box<Self>>;
    /// Create a bytes representation of the packet
    fn to_bytes(&self) -> CommsResult<Vec<u8>>;
    /// The Command ID of the packet
    fn command_id(&self) -> u64;
    /// The payload or data of the packet
    fn payload(&self) -> Vec<u8>;
    /// The type of payload contained in the packet
    fn payload_type(&self) -> PayloadType;
    /// The Destination port of the packet
    fn destination(&self) -> u16;
    /// Validate the contents of the link packet
    fn validate(&self) -> bool {
        true
    }
    /// The maximum allowed size of the packet
    /// We are still assuming that at some point these packets
    /// will be sent over IP/UDP
    fn max_size() -> usize {
        // (65,535 - 20 byte IP header - 8 byte UDP header)
        65507
    }
}
