pub mod echo;
pub mod nsl_serial;
pub mod udp;

use codecs::udp as udp_codec;

pub trait Transport {
    fn read(&self) -> Result<Option<udp_codec::UdpData>, String>;

    fn write(&mut self, data: udp_codec::UdpData) -> Result<(), String>;
}
