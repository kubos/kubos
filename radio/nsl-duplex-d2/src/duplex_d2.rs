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

use radio_api::{Connection, Radio, RadioError, RadioReset};
use commands::Command;

pub struct DuplexD2 {
    conn: Box<Connection>,
}

impl DuplexD2 {
    pub fn new(conn: Box<Connection>) -> DuplexD2 {
        DuplexD2 { conn }
    }

    pub fn send_command<T>(&self, command: &Command<T>) -> Result<T, String> {
        self.conn.send(&command.request)?;
        let (_, res) = (command.parse)(&self.conn.receive()?).or(Err("Parse problem"))?;
        Ok(res)
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

    fn configure(&self, _json_config: &str) -> Result<(), String> {
        Ok(())
    }

    fn send(&self, _buffer: Vec<u8>) -> Result<(), RadioError> {
        Ok(())
    }

    fn receive(&self) -> Result<(Vec<u8>), RadioError> {
        Ok(vec![1])
    }

    fn get_telemetry<TelemetryType>(&self, _telem_type: TelemetryType) -> Result<&str, RadioError> {
        Ok("telemetry")
    }
}

#[cfg(test)]
mod tests {
    use duplex_d2::*;
    use nom::IResult;

    struct TestGoodConnection {
        data: Vec<u8>,
    }

    impl Connection for TestGoodConnection {
        fn send(&self, _data: &[u8]) -> Result<(), String> {
            Ok(())
        }

        fn receive(&self) -> Result<Vec<u8>, String> {
            Ok(self.data.clone())
        }
    }

    struct TestBadConnection {}

    impl Connection for TestBadConnection {
        fn send(&self, _: &[u8]) -> Result<(), String> {
            return Err(String::from("Send failed"));
        }

        fn receive(&self) -> Result<Vec<u8>, String> {
            return Err(String::from("Receive failed"));
        }
    }

    fn test_command() -> Command<u32> {
        let request = vec![0x47, 0x55, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        fn parse(_: &[u8]) -> IResult<&[u8], u32> {
            Ok((b"", 1))
        }
        Command { request, parse }
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

        let command = test_command();
        match radio.send_command(&command) {
            Ok(_) => assert!(false, "Expected send_command to fail.".to_string()),
            Err(message) => assert!(true, message),
        }
    }

    #[test]
    fn test_send_command_succeeds() {
        let radio = DuplexD2 {
            conn: Box::new(TestGoodConnection { data: Vec::new() }),
        };

        let command = test_command();
        match radio.send_command(&command) {
            Ok(_) => assert!(true),
            Err(message) => assert!(false, message),
        }
    }
}
