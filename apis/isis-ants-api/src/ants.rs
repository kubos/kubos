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

use ffi;
use parse::*;
use std::ptr;

/// Common Error for AntS Actions
#[derive(Fail, Display, Debug, Clone)]
pub enum AntsError {
    /// Catch-all error
    #[display(fmt = "Generic error")]
    GenericError,
    /// Configuration error. Thrown when a parameter passed to a C API function
    /// is out-of-bounds.
    #[display(fmt = "Configuration error")]
    ConfigError,
}

/// Custom result type for antenna operations
pub type AntSResult<T> = Result<T, AntsError>;

/// Trait used to represent the AntS object. Allows for mock objects to be created for unit tests
pub trait IAntS {
    /// Construct a new AntS instance
    fn new(bus: &str, primary: u8, secondary: u8, ant_count: u8, timeout: u32) -> AntSResult<Self>
    where
        Self: ::std::marker::Sized;
    /// Configure which microcontroller should be used to control the system
    fn configure(&self, config: KANTSController) -> AntSResult<()>;
    /// Perform a software reset of the microcontrollers
    fn reset(&self) -> AntSResult<()>;
    /// Arm the system for deployment
    fn arm(&self) -> AntSResult<()>;
    /// Disable deployment
    fn disarm(&self) -> AntSResult<()>;
    /// Deploy one antenna
    fn deploy(&self, antenna: KANTSAnt, force: bool, timeout: u8) -> AntSResult<()>;
    /// Automatically deploy all antennas
    fn auto_deploy(&self, timeout: u8) -> AntSResult<()>;
    /// Cancel all current deployment actions
    fn cancel_deploy(&self) -> AntSResult<()>;
    /// Get the current deployment status of the system
    fn get_deploy(&self) -> AntSResult<DeployStatus>;
    /// Get the system uptime
    fn get_uptime(&self) -> AntSResult<u32>;
    /// Get the system telemetry data
    fn get_system_telemetry(&self) -> AntSResult<AntsTelemetry>;
    /// Get an antenna's activation count
    fn get_activation_count(&self, antenna: KANTSAnt) -> AntSResult<u8>;
    /// Get the amount of time spent attempting to deploy an antenna
    fn get_activation_time(&self, antenna: KANTSAnt) -> AntSResult<u16>;
    /// Kick the hardware watchdog
    fn watchdog_kick(&self) -> AntSResult<()>;
    /// Start automatic watchdog kicking
    fn watchdog_start(&self) -> AntSResult<()>;
    /// Stop automatic watchdog kicking
    fn watchdog_stop(&self) -> AntSResult<()>;
    /// Pass a data packet directly through to the device
    fn passthrough(&self, tx: &[u8], rx_in: &mut [u8]) -> AntSResult<()>;
}

/// Structure for interacting with an ISIS Antenna System
pub struct AntS;

