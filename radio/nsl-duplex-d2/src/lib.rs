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
//! https://nearspacelaunch.com/product/eyestar-d2/

// #![deny(missing_docs)]

extern crate radio_api;
extern crate serde_json;
extern crate serial;

mod radio_stub;
pub mod comms;

use serde_json::Error as SerdeJsonError;

use radio_api::{Radio, RadioError, RadioReset};

/// Structure implementing Radio functionality for Duplex-D2
pub struct DuplexD2 {}

impl Radio for DuplexD2 {
    fn init(&self) -> Result<(), RadioError> {
        Ok(())
    }

    fn terminate(&self) -> Result<(), RadioError> {
        Ok(())
    }

    fn reset(&self, reset_type: RadioReset) -> Result<(), RadioError> {
        match reset_type {
            RadioReset::HardReset => {
                // A hardware reset is signaled via a GPIO tied
                // to the modem.
                Ok(())
            }
            RadioReset::SoftReset => {
                // A software reset is hopefully trigged by
                // a command sent to the modem.
                Ok(())
            }
        }
    }

    fn configure(&self, _json_config: &str) -> Result<(), SerdeJsonError> {
        Ok(())
    }

    fn send(&self, _buffer: Vec<u8>) -> Result<(), RadioError> {
        Ok(())
    }

    fn receive(&self) -> Result<(Vec<u8>), RadioError> {
        match comms::get_uploaded_file() {
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

    #[test]
    fn test_init() {
        let d = DuplexD2 {};
        assert!(d.init().is_ok(), "Init should pass")
    }

    #[test]
    fn test_terminate() {
        let d = DuplexD2 {};
        assert!(d.terminate().is_ok(), "Terminate should pass")
    }

    #[test]
    fn test_configure() {
        let d = DuplexD2 {};
        let config = r#"{
                     "retries": 2
                    }"#;
        assert!(d.configure(config).is_ok(), "Config should pass")
    }

    #[test]
    fn test_reset() {
        let d = DuplexD2 {};
        assert!(d.reset(RadioReset::HardReset).is_ok(), "Reset should pass")
    }

    #[test]
    fn test_send() {
        let d = DuplexD2 {};
        let data: Vec<u8> = Vec::new();
        assert!(d.send(data).is_ok(), "Send should pass")
    }

    #[test]
    fn test_receive() {
        let d = DuplexD2 {};
        assert!(d.receive().is_ok(), "Receive should pass")
    }
}
