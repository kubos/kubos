//
// Copyright (C) 2018 Kubos Corporation
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

#[allow(dead_code)]
#[repr(C)]
pub enum KI2CNum {
    KI2CNoBus,
    KI2C1,
    KI2C2,
    KI2C3,
}

#[allow(dead_code)]
#[repr(C)]
pub enum KANTSStatus {
    AntsOK,
    AntsError,
    AntsErrorConfig,
    AntsErrorNotImplemented,
}

#[repr(C)]
pub enum KANTSController {
    Primary,
    Secondary,
}

#[repr(C)]
pub enum KANTSAnt {
    Ant1,
    Ant2,
    Ant3,
    Ant4,
}

#[repr(C, packed)]
pub struct AntsTelemetry {
    pub raw_temp: u16,
    pub deploy_status: u16,
    pub uptime: u32,
}

/// Bring in C functions from isis-ants-api
extern "C" {
    pub fn k_ants_init(
        bus: KI2CNum,
        primary: u8,
        secondary: u8,
        ant_count: u8,
        timeout: u32,
    ) -> KANTSStatus;
    pub fn k_ants_terminate();
    pub fn k_ants_configure(config: KANTSController) -> KANTSStatus;
    pub fn k_ants_reset() -> KANTSStatus;
    pub fn k_ants_arm() -> KANTSStatus;
    pub fn k_ants_disarm() -> KANTSStatus;
    pub fn k_ants_deploy(antenna: KANTSAnt, force: bool, timeout: u8) -> KANTSStatus;
    pub fn k_ants_auto_deploy(timeout: u8) -> KANTSStatus;
    pub fn k_ants_cancel_deploy() -> KANTSStatus;
    pub fn k_ants_get_deploy_status(resp: *mut u8) -> KANTSStatus;
    pub fn k_ants_get_uptime(uptime: *mut u32) -> KANTSStatus;
    pub fn k_ants_get_system_telemetry(telem: *mut AntsTelemetry) -> KANTSStatus;
    pub fn k_ants_get_activation_count(antenna: KANTSAnt, count: *mut u8) -> KANTSStatus;
    pub fn k_ants_get_activation_time(antenna: KANTSAnt, time: *mut u16) -> KANTSStatus;
    pub fn k_ants_watchdog_kick() -> KANTSStatus;
    pub fn k_ants_watchdog_start() -> KANTSStatus;
    pub fn k_ants_watchdog_stop() -> KANTSStatus;
    pub fn k_ants_passthrough(tx: *const u8, tx_len: u8, rx: *mut u8, rx_len: u8) -> KANTSStatus;
}
