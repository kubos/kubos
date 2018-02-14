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

use serde_json::Error as SerdeJsonError;
use radio_api::{Connection, Radio, RadioError, RadioReset};
use comms::*;
use file::*;
use state_of_health_record::*;
use d2_message::*;
use nums_as_bytes::AsBytes;

/// Structure implementing Radio functionality for Duplex-D2
pub struct DuplexD2 {
    conn: Box<Connection>,
}

impl DuplexD2 {
    pub fn new(conn: Box<Connection>) -> DuplexD2 {
        DuplexD2 { conn: conn }
    }

    pub fn get_uploaded_file(&self) -> Result<File, String> {
        File::from_response(&self.send_command(GET_UPLOADED_FILE)?)
    }

    pub fn get_uploaded_file_count(&self) -> Result<u32, String> {
        Ok(process_file_count(&self.send_command(
            GET_UPLOADED_FILE_COUNT,
        )?))
    }

    pub fn get_state_of_health_record(&self) -> Result<StateOfHealthRecord, String> {
        Ok(StateOfHealthRecord::new(
            self.send_command(GET_MODEM_STATE_OF_HEALTH)?,
        ))
    }

    fn send_command(&self, command: u64) -> Result<Vec<u8>, String> {
        self.conn.send(command.as_bytes())?;
        Ok(self.conn.receive()?)
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
            Ok(r) => Ok(r.data),
            Err(_) => Err(RadioError::RxEmpty),
        }
    }

    fn get_telemetry<TelemetryType>(&self, _telem_type: TelemetryType) -> Result<&str, RadioError> {
        Ok("telemetry")
    }
}

#[cfg(test)]
mod tests {
    use duplex_d2::*;
    use state_of_health_record::tests::*;

    struct TestGoodConnection {
        data: Vec<u8>,
    }

    impl Connection for TestGoodConnection {
        fn send(&self, _data: Vec<u8>) -> Result<(), String> {
            Ok(())
        }

        fn receive(&self) -> Result<Vec<u8>, String> {
            Ok(self.data.clone())
        }
    }

    struct TestBadConnection {}

    impl Connection for TestBadConnection {
        fn send(&self, _: Vec<u8>) -> Result<(), String> {
            return Err(String::from("Send failed"));
        }

        fn receive(&self) -> Result<Vec<u8>, String> {
            return Err(String::from("Receive failed"));
        }
    }

    #[test]
    fn test_state_of_health_message() {
        let radio = DuplexD2 {
            conn: Box::new(TestGoodConnection {
                data: soh_message(),
            }),
        };
        let soh_response = radio.get_state_of_health_record();
        match soh_response {
            Ok(response) => assert_eq!([0, 0, 1, 2], response.reset_count),
            Err(_) => assert!(false, "Expected the SOH call to succeed."),
        }
    }

    #[test]
    fn test_init() {
        let d = DuplexD2 {
            conn: Box::new(TestGoodConnection { data: Vec::new() }),
        };
        assert!(d.init().is_ok(), "Init should pass")
    }

    #[test]
    fn test_terminate() {
        let d = DuplexD2 {
            conn: Box::new(TestGoodConnection { data: Vec::new() }),
        };
        assert!(d.terminate().is_ok(), "Terminate should pass")
    }

    #[test]
    fn test_configure() {
        let d = DuplexD2 {
            conn: Box::new(TestGoodConnection { data: Vec::new() }),
        };
        let config = r#"{
                     "retries": 2
                    }"#;
        assert!(d.configure(config).is_ok(), "Config should pass")
    }

    #[test]
    fn test_reset() {
        let d = DuplexD2 {
            conn: Box::new(TestGoodConnection { data: Vec::new() }),
        };
        assert!(d.reset(RadioReset::HardReset).is_ok(), "Reset should pass")
    }

    #[test]
    fn test_send_command_fails() {
        let radio = DuplexD2 {
            conn: Box::new(TestBadConnection {}),
        };
        let command: u64 = 11111111111111111;
        match radio.send_command(command) {
            Ok(_) => assert!(false, "Expected send_command to fail.".to_string()),
            Err(message) => assert!(true, message),
        }
    }

    #[test]
    fn test_send_succeeds() {
        let radio = DuplexD2 {
            conn: Box::new(TestGoodConnection { data: Vec::new() }),
        };
        let data: Vec<u8> = Vec::new();
        assert!(radio.send(data).is_ok(), "Send should pass")
    }
}
