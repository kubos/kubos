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

use mai400_api::*;

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
    /// System power state was changed
    ControlPower,
    /// System configuration was updated
    ConfigureHardware,
    /// A hardware test was performed
    TestHardware,
    /// A raw command was passed through to the system
    IssueRawCommand,
    /// System mode was changed
    SetMode,
    /// GPS time and/or rv values were updated
    Update,
}

/// Response fields for 'configureHardware' mutation
#[derive(GraphQLObject)]
pub struct ConfigureHardwareResponse {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
}

/// Input field for 'controlPower' mutation and
/// response field for 'power' query
#[derive(GraphQLEnum, Clone, Eq, PartialEq, Debug)]
pub enum PowerState {
    /// System is on
    On,
    /// System is off or unavailable
    Off,
    /// System will be reset
    Reset,
}

/// Response fields for 'power' query
#[derive(GraphQLObject)]
pub struct GetPowerResponse {
    /// Current power state of the system
    pub state: PowerState,
    /// Number of valid commands run by the system.
    /// This corresponds to the gus_cmdValidCntr parameter of the
    /// standard telemetry packet.
    ///
    /// Note: This field is named "uptime" to help maintain parity
    /// with the other services. The MAI-400 does not give a
    /// traditional uptime value.
    pub uptime: i32,
}

/// Response fields for 'controlPower' mutation
#[derive(GraphQLObject)]
pub struct ControlPowerResponse {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
    /// Current power state
    pub power: PowerState,
}

/// Input field for 'testHardware' mutation
///
/// Indicates which test should be run against the AntS device
#[derive(GraphQLEnum)]
pub enum TestType {
    /// Integration (non-invasive) test
    Integration,
    /// Hardware (invasive) test
    Hardware,
}

/// Enum for the 'testHardware' mutation response union
pub enum TestResults {
    /// Integration test results
    Integration(IntegrationTestResults),
    /// Hardware test results
    Hardware(HardwareTestResults),
}

/// Response union for 'testHardware' mutation
graphql_union!(TestResults: () where Scalar = <S> |&self| {
    instance_resolvers: |&_| {
        &IntegrationTestResults => match *self {
            TestResults::Integration(ref i) => Some(i),
            _ => None
        },
        &HardwareTestResults => match *self { TestResults::Hardware(ref h) => Some(h), _ => None},
    }
});

/// Response fields for 'testHardware(test: INTEGRATION)' mutation
#[derive(GraphQLObject)]
pub struct IntegrationTestResults {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
    /// Nominal telemetry
    pub telemetry_nominal: StdTelem,
    /// Debug telemetry
    pub telemetry_debug: TelemetryDebug,
}

/// Response fields for 'testHardware(test: HARDWARE)' mutation
#[derive(GraphQLObject)]
pub struct HardwareTestResults {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
    /// Test results
    pub data: String,
}

/// System mode
#[derive(GraphQLEnum, Clone, Copy)]
pub enum Mode {
    /// Test mode
    TestMode = 0,
    /// Rate nulling
    RateNulling = 1,
    /// Reserved for future use
    Reserved1 = 2,
    /// Nadir pointing (normal mode)
    NadirPointing = 3,
    /// Lat/Long pointing
    LatLongPointing = 4,
    /// QbX mode
    QbxMode = 5,
    /// Reserved for future use
    Reserved2 = 6,
    /// Normal sun (nadir with sun rotation)
    NormalSun = 7,
    /// Lat/long sun
    LatLongSun = 8,
    /// Qinertial
    Qintertial = 9,
    /// Reserved for future use
    Reserved3 = 10,
    /// QTable
    Qtable = 11,
    /// Sun-Ram
    SunRam = 12,
    /// Unknown mode detected
    Unknown = 0xFF,
}

impl From<u8> for Mode {
    fn from(raw: u8) -> Mode {
        match raw {
            0 => Mode::TestMode,
            1 => Mode::RateNulling,
            2 => Mode::Reserved1,
            3 => Mode::NadirPointing,
            4 => Mode::LatLongPointing,
            5 => Mode::QbxMode,
            6 => Mode::Reserved2,
            7 => Mode::NormalSun,
            8 => Mode::LatLongSun,
            9 => Mode::Qintertial,
            10 => Mode::Reserved3,
            11 => Mode::Qtable,
            12 => Mode::SunRam,
            _ => Mode::Unknown,
        }
    }
}

