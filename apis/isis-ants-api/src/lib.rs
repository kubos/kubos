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

//TODO: remove before publishing
#![allow(unused)]

#[macro_use]
extern crate failure;
use failure::Error;

//TODO: Only making this pub for KI2CNum, maybe create
//a conversion function with a new Rusty enum?
pub mod ffi;

/// Common Error for AntS Actions
#[derive(Fail, Display, Debug)]
pub enum AntsError {
    //TODO: need to do anything with 'cause'?
    #[display(fmt = "Generic error")]
    GenericError,
    #[display(fmt = "Configuration error")]
    ConfigError,
}

pub type Err = AntsError;
pub type AntSResult<T> = Result<T, Error>;

pub struct AntS;

impl AntS {
    pub fn new(
        bus: ffi::KI2CNum,
        primary: u8,
        secondary: u8,
        ant_count: u8,
        timeout: u32,
    ) -> AntSResult<Self> {
        //call init
        match unsafe { ffi::k_ants_init(bus, primary, secondary, ant_count, timeout) } {
            //TODO: better error handling?
            ffi::KANTSStatus::AntsOK => Ok(AntS),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    pub fn configure(config: ffi::KANTSController) -> AntSResult<()> {
        match unsafe { ffi::k_ants_configure(config) } {
            //TODO: better error handling?
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    pub fn reset() -> AntSResult<()> {
        match unsafe { ffi::k_ants_reset() } {
            //TODO: better error handling?
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    pub fn arm() -> AntSResult<()> {
        match unsafe { ffi::k_ants_arm() } {
            //TODO: better error handling?
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    pub fn disarm() -> AntSResult<()> {
        match unsafe { ffi::k_ants_disarm() } {
            //TODO: better error handling?
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    pub fn deploy(antenna: ffi::KANTSAnt, force: bool, timeout: u8) -> AntSResult<()> {
        match unsafe { ffi::k_ants_deploy(antenna, force, timeout) } {
            //TODO: better error handling?
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    pub fn auto_deploy(timeout: u8) -> AntSResult<()> {
        match unsafe { ffi::k_ants_auto_deploy(timeout) } {
            //TODO: better error handling?
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    pub fn cancel_deploy() -> AntSResult<()> {
        match unsafe { ffi::k_ants_cancel_deploy() } {
            //TODO: better error handling?
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    pub fn get_deploy() -> AntSResult<u16> {

        let mut status: u16 = 0;

        match unsafe { ffi::k_ants_get_deploy_status(&mut status) } {
            //TODO: better error handling?
            ffi::KANTSStatus::AntsOK => Ok(status),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    pub fn get_uptime(&self) -> AntSResult<u8> {

        let mut uptime = 0;

        match unsafe { ffi::k_ants_get_uptime(&mut uptime) } {
            //TODO: better error handling?
            ffi::KANTSStatus::AntsOK => Ok(uptime),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    pub fn get_system_telemetry() -> AntSResult<ffi::AntsTelemetry> {

        let mut telem = ffi::AntsTelemetry {
            raw_temp: 0,
            deploy_status: 0,
            uptime: 0,
        };

        match unsafe { ffi::k_ants_get_system_telemetry(&mut telem) } {
            //TODO: better error handling?
            ffi::KANTSStatus::AntsOK => Ok(telem),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    pub fn get_activation_count(antenna: ffi::KANTSAnt) -> AntSResult<u8> {

        let mut count: u8 = 0;

        match unsafe { ffi::k_ants_get_activation_count(antenna, &mut count) } {
            //TODO: better error handling?
            ffi::KANTSStatus::AntsOK => Ok(count),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    pub fn get_activation_time(antenna: ffi::KANTSAnt) -> AntSResult<u16> {

        let mut time: u16 = 0;

        match unsafe { ffi::k_ants_get_activation_time(antenna, &mut time) } {
            //TODO: better error handling?
            ffi::KANTSStatus::AntsOK => Ok(time),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }


    pub fn watchdog_kick() -> AntSResult<()> {
        match unsafe { ffi::k_ants_watchdog_kick() } {
            //TODO: better error handling?
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    pub fn watchdog_start() -> AntSResult<()> {
        match unsafe { ffi::k_ants_watchdog_start() } {
            //TODO: better error handling?
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    pub fn watchdog_stop() -> AntSResult<()> {
        match unsafe { ffi::k_ants_watchdog_stop() } {
            //TODO: better error handling?
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    pub fn passthrough(tx: &[u8], rx: &mut [u8]) -> AntSResult<()> {

        let tx_len: u8 = tx.len() as u8;
        let rx_len: u8 = rx.len() as u8;

        //TODO: Double check that this is the correct way to pass pointers to buffers
        match unsafe { ffi::k_ants_passthrough(tx.as_ptr(), tx_len, rx.as_mut_ptr(), rx_len) } {
            //TODO: better error handling?
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    fn drop(&mut self) {
        unsafe { ffi::k_ants_terminate() }
    }
}
