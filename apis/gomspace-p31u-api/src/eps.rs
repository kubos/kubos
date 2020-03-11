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
//use std::error::Error;
use std::ptr;
//use std::io;

/// EpsError
///
/// Describes various errors which may result from using EPS APIs
#[derive(Fail, Debug, Clone)]
pub enum EpsError {
    /// Generic error condition => EPS_ERROR
    #[fail(display = "Generic Error")]
    GenericError,
    /// Configuration error. Thrown when a parameter passed to a C API function
    /// is out-of-bounds => EPS_ERROR_CONFIG
    #[fail(display = "Configuration error")]
    ConfigError,

    /// Error resulting from underlying Io functions
    #[fail(display = "I2C error")]
    I2cError,

    /// Error resulting from underlying Subsystem
    #[fail(display = "EPS internal error")]
    InternalError,
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
    fn set_single_output(&self, channel: u8, value: u8, delay: u16) -> EpsResult<()>;
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
    fn set_heater(&self, cmd: u8, heater: u8, mode: u8) -> EpsResult<()>;
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
    // fn passthrough(&self,tx:*mut u8,tx_len:i32,rx:*mut u8,rx_len:i32) -> EpsResult<()>;
    //	fn prv_transfer(&self, tx:*const u8,tx_len:i32,rx:*mut u8,rx_len:i32) -> EpsResult<()>;
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
        match unsafe { ffi::k_eps_init(k_config) } {
            ffi::KEPSStatus::EpsOk => Ok(Eps),
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
            _ => Err(EpsError::GenericError),
        }
    }

    /// Ping the EPS. Send a cmd (1) to the eps.
    /// Expect the same command returned by the EPS

    fn ping(&self) -> EpsResult<()> {
        //println!("before ping, eps.rs");
        match unsafe { ffi::k_eps_ping() } {
            ffi::KEPSStatus::EpsOk => Ok(()),
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            _ => Err(EpsError::GenericError),
        }
    }

    /// Hard reset the EPS's microcontrollers
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    ///
    fn reset(&self) -> EpsResult<()> {
        match unsafe { ffi::k_eps_reset() } {
            ffi::KEPSStatus::EpsOk => Ok(()),
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            _ => Err(EpsError::GenericError),
        }
    }

    /// Soft reset the EPS's microcontrollers
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.
    ///
    fn reboot(&self) -> EpsResult<()> {
        match unsafe { ffi::k_eps_reboot() } {
            ffi::KEPSStatus::EpsOk => Ok(()),
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            _ => Err(EpsError::GenericError),
        }
    }

    /// System Configuration (conf)
    ///
    /// Set the system configuration
    ///
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
    fn configure_system(&self, config: EpsSystemConfig) -> EpsResult<()> {
        //let mut epssysconf =ffi:: EpsSystemConfig::default();
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

        match unsafe { ffi::k_eps_configure_system(&epssysconf) } {
            ffi::KEPSStatus::EpsOk => Ok(()),
            // {
            // let sysconf = EpsSystemConfig :: new(&mut epssysconf)?;
            // Ok(sysconf)
            // }
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
            ffi::KEPSStatus::EpsErrorInternal => Err(EpsError::InternalError),
            _ => Err(EpsError::GenericError),
        }
    }

    /// Battery Configuration (conf2)
    ///
    /// Set the battery configuration
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

    // fn configure_battery(&self,  config: EpsBatteryConfig) -> EpsResult<()>{
    //     Ok(())
    // }
    fn configure_battery(&self, config: EpsBatteryConfig) -> EpsResult<()> {
        //  let mut epsbattconf = ffi::EpsBatteryConfig::default();
        let epsbatconf = ffi::EpsBatteryConfig {
            batt_maxvoltage: config.batt_maxvoltage,
            batt_safevoltage: config.batt_safevoltage,
            batt_criticalvoltage: config.batt_criticalvoltage,
            batt_normalvoltage: config.batt_normalvoltage,
            reserved1: config.reserved1,
            reserved2: config.reserved2,
        };
        match unsafe { ffi::k_eps_configure_battery(&epsbatconf) } {
            ffi::KEPSStatus::EpsOk => Ok(()),
            // {
            //     let battconf = EpsBatteryConfig :: new (&mut epsbattconf)?;
            //     Ok(battconf)
            // },
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
            ffi::KEPSStatus::EpsErrorInternal => Err(EpsError::InternalError),
            _ => Err(EpsError::GenericError),
        }
    }

    /// Save Battery Configuration
    ///
    /// Save the battery configuration, must be done after battery configuration
    /// or it will fall back after a while
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.

    fn save_battery_config(&self) -> EpsResult<()> {
        match unsafe { ffi::k_eps_save_battery_config() } {
            ffi::KEPSStatus::EpsOk => Ok(()),
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
            ffi::KEPSStatus::EpsErrorInternal => Err(EpsError::InternalError),
            _ => Err(EpsError::GenericError),
        }
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
        match unsafe { ffi::k_eps_set_output(channel_mask) } {
            ffi::KEPSStatus::EpsOk => Ok(()),
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
            ffi::KEPSStatus::EpsErrorInternal => Err(EpsError::InternalError),
            _ => Err(EpsError::GenericError),
        }
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

    fn set_single_output(&self, channel: u8, value: u8, delay: u16) -> EpsResult<()> {
        match unsafe { ffi::k_eps_set_single_output(channel, value, delay) } {
            ffi::KEPSStatus::EpsOk => Ok(()),
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
            ffi::KEPSStatus::EpsErrorInternal => Err(EpsError::InternalError),
            _ => Err(EpsError::GenericError),
        }
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
        match unsafe { ffi::k_eps_set_input_value(in1_voltage, in2_voltage, in3_voltage) } {
            ffi::KEPSStatus::EpsOk => Ok(()),
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
            ffi::KEPSStatus::EpsErrorInternal => Err(EpsError::InternalError),
            _ => Err(EpsError::GenericError),
        }
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
        match unsafe { ffi::k_eps_set_input_mode(mode) } {
            ffi::KEPSStatus::EpsOk => Ok(()),
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
            ffi::KEPSStatus::EpsErrorInternal => Err(EpsError::InternalError),
            _ => Err(EpsError::GenericError),
        }
    }

    /// Set heater ON/OFF
    ///
    /// Cmd = 0: Set heater on/off
    /// Heater: 0 = BP4, 1= Onboard, 2 = Both
    /// Mode: 0 = OFF, 1 = ON
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.

    fn set_heater(&self, cmd: u8, heater: u8, mode: u8) -> EpsResult<()> {
        match unsafe { ffi::k_eps_set_heater(cmd, heater, mode) } {
            ffi::KEPSStatus::EpsOk => Ok(()),
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
            ffi::KEPSStatus::EpsErrorInternal => Err(EpsError::InternalError),
            _ => Err(EpsError::GenericError),
        }
    }

    /// Reset the EPS configuration to default
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.

    fn reset_system_config(&self) -> EpsResult<()> {
        match unsafe { ffi::k_eps_reset_system_config() } {
            ffi::KEPSStatus::EpsOk => Ok(()),
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
            ffi::KEPSStatus::EpsErrorInternal => Err(EpsError::InternalError),
            _ => Err(EpsError::GenericError),
        }
    }

    /// Reset the Battery configuration to default
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.

    fn reset_battery_config(&self) -> EpsResult<()> {
        match unsafe { ffi::k_eps_reset_battery_config() } {
            ffi::KEPSStatus::EpsOk => Ok(()),
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
            ffi::KEPSStatus::EpsErrorInternal => Err(EpsError::InternalError),
            _ => Err(EpsError::GenericError),
        }
    }

    /// Reset boot counter and WDT counters (excluding the dedicated WDT)
    ///
    /// magic = 0x78
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.

    fn reset_counters(&self) -> EpsResult<()> {
        match unsafe { ffi::k_eps_reset_counters() } {
            ffi::KEPSStatus::EpsOk => Ok(()),
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
            ffi::KEPSStatus::EpsErrorInternal => Err(EpsError::InternalError),
            _ => Err(EpsError::GenericError),
        }
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

        match unsafe { ffi::k_eps_get_housekeeping(&mut buff) } {
            ffi::KEPSStatus::EpsOk => {
                let epshk = EpsHk::new(&buff)?;
                Ok(epshk)
            }
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
            ffi::KEPSStatus::EpsErrorInternal => Err(EpsError::InternalError),
            _ => Err(EpsError::GenericError),
        }
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

        match unsafe { ffi::k_eps_get_system_config(&mut config) } {
            ffi::KEPSStatus::EpsOk => {
                let epssysconf = EpsSystemConfig::new(&config)?;
                Ok(epssysconf)
            }
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
            ffi::KEPSStatus::EpsErrorInternal => Err(EpsError::InternalError),
            _ => Err(EpsError::GenericError),
        }
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

        match unsafe { ffi::k_eps_get_battery_config(&mut config) } {
            ffi::KEPSStatus::EpsOk => {
                let epsbatconf = EpsBatteryConfig::new(&config)?;
                Ok(epsbatconf)
            }
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
            ffi::KEPSStatus::EpsErrorInternal => Err(EpsError::InternalError),
            _ => Err(EpsError::GenericError),
        }
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

        match unsafe { ffi::k_eps_get_heater(&mut bp4, &mut onboard) } {
            ffi::KEPSStatus::EpsOk => Ok(onboard as i32),
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
            ffi::KEPSStatus::EpsErrorInternal => Err(EpsError::InternalError),
            _ => Err(EpsError::GenericError),
        }
    }

    /// Kick Watchdog
    ///
    /// Send this command to reset (kick) dedicated WDT.
    /// magic = 0x78
    ///
    /// # Errors
    /// If this function encounters any errors, an [`EpsError`] variant will be returned.

    fn watchdog_kick(&self) -> EpsResult<()> {
        match unsafe { ffi::k_eps_watchdog_kick() } {
            ffi::KEPSStatus::EpsOk => Ok(()),
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
            ffi::KEPSStatus::EpsErrorInternal => Err(EpsError::InternalError),
            _ => Err(EpsError::GenericError),
        }
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

        match unsafe { ffi::k_eps_passthrough(tx.as_ptr(), tx_len, rx_in, rx_len) } {
            ffi::KEPSStatus::EpsOk => Ok(()),
            ffi::KEPSStatus::EpsI2CError => Err(EpsError::I2cError),
            ffi::KEPSStatus::EpsErrorConfig => Err(EpsError::ConfigError),
            ffi::KEPSStatus::EpsErrorInternal => Err(EpsError::InternalError),
            _ => Err(EpsError::GenericError),
        }
    }
}