/// RV input fields for `update` mutation
#[derive(GraphQLInputObject)]
pub struct RVInput {
    /// X, Y, Z ECI position values
    pub eci_pos: Vec<f64>,
    /// X, Y, Z ECI velocity values
    pub eci_vel: Vec<f64>,
    /// GPS time at Epoch
    pub time_epoch: i32,
}

/// Response fields for `spin` query
#[derive(GraphQLObject)]
pub struct Spin {
    /// X-axis
    pub x: f64,
    /// Y-axis
    pub y: f64,
    /// Z-axis
    pub z: f64,
}

/// Response fields for `telemetry` query
#[derive(GraphQLObject)]
pub struct Telemetry {
    /// Nominal telemetry
    pub nominal: StdTelem,
    /// Debug telemetry
    pub debug: TelemetryDebug,
}

/// Response fields for 'telemetry(telem: NOMINAL)' query
#[derive(Debug, Default, PartialEq)]
pub struct StdTelem(pub StandardTelemetry);

graphql_object!(StdTelem: () where Scalar = <S> |&self| {

    field tlm_counter() -> i32 {
        i32::from(self.0.tlm_counter)
    }

    field gps_time() -> i32 {
        self.0.gps_time as i32
    }

    field time_subsec() -> i32 {
        i32::from(self.0.time_subsec)
    }

    field cmd_valid_cntr() -> i32 {
        i32::from(self.0.cmd_valid_cntr)
    }

    field cmd_invalid_cntr() -> i32 {
        i32::from(self.0.cmd_invalid_cntr)
    }

    field cmd_invalid_chksum_cntr() -> i32 {
        i32::from(self.0.cmd_invalid_chksum_cntr)
    }

    field last_command() -> i32 {
        i32::from(self.0.last_command)
    }

    field acs_mode() -> Mode {
        Mode::from(self.0.acs_mode)
    }

    field css() -> Vec<i32> {
        self.0.css.iter().map(|&elem| i32::from(elem)).collect()
    }

    field eclipse_flag() -> i32 {
        i32::from(self.0.eclipse_flag)
    }

    field sun_vec_b() -> Vec<i32> {
        self.0.sun_vec_b.iter().map(|&elem| i32::from(elem)).collect()
    }

    field i_b_field_meas() -> Vec<i32> {
        self.0.i_b_field_meas.iter().map(|&elem| i32::from(elem)).collect()
    }

    field bd() -> Vec<f64> {
        self.0.bd.iter().map(|&elem| f64::from(elem)).collect()
    }

    field rws_speed_cmd() -> Vec<i32> {
        self.0.rws_speed_cmd.iter().map(|&elem| i32::from(elem)).collect()
    }

    field rws_speed_tach() -> Vec<i32> {
        self.0.rws_speed_tach.iter().map(|&elem| i32::from(elem)).collect()
    }

    field rwa_torque_cmd() -> Vec<f64> {
        self.0.rwa_torque_cmd.iter().map(|&elem| f64::from(elem)).collect()
    }

    field gc_rwa_torque_cmd() -> Vec<i32> {
        self.0.gc_rwa_torque_cmd.iter().map(|&elem| i32::from(elem)).collect()
    }

    field torque_coil_cmd() -> Vec<f64> {
        self.0.torque_coil_cmd.iter().map(|&elem| f64::from(elem)).collect()
    }

    field gc_torque_coil_cmd() -> Vec<i32> {
        self.0.gc_torque_coil_cmd.iter().map(|&elem| i32::from(elem)).collect()
    }

    field qbo_cmd() -> Vec<i32> {
        self.0.qbo_cmd.iter().map(|&elem| i32::from(elem)).collect()
    }

    field qbo_hat() -> Vec<i32> {
        self.0.qbo_hat.iter().map(|&elem| i32::from(elem)).collect()
    }

    field angle_to_go() -> f64 {
        f64::from(self.0.angle_to_go)
    }

    field q_error() -> Vec<i32> {
        self.0.q_error.iter().map(|&elem| i32::from(elem)).collect()
    }

    field omega_b() -> Vec<f64> {
        self.0.omega_b.iter().map(|&elem| f64::from(elem)).collect()
    }

    field rotating_variable_a() -> i32 {
        self.0.rotating_variable_a as i32
    }

    field rotating_variable_b() -> i32 {
        self.0.rotating_variable_b as i32
    }

    field rotating_variable_c() -> i32 {
        self.0.rotating_variable_c as i32
    }

    field nb() -> Vec<i32> {
        self.0.nb.iter().map(|&elem| i32::from(elem)).collect()
    }

    field neci() -> Vec<i32> {
        self.0.neci.iter().map(|&elem| i32::from(elem)).collect()
    }
});

