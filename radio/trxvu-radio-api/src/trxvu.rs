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

use std::mem;

use radio_api::{Connection, RadioResult, nom_to_radio_error};
use messages::{TxTelemetry, RxTelemetry};
use ffi;

/// Structure for interacting with the TRXVU Radio API
pub struct Trxvu { }

impl Trxvu {
    /// Constructor
    pub fn new() -> Self { Trxvu { } }

    /// Retrieves the tx telemetry
    pub fn get_tx_telemetry(&self) -> RadioResult<TxTelemetry> {
        let mut telem : ffi::radio_telem = unsafe { mem::uninitialized() };
        unsafe { ffi::k_radio_get_telemetry(&mut telem, ffi::radio_telem_type::tx_telem_all) };
        let (extra, value) = TxTelemetry::parse(&telem.0).or_else(nom_to_radio_error)?;
        Ok(value)
    }

    /// Retrieves the rx telemetry
    pub fn get_rx_telemetry(&self) -> RadioResult<RxTelemetry> {
        let mut telem : ffi::radio_telem = unsafe { mem::uninitialized() };
        unsafe { ffi::k_radio_get_telemetry(&mut telem, ffi::radio_telem_type::rx_telem_all) };
        let (extra, value) = RxTelemetry::parse(&telem.0).or_else(nom_to_radio_error)?;
        Ok(value)
    }
}
