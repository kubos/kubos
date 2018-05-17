pub mod kiss;
pub mod udp;

pub trait Codec {
    fn encode(&self, frame: &[u8]) -> Result<Vec<u8>, String>;
    fn decode(&self, chunk: &[u8], index: u32) -> Result<(Vec<u8>, u32), String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let orig = vec![0, 130, 26, 0, 1, 218, 134, 245];
        let orig_udp = udp::UdpData {
            source: 7000,
            dest: 7001,
            data: orig,
            checksum: true,
        };

        let udp_encoded = udp::encode(&orig_udp).unwrap();
        let kiss_encoded = kiss::encode(&udp_encoded).unwrap();

        let (kiss_decoded_data, kiss_decoded_len) = kiss::decode(&kiss_encoded, 0).unwrap();
        let udp_decoded = udp::framed_decode(&kiss_decoded_data, 0).unwrap();

        assert_eq!(kiss_decoded_data, udp_encoded);
        assert_eq!(orig_udp, udp_decoded);
    }

    #[test]
    fn test_encode_decode_more() {
        let data = vec![
            0, 131, 120, 32, 50, 102, 57, 100, 53, 52, 53, 98, 50, 102, 57, 51, 99, 99, 100, 52,
            55, 56, 54, 57, 102, 55, 97, 54, 48, 54, 100, 54, 56, 54, 56, 101, 245, 1,
        ];

        let orig_udp = udp::UdpData {
            source: 7000,
            dest: 7001,
            data: data.to_vec(),
            checksum: true,
        };

        let udp_encoded = udp::encode(&orig_udp).unwrap();
        let kiss_encoded = kiss::encode(&udp_encoded).unwrap();

        let (kiss_decoded_data, kiss_decoded_len) = kiss::decode(&kiss_encoded, 0).unwrap();
        let udp_decoded = udp::framed_decode(&kiss_decoded_data, 0).unwrap();

        assert_eq!(kiss_decoded_data, udp_encoded);
        assert_eq!(orig_udp, udp_decoded);
        assert_eq!(data, udp_decoded.data);
    }
}
