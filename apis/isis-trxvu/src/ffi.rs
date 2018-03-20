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

use radio_api::RadioError;

pub const RX_MAX_SIZE: usize = 200;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct TxTelemRaw {
    pub inst_rf_reflected: u16,
    pub inst_rf_forward: u16,
    pub supply_voltage: u16,
    pub supply_current: u16,
    pub temp_power_amp: u16,
    pub temp_oscillator: u16,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct RxTelemRaw {
    pub inst_doppler_offset: u16,
    pub supply_current: u16,
    pub supply_voltage: u16,
    pub temp_oscillator: u16,
    pub temp_power_amp: u16,
    pub inst_signal_strength: u16,
}

/// The radio_telem is a union in the C source
/// The largest element in the union holds six uint16_t
/// For simplicity we will use a buffer of two uint8_t
#[repr(C)]
pub union TelemRaw {
    pub tx_state: u8,
    pub uptime: u32,
    pub tx_telem_raw: TxTelemRaw,
    pub rx_telem_raw: RxTelemRaw,
}

/// Enum for selecting the telemetry type
#[repr(C)]
pub enum radio_telem_type {
    TxTelemAll,
    TxTelemLast,
    TxUptime,
    TxState,
    RxTelemAll,
    RxUptime,
}

#[repr(C)]
pub enum RawTxStateFirstBit {
    IdleOff = 0x00,
    IdleOn = 0x01,
    BeaconActive = 0x02,
}

#[repr(C)]
pub enum RawTxStateSecondBit {
    B1200 = 0x00,
    B2400 = 0x01,
    B4800 = 0x02,
    B9600 = 0x03,
}

/// Enum for radio status
/// from radio-api/radio-struct.h
#[repr(C)]
#[derive(Debug)]
pub enum radio_status {
    RadioOk,
    RadioRxEmpty,
    RadioError,
    RadioErrorConfig,
}

impl From<radio_status> for RadioError {
    fn from(s: radio_status) -> Self {
        match s {
            _ => RadioError::HardwareError {
                message: format!("{:?}", s),
            },
        }
    }
}

// Helper function to convert radio status to radio error
pub fn radio_status_to_err(status: radio_status) -> Result<(), RadioError> {
    match status {
        radio_status::RadioOk => Ok(()),
        // I don't feel like this should be an error...
        radio_status::RadioRxEmpty => Ok(()),
        _ => Err(RadioError::HardwareError {
            message: format!("TRXVU radio error {:?}", status),
        }),
    }
}

#[repr(C)]
pub struct radio_rx_message {
    pub msg_size: u16,
    pub doppler_offset: u16,
    pub signal_strength: u16,
    pub message: [u8; RX_MAX_SIZE],
}

extern "C" {
    pub fn k_radio_init() -> radio_status;
    pub fn k_radio_watchdog_start() -> radio_status;
    pub fn k_radio_watchdog_stop() -> radio_status;
    pub fn k_radio_terminate();
    pub fn k_radio_get_telemetry(
        buffer: *mut TelemRaw,
        telem_type: radio_telem_type,
    ) -> radio_status;

    pub fn k_radio_send(buffer: *const u8, len: i32, response: *mut u8) -> radio_status;

    pub fn k_radio_recv(buffer: *mut radio_rx_message, len: *mut u8) -> radio_status;

}
