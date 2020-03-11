#[allow(dead_code)]
#[repr(C)]
pub enum KEPSStatus {
    EpsOk,
    EpsError,
    EpsI2CError,
    EpsErrorConfig,
    EpsErrorInternal,
}

#[repr(C)]
pub struct KEPSConf {
    pub k_bus: *const u8,
    pub k_addr: u8,
}

#[repr(C)]
#[derive(Default)]
pub struct EpsSystemConfig {
    pub ppt_mode: u8,
    pub battheater_mode: u8,
    pub battheater_low: i8,
    pub battheater_high: i8,
    pub output_normal_value: [u8; 8],
    pub output_safe_value: [u8; 8],
    pub output_initial_on_delay: [u16; 8],
    pub output_initial_off_delay: [u16; 8],
    pub vboost: [u16; 3],
}
#[repr(C)]
#[derive(Default)]
pub struct EpsBatteryConfig {
    pub batt_maxvoltage: u16,
    pub batt_safevoltage: u16,
    pub batt_criticalvoltage: u16,
    pub batt_normalvoltage: u16,
    pub reserved1: [u32; 2],
    pub reserved2: [u8; 4],
}

#[repr(C)]
#[derive(Default)]
pub struct EpsHk {
    pub vboost: [u16; 3],
    pub vbatt: u16,
    pub curin: [u16; 3],
    pub cursun: u16,
    pub cursys: u16,
    pub reserved1: u16,
    pub curout: [u16; 6],
    pub output: [u8; 8],
    pub output_on_delta: [u16; 8],
    pub output_off_delta: [u16; 8],
    pub latchup: [u16; 6],
    pub wdt_i2c_time_left: u32,
    pub wdt_gnd_time_left: u32,
    pub wdt_csp_pings_left: [u8; 2],
    pub counter_wdt_i2c: u32,
    pub counter_wdt_gnd: u32,
    pub counter_wdt_csp: [u32; 2],
    pub counter_boot: u32,
    pub temp: [i16; 6],
    pub boot_cause: u8,
    pub batt_mode: u8,
    pub ppt_mode: u8,
    pub reserved2: u16,
}

extern "C" {
    pub fn k_eps_init(config: KEPSConf) -> KEPSStatus;
    pub fn k_eps_ping() -> KEPSStatus;
    pub fn k_eps_reset() -> KEPSStatus;
    pub fn k_eps_reboot() -> KEPSStatus;
    pub fn k_eps_configure_system(config: *const EpsSystemConfig) -> KEPSStatus;
    pub fn k_eps_configure_battery(config: *const EpsBatteryConfig) -> KEPSStatus;
    pub fn k_eps_save_battery_config() -> KEPSStatus;
    pub fn k_eps_set_output(channel_mask: u8) -> KEPSStatus;
    pub fn k_eps_set_single_output(channel: u8, value: u8, delay: u16) -> KEPSStatus;
    pub fn k_eps_set_input_value(
        in1_voltage: u16,
        in2_voltage: u16,
        in3_voltage: u16,
    ) -> KEPSStatus;
    pub fn k_eps_set_input_mode(mode: u8) -> KEPSStatus;
    pub fn k_eps_set_heater(cmd: u8, heater: u8, mode: u8) -> KEPSStatus;
    pub fn k_eps_reset_system_config() -> KEPSStatus;
    pub fn k_eps_reset_battery_config() -> KEPSStatus;
    pub fn k_eps_reset_counters() -> KEPSStatus;
    pub fn k_eps_get_housekeeping(buff: *mut EpsHk) -> KEPSStatus;
    pub fn k_eps_get_system_config(buff: *mut EpsSystemConfig) -> KEPSStatus;
    pub fn k_eps_get_battery_config(buff: *mut EpsBatteryConfig) -> KEPSStatus;
    pub fn k_eps_get_heater(bp4: *mut u8, onboard: *mut u8) -> KEPSStatus;
    pub fn k_eps_watchdog_kick() -> KEPSStatus;
    pub fn k_eps_passthrough(tx: *const u8, tx_len: u8, rx: *mut u8, rx_len: u8) -> KEPSStatus;
}
