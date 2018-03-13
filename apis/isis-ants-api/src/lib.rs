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
#[derive(Debug, Fail)]
pub enum AntsError {
    //TODO: need to do anything with 'cause'?
    #[fail(display = "Generic error")]
    GenericError,
    #[fail(display = "Configuration error")]
    ConfigError,
}

pub type Err = AntsError;
pub type AntSResult<T> = Result<T, Error>;

pub fn init(
    bus: ffi::KI2CNum,
    primary: u8,
    secondary: u8,
    ant_count: u8,
    timeout: u32,
) -> AntSResult<()> {

    match unsafe { ffi::k_ants_init(bus, primary, secondary, ant_count, timeout) } {
        //TODO: better error handling?
        AntsOk => Ok(()),
        AntsErrorConfig => Err(AntsError::ConfigError.into()),
        _ => Err(AntsError::GenericError.into()),
    }

}
