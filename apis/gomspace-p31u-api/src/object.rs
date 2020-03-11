use crate::eps::*;
use crate::ffi;

/// EPS system configuration structure
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct EpsSystemConfig {
    /// MPPT mode: 0: Hardware default, 1: MPPT, 2 Fixed software powerpoint
    pub ppt_mode: u8,
    /// Batter heater mode: 0 = OFF, 1 = ON
    pub battheater_mode: u8,
    /// Battery heater low
    pub battheater_low: i8,
    /// Battery heater high
    pub battheater_high: i8,
    /// Nominal output level
    pub output_normal_value: [u8; 8],
    /// Safe model output level
    pub output_safe_value: [u8; 8],
    /// Initial power on delay
    pub output_initial_on_delay: [u16; 8],
    /// Initial Power off delay
    pub output_initial_off_delay: [u16; 8],
    /// Set MPPT level
    pub vboost: [u16; 3],
}

impl EpsSystemConfig {
    #[doc(hidden)]
    pub fn new(k_epssysconf: &ffi::EpsSystemConfig) -> Result<EpsSystemConfig, EpsError> {
        let epssysconf = EpsSystemConfig {
            ppt_mode: k_epssysconf.ppt_mode,
            battheater_mode: k_epssysconf.battheater_mode,
            battheater_low: k_epssysconf.battheater_low,
            battheater_high: k_epssysconf.battheater_high,
            output_normal_value: k_epssysconf.output_normal_value,
            output_safe_value: k_epssysconf.output_safe_value,
            output_initial_on_delay: k_epssysconf.output_initial_on_delay,
            output_initial_off_delay: k_epssysconf.output_initial_off_delay,
            vboost: k_epssysconf.vboost,
        };
        Ok(epssysconf)
    }
}

/// EPS battery configuration structure
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct EpsBatteryConfig {
    /// Battery Max level
    pub batt_maxvoltage: u16,
    /// Safe mode level
    pub batt_safevoltage: u16,
    /// Critical mode level
    pub batt_criticalvoltage: u16,
    /// Nominal mode level
    pub batt_normalvoltage: u16,
    /// reserved1
    pub reserved1: [u32; 2],
    /// reserved2
    pub reserved2: [u8; 4],
}

impl EpsBatteryConfig {
    #[doc(hidden)]
    pub fn new(k_epsbatconf: &ffi::EpsBatteryConfig) -> Result<EpsBatteryConfig, EpsError> {
        let epsbatconf = EpsBatteryConfig {
            batt_maxvoltage: k_epsbatconf.batt_maxvoltage,
            batt_safevoltage: k_epsbatconf.batt_safevoltage,
            batt_criticalvoltage: k_epsbatconf.batt_criticalvoltage,
            batt_normalvoltage: k_epsbatconf.batt_normalvoltage,
            reserved1: k_epsbatconf.reserved1,
            reserved2: k_epsbatconf.reserved2,
        };

        Ok(epsbatconf)
    }
}

/// System telemetry fields returned from [`EpsHk`]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct EpsHk {
    /// MPPT level
    pub vboost: [u16; 3],
    /// Battery Voltage
    pub vbatt: u16,
    /// Battery input current
    pub curin: [u16; 3],
    /// Sun sensor current
    pub cursun: u16,
    /// Total system current
    pub cursys: u16,
    /// Reserved1
    pub reserved1: u16,
    /// Current outputs
    pub curout: [u16; 6],
    /// output status
    pub output: [u8; 8],
    /// output on delay
    pub output_on_delta: [u16; 8],
    /// output off delay
    pub output_off_delta: [u16; 8],
    /// Current latchup setting
    pub latchup: [u16; 6],
    /// i2c watchdog time left
    pub wdt_i2c_time_left: u32,
    /// GND watchdog time left
    pub wdt_gnd_time_left: u32,
    /// CSP watchdog
    pub wdt_csp_pings_left: [u8; 2],
    /// i2c watchdog counter
    pub counter_wdt_i2c: u32,
    /// GND watchdog counter    
    pub counter_wdt_gnd: u32,
    /// CSP watchdog counter
    pub counter_wdt_csp: [u32; 2],
    /// Boot counter
    pub counter_boot: u32,
    /// Temperature
    pub temp: [i16; 6],
    /// Bootcause
    pub boot_cause: u8,
    /// Battery mode
    pub batt_mode: u8,
    /// Power point tracking mode
    pub ppt_mode: u8,
    /// Reserved2
    pub reserved2: u16,
}

impl EpsHk {
    #[doc(hidden)]
    pub fn new(k_epshk: &ffi::EpsHk) -> Result<EpsHk, EpsError> {
        let epshk = EpsHk {
            vboost: k_epshk.vboost,
            vbatt: k_epshk.vbatt,
            curin: k_epshk.curin,
            cursun: k_epshk.cursun,
            cursys: k_epshk.cursys,
            reserved1: k_epshk.reserved1,
            curout: k_epshk.curout,
            output: k_epshk.output,
            output_on_delta: k_epshk.output_on_delta,
            output_off_delta: k_epshk.output_off_delta,
            latchup: k_epshk.latchup,
            wdt_i2c_time_left: k_epshk.wdt_i2c_time_left,
            wdt_gnd_time_left: k_epshk.wdt_gnd_time_left,
            wdt_csp_pings_left: k_epshk.wdt_csp_pings_left,
            counter_wdt_i2c: k_epshk.counter_wdt_i2c,
            counter_wdt_gnd: k_epshk.counter_wdt_gnd,
            counter_wdt_csp: k_epshk.counter_wdt_csp,
            counter_boot: k_epshk.counter_boot,
            temp: k_epshk.temp,
            boot_cause: k_epshk.boot_cause,
            batt_mode: k_epshk.batt_mode,
            ppt_mode: k_epshk.ppt_mode,
            reserved2: k_epshk.reserved2,
        };
        Ok(epshk)
    }
}