impl IAntS for AntS {
    /// Constructor
    ///
    /// Opens a connection to the underlying I<sup>2</sup>C device
    ///
    /// # Arguments
    ///
    /// * bus - The I<sup>2</sup>C bus to use to communicate with the device
    /// * primary - The I<sup>2</sup>C address of the system's primary microcontroller
    /// * secondary - The I<sup>2</sup>C address of the system's secondary microcontroller
    ///	  (should be `0x00` if no secondary microcontroller is available)
    /// * ant_count - The number of antennas present in the antenna system
    /// * timeout - The watchdog timeout interval, in seconds
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`AntsError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use isis_ants_api::*;
    ///
    /// # fn func() -> AntSResult<()> {
    /// let ants = AntS::new("KI2C1", 0x31, 0x32, 4, 10)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`AntsError`]: enum.AntsError.html
    fn new(bus: &str, primary: u8, secondary: u8, ant_count: u8, timeout: u32) -> AntSResult<AntS> {
        match unsafe { ffi::k_ants_init(convert_bus(bus), primary, secondary, ant_count, timeout) }
        {
            ffi::KANTSStatus::AntsOK => {
                if timeout > 0 {
                    match unsafe { ffi::k_ants_watchdog_start() } {
                        ffi::KANTSStatus::AntsOK => Ok(AntS),
                        ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
                        _ => Err(AntsError::GenericError.into()),
                    }
                } else {
                    Ok(AntS)
                }
            }
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    /// Configure the system to send future commands to the requested microcontroller
    ///
    /// # Arguments
    ///
    /// * config - The microcontroller which should be used for future commands to the antenna system
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`AntsError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use isis_ants_api::*;
    /// # fn func() -> AntSResult<()> {
    /// let ants = AntS::new("KI2C1", 0x31, 0x00, 2, 20)?;
    /// ants.configure(KANTSController::Secondary)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`AntsError`]: enum.AntsError.html
    fn configure(&self, config: KANTSController) -> AntSResult<()> {
        match unsafe { ffi::k_ants_configure(convert_controller(config)) } {
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    /// Reset both of the antenna's microcontrollers
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`AntsError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use isis_ants_api::*;
    /// # fn func() -> AntSResult<()> {
    /// let ants = AntS::new("KI2C1", 0x31, 0x32, 4, 10)?;
    /// ants.reset()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`AntsError`]: enum.AntsError.html
    fn reset(&self) -> AntSResult<()> {
        match unsafe { ffi::k_ants_reset() } {
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    /// Arm the antenna system for deployment
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`AntsError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use isis_ants_api::*;
    /// # fn func() -> AntSResult<()> {
    /// let ants = AntS::new("KI2C1", 0x31, 0x32, 4, 10)?;
    /// ants.arm()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`AntsError`]: enum.AntsError.html
    fn arm(&self) -> AntSResult<()> {
        match unsafe { ffi::k_ants_arm() } {
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    /// Disarm the antenna system
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`AntsError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use isis_ants_api::*;
    /// # fn func() -> AntSResult<()> {
    /// let ants = AntS::new("KI2C1", 0x31, 0x32, 4, 10)?;
    /// ants.disarm()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`AntsError`]: enum.AntsError.html
    fn disarm(&self) -> AntSResult<()> {
        match unsafe { ffi::k_ants_disarm() } {
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    /// Deploy a particular antenna
    ///
    /// # Arguments
    ///
    /// * antenna - The antenna to deploy
    /// * force - Whether the system should ignore previous successful deployment
    /// * timeout - The maximum time, in seconds, the system should spend deploying the antenna
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`AntsError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use isis_ants_api::*;
    /// # fn func() -> AntSResult<()> {
    /// let ants = AntS::new("KI2C1", 0x31, 0x00, 2, 20)?;
    /// ants.deploy(KANTSAnt::Ant2, false, 10)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`AntsError`]: enum.AntsError.html
    fn deploy(&self, antenna: KANTSAnt, force: bool, timeout: u8) -> AntSResult<()> {
        match unsafe { ffi::k_ants_deploy(convert_antenna(antenna), force, timeout) } {
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    /// Auto-deploy all antennas sequentially.
    ///
    /// # Arguments
    ///
    /// * timeout - The maximum time, in seconds, the system should spend deploying each antenna
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`AntsError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use isis_ants_api::*;
    /// # fn func() -> AntSResult<()> {
    /// let ants = AntS::new("KI2C1", 0x31, 0x00, 2, 20)?;
    /// ants.auto_deploy(5)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`AntsError`]: enum.AntsError.html
    fn auto_deploy(&self, timeout: u8) -> AntSResult<()> {
        match unsafe { ffi::k_ants_auto_deploy(timeout) } {
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    /// Cancel all current deployment actions
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`AntsError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use isis_ants_api::*;
    /// # fn func() -> AntSResult<()> {
    /// let ants = AntS::new("KI2C1", 0x31, 0x32, 4, 10)?;
    /// ants.cancel_deploy()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`AntsError`]: enum.AntsError.html
    fn cancel_deploy(&self) -> AntSResult<()> {
        match unsafe { ffi::k_ants_cancel_deploy() } {
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    /// Get the current deployment status
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`AntsError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use isis_ants_api::*;
    /// # fn func() -> AntSResult<()> {
    /// let ants = AntS::new("KI2C1", 0x31, 0x32, 4, 10)?;
    /// let deploy = ants.get_deploy()?;
    /// println!("Antenna 1 deployed: {}", !deploy.ant_1_not_deployed);
    /// println!("Antenna 2 deployment active: {}", deploy.ant_2_active);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`AntsError`]: enum.AntsError.html
    fn get_deploy(&self) -> AntSResult<DeployStatus> {
        let mut status: [u8; 2] = [0; 2];

        match unsafe { ffi::k_ants_get_deploy_status(status.as_mut_ptr()) } {
            ffi::KANTSStatus::AntsOK => {
                let decoded = DeployStatus::new(&status)?;
                Ok(decoded)
            }
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    /// Get the system's uptime
    ///
    /// Returns the systems uptime, in seconds
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`AntsError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use isis_ants_api::*;
    /// # fn func() -> AntSResult<()> {
    /// let ants = AntS::new("KI2C1", 0x31, 0x32, 4, 10)?;
    /// let uptime = ants.get_uptime()?;
    /// println!("Antenna system uptime: {}", uptime);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`AntsError`]: enum.AntsError.html
    fn get_uptime(&self) -> AntSResult<u32> {
        let mut uptime = 0;

        match unsafe { ffi::k_ants_get_uptime(&mut uptime) } {
            ffi::KANTSStatus::AntsOK => Ok(uptime),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    /// Get the current system telemetry
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`AntsError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use isis_ants_api::*;
    /// # fn func() -> AntSResult<()> {
    /// let ants = AntS::new("KI2C1", 0x31, 0x32, 4, 10)?;
    /// let sys_telem = ants.get_system_telemetry()?;
    ///
    /// println!("Antenna system telemetry:");
    /// println!("    raw_temp: {}", sys_telem.raw_temp);
    /// println!("    deploy_status:");
    /// println!("        Antenna 1 deployed: {}", !sys_telem.deploy_status.ant_1_not_deployed);
    /// println!("        Antenna 2 deployment active: {}", sys_telem.deploy_status.ant_2_active);
    /// println!("    uptime: {}\n", sys_telem.uptime);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`AntsError`]: enum.AntsError.html
    fn get_system_telemetry(&self) -> AntSResult<AntsTelemetry> {
        let mut c_telem = ffi::AntsTelemetry {
            raw_temp: 0,
            deploy_status: 0,
            uptime: 0,
        };

        match unsafe { ffi::k_ants_get_system_telemetry(&mut c_telem) } {
            ffi::KANTSStatus::AntsOK => {
                let telem = AntsTelemetry::new(c_telem)?;
                Ok(telem)
            }
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    /// Get an antenna's activation count
    ///
    /// # Arguments
    ///
    /// * antenna - Antenna to query
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`AntsError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use isis_ants_api::*;
    /// # fn func() -> AntSResult<()> {
    /// let ants = AntS::new("KI2C1", 0x31, 0x00, 2, 20)?;
    /// let act_count = ants.get_activation_count(KANTSAnt::Ant3)?;
    ///
    /// println!("Antenna 3 activation count - {}", act_count);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`AntsError`]: enum.AntsError.html
    fn get_activation_count(&self, antenna: KANTSAnt) -> AntSResult<u8> {
        let mut count: u8 = 0;

        match unsafe { ffi::k_ants_get_activation_count(convert_antenna(antenna), &mut count) } {
            ffi::KANTSStatus::AntsOK => Ok(count),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    /// Get an antenna's activation time
    ///
    /// Returns the total amount of time spent attempting to active the antenna, in seconds
    ///
    /// # Arguments
    ///
    /// * antenna - Antenna to query
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`AntsError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use isis_ants_api::*;
    /// # fn func() -> AntSResult<()> {
    /// let ants = AntS::new("KI2C1", 0x31, 0x00, 2, 20)?;
    /// let act_count = ants.get_activation_time(KANTSAnt::Ant1)?;
    ///
    /// println!("Antenna 1 activation time - {}", act_count);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`AntsError`]: enum.AntsError.html
    fn get_activation_time(&self, antenna: KANTSAnt) -> AntSResult<u16> {
        let mut time: u16 = 0;

        match unsafe { ffi::k_ants_get_activation_time(convert_antenna(antenna), &mut time) } {
            ffi::KANTSStatus::AntsOK => Ok(time),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    /// Kick both antenna system's watchdogs once
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`AntsError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use isis_ants_api::*;
    /// # fn func() -> AntSResult<()> {
    /// let ants = AntS::new("KI2C1", 0x31, 0x32, 4, 10)?;
    /// ants.watchdog_kick()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`AntsError`]: enum.AntsError.html
    fn watchdog_kick(&self) -> AntSResult<()> {
        match unsafe { ffi::k_ants_watchdog_kick() } {
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    /// Start a thread to kick the system's watchdogs at an interval of
    /// (timeout)/3 seconds
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`AntsError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use isis_ants_api::*;
    /// # fn func() -> AntSResult<()> {
    /// let ants = AntS::new("KI2C1", 0x31, 0x32, 4, 10)?;
    /// ants.watchdog_start()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`AntsError`]: enum.AntsError.html
    fn watchdog_start(&self) -> AntSResult<()> {
        match unsafe { ffi::k_ants_watchdog_start() } {
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    /// Stop the watchdog thread
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`AntsError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use isis_ants_api::*;
    /// # fn func() -> AntSResult<()> {
    /// let ants = AntS::new("KI2C1", 0x31, 0x32, 4, 10)?;
    /// ants.watchdog_start()?;
    /// //...
    /// ants.watchdog_stop()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`AntsError`]: enum.AntsError.html
    fn watchdog_stop(&self) -> AntSResult<()> {
        match unsafe { ffi::k_ants_watchdog_stop() } {
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }

    /// Pass a command packet directly through to the antenna system
    /// Useful for executing commands which have not been implemented in either the
    /// generic or specific antenna APIs.
    ///
    /// # Arguments
    ///
    /// * tx - Reference to byte array data to send
    /// * rx - Reference to byte array which returned data should be stored in
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`AntsError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use isis_ants_api::*;
    /// # fn func() -> AntSResult<()> {
    /// let ants = AntS::new("KI2C1", 0x31, 0x00, 2, 20)?;
    /// let tx: [u8; 1] = [0xC3];
    /// let mut rx: [u8; 2] = [0; 2];
    ///
    /// ants.passthrough(&tx, &mut rx).unwrap();
    /// println!("Antenna passthrough response: {:?}", rx);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`AntsError`]: enum.AntsError.html
    fn passthrough(&self, tx: &[u8], rx_in: &mut [u8]) -> AntSResult<()> {
        let tx_len: u8 = tx.len() as u8;
        let rx_len: u8 = rx_in.len() as u8;

        let rx: *mut u8 = match rx_len {
            0 => ptr::null_mut(),
            _ => rx_in.as_mut_ptr(),
        };

        match unsafe { ffi::k_ants_passthrough(tx.as_ptr(), tx_len, rx, rx_len) } {
            ffi::KANTSStatus::AntsOK => Ok(()),
            ffi::KANTSStatus::AntsErrorConfig => Err(AntsError::ConfigError.into()),
            _ => Err(AntsError::GenericError.into()),
        }
    }
}

/// Close the connection to the I2C bus
impl Drop for AntS {
    fn drop(&mut self) {
        let _ = self.watchdog_stop();
        unsafe { ffi::k_ants_terminate() }
    }
}
