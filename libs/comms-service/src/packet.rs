//! General packet definition used by Comms Service

use crate::CommsResult;

pub enum LinkType {
    /// Packet intended for UDP passthrough
    UDP,
    /// Packet intended for GraphQL request/response
    GraphQL,
}

pub trait LinkPacket {
    /// Parse packet from raw bytes
    fn parse(raw: &[u8]) -> CommsResult<Box<Self>>;
    /// Build packet from necessary parts
    fn build(
        command_id: u64,
        link_type: LinkType,
        destination_port: u16,
        payload: &[u8],
    ) -> CommsResult<Box<Self>>;
    /// Create a bytes representation of the packet
    fn to_bytes(&self) -> CommsResult<Vec<u8>>;
    /// The Command ID of the packet
    fn command_id(&self) -> u64;
    /// The payload or data of the packet
    fn payload(&self) -> Vec<u8>;
    /// The Link Type of the packet
    fn link_type(&self) -> LinkType;
    /// The Destination port of the packet
    fn destination(&self) -> u16;
    /// Validate the contents of the link packet
    fn validate(&self) -> bool {
        true
    }
}
