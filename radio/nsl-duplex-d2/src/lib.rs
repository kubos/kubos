/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Device level API for interacting with the NSL EyeStar-D2 Duplex radio
//! `<https://nearspacelaunch.com/product/eyestar-d2/>`

// #![deny(missing_docs)]

extern crate radio_api;
extern crate serde_json;
extern crate serial;

pub mod serial_comm;
pub mod comms;
pub mod file;

use serde_json::Error as SerdeJsonError;
use radio_api::{Connection, Radio, RadioError, RadioReset};
use comms::*;
use file::*;
use serial_comm::*;

/// Structure implementing Radio functionality for Duplex-D2
pub struct DuplexD2 {
    conn: Box<Connection>,
}

impl DuplexD2 {
    pub fn init() -> DuplexD2 {
        DuplexD2 {
            conn: Box::new(SerialConnection),
        }
    }

    pub fn get_uploaded_file(&self) -> Result<File, String> {
        match self.send_command(GET_UPLOADED_FILE) {
            Ok(response) => { Ok(File::from_response(response)) },
            Err(e) => Err(e),
        }
    }

    pub fn get_uploaded_file_count(&self) -> Result<u32, String> {
        match self.send_command(GET_UPLOADED_FILE_COUNT) {
            Ok(file_count) => { Ok(self.process_file_count(file_count)) },
            Err(e) => Err(e),
        }
    }

    fn process_file_count(&self, file_count: Vec<u8>) -> u32 {
        u32::from(file_count[2])
            | u32::from(file_count[3]) << 8
            | u32::from(file_count[4]) << 16
            | u32::from(file_count[5]) << 24
    }

    fn send_command(&self, command: &str) -> Result<Vec<u8>, String> {
        self.conn.send(command).unwrap();
        // wait with timeout here?
        let resp = match self.conn.receive() {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        // Check if resp header exists
        if (resp[0] == b'G') && (resp[1] == b'U') {
            return Ok(resp);
        } else {
            // retries?
            return Err(String::from("Invalid resp header"));
        }
    }

}

impl Radio for DuplexD2 {
    fn init(&self) -> Result<(), RadioError> {
        Ok(())
    }

    fn terminate(&self) -> Result<(), RadioError> {
        Ok(())
    }

    fn reset(&self, reset_type: RadioReset) -> Result<(), RadioError> {
        match reset_type {
            // A hardware reset is signaled via a GPIO tied
            // to the modem.

            // A software reset is hopefully trigged by
            // a command sent to the modem.
            RadioReset::HardReset | RadioReset::SoftReset => Ok(()),
        }
    }

    fn configure(&self, _json_config: &str) -> Result<(), SerdeJsonError> {
        Ok(())
    }

    fn send(&self, _buffer: Vec<u8>) -> Result<(), RadioError> {
        Ok(())
    }

    fn receive(&self) -> Result<(Vec<u8>), RadioError> {
        match self.get_uploaded_file() {
            Ok(r) => Ok(r.payload),
            Err(_) => Err(RadioError::RxEmpty),
        }
    }

    fn get_telemetry<TelemetryType>(&self, _telem_type: TelemetryType) -> Result<&str, RadioError> {
        Ok("telemetry")
    }
}

#[cfg(test)]
mod tests {
    use ::*;

    struct TestConnection {
        data: Vec<u8>,
    }

    impl Connection for TestConnection {
        /// Basic send command function. Sends and receives
        fn send(&self, _cmd: &str) -> Result<(), String> {
            Ok(())
        }

        /// Basic receive function
        fn receive(&self) -> Result<Vec<u8>, String> {
            Ok(self.data.clone())
        }
    }

    #[test]
    fn test_init() {
        let d = DuplexD2 {
            conn: Box::new(TestConnection { data: Vec::new() }),
        };
        assert!(d.init().is_ok(), "Init should pass")
    }

    #[test]
    fn test_terminate() {
        let d = DuplexD2 {
            conn: Box::new(TestConnection { data: Vec::new() }),
        };
        assert!(d.terminate().is_ok(), "Terminate should pass")
    }

    #[test]
    fn test_configure() {
        let d = DuplexD2 {
            conn: Box::new(TestConnection { data: Vec::new() }),
        };
        let config = r#"{
                     "retries": 2
                    }"#;
        assert!(d.configure(config).is_ok(), "Config should pass")
    }

    #[test]
    fn test_reset() {
        let d = DuplexD2 {
            conn: Box::new(TestConnection { data: Vec::new() }),
        };
        assert!(d.reset(RadioReset::HardReset).is_ok(), "Reset should pass")
    }

    #[test]
    fn test_send() {
        let d = DuplexD2 {
            conn: Box::new(TestConnection { data: Vec::new() }),
        };
        let data: Vec<u8> = Vec::new();
        assert!(d.send(data).is_ok(), "Send should pass")
    }

    #[test]
    fn test_uploaded_file_count_one() {
        let mut ret_msg = Vec::<u8>::new();
        ret_msg.extend(RESP_HEADER.as_bytes().iter().cloned());
        ret_msg.push(1 as u8);
        ret_msg.push(0 as u8);
        ret_msg.push(0 as u8);
        ret_msg.push(0 as u8);
        let d = DuplexD2 {
            conn: Box::new(TestConnection { data: ret_msg }),
        };
        let count = d.get_uploaded_file_count().unwrap();
        assert_eq!(count, 1, "File count should be one")
    }

    #[test]
    fn test_uploaded_file_count_zero() {
        let mut ret_msg = Vec::<u8>::new();
        ret_msg.extend(RESP_HEADER.as_bytes().iter().cloned());
        ret_msg.push(0 as u8);
        ret_msg.push(0 as u8);
        ret_msg.push(0 as u8);
        ret_msg.push(0 as u8);
        let d = DuplexD2 {
            conn: Box::new(TestConnection { data: ret_msg }),
        };
        let count = d.get_uploaded_file_count().unwrap();
        assert_eq!(count, 0, "File count should be zero")
    }

    #[test]
    fn test_uploaded_file_count_many() {
        let mut ret_msg = Vec::<u8>::new();
        ret_msg.extend(RESP_HEADER.as_bytes().iter().cloned());
        ret_msg.push(0 as u8);
        ret_msg.push(0 as u8);
        ret_msg.push(0 as u8);
        ret_msg.push(1 as u8);
        let d = DuplexD2 {
            conn: Box::new(TestConnection { data: ret_msg }),
        };
        let count = d.get_uploaded_file_count().unwrap();
        assert_eq!(count, 16777216, "File count should be 16777216")
    }

    #[test]
    fn test_get_uploaded_file() {
        let mut ret_msg = Vec::<u8>::new();
        ret_msg.extend(RESP_HEADER.as_bytes().iter().cloned());
        let name_size = String::from("008");
        let size = String::from("000004");
        let name = String::from("test.txt");
        let data = String::from("test");
        let crc = String::from("44");

        ret_msg.extend(name_size.as_bytes().iter().cloned());
        ret_msg.extend(size.as_bytes().iter().cloned());
        ret_msg.extend(name.as_bytes().iter().cloned());
        ret_msg.extend(data.as_bytes().iter().cloned());
        ret_msg.extend(crc.as_bytes().iter().cloned());
        let d = DuplexD2 {
            conn: Box::new(TestConnection { data: ret_msg }),
        };
        let file = d.get_uploaded_file().unwrap();
        // check file name
        assert_eq!(file.name, String::from("test.txt"));
        // check payload
        assert_eq!(
            String::from_utf8(file.payload).unwrap(),
            String::from("test")
        );
    }
}
