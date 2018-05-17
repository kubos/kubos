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

    fn device_read(&self) -> Result<Option<Vec<u8>>, String> {
        let upload_count = match self.radio.get_uploaded_file_count() {
            Ok(c) => c,
            Err(e) => return Err(format!("Failed to get uploaded file count {:?}", e)),
        };

        if upload_count > 0 {
            let uploaded_file = match self.radio.get_uploaded_file() {
                Ok(f) => f,
                Err(e) => return Err(format!("Failed to get uploaded file {:?}", e)),
            };

            return Ok(Some(uploaded_file.body));
        }

        let download_count = match self.radio.get_download_file_count() {
            Ok(c) => c,
            Err(e) => return Err(format!("Failed to get uploaded file count {:?}", e)),
        };

        if download_count == 0 {
            print!("ping");
            match self.radio.put_download_file(&File {
                name: String::from("PING"),
                body: vec![],
            }) {
                Ok(_) => (),
                Err(e) => return Err(format!("Failed to send ping {:?}", e)),
            }
        }

        Ok(None)
    }

    fn device_write(&self, data: &[u8]) -> Result<(), String> {
        let f = File {
            name: String::from("UPLOAD"),
            body: data.to_vec(),
        };
        match self.radio.put_download_file(&f) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to put download file {:?}", e)),
        }
    }

    pub fn read(&self) -> Result<Option<udp::UdpData>, String> {
        //        println!("nsl_read");
        match self.device_read()? {
            Some(data) => {
                println!("decode nsl read");
                let (decoded, _) = kiss::decode(&data, 0)?;
                let decoded = udp::framed_decode(&decoded, 0)?;
                Ok(Some(decoded))
            }
            None => Ok(None),
        }
    }

    pub fn write(&self, data: udp::UdpData) -> Result<(), String> {
        //      println!("nsl_write");
        // We have raw data so we'll encode it here...
        // encode/decode layers to be moved elsewhere eventually
        self.device_write(&kiss::encode(&udp::encode(&data)?)?)
    }
}
