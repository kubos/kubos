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

use radio_api::{Connection, Radio, RadioError, RadioReset};
use messages::{File, FileCount, StateOfHealth};

/// Structure for interacting with Duplex-D2 Radio API
pub struct DuplexD2 {
    conn: Connection,
}

impl DuplexD2 {
    /// Constructor for DuplexD2 structure
    pub fn new(conn: Connection) -> DuplexD2 {
        DuplexD2 { conn }
    }

    /// Helper function for generating Command<File>
    pub fn get_file(&self) -> Result<File, String> {
        self.conn.send(b"GUGET_UF")?;
        let result = self.conn.read(File::parse)?;
        self.conn.send(b"GU\x06")?;
        Ok(result)
    }

    /// Helper function for generating Command<FileCount>
    pub fn get_file_count(&self) -> Result<FileCount, String> {
        self.conn.send(b"GUGETUFC")?;
        self.conn.read(FileCount::parse)
    }

    /// Helper function for generating Command<StateOfHealth>
    pub fn get_state_of_health(&self) -> Result<StateOfHealth, String> {
        self.conn.send(b"GUGETSOH")?;
        self.conn.read(StateOfHealth::parse)
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

    // fn good_send(data: &[u8]) -> Result<(), String> {
    //     Ok(())
    // }
    //
    // fn good_receive(&self) -> Result<Vec<u8>, String> {
    //     Ok(self.data.clone())
    // }
    //
    // struct TestBadConnection {}
    //
    // impl Connection for TestBadConnection {
    //     fn send(&self, _: &[u8]) -> Result<(), String> {
    //         return Err(String::from("Send failed"));
    //     }
    //
    //     fn receive(&self) -> Result<Vec<u8>, String> {
    //         return Err(String::from("Receive failed"));
    //     }
    // }

    // fn test_command() -> Command<u32> {
    //     let request = vec![0x47, 0x55, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    //     fn parse(_: &[u8]) -> IResult<&[u8], u32> {
    //         Ok((b"", 1))
    //     }
    //     Command { request, parse }
    // }

    // #[test]
    // fn test_init() {
    //     let d = DuplexD2::new(Connection::new(|| {}, || {}));
    //     assert!(d.init().is_ok(), "Init should pass")
    // }
    //
    // #[test]
    // fn test_terminate() {
    //     let d = DuplexD2::new(Connection::new(|| {}, || {}));
    //     assert!(d.terminate().is_ok(), "Terminate should pass")
    // }
    //
    // #[test]
    // fn test_configure() {
    //     let d = DuplexD2::new(Connection::new(|| {}, || {}));
    //     let config = r#"{
    //                  "retries": 2
    //                 }"#;
    //     assert!(d.configure(config).is_ok(), "Config should pass")
    // }
    //
    // #[test]
    // fn test_reset() {
    //     let d = DuplexD2::new(Connection::new(|| {}, || {}));
    //     assert!(d.reset(RadioReset::HardReset).is_ok(), "Reset should pass")
    // }

    #[test]
    fn test_send_command_fails() {
        let radio = DuplexD2::new(Connection::new(
            |_| Err("Send failed".to_string()),
            || Err("Receive failed".to_string()),
        ));

        match radio.get_file_count() {
            Ok(_) => assert!(false, "Expected send_command to fail.".to_string()),
            Err(message) => assert!(true, message),
        }
    }

    // TODO: redesign everything so this works!!!
    // impl DuplexD2 {
    //     fn test_command(&self) -> Result<u32, String> {
    //         Ok(42)
    //     }
    // }
    // #[test]
    // fn test_send_command_succeeds() {
    //     let buffer: Vec<u8> = Vec::new();
    //     let radio = DuplexD2::new(Connection::new(
    //         |data| {
    //             buffer.extend_from_slice(data);
    //             Ok(())
    //         },
    //         || Ok(buffer),
    //     ));
    //
    //     match radio.test_command() {
    //         Ok(_) => assert!(true),
    //         Err(message) => assert!(false, message),
    //     }
    // }
}
