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

use adcs_api::AdcsError;

#[repr(C)]
#[derive(Clone, Debug)]
pub enum KADCSStatus {
    Ok,
    Error,
    ErrorConfig,
    ErrorNoResponse,
    ErrorInternal,
    ErrorMutex,
    ErrorNotImplemented,
}

impl Default for KADCSStatus {
    fn default() -> Self {
        KADCSStatus::Ok
    }
}

pub fn adcs_status_to_err(status: KADCSStatus) -> Result<(), AdcsError> {
    match status {
        KADCSStatus::Ok => Ok(()),
        KADCSStatus::Error => Err(AdcsError::Generic),
        KADCSStatus::ErrorConfig => Err(AdcsError::Config),
        KADCSStatus::ErrorNoResponse => Err(AdcsError::NoResponse),
        KADCSStatus::ErrorInternal => Err(AdcsError::Internal),
        KADCSStatus::ErrorMutex => Err(AdcsError::Mutex),
        KADCSStatus::ErrorNotImplemented => Err(AdcsError::NotImplemented),
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum KI2CNum {
    K_I2C_NO_BUS,
    K_I2C1,
    K_I2C2,
    K_I2C3,
}

impl From<u8> for KI2CNum {
    fn from(num: u8) -> Self {
        match num {
            1 => KI2CNum::K_I2C1,
            2 => KI2CNum::K_I2C2,
            3 => KI2CNum::K_I2C3,
            _ => KI2CNum::K_I2C_NO_BUS,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: i32,
    pub tv_nsec: i64,
}

pub trait ImtqFFI: Clone {
    fn k_adcs_init(&self, bus: KI2CNum, addr: u16, timeout: i32) -> KADCSStatus;
    fn k_adcs_terminate(&self);
    fn k_adcs_passthrough(
        &self,
        tx: *const u8,
        len: i32,
        rx: *mut u8,
        rx_len: i32,
        delay: *const timespec,
    ) -> KADCSStatus;

    fn k_adcs_configure(&self, node: *const u8) -> KADCSStatus;

    fn k_imtq_reset(&self) -> KADCSStatus;
    fn k_imtq_watchdog_start(&self) -> KADCSStatus;
    fn k_imtq_watchdog_stop(&self) -> KADCSStatus;
}

#[derive(Debug, Clone)]
pub struct ImtqRaw {}

impl ImtqFFI for ImtqRaw {
    fn k_adcs_init(&self, bus: KI2CNum, addr: u16, timeout: i32) -> KADCSStatus {
        unsafe { k_adcs_init(bus, addr, timeout) }
    }

    fn k_adcs_terminate(&self) {
        unsafe {
            k_adcs_terminate();
        }
    }

    fn k_adcs_passthrough(
        &self,
        tx: *const u8,
        len: i32,
        rx: *mut u8,
        rx_len: i32,
        delay: *const timespec,
    ) -> KADCSStatus {
        unsafe { k_adcs_passthrough(tx, len, rx, rx_len, delay) }
    }

    fn k_adcs_configure(&self, node: *const u8) -> KADCSStatus {
        unsafe { k_adcs_configure(node) }
    }

    fn k_imtq_reset(&self) -> KADCSStatus {
        unsafe { k_imtq_reset() }
    }

    fn k_imtq_watchdog_start(&self) -> KADCSStatus {
        unsafe { k_imtq_watchdog_start() }
    }

    fn k_imtq_watchdog_stop(&self) -> KADCSStatus {
        unsafe { k_imtq_watchdog_stop() }
    }
}

extern "C" {
    pub fn k_adcs_init(bus: KI2CNum, addr: u16, timeout: i32) -> KADCSStatus;
    pub fn k_adcs_terminate();
    pub fn k_adcs_passthrough(
        tx: *const u8,
        len: i32,
        rx: *mut u8,
        rx_len: i32,
        delay: *const timespec,
    ) -> KADCSStatus;
    pub fn k_adcs_configure(node: *const u8) -> KADCSStatus;

    pub fn k_imtq_reset() -> KADCSStatus;
    pub fn k_imtq_watchdog_start() -> KADCSStatus;
    pub fn k_imtq_watchdog_stop() -> KADCSStatus;
}