/// Response fields for 'telemetry(telem: DEBUG)' query
#[derive(GraphQLObject)]
pub struct TelemetryDebug {
    /// IREHS telemetry values
    pub irehs: IREHSTelem,
    /// Raw IMU telemetry values
    pub raw_imu: RawIMUTelem,
    /// Rotating telemetry values
    pub rotating: Rotating,
}

/// IR Earth horizon sensor telemetry values
#[derive(Debug, Default, PartialEq)]
pub struct IREHSTelem(pub IREHSTelemetry);

/// Thermopile telemetry values
#[derive(GraphQLObject)]
pub struct ThermopileStruct {
    /// Calculated dip angle of Earth limb, in degrees
    dip_angle: i32,
    /// Earth limb thermopile sensor
    earth_limb: ThermopileSensor,
    /// Earth reference thermopile sensor
    earth_ref: ThermopileSensor,
    /// Space reference thermopile sensor
    space_ref: ThermopileSensor,
    /// Wide FOV thermopile sensor
    wide_fov: ThermopileSensor,
}

/// Thermopile sensor telemetry values
#[derive(GraphQLObject)]
pub struct ThermopileSensor {
    /// Thermopile sensor ADC value
    adc: i32,
    /// Thermistor temperature ADC value
    temp: i32,
    /// Indicates whether the following `flag` field is empty
    errors: bool,
    /// Solution degradation codes
    flags: Vec<String>,
}

graphql_object!(IREHSTelem: () where Scalar = <S> |&self| {
    field thermopile_struct_a() -> ThermopileStruct {
        ThermopileStruct {
                dip_angle: self.0.dip_angle_a as i32,
                earth_limb: ThermopileSensor {
                    adc: self.0.thermopiles_a[0] as i32,
                    temp: self.0.temp_a[0] as i32,
                    errors: !self.0.solution_degraded[0].is_empty(),
                    flags: self.0.solution_degraded[0].to_vec()
                },
                earth_ref: ThermopileSensor {
                    adc: self.0.thermopiles_a[1] as i32,
                    temp: self.0.temp_a[1] as i32,
                    errors: !self.0.solution_degraded[1].is_empty(),
                    flags: self.0.solution_degraded[1].to_vec()
                },
                space_ref: ThermopileSensor {
                    adc: self.0.thermopiles_a[2] as i32,
                    temp: self.0.temp_a[2] as i32,
                    errors: !self.0.solution_degraded[2].is_empty(),
                    flags: self.0.solution_degraded[2].to_vec()
                },
                wide_fov: ThermopileSensor {
                    adc: self.0.thermopiles_a[3] as i32,
                    temp: self.0.temp_a[3] as i32,
                    errors: !self.0.solution_degraded[3].is_empty(),
                    flags: self.0.solution_degraded[3].to_vec()
                }
            }
        
    }

    field thermopile_struct_b() -> ThermopileStruct {
        ThermopileStruct {
                dip_angle: self.0.dip_angle_b as i32,
                earth_limb: ThermopileSensor {
                    adc: self.0.thermopiles_b[0] as i32,
                    temp: self.0.temp_b[0] as i32,
                    errors: !self.0.solution_degraded[4].is_empty(),
                    flags: self.0.solution_degraded[4].to_vec()
                },
                earth_ref: ThermopileSensor {
                    adc: self.0.thermopiles_b[1] as i32,
                    temp: self.0.temp_b[1] as i32,
                    errors: !self.0.solution_degraded[5].is_empty(),
                    flags: self.0.solution_degraded[5].to_vec()
                },
                space_ref: ThermopileSensor {
                    adc: self.0.thermopiles_b[2] as i32,
                    temp: self.0.temp_b[2] as i32,
                    errors: !self.0.solution_degraded[6].is_empty(),
                    flags: self.0.solution_degraded[6].to_vec()
                },
                wide_fov: ThermopileSensor {
                    adc: self.0.thermopiles_b[3] as i32,
                    temp: self.0.temp_b[3] as i32,
                    errors: !self.0.solution_degraded[7].is_empty(),
                    flags: self.0.solution_degraded[7].to_vec()
                }
            }
        
    }

    field thermopiles_a() -> Vec<i32> {
        self.0.thermopiles_a.iter().map(|&elem| i32::from(elem)).collect()
    }

    field thermopiles_b() -> Vec<i32> {
        self.0.thermopiles_b.iter().map(|&elem| i32::from(elem)).collect()
    }

    field temp_a() -> Vec<i32> {
        self.0.temp_a.iter().map(|&elem| i32::from(elem)).collect()
    }

    field temp_b() -> Vec<i32> {
        self.0.temp_b.iter().map(|&elem| i32::from(elem)).collect()
    }

    field dip_angle_a() -> i32 {
        i32::from(self.0.dip_angle_a)
    }

    field dip_angle_b() -> i32 {
        i32::from(self.0.dip_angle_b)
    }

    field solution_degraded() -> Vec<Vec<String>> {
        let mut parent = vec![];
        for elem in self.0.solution_degraded.iter() {
            if elem.is_empty() {
                parent.push(vec![]);
            } else {
                parent.push(elem.to_vec());
            }
        }

        parent
    }
});

