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

use radio_api::{nom_to_radio_error, Connection, RadioResult};
use messages::{RxTelemetry, TxTelemetry};
use ffi;

use libc;

/// Structure for interacting with the TRXVU Radio API
pub struct Trxvu {}

impl Trxvu {
    /// Constructor
    pub fn new() -> Self {
        unsafe {
            match ffi::k_radio_init() {
                ffi::radio_status::radio_ok => (),
                e => panic!("Error on radio_init {:?}", e),
            };
            match ffi::k_radio_watchdog_start() {
                ffi::radio_status::radio_ok => (),
                e => panic!("Error on radio watchdog start {:?}", e),
            };
        };
        Trxvu {}
    }

    /// Retrieves the tx telemetry
    pub fn get_tx_telemetry(&self) -> RadioResult<TxTelemetry> {
        let mut telem: ffi::TelemRaw = unsafe { mem::uninitialized() };
        ffi::radio_status_to_err(unsafe {
            ffi::k_radio_get_telemetry(&mut telem, ffi::radio_telem_type::tx_telem_all)
        })?;
        Ok(unsafe { TxTelemetry::parse(&telem.tx_telem_raw) })
    }

    /// Retrieves the rx telemetry
    pub fn get_rx_telemetry(&self) -> RadioResult<RxTelemetry> {
        let mut telem: ffi::TelemRaw = unsafe { mem::uninitialized() };
        ffi::radio_status_to_err(unsafe {
            ffi::k_radio_get_telemetry(&mut telem, ffi::radio_telem_type::rx_telem_all)
        })?;
        Ok(unsafe { RxTelemetry::parse(&telem.rx_telem_raw) })
    }

    pub fn get_rx_uptime(&self) -> RadioResult<u32> {
        let mut telem: ffi::TelemRaw = unsafe { mem::uninitialized() };
        ffi::radio_status_to_err(unsafe {
            ffi::k_radio_get_telemetry(&mut telem, ffi::radio_telem_type::rx_uptime)
        })?;
        Ok(unsafe { telem.uptime })
    }

    pub fn send(&self, message: &[u8]) -> RadioResult<()> {
        let mut response: u8 = 0;
        unsafe {
            ffi::radio_status_to_err(ffi::k_radio_send(
                message.as_ptr(),
                message.len() as i32,
                &mut response,
            ))?;
        };
        Ok(())
    }

    pub fn read(&self) -> RadioResult<Vec<u8>> {
        let mut response: Vec<u8> = Vec::new();
        let mut rx_msg: ffi::radio_rx_message = unsafe { mem::uninitialized() };
        let mut len: u8 = 0;
        unsafe {
            ffi::k_radio_recv(&mut rx_msg, &mut len);
        }
        response.extend_from_slice(&rx_msg.message);
        Ok(response)
    }
}

impl Drop for Trxvu {
    fn drop(&mut self) {
        unsafe {
            ffi::k_radio_watchdog_stop();
            ffi::k_radio_terminate();
        }
    }
}
