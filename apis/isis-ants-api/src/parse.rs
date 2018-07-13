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

use ants::*;
use ffi;
use std::mem::transmute;

/// I<sup>2</sup>C bus which will be used for communication
///
/// *Note: Not all OBCs will have all of these buses avaialable*
pub enum KI2CNum {
    /// I<sup>2</sup>C Bus 1
    KI2C1,
    /// I<sup>2</sup>C Bus 2
    KI2C2,
    /// I<sup>2</sup>C Bus 3
    KI2C3,
}

/// Specific antenna to control
///
/// *Note: Not all antenna systems have four antennas*
pub enum KANTSAnt {
    /// Antenna 1
    Ant1,
    /// Antenna 2
    Ant2,
    /// Antenna 3
    Ant3,
    /// Antenna 4
    Ant4,
}

/// Antenna microcontroller which any commands should be run against
pub enum KANTSController {
    /// Primary microcontroller
    Primary,
    /// Secondary/redundant microcontroller
    Secondary,
}

/// System telemetry fields returned from [`get_system_telemetry`]
///
/// [`get_system_telemetry`]: struct.AntS.html#method.get_system_telemetry
pub struct AntsTelemetry {
    /// Current system temperature (raw value)
    pub raw_temp: u16,
    /// Current deployment status flags
    pub deploy_status: DeployStatus,
    /// System uptime (in seconds)
    pub uptime: u32,
}

/// Current deployment status returned from [`get_deploy`]
///
/// [`get_deploy`]: struct.AntS.html#method.get_deploy
pub struct DeployStatus {
    /// Whether antenna system independent burn is active
    pub sys_burn_active: bool,
    /// Whether the antenna system is ignoring the deployment switches
    pub sys_ignore_deploy: bool,
    /// Whether the antenna system is armed (ready for deployment)
    pub sys_armed: bool,
    /// Whether antenna 1 is *not* deployed
    pub ant_1_not_deployed: bool,
    /// Whether antenna 1's deployment time limit was reached before achieving full deployment
    pub ant_1_stopped_time: bool,
    /// Whether antenna 1's deployment system is currently active
    pub ant_1_active: bool,
    /// Whether antenna 2 is *not* deployed
    pub ant_2_not_deployed: bool,
    /// Whether antenna 2's deployment time limit was reached before achieving full deployment
    pub ant_2_stopped_time: bool,
    /// Whether antenna 2's deployment system is currently active
    pub ant_2_active: bool,
    /// Whether antenna 3 is *not* deployed
    pub ant_3_not_deployed: bool,
    /// Whether antenna 3's deployment time limit was reached before achieving full deployment
    pub ant_3_stopped_time: bool,
    /// Whether antenna 3's deployment system is currently active
    pub ant_3_active: bool,
    /// Whether antenna 4 is *not* deployed
    pub ant_4_not_deployed: bool,
    /// Whether antenna 4's deployment time limit was reached before achieving full deployment
    pub ant_4_stopped_time: bool,
    /// Whether antenna 4's deployment system is currently active
    pub ant_4_active: bool,
}

named!(parse_status<&[u8], DeployStatus>,
	do_parse!(
		bits: bits!(
				tuple!(
				    take_bits!(u8, 1),
				    take_bits!(u8, 1),
				    take_bits!(u8, 1),
				    take_bits!(u8, 1),
				    take_bits!(u8, 1),
				    take_bits!(u8, 1),
				    take_bits!(u8, 1),
				    take_bits!(u8, 1),
				    take_bits!(u8, 1),
				    take_bits!(u8, 1),
				    take_bits!(u8, 1),
				    take_bits!(u8, 1),
				    take_bits!(u8, 1),
				    take_bits!(u8, 1),
				    take_bits!(u8, 1),
				    take_bits!(u8, 1)
				)
		) >>
	    (DeployStatus {
	         //Note: Bit 11 is unused
	         sys_burn_active: bits.3 == 1,
	         sys_ignore_deploy: bits.15 == 1,
	         sys_armed: bits.7 == 1,
	         ant_1_not_deployed: bits.8 == 1,
	         ant_1_stopped_time: bits.9 == 1,
	         ant_1_active: bits.10 == 1,
	         ant_2_not_deployed: bits.12 == 1,
	         ant_2_stopped_time: bits.13 == 1,
	         ant_2_active: bits.14 == 1,
	         ant_3_not_deployed: bits.0 == 1,
	         ant_3_stopped_time: bits.1 == 1,
	         ant_3_active: bits.2 == 1,
	         ant_4_not_deployed: bits.4 == 1,
	         ant_4_stopped_time: bits.5 == 1,
	         ant_4_active: bits.6 == 1,
	     })
	)
);

impl AntsTelemetry {
    #[doc(hidden)]
    pub fn new(c_telem: ffi::AntsTelemetry) -> Result<AntsTelemetry, AntsError> {
        let raw_status: [u8; 2] = unsafe { transmute(c_telem.deploy_status) };

        let status = DeployStatus::new(&raw_status)?;

        let telem = AntsTelemetry {
            raw_temp: c_telem.raw_temp,
            deploy_status: status,
            uptime: c_telem.uptime,
        };
        Ok(telem)
    }
}

impl DeployStatus {
    #[doc(hidden)]
    pub fn new(input: &[u8]) -> Result<DeployStatus, AntsError> {
        match parse_status(input) {
            Ok(v) => {
                let (_input, status) = v;
                Ok(status)
            }
            _ => Err(AntsError::GenericError.into()),
        }
    }
}

pub fn convert_bus(bus: KI2CNum) -> ffi::KI2CNum {
    match bus {
        self::KI2CNum::KI2C1 => ffi::KI2CNum::KI2C1,
        self::KI2CNum::KI2C2 => ffi::KI2CNum::KI2C2,
        self::KI2CNum::KI2C3 => ffi::KI2CNum::KI2C3,
    }
}

pub fn convert_controller(controller: KANTSController) -> ffi::KANTSController {
    match controller {
        self::KANTSController::Primary => ffi::KANTSController::Primary,
        self::KANTSController::Secondary => ffi::KANTSController::Secondary,
    }
}

pub fn convert_antenna(antenna: KANTSAnt) -> ffi::KANTSAnt {
    match antenna {
        self::KANTSAnt::Ant1 => ffi::KANTSAnt::Ant1,
        self::KANTSAnt::Ant2 => ffi::KANTSAnt::Ant2,
        self::KANTSAnt::Ant3 => ffi::KANTSAnt::Ant3,
        self::KANTSAnt::Ant4 => ffi::KANTSAnt::Ant4,
    }
}
