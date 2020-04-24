//
// Copyright (C) 2020 Kubos Corporation
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

// The API wrapper is contributed by Xueliang Bai <x.bai@sydney.edu.au> on behalf of the
// ARC Training Centre for CubeSats, UAVs & Their Applications (CUAVA) team (www.cuava.com.au)
// at the University of Sydney

//! Kubos API wrapper for interacting with [GomSpace p31u EPS]

/*
 * Note that the conf save function is not implemented
 * so the EPS's default seting can't be reconfigured in orbit.
 * A default setting need to be configured in EPS using the GomSpace shell (GOSH) before launch.
 * Note that if the dedicated WDT times out,
 * the default config is restored on NanoPower
 *
*/

use crate::ffi;
use crate::object::*;
use failure::Fail;
use std::ptr;

/// EpsError
///
/// Describes various errors which may result from using EPS APIs
#[derive(Fail, Debug, Clone)]
pub enum EpsError {
    /// Generic error condition => EPS_ERROR
    #[fail(display = "Generic Error")]
    GenericError,
    /// Configuration error. Thrown when a parameter passed to a C API function
    #[fail(display = "Configuration error")]
    ConfigError,

    /// Error resulting from underlying Io functions
    #[fail(display = "I2C error")]
    I2cError,

    /// Error resulting from underlying Subsystem
    #[fail(display = "EPS internal error")]
    InternalError,
}

/// Power state enum
#[derive(Clone, Debug, GraphQLEnum, PartialEq)]
pub enum EpsPowerState {
    /// Power Off
    Off,
    /// Power On
    On,
}

fn powerstate_to_u8(powerstate: EpsPowerState) -> u8 {
    match powerstate {
        EpsPowerState::Off => 0,
        EpsPowerState::On => 1,
    }
}

/// Enum for EPS power channels
#[derive(Clone, Debug, GraphQLEnum, PartialEq)]
pub enum EpsChannels {
    /// EPS channel 0 => H1-47
    Output0,
    /// EPS channel 1 => H1-49	    
    Output1,
    /// EPS channel 2 => H1-51	    
    Output2,
    /// EPS channel 3 => H1-48    
    Output3,
    /// EPS channel 4 => H1-50
    Output4,
    /// EPS channel 5 => H1-52    
    Output5,
    /// BP4 heater switch
    Output6,
    /// BP4 switch
    Output7,
}

fn epschn_to_u8(epschn: EpsChannels) -> u8 {
    match epschn {
        EpsChannels::Output0 => 0,
        EpsChannels::Output1 => 1,
        EpsChannels::Output2 => 2,
        EpsChannels::Output3 => 3,
        EpsChannels::Output4 => 4,
        EpsChannels::Output5 => 5,
        EpsChannels::Output6 => 6,
        EpsChannels::Output7 => 7,
    }
}

///Enum for heater selection
#[derive(Clone, Debug, GraphQLEnum, PartialEq)]
pub enum HeaterSelect {
    ///Heater on BP4
    BP4,
    ///Heater on EPS
    Onboard,
    ///Both
    Both,
}

fn heater_sel_to_u8(heater_sel: HeaterSelect) -> u8 {
    match heater_sel {
        HeaterSelect::BP4 => 0,
        HeaterSelect::Onboard => 1,
        HeaterSelect::Both => 2,
    }
}

/// Convenience function converting KEPSStatus to Result
fn convert_status(status: ffi::KEPSStatus) -> Result<(), EpsError> {
    match status {
        ffi::KEPSStatus::EpsOk => Ok(()),
        ffi::KEPSStatus::EpsError => Err(EpsError::GenericError),
        ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
        ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
        ffi::KEPSStatus::EpsErrorInternal => Err(EpsError::InternalError),
    }
}

/// Universal return type for EPS api functions
pub type EpsResult<T> = Result<T, EpsError>;

