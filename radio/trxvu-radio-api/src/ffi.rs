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

use libc::uint8_t;

/// The radio_telem is a union in the C source
/// The largest element in the union holds six uint16_t
/// For simplicity we will use a buffer of two uint8_t
#[repr(C)]
pub struct radio_telem(pub [uint8_t; 12]);

/// Enum for selecting the telemetry type
#[repr(C)]
pub enum radio_telem_type {
    tx_telem_all,
    tx_telem_last,
    tx_uptime,
    tx_state,
    rx_telem_all,
    rx_uptime
}

extern "C" {
    pub fn k_radio_get_telemetry(buffer: *mut radio_telem, telem_type: radio_telem_type) -> uint8_t;
}
