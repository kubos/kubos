//
// Copyright (C) 2019 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//!
//! Data model used to expose communications service telemetry
//! information over the GraphQL interface.
//!

use crate::comms::DuplexComms;
use comms_service::CommsTelemetry;
use nsl_duplex_d2::{GeoRecord, StateOfHealth};
use std::convert::From;
use std::sync::{Arc, Mutex};

#[derive(GraphQLObject)]
pub struct GeoRecordResponse {
    lon: f64,
    lat: f64,
    time: i32,
    max_error: i32,
}

impl From<GeoRecord> for GeoRecordResponse {
    fn from(item: GeoRecord) -> GeoRecordResponse {
        GeoRecordResponse {
            lon: f64::from(item.lon),
            lat: f64::from(item.lat),
            time: item.time as i32,
            max_error: item.max_error as i32,
        }
    }
}

#[derive(GraphQLObject)]
pub struct StateOfHealthResponse {
    /// (4 byte integer) Current epoch reset count, starts at 0,
    /// incremented for each power system reset, persistent over the life of the mission
    pub reset_count: i32,
    /// (4 byte integer) Current time (seconds) from start of most recent reset
    pub current_time: i32,
    /// (1 byte integer) Current RSSI (Received Signal Strength Indicator), 0 to 4
    pub current_rssi: i32,
    /// (1 byte integer) Connection status, 0 (connected) or 1 (disconnected)
    pub connection_status: i32,
    /// (1 byte integer) Globalstar gateway connected to, proprietary ID, 0 to 255
    pub globalstar_gateway: i32,
    /// (4 byte integer) Last contact time, seconds since latest reset
    pub last_contact_time: i32,
    /// (4 byte integer) Last attempt time, seconds since latest reset
    pub last_attempt_time: i32,
    /// (4 byte integer) Count of call attempts since latest reset
    pub call_attempts_since_reset: i32,
    /// (4 byte integer) Count of successful connects since latest reset
    pub successful_connects_since_reset: i32,
    /// (4 byte integer) Average connection duration (seconds)
    pub average_connection_duration: i32,
    /// (4 byte integer) Connection duration standard deviation (seconds)
    pub connection_duration_std_dev: i32,
}

impl From<StateOfHealth> for StateOfHealthResponse {
    fn from(item: StateOfHealth) -> StateOfHealthResponse {
        StateOfHealthResponse {
            reset_count: item.reset_count as i32,
            current_time: item.current_time as i32,
            current_rssi: i32::from(item.current_rssi),
            connection_status: i32::from(item.connection_status),
            globalstar_gateway: i32::from(item.globalstar_gateway),
            last_contact_time: item.last_contact_time as i32,
            last_attempt_time: item.last_attempt_time as i32,
            call_attempts_since_reset: item.call_attempts_since_reset as i32,
            successful_connects_since_reset: item.successful_connects_since_reset as i32,
            average_connection_duration: item.average_connection_duration as i32,
            connection_duration_std_dev: item.connection_duration_std_dev as i32,
        }
    }
}

#[derive(Clone)]
pub struct Subsystem {
    telem: Arc<Mutex<CommsTelemetry>>,
    pub duplex: Arc<Mutex<DuplexComms>>,
}

impl Subsystem {
    pub fn new(telem: Arc<Mutex<CommsTelemetry>>, duplex: Arc<Mutex<DuplexComms>>) -> Subsystem {
        Subsystem { telem, duplex }
    }

    pub fn failed_packets_up(&self) -> Result<i32, String> {
        match self.telem.lock() {
            Ok(data) => Ok(data.failed_packets_up),
            Err(_) => Err("Failed to lock telemetry".to_owned()),
        }
    }

    pub fn failed_packets_down(&self) -> Result<i32, String> {
        match self.telem.lock() {
            Ok(data) => Ok(data.failed_packets_down),
            Err(_) => Err("Failed to lock telemetry".to_owned()),
        }
    }

    pub fn packets_up(&self) -> Result<i32, String> {
        match self.telem.lock() {
            Ok(data) => Ok(data.packets_up),
            Err(_) => Err("Failed to lock telemetry".to_owned()),
        }
    }

    pub fn packets_down(&self) -> Result<i32, String> {
        match self.telem.lock() {
            Ok(data) => Ok(data.packets_down),
            Err(_) => Err("Failed to lock telemetry".to_owned()),
        }
    }

    pub fn errors(&self) -> Result<Vec<String>, String> {
        match self.telem.lock() {
            Ok(data) => Ok(data.errors.to_owned()),
            Err(_) => Err("Failed to lock telemetry".to_owned()),
        }
    }

    pub fn get_alive(&self) -> Result<bool, String> {
        match self.duplex.lock() {
            Ok(duplex) => Ok(duplex.radio.get_alive().map_err(|e| e.to_string())?),
            Err(_) => Err("Failed to lock duplex".to_owned()),
        }
    }

    pub fn file_queue_count(&self) -> Result<i32, String> {
        match self.duplex.lock() {
            Ok(duplex) => Ok(duplex
                .radio
                .get_download_file_count()
                .map_err(|e| e.to_string())? as i32),
            Err(_) => Err("Failed to lock duplex".to_owned()),
        }
    }

    pub fn modem_health(&self) -> Result<StateOfHealthResponse, String> {
        match self.duplex.lock() {
            Ok(duplex) => Ok(StateOfHealthResponse::from(
                duplex
                    .radio
                    .get_state_of_health_for_modem()
                    .map_err(|e| e.to_string())?,
            )),
            Err(_) => Err("Failed to lock duplex".to_owned()),
        }
    }

    pub fn geolocation(&self) -> Result<GeoRecordResponse, String> {
        match self.duplex.lock() {
            Ok(duplex) => Ok(GeoRecordResponse::from(
                duplex
                    .radio
                    .get_geolocation_position_estimate()
                    .map_err(|e| e.to_string())?,
            )),
            Err(_) => Err("Failed to lock duplex".to_owned()),
        }
    }
}