/// Trait used to represent the GsEps object. Allows for mock objects to be created for unit tests
pub trait GsEps: Send {
    /// Construct a new GsEps instance
    fn new(bus: &str, addr: u8) -> EpsResult<Self>
    where
        Self: ::std::marker::Sized;
    /// Ping subsystems
    fn ping(&self) -> EpsResult<()>;
    /// Reset
    fn reset(&self) -> EpsResult<()>;
    /// Reboot
    fn reboot(&self) -> EpsResult<()>;
    /// System configuration
    fn configure_system(&self, config: EpsSystemConfig) -> EpsResult<()>;
    /// Battery configuration
    fn configure_battery(&self, config: EpsBatteryConfig) -> EpsResult<()>;
    /// Battery configuration save
    fn save_battery_config(&self) -> EpsResult<()>;
    /// Batch set EPS outputs
    fn set_output(&self, channel_mask: u8) -> EpsResult<()>;
    ///Set a channel on/off
    fn set_single_output(
        &self,
        channel: EpsChannels,
        value: EpsPowerState,
        delay: u16,
    ) -> EpsResult<()>;
    /// Set MPPT input level
    fn set_input_value(
        &self,
        in1_voltage: u16,
        in2_voltage: u16,
        in3_voltage: u16,
    ) -> EpsResult<()>;
    /// Set the MPPT mode
    fn set_input_mode(&self, mode: u8) -> EpsResult<()>;
    /// Set heater configuration
    fn set_heater(&self, heater: HeaterSelect, mode: EpsPowerState) -> EpsResult<()>;
    /// Reset system configuration
    fn reset_system_config(&self) -> EpsResult<()>;
    /// Reset battery configuration
    fn reset_battery_config(&self) -> EpsResult<()>;
    /// Reset conuters
    fn reset_counters(&self) -> EpsResult<()>;
    /// Get the telemetry data
    fn get_housekeeping(&self) -> EpsResult<(EpsHk)>;
    /// Get the system configuration
    fn get_system_config(&self) -> EpsResult<(EpsSystemConfig)>;
    /// Get battery configuration
    fn get_battery_config(&self) -> EpsResult<(EpsBatteryConfig)>;
    /// Get the heater status
    fn get_heater(&self) -> EpsResult<i32>;
    /// Kick the hardware watchdog
    fn watchdog_kick(&self) -> EpsResult<()>;
    /// Pass a data packet directly through to the device
    fn passthrough(&self, tx: &[u8], rx: &mut [u8]) -> EpsResult<()>;
}

/// Structure for interacting with a GomSpace EPS System
pub struct Eps;

impl GsEps for Eps {
    /// Constructor
    ///
    /// Opens a connection to the underlying I2C device
    ///
    /// # Arguments
    ///
    /// * bus - The I2C bus to use to communicate with the device
    /// * I2C_addr - The I2C  address of the EPS, Default 0x08
    /// * WD_timeout - The watchdog timeout interval, in seconds
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    fn new(bus: &str, addr: u8) -> EpsResult<Eps> {
        let k_config = ffi::KEPSConf {
            k_bus: bus.as_ptr(),
            k_addr: addr,
        };

        convert_status(unsafe { ffi::k_eps_init(k_config) })?;
        Ok(Eps)
    }

    /// Ping the EPS. Send a cmd (1) to the eps.
    /// Expect the same command returned by the EPS

    fn ping(&self) -> EpsResult<()> {
        convert_status(unsafe { ffi::k_eps_ping() })
    }

    /// Hard reset the EPS's microcontrollers
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    ///
    fn reset(&self) -> EpsResult<()> {
        convert_status(unsafe { ffi::k_eps_reset() })
    }

    /// Soft reset the EPS's microcontrollers
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    ///
    fn reboot(&self) -> EpsResult<()> {
        convert_status(unsafe { ffi::k_eps_reboot() })
    }

    /// System Configuration (conf)
    ///
    /// Set the system configuration
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    fn configure_system(&self, config: EpsSystemConfig) -> EpsResult<()> {
        let epssysconf = ffi::EpsSystemConfig {
            ppt_mode: config.ppt_mode,
            battheater_mode: config.battheater_mode,
            battheater_low: config.battheater_low,
            battheater_high: config.battheater_high,
            output_normal_value: config.output_normal_value,
            output_safe_value: config.output_safe_value,
            output_initial_on_delay: config.output_initial_on_delay,
            output_initial_off_delay: config.output_initial_off_delay,
            vboost: config.vboost,
        };
        convert_status(unsafe { ffi::k_eps_configure_system(&epssysconf) })
    }

    /// Battery Configuration (conf2)
    ///
    /// Set the battery configuration
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    fn configure_battery(&self, config: EpsBatteryConfig) -> EpsResult<()> {
        let epsbatconf = ffi::EpsBatteryConfig {
            batt_maxvoltage: config.batt_maxvoltage,
            batt_safevoltage: config.batt_safevoltage,
            batt_criticalvoltage: config.batt_criticalvoltage,
            batt_normalvoltage: config.batt_normalvoltage,
            reserved1: config.reserved1,
            reserved2: config.reserved2,
        };
        convert_status(unsafe { ffi::k_eps_configure_battery(&epsbatconf) })
    }

