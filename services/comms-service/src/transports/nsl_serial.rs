use codecs::*;
use nsl_duplex_d2::{serial_connection, DuplexD2, File};

pub struct Transport {
    radio: DuplexD2,
}

impl Transport {
    pub fn new() -> Self {
        Self {
            radio: DuplexD2::new(serial_connection()),
        }
    }

    pub fn read(&self) -> Result<Option<udp::UdpData>, String> {
        match self.radio.get_uploaded_file_count() {
            Ok(c) => {
                if c > 0 {
                    print!("Fetching Uploaded File: ");
                    if let Ok(file) = self.radio.get_uploaded_file() {
                        //print!("{:?}", file);
                        println!("Kiss decoding file");
                        match kiss::decode(&file.body, 0) {
                            Ok((frame, index)) => {
                                //println!("Got kiss file index {}\n{:#?}", index, frame);
                                println!("Udp decoding file");
                                match udp::framed_decode(&frame, 0) {
                                    Ok(u) => return Ok(Some(u)),
                                    Err(e) => println!("Udp decode err {:?}", e),
                                }
                            }
                            Err(e) => println!("Kiss decode err {:?}", e),
                        }
                    }
                }
            }
            Err(e) => print!("{:?}\n", e),
        };

        match self.radio.get_download_file_count() {
            Ok(c) => {
                print!("D:{}", c);
                if c == 0 {
                    println!("No files in queue. Sending ping");
                    let f = File {
                        name: String::from("PING"),
                        body: vec![],
                    };
                    println!("{:?}", self.radio.put_download_file(&f));
                }
            }
            Err(e) => print!("{:?}\n", e),
        };

        Ok(None)
    }

    pub fn write(&self, data: udp::UdpData) -> Result<(), String> {
        // We have raw data so we'll encode it here...
        // encode/decode layers to be moved elsewhere eventually
        let udp_encoded = udp::encode(&data)?;
        let kiss_encoded = kiss::encode(&udp_encoded)?;

        let f = File {
            name: String::from("UPLOAD"),
            body: kiss_encoded.to_vec(),
        };
        match self.radio.put_download_file(&f) {
            Ok(o) => Ok(()),
            Err(e) => Err(format!("Download error {:?}", e)),
        }
    }
}
