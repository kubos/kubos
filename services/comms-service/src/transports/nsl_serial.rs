use codecs;
use nsl_duplex_d2::{serial_connection, DuplexD2, File};
use transports::Transport;

pub struct NslSerial {
    count: u32,
    radio: DuplexD2,
}

impl NslSerial {
    pub fn new() -> Self {
        info!("nsl transport starting");
        Self {
            count: 0,
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

            info!("NSL received file {}", uploaded_file.name);

            return Ok(Some(uploaded_file.body));
        }

        let download_count = match self.radio.get_download_file_count() {
            Ok(c) => c,
            Err(e) => return Err(format!("Failed to get uploaded file count {:?}", e)),
        };

        if download_count == 0 {
            info!("ping");
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

    fn device_write(&mut self, data: &[u8]) -> Result<(), String> {
        let f = File {
            name: format!("UPLOAD{}", self.count),
            body: data.to_vec(),
        };
        self.count += 1;
        info!("NSL sending file {}", f.name);
        match self.radio.put_download_file(&f) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to put download file {:?}", e)),
        }
    }
}

impl Transport for NslSerial {
    fn read(&self) -> Result<Option<codecs::udp::UdpData>, String> {
        match self.device_read()? {
            Some(data) => {
                let decoded = codecs::kiss::decode(&data)?;
                let decoded = codecs::udp::framed_decode(&decoded)?;
                Ok(Some(decoded))
            }
            None => Ok(None),
        }
    }

    fn write(&mut self, data: codecs::udp::UdpData) -> Result<(), String> {
        // We have raw data so we'll encode it here...
        // encode/decode layers to be moved elsewhere eventually
        self.device_write(&codecs::kiss::encode(&codecs::udp::encode(&data)?)?)
    }
}