    /// Save Battery Configuration
    ///
    /// Save the battery configuration, must be done after battery configuration
    /// or it will fall back after a while
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    fn save_battery_config(&self) -> EpsResult<()> {
        convert_status(unsafe { ffi::k_eps_save_battery_config() })
    }

    /// Batch set EPS output
    ///
    ///
    /// Set output switch states by a bitmask where "1"
    /// means the channel is switched on and "0" means
    /// it is switched off. LSB is channel 1, next bit is
    /// channel 2 etc. (Quadbat switch and heater cannot
    /// be controlled through this command)
    /// [NC NC 3.3V3 3.3V2 3.3V1 5V3 5V2 5V1]
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    fn set_output(&self, channel_mask: u8) -> EpsResult<()> {
        convert_status(unsafe { ffi::k_eps_set_output(channel_mask) })
    }

    /// Set single EPS output
    ///
    /// Set output %channel% to value %value% with
    /// delay %delay%, Channel (0-5), Quadbat heater
    /// (6), Quadbat switch (7)
    /// Value 0 = Off, 1 = On
    /// Delay in seconds.
    ///
    /// Example: set_single_output(0, 1, 0)
    /// This will set channel 0 to be ON with no delay
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    fn set_single_output(
        &self,
        channel: EpsChannels,
        value: EpsPowerState,
        delay: u16,
    ) -> EpsResult<()> {
        let epschn = epschn_to_u8(channel);
        let powerstate = powerstate_to_u8(value);
        convert_status(unsafe { ffi::k_eps_set_single_output(epschn, powerstate, delay) })
    }

    /// Set the MPPT value for each channel
    ///
    /// Set the voltage on the photo-voltaic inputs V1, V2, V3 in mV.
    /// Takes effect when MODE = 2, See SET_PV_AUTO.
    /// Transmit voltage1 first and voltage3 last.
    ///
    /// The voltage needs to changed to align with the number of solar panels
    /// you have on each channel
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    fn set_input_value(
        &self,
        in1_voltage: u16,
        in2_voltage: u16,
        in3_voltage: u16,
    ) -> EpsResult<()> {
        convert_status(unsafe { ffi::k_eps_set_input_value(in1_voltage, in2_voltage, in3_voltage) })
    }

    /// Set the MPPT mode
    ///
    /// Sets the solar cell power tracking mode:
    /// MODE = 0: Hardware default power point
    /// MODE = 1: Maximum power point tracking
    /// MODE = 2: Fixed software powerpoint, value set with SET_PV_VOLT, default 4V
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    fn set_input_mode(&self, mode: u8) -> EpsResult<()> {
        convert_status(unsafe { ffi::k_eps_set_input_mode(mode) })
    }

    /// Set heater ON/OFF
    ///
    /// Cmd = 0: Set heater on/off
    /// Heater: 0 = BP4, 1= Onboard, 2 = Both
    /// Mode: 0 = OFF, 1 = ON
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    fn set_heater(&self, heater: HeaterSelect, mode: EpsPowerState) -> EpsResult<()> {
        let cmd = 0;
        let heater_sel = heater_sel_to_u8(heater);
        let powerstate = powerstate_to_u8(mode);
        convert_status(unsafe { ffi::k_eps_set_heater(cmd, heater_sel, powerstate) })
    }

    /// Reset the EPS configuration to default
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    fn reset_system_config(&self) -> EpsResult<()> {
        convert_status(unsafe { ffi::k_eps_reset_system_config() })
    }

    /// Reset the Battery configuration to default
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    fn reset_battery_config(&self) -> EpsResult<()> {
        convert_status(unsafe { ffi::k_eps_reset_battery_config() })
    }

    /// Reset boot counter and WDT counters (excluding the dedicated WDT)
    ///
    /// magic = 0x78
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    fn reset_counters(&self) -> EpsResult<()> {
        convert_status(unsafe { ffi::k_eps_reset_counters() })
    }