/// Raw IMU telemetry values
#[derive(Debug, Default, PartialEq)]
pub struct RawIMUTelem(pub RawIMU);

graphql_object!(RawIMUTelem: () where Scalar = <S> |&self| {
    field accel() -> Vec<i32> {
        self.0.accel.iter().map(|&elem| i32::from(elem)).collect()
    }

    field gyro() -> Vec<i32> {
        self.0.gyro.iter().map(|&elem| i32::from(elem)).collect()
    }

    field gyro_temp() -> i32 {
        i32::from(self.0.gyro_temp)
    }
});

/// Rotating telemetry values.
/// These values aren't updated with each returned telemetry packet.
/// Instead, sections are updated each iteration.
/// The full rotation is updated every six seconds.
#[derive(Debug, Default, PartialEq)]
pub struct Rotating(pub RotatingTelemetry);

graphql_object!(Rotating: () where Scalar = <S> |&self| {

    field b_field_igrf() -> Vec<f64> {
        self.0.b_field_igrf.iter().map(|&elem| f64::from(elem)).collect()
    }

    field sun_vec_eph() -> Vec<f64> {
        self.0.sun_vec_eph.iter().map(|&elem| f64::from(elem)).collect()
    }

    field sc_pos_eci() -> Vec<f64> {
        self.0.sc_pos_eci.iter().map(|&elem| f64::from(elem)).collect()
    }

    field sc_vel_eci() -> Vec<f64> {
        self.0.sc_vel_eci.iter().map(|&elem| f64::from(elem)).collect()
    }

    field kepler_elem() -> Kepler {
        Kepler(self.0.kepler_elem.clone())
    }

    field k_bdot() -> Vec<f64> {
        self.0.k_bdot.iter().map(|&elem| f64::from(elem)).collect()
    }

    field kp() -> Vec<f64> {
        self.0.kp.iter().map(|&elem| f64::from(elem)).collect()
    }

    field kd() -> Vec<f64> {
        self.0.kd.iter().map(|&elem| f64::from(elem)).collect()
    }

    field k_unload() -> Vec<f64> {
        self.0.k_unload.iter().map(|&elem| f64::from(elem)).collect()
    }

    field css_bias() -> Vec<i32> {
        self.0.css_bias.iter().map(|&elem| i32::from(elem)).collect()
    }

    field mag_bias() -> Vec<i32> {
        self.0.mag_bias.iter().map(|&elem| i32::from(elem)).collect()
    }

    field rws_volt() -> i32 {
        i32::from(self.0.rws_volt)
    }

    field rws_press() -> i32 {
        i32::from(self.0.rws_press)
    }

    field att_det_mode() -> i32 {
        i32::from(self.0.att_det_mode)
    }

    field rws_reset_cntr() -> Vec<i32> {
        self.0.rws_reset_cntr.iter().map(|&elem| i32::from(elem)).collect()
    }

    field sun_mag_aligned() -> i32 {
        i32::from(self.0.sun_mag_aligned)
    }

    field minor_version() -> i32 {
        i32::from(self.0.minor_version)
    }

    field mai_sn() -> i32 {
        i32::from(self.0.mai_sn)
    }

    field orbit_prop_mode() -> i32 {
        i32::from(self.0.orbit_prop_mode)
    }

    field acs_op_mode() -> i32 {
        i32::from(self.0.acs_op_mode)
    }

    field proc_reset_cntr() -> i32 {
        i32::from(self.0.proc_reset_cntr)
    }

    field major_version() -> i32 {
        i32::from(self.0.major_version)
    }

    field ads_op_mode() -> i32 {
        i32::from(self.0.ads_op_mode)
    }

    field css_gain() -> Vec<f64> {
        self.0.css_gain.iter().map(|&elem| f64::from(elem)).collect()
    }

    field mag_gain() -> Vec<f64> {
        self.0.mag_gain.iter().map(|&elem| f64::from(elem)).collect()
    }

    field orbit_epoch() -> i32 {
        self.0.orbit_epoch as i32
    }

    field true_anomoly_epoch() -> f64 {
        f64::from(self.0.true_anomoly_epoch)
    }

    field orbit_epoch_next() -> i32 {
        self.0.orbit_epoch_next as i32
    }

    field sc_pos_eci_epoch() -> Vec<f64> {
        self.0.sc_pos_eci_epoch.iter().map(|&elem| f64::from(elem)).collect()
    }

    field sc_vel_eci_epoch() -> Vec<f64> {
        self.0.sc_vel_eci_epoch.iter().map(|&elem| f64::from(elem)).collect()
    }

    field qb_x_wheel_speed() -> i32 {
        i32::from(self.0.qb_x_wheel_speed)
    }

    field qb_x_filter_gain() -> f64 {
        f64::from(self.0.qb_x_filter_gain)
    }

    field qb_x_dipole_gain() -> f64 {
        f64::from(self.0.qb_x_dipole_gain)
    }

    field dipole_gain() -> Vec<f64> {
        self.0.dipole_gain.iter().map(|&elem| f64::from(elem)).collect()
    }

    field wheel_speed_bias() -> Vec<i32> {
        self.0.wheel_speed_bias.iter().map(|&elem| i32::from(elem)).collect()
    }

    field cos_sun_mag_align_thresh() -> f64 {
        f64::from(self.0.cos_sun_mag_align_thresh)
    }

    field unload_ang_thresh() -> f64 {
        f64::from(self.0.unload_ang_thresh)
    }

    field q_sat() -> f64 {
        f64::from(self.0.q_sat)
    }

    field rwa_trq_max() -> f64 {
        f64::from(self.0.rwa_trq_max)
    }

    field rws_motor_current() -> Vec<i32> {
        self.0.rws_motor_current.iter().map(|&elem| i32::from(elem)).collect()
    }

    field rws_motor_temp() -> i32 {
        i32::from(self.0.rws_motor_temp)
    }

});

/// Kepler element telemetry values
#[derive(Debug, Default, PartialEq)]
pub struct Kepler(pub KeplerElem);

graphql_object!(Kepler: () where Scalar = <S> |&self| {
    field semi_major_axis() -> f64 {
        f64::from(self.0.semi_major_axis)
    }

    field eccentricity() -> f64 {
        f64::from(self.0.eccentricity)
    }

    field inclination() -> f64 {
        f64::from(self.0.inclination)
    }

    field raan() -> f64 {
        f64::from(self.0.raan)
    }

    field arg_parigee() -> f64 {
        f64::from(self.0.arg_parigee)
    }

    field true_anomoly() -> f64 {
        f64::from(self.0.true_anomoly)
    }
});
