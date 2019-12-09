// Convert the objects to GraphQLEnum objects

use juniper::{FieldResult,GraphQLObject};
//use kubos_service::{process_errors, push_err, run};
//use std::sync::{Arc, Mutex, RwLock};

/// Common response fields structure for requests
/// which don't return any specific data
#[derive(GraphQLObject)]
pub struct GenericResponse {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
}

/// Return field for 'ack' query
///
/// Indicates last mutation executed by the service
#[derive(GraphQLEnum, Clone, Copy)]
pub enum AckCommand {
    /// No mutations have been executed
    None,
    /// No-Op
    Noop,
    /// Get System and Battery and heater configuration 
    /// Set System and Battery,heater and MPPT configuration 
    SetEpsConfiguration,
    /// Set MPPT value for all three channels
    SetEpsChannels,
    /// Reboot EPS (Hard reboot and softreboot)
    EpsReboot,
    ///Set EPS MPPT mode
    EpsSetMPPTmode,
    /// Set EPS MPPT level
    EpsSetMPPTLevel,
    ///Set heater
    EpsHeaterToggle,
    /// Kick, start, stop, reset watch dog
    EpsWatchDog,
    /// A hardware test was performed
    TestHardware,
    /// A raw command was passed through to the system
    IssueRawCommand,
}

/// Response fields for 'issueRawCommand' mutation
#[derive(GraphQLObject)]
pub struct RawCommandResponse {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
    /// Command response from system
    pub response: String,
}

/// Fields for EPS channels
#[derive(GraphQLEnum, Clone, Eq, PartialEq, Debug)]
pub enum EpsChannels {
    ///5V channel
    Channel0,
    ///5V channel
    Channel1,
    ///5V channel
    Channel2,
    ///3.3V channel
    Channel3,
    ///3.3V channel
    Channel4,
    ///3.3V channel
    Channel5,
}

/// Fields for EPS power states
#[derive(GraphQLEnum, Clone, Eq, PartialEq, Debug)]
pub enum PowerState {
    /// System is on
    On,
    /// System is off or unavailable
    Off,
}

/// Response fields for 'controlPower' mutation
pub struct EnSubsysResponse {
    /// Current power status
    pub state: PowerState,
    /// select output channel
    pub channel: EpsChannels,
    /// Power delay
    pub delay:i32,
}

graphql_object!(EnSubsysResponse:() |&self| {
    field state() -> FieldResult<PowerState> {
        Ok(self.state.clone())
    }

    field channel() -> FieldResult<EpsChannels> {
        Ok(self.channel.clone())
    }

    field delay() -> FieldResult<i32> {
        Ok(self.delay as i32)
    }
});

/// Response fields for 'SchEpsSystemConfig' mutation
#[derive(GraphQLObject)]
#[doc(hidden)]
pub struct SchEpsSystemConfig {
    pub ppt_mode: i32,
    pub battheater_mode: i32,
	pub battheater_low:i32,
	pub battheater_high:i32,
	pub output_normal_value:Vec<i32>,
	pub output_safe_value:Vec<i32>,
	pub output_initial_on_delay:Vec<i32>,
	pub output_initial_off_delay:Vec<i32>,
	pub vboost:Vec<i32>,
}

/// Response fields for 'SchEpsBatteryConfig' mutation
#[derive(GraphQLObject)]
#[doc(hidden)]
pub struct SchEpsBatteryConfig {
    pub batt_maxvoltage: i32,
    pub batt_safevoltage: i32,
	pub batt_criticalvoltage: i32,
	pub batt_normalvoltage: i32,
	pub reserved1:Vec<i32>,
	pub reserved2:Vec<i32>,
}

/// Response fields for 'SchEpsHk' mutation
#[derive(GraphQLObject)]
#[doc(hidden)]
pub struct SchEpsHk {
    pub vboost:Vec<i32>,
    pub vbatt:i32,
	pub curin:Vec<i32>,
	pub cursun:i32,
	pub cursys:i32,
	pub reserved1:i32,
	pub curout:Vec<i32>,
	pub output:Vec<i32>,
	pub output_on_delta:Vec<i32>,
	pub output_off_delta:Vec<i32>,
	pub latchup:Vec<i32>,
	pub wdt_i2c_time_left:i32,
	pub wdt_gnd_time_left:i32,
	pub wdt_csp_pings_left:Vec<i32>,
	pub counter_wdt_i2c:i32,
	pub counter_wdt_gnd:i32,
	pub counter_wdt_csp:Vec<i32>,
	pub counter_boot:i32,
	pub temp:Vec<i32>,
	pub boot_cause:i32,	
	pub batt_mode:i32,
	pub ppt_mode:i32,
	pub reserved2:i32,
}