    /// Get Housekeeping data
    ///
    /// HK structure:
    ///
    /// vboost[3]             : Voltage of input voltage boost converters [mV]
    /// vbatt                 : Voltage of battery [mV]  
    /// curin[3]              : Input currents [mA]  
    /// cursun                : Current from boost converters [mA]  
    /// cursys                : Current out of battery [mA]  
    /// reserved1             : Reserved for future use  
    /// curout[6]             : Output currents [mA]  
    ///  output[8]             : Output statuses [0 = Off, 1 = On]  
    /// output_on_delta[8]    : Time until output power on [seconds]  
    /// output_off_delta[8]   : Time until output power off [seconds]  
    /// latchup[6]            : Number of output latch-up events  
    /// wdt_i2c_time_left     : Time left for I2C watchdog [seconds]  
    /// wdt_gnd_time_left     : Time left for dedicated watchdog [seconds]  
    ///  wdt_csp_pings_left[2] : Pings left for CSP watchdog  
    /// counter_wdt_i2c       : Number of I2C watchdog reboots  
    /// counter_wdt_gnd       : Number of dedicated watchdog reboots  
    /// counter_wdt_csp[2]    : Number of CSP watchdog reboots  
    /// counter_boot          : Number of EPS reboots  
    ///  temp[6]               : Temperatures [degC] [0 = Temp1, Temp2, Temp3, Temp4, BP4a, BP4b]  
    ///  boot_cause            : Cause of last EPS reset  
    ///  batt_mode             : Mode for battery [0 = Initial, 1 = Critical, 2 = Safe, 3 = Normal, 4 = Full]  
    ///  ppt_mode              : Mode of power-point tracker [1 = Automatic maximum, 2 = Fixed]  
    /// reserved2             : Reserved  
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    fn get_housekeeping(&self) -> EpsResult<(EpsHk)> {
        let mut buff = ffi::EpsHk::default();

        convert_status(unsafe { ffi::k_eps_get_housekeeping(&mut buff) })?;
        Ok(EpsHk::new(&buff)?)
    }

    /// Query the system configuration (conf)
    ///
    /// # Arguments
    ///
    /// pub ppt_mode: u8,
    /// pub battheater_mode: u8,
    /// pub battheater_low:i8,
    /// pub battheater_high:i8,
    /// pub output_normal_value:[u8;8],
    /// pub output_safe_value:[u8;8],
    /// pub output_initial_on_delay:[u16;8],
    /// pub output_initial_off_delay:[u16;8],
    /// pub vboost:[u16;3],
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    fn get_system_config(&self) -> EpsResult<(EpsSystemConfig)> {
        let mut config = ffi::EpsSystemConfig::default();

        convert_status(unsafe { ffi::k_eps_get_system_config(&mut config) })?;
        Ok(EpsSystemConfig::new(&config)?)
    }

    /// Get the battery configuration (conf2)
    ///
    /// # Arguments     
    ///
    /// batt_maxvoltage: 0,
    /// batt_safevoltage: 0,
    /// batt_criticalvoltage:0,
    /// batt_normalvoltage:0,
    /// reserved1:[0;2],
    /// reserved2:[0;4],
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    fn get_battery_config(&self) -> EpsResult<(EpsBatteryConfig)> {
        let mut config = ffi::EpsBatteryConfig::default();

        convert_status(unsafe { ffi::k_eps_get_battery_config(&mut config) })?;
        Ok(EpsBatteryConfig::new(&config)?)
    }

    /// Get heater status
    ///
    /// Command replies with heater modes. 0=OFF, 1=ON.
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    fn get_heater(&self) -> EpsResult<i32> {
        let mut bp4: u8 = 0;
        let mut onboard: u8 = 0;
        convert_status(unsafe { ffi::k_eps_get_heater(&mut bp4, &mut onboard) })?;
        Ok(onboard as i32)
    }

    /// Kick Watchdog
    ///
    /// Send this command to reset (kick) dedicated WDT.
    /// magic = 0x78
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    fn watchdog_kick(&self) -> EpsResult<()> {
        convert_status(unsafe { ffi::k_eps_watchdog_kick() })
    }

    /// Via function. Pass the infromation through
    ///      
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    fn passthrough(&self, tx: &[u8], rx: &mut [u8]) -> EpsResult<()> {
        let tx_len: u8 = tx.len() as u8;
        let rx_len: u8 = rx.len() as u8;

        let rx_in: *mut u8 = match rx_len {
            0 => ptr::null_mut(),
            _ => rx.as_mut_ptr(),
        };
        convert_status(unsafe { ffi::k_eps_passthrough(tx.as_ptr(), tx_len, rx_in, rx_len) })
    }
}
