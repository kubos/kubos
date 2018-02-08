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

#![deny(missing_docs)]

extern crate chrono;
extern crate radio_api;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod radio_stub;
mod comms;

use chrono::{DateTime, Utc};

use serde_json::Error as SerdeJsonError;

use radio_api::{Radio, RadioError, RadioReset};

/// Structure implementing Radio functionality for Duplex-D2
pub struct DuplexD2 {}

/// Structure implementing Duplex-D2 configuration values
/// The retries member is a stand-in config value to allow
/// for testing of config string parsing
#[derive(Serialize, Deserialize)]
pub struct Config {
    retries: i32,
}

/// Structure for State of Health telemetry record
/// According to the ICD all of these integers are unsigned, big-endian
pub struct StateOfHealth {
    /// Current epoch reset count. Starts at 0. Incremented for
    /// each power system reset. Persistent over mission life
    epoch_reset_count: u32,
    /// Current time (seconds) from start of most recent reset
    current_time: u32,
    /// Current RSSI (Received Signal Strength Indicator)
    /// 0 to 4
    current_rssi: u8,
    /// Connection status 0 (connection) or 1 (disconnected)
    connection_status: u8,
    /// Globalstar gateway connected to (proprietary ID)
    gateway_id: u8,
    /// Last contact time (seconds) since last reset
    last_contact: u32,
    /// Last attempt time (seconds) since latest reset
    last_attempt: u32,
    /// Count of call attempts since latest reset
    num_call_attempts: u32,
    /// Count of successful connects since latest reset
    num_connects: u32,
    /// Average connection duration (secondds)
    avg_connection_time: u32,
    /// Connection duration standard deviation (seconds)
    connection_time_std_dev: u32,
}

/// Coordinate Direction
pub enum CoordDirection {
    /// North
    North,
    /// South
    South,
    /// East
    East,
    /// West
    West,
}

/// Longitude/Latitude
pub struct Coord {
    /// Direction of coordinate
    direction: CoordDirection,
    /// Degrees
    degrees: i16,
    /// Minutes
    minutes: i16,
    /// Seconds
    seconds: i16,
}

/// Structure for Geolocation record format
pub struct GeolocationRecord {
    /// N:DDD MM SS
    latitude: Coord,
    /// W:DDD MM SS
    longitude: Coord,
    /// TIME: DD MM YYY HH:MM:SS
    /// What timezone or UTC offset will this time be in??
    time: DateTime<Utc>,
    /// ERR:
    err: i32,
}

/// Different types of telemetry which can be requested
/// from the Duplex-D2 radio
pub enum TelemetryType {
    /// Retrieves a record of information regarding modem functionality
    StateOfHealth,
    /// Retrieves an estimate of the modem's latitude and longitude
    /// coordinates at the time of the last connection
    Geolocation,
    /// Retrieves a count of files that have been received by the modem
    UploadedFileCount,
    /// Retrieves a count of files still in queue on the modem
    /// awaiting download
    DownloadFileCount,
    /// Retrieves a count of messages that have been received by
    /// the modem and await retrieval
    UploadedMsgCount,
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

    fn configure(&self, json_config: &str) -> Result<(), SerdeJsonError> {
        let _c: Config = serde_json::from_str(json_config)?;
        Ok(())
    }

    fn send(&self, _buffer: Vec<u8>) -> Result<(), RadioError> {
        Ok(())
    }

    fn receive(&self) -> Result<(Vec<u8>), RadioError> {
        let d: Vec<u8> = Vec::new();
        Ok(d)
    }

    fn get_telemetry<TelemetryType>(&self, _telem_type: TelemetryType) -> Result<&str, RadioError> {
        comms::fetch_state_of_health();
        Ok("hi")
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
    fn test_configure_bad_config() {
        let d = DuplexD2 {};
        let config = r#"{
                    "timer": 100
                  }"#;
        assert!(d.configure(config).is_err(), "Config should fail")
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

    #[test]
    fn test_get_telemetry() {
        let d = DuplexD2 {};
        assert!(
            d.get_telemetry(TelemetryType::StateOfHealth).is_ok(),
            "Telemetry should pass"
        )
    }
}
