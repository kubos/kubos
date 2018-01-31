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

//! A high level interface for interacting with radios

#![deny(missing_docs)]

extern crate serde_json;

use serde_json::Error as SerdeJsonError;

/// The kind of error that can be produced while interfacing with a radio
pub enum RadioError {
    /// Returned if rx buffer is empty while trying to receive
    RxEmpty,
    /// Returned for generic error conditions
    Error,
    /// Returned if an error is found during configuration
    ErrorConfig,
}

/// The kind of resets a radio can perform
pub enum RadioReset {
    /// Hardware level reset
    HardReset,
    /// Software level reset
    SoftReset,
}

/// Radio trait describes a high level interface for interacting
/// with radio hardware
pub trait Radio {
    /// Initializes radio interface
    fn init(&self) -> Result<(), RadioError>;

    /// Terminates radio interface
    fn terminate(&self) -> Result<(), RadioError>;

    /// Configures radio device
    fn configure(&self, json_config: &str) -> Result<(), SerdeJsonError>;

    /// Resets radio device
    fn reset(&self, reset_type: RadioReset) -> Result<(), RadioError>;

    /// Sends buffer to (and hopefully through) radio
    fn send(&self, buffer: Vec<u8>) -> Result<(), RadioError>;

    /// Receives and reads data off radio buffer
    fn receive(&self) -> Result<Vec<u8>, RadioError>;

    /// Requests and retrieves radio telemetry
    /// Telemetry is returned as json stored in a String
    fn get_telemetry<T>(&self, telem_type: T) -> Result<&str, RadioError>;
}
