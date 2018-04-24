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
#![allow(unused_variables)]

use juniper::FieldResult;
use mai400_api::*;

/// Common response fields structure for requests
/// which don't return any specific data
#[derive(GraphQLObject)]
pub struct GenericResponse {
    pub errors: String,
    pub success: bool,
}

/// Return field for 'ack' query
///
/// Indicates last mutation executed by the service
// Future work: Actually implement this. Rust lifetimes are hard...
#[derive(GraphQLEnum, Clone, Copy)]
pub enum AckCommand {
    None,
    Noop,
    ControlPower,
    ConfigureHardware,
    TestHardware,
    IssueRawCommand,
    SetMode,
    Update,
}

#[derive(GraphQLObject)]
pub struct Config {}

/// Response fields for 'configureHardware' mutation
#[derive(GraphQLObject)]
pub struct ConfigureHardwareResponse {
    pub errors: String,
    pub success: bool,
}

/// Input field for 'controlPower' mutation and
/// response field for 'power' query
#[derive(GraphQLEnum, Clone, Eq, PartialEq, Debug)]
pub enum PowerState {
    On,
    Off,
    Reset,
}

/// Response fields for 'power' query
#[derive(GraphQLObject)]
pub struct GetPowerResponse {
    pub state: PowerState,
    pub uptime: i32,
}

/// Response fields for 'controlPower' mutation
#[derive(GraphQLObject)]
pub struct ControlPowerResponse {
    pub errors: String,
    pub success: bool,
    pub power: PowerState,
}

/// Response fields for 'noop' mutation
pub type NoopResponse = GenericResponse;

/// Input field for 'testHardware' mutation
///
/// Indicates which test should be run against the AntS device
#[derive(GraphQLEnum)]
pub enum TestType {
    Integration,
    Hardware,
}

/// Response fields for 'testHardware(test: INTEGRATION)' mutation
#[derive(GraphQLObject)]
pub struct IntegrationTestResults {
    pub errors: String,
    pub success: bool,
    pub telemetry_nominal: StdTelem,
    pub telemetry_debug: TelemetryDebug,
}

/// Response fields for 'testHardware(test: HARDWARE)' mutation
#[derive(GraphQLObject)]
pub struct HardwareTestResults {
    pub errors: String,
    pub success: bool,
    pub data: String,
}

#[derive(GraphQLEnum, Clone, Copy)]
pub enum Mode {
    TestMode = 0,
    RateNulling = 1,
    Reserved1 = 2,
    NadirPointing = 3,
    LatLongPointing = 4,
    QbxMode = 5,
    Reserved2 = 6,
    NormalSun = 7,
    LatLongSun = 8,
    Qintertial = 9,
    Reserved3 = 10,
    Qtable = 11,
    SunRam = 12,
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

#[derive(GraphQLObject)]
pub struct Orientation {}

#[derive(GraphQLInputObject)]
pub struct RVInput {
    pub eci_pos: Vec<f64>,
    pub eci_vel: Vec<f64>,
    pub time_epoch: i32,
}

#[derive(GraphQLObject)]
pub struct Spin {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(GraphQLObject)]
pub struct Telemetry {
    pub nominal: StdTelem,
    pub debug: TelemetryDebug,
}

#[derive(Debug, Default, PartialEq)]
pub struct StdTelem(pub StandardTelemetry);

graphql_object!(StdTelem: () |&self| {

    field tlm_counter() -> FieldResult<i32> {
        Ok(self.0.tlm_counter as i32)
    }
    
    field gps_time() -> FieldResult<i32> {
        Ok(self.0.gps_time as i32)
    }
    
    field time_subsec() -> FieldResult<i32> {
        Ok(self.0.time_subsec as i32)
    }
    
    field cmd_valid_cntr() -> FieldResult<i32> {
        Ok(self.0.cmd_valid_cntr as i32)
    }
    
    field cmd_invalid_cntr() -> FieldResult<i32> {
        Ok(self.0.cmd_invalid_cntr as i32)
    }
    
    field cmd_invalid_chksum_cntr() -> FieldResult<i32> {
        Ok(self.0.cmd_invalid_chksum_cntr as i32)
    }
    
    field last_command() -> FieldResult<i32> {
        Ok(self.0.last_command as i32)
    }
    
    field acs_mode() -> FieldResult<Mode> {
        Ok(Mode::from(self.0.acs_mode))
    }
    
    field css() -> FieldResult<Vec<i32>> {
        Ok(self.0.css.iter().map(|&elem| elem as i32).collect())
    }
    
    field eclipse_flag() -> FieldResult<i32> {
        Ok(self.0.eclipse_flag as i32)
    }
    
    field sun_vec_b() -> FieldResult<Vec<i32>> {
        Ok(self.0.sun_vec_b.iter().map(|&elem| elem as i32).collect())
    }
    
    field i_b_field_meas() -> FieldResult<Vec<i32>> {
        Ok(self.0.i_b_field_meas.iter().map(|&elem| elem as i32).collect())
    }
    
    field bd() -> FieldResult<Vec<f64>> {
        Ok(self.0.bd.iter().map(|&elem| elem as f64).collect())
    }
    
    field rws_speed_cmd() -> FieldResult<Vec<i32>> {
        Ok(self.0.rws_speed_cmd.iter().map(|&elem| elem as i32).collect())
    }
    
    field rws_speed_tach() -> FieldResult<Vec<i32>> {
        Ok(self.0.rws_speed_tach.iter().map(|&elem| elem as i32).collect())
    }
    
    field rwa_torque_cmd() -> FieldResult<Vec<f64>> {
        Ok(self.0.rwa_torque_cmd.iter().map(|&elem| elem as f64).collect())
    }
    
    field gc_rwa_torque_cmd() -> FieldResult<Vec<i32>> {
        Ok(self.0.gc_rwa_torque_cmd.iter().map(|&elem| elem as i32).collect())
    }
    
    field torque_coil_cmd() -> FieldResult<Vec<f64>> {
        Ok(self.0.torque_coil_cmd.iter().map(|&elem| elem as f64).collect())
    }
    
    field gc_torque_coil_cmd() -> FieldResult<Vec<i32>> {
        Ok(self.0.gc_torque_coil_cmd.iter().map(|&elem| elem as i32).collect())
    }
    
    field qbo_cmd() -> FieldResult<Vec<i32>> {
        Ok(self.0.qbo_cmd.iter().map(|&elem| elem as i32).collect())
    }
    
    field qbo_hat() -> FieldResult<Vec<i32>> {
        Ok(self.0.qbo_hat.iter().map(|&elem| elem as i32).collect())
    }
    
    field angle_to_go() -> FieldResult<f64> {
        Ok(self.0.angle_to_go as f64)
    }
    
    field q_error() -> FieldResult<Vec<i32>> {
        Ok(self.0.q_error.iter().map(|&elem| elem as i32).collect())
    }
    
    field omega_b() -> FieldResult<Vec<f64>> {
        Ok(self.0.omega_b.iter().map(|&elem| elem as f64).collect())
    }
    
    field rotating_variable_a() -> FieldResult<i32> {
        Ok(self.0.rotating_variable_a as i32)
    }
    
    field rotating_variable_b() -> FieldResult<i32> {
        Ok(self.0.rotating_variable_b as i32)
    }
    
    field rotating_variable_c() -> FieldResult<i32> {
        Ok(self.0.rotating_variable_c as i32)
    }
    
    field nb() -> FieldResult<Vec<i32>> {
        Ok(self.0.nb.iter().map(|&elem| elem as i32).collect())
    }
    
    field neci() -> FieldResult<Vec<i32>> {
        Ok(self.0.neci.iter().map(|&elem| elem as i32).collect())
    }
});


/// Response fields for 'telemetry(telem: DEBUG)' query
#[derive(GraphQLObject)]
pub struct TelemetryDebug {
    pub irehs: IREHSTelem,
    pub raw_imu: RawIMUTelem,
    pub rotating: Rotating,
}

#[derive(Debug, Default, PartialEq)]
pub struct IREHSTelem(pub IREHSTelemetry);

graphql_object!(IREHSTelem: () |&self| {
    field thermopiles_a() -> FieldResult<Vec<i32>> {
        Ok(self.0.thermopiles_a.iter().map(|&elem| elem as i32).collect())
    }
    
    field thermopiles_b() -> FieldResult<Vec<i32>> {
        Ok(self.0.thermopiles_b.iter().map(|&elem| elem as i32).collect())
    }
    
    field temp_a() -> FieldResult<Vec<i32>> {
        Ok(self.0.temp_a.iter().map(|&elem| elem as i32).collect())
    }
    
    field temp_b() -> FieldResult<Vec<i32>> {
        Ok(self.0.temp_b.iter().map(|&elem| elem as i32).collect())
    }
    
    field dip_angle_a() -> FieldResult<i32> {
        Ok(self.0.dip_angle_a as i32)
    }
    
    field dip_angle_b() -> FieldResult<i32> {
        Ok(self.0.dip_angle_b as i32)
    }
    
    field solution_degraded() -> FieldResult<i32> {
        unimplemented!();
    }
});

#[derive(Debug, Default, PartialEq)]
pub struct RawIMUTelem(pub RawIMU);

graphql_object!(RawIMUTelem: () |&self| {
    field accel() -> FieldResult<Vec<i32>> {
        Ok(self.0.accel.iter().map(|&elem| elem as i32).collect())
    }
    
    field gyro() -> FieldResult<Vec<i32>> {
        Ok(self.0.gyro.iter().map(|&elem| elem as i32).collect())
    }
    
    field gyro_temp() -> FieldResult<i32> {
        Ok(self.0.gyro_temp as i32)
    }
});

#[derive(Debug, Default, PartialEq)]
pub struct Rotating(pub RotatingTelemetry);

graphql_object!(Rotating: () |&self| {

    field b_field_igrf() -> FieldResult<Vec<f64>> {
        Ok(self.0.b_field_igrf.iter().map(|&elem| elem as f64).collect())
    }
    
    field sun_vec_eph() -> FieldResult<Vec<f64>> {
        Ok(self.0.sun_vec_eph.iter().map(|&elem| elem as f64).collect())
    }
    
    field sc_pos_eci() -> FieldResult<Vec<f64>> {
        Ok(self.0.sc_pos_eci.iter().map(|&elem| elem as f64).collect())
    }
    
    field sc_vel_eci() -> FieldResult<Vec<f64>> {
        Ok(self.0.sc_vel_eci.iter().map(|&elem| elem as f64).collect())
    }
    
    field kepler_elem() -> FieldResult<i32> {
        unimplemented!();
        //TODO
    }
    
    field k_bdot() -> FieldResult<Vec<f64>> {
        Ok(self.0.k_bdot.iter().map(|&elem| elem as f64).collect())
    }
    
    field kp() -> FieldResult<Vec<f64>> {
        Ok(self.0.kp.iter().map(|&elem| elem as f64).collect())
    }
    
    field kd() -> FieldResult<Vec<f64>> {
        Ok(self.0.kd.iter().map(|&elem| elem as f64).collect())
    }
    
    field k_unload() -> FieldResult<Vec<f64>> {
        Ok(self.0.k_unload.iter().map(|&elem| elem as f64).collect())
    }
    
    field css_bias() -> FieldResult<Vec<i32>> {
        Ok(self.0.css_bias.iter().map(|&elem| elem as i32).collect())
    }
    
    field mag_bias() -> FieldResult<Vec<i32>> {
        Ok(self.0.mag_bias.iter().map(|&elem| elem as i32).collect())
    }
    
    field rws_volt() -> FieldResult<i32> {
        Ok(self.0.rws_volt as i32)
    }
    
    field rws_press() -> FieldResult<i32> {
        Ok(self.0.rws_press as i32)
    }
    
    field att_det_mode() -> FieldResult<i32> {
        Ok(self.0.att_det_mode as i32)
    }
    
    field rws_reset_cntr() -> FieldResult<Vec<i32>> {
        Ok(self.0.rws_reset_cntr.iter().map(|&elem| elem as i32).collect())
    }
    
    field sun_mag_aligned() -> FieldResult<i32> {
        Ok(self.0.sun_mag_aligned as i32)
    }
    
    field minor_version() -> FieldResult<i32> {
        Ok(self.0.minor_version as i32)
    }
    
    field mai_sn() -> FieldResult<i32> {
        Ok(self.0.mai_sn as i32)
    }
    
    field orbit_prop_mode() -> FieldResult<i32> {
        Ok(self.0.orbit_prop_mode as i32)
    }
    
    field acs_op_mode() -> FieldResult<i32> {
        Ok(self.0.acs_op_mode as i32)
    }
    
    field proc_reset_cntr() -> FieldResult<i32> {
        Ok(self.0.proc_reset_cntr as i32)
    }
    
    field major_version() -> FieldResult<i32> {
        Ok(self.0.major_version as i32)
    }
    
    field ads_op_mode() -> FieldResult<i32> {
        Ok(self.0.ads_op_mode as i32)
    }
    
    field css_gain() -> FieldResult<Vec<f64>> {
        Ok(self.0.css_gain.iter().map(|&elem| elem as f64).collect())
    }
    
    field mag_gain() -> FieldResult<Vec<f64>> {
        Ok(self.0.mag_gain.iter().map(|&elem| elem as f64).collect())
    }
    
    field orbit_epoch() -> FieldResult<i32> {
        Ok(self.0.orbit_epoch as i32)
    }
    
    field true_anomoly_epoch() -> FieldResult<f64> {
        Ok(self.0.true_anomoly_epoch as f64)
    }
    
    field orbit_epoch_next() -> FieldResult<i32> {
        Ok(self.0.orbit_epoch_next as i32)
    }
    
    field sc_pos_eci_epoch() -> FieldResult<Vec<f64>> {
        Ok(self.0.sc_pos_eci_epoch.iter().map(|&elem| elem as f64).collect())
    }
    
    field sc_vel_eci_epoch() -> FieldResult<Vec<f64>> {
        Ok(self.0.sc_vel_eci_epoch.iter().map(|&elem| elem as f64).collect())
    }
    
    field qb_x_wheel_speed() -> FieldResult<i32> {
        Ok(self.0.qb_x_wheel_speed as i32)
    }
    
    field qb_x_filter_gain() -> FieldResult<f64> {
        Ok(self.0.qb_x_filter_gain as f64)
    }
    
    field qb_x_dipole_gain() -> FieldResult<f64> {
        Ok(self.0.qb_x_dipole_gain as f64)
    }
    
    field dipole_gain() -> FieldResult<Vec<f64>> {
        Ok(self.0.dipole_gain.iter().map(|&elem| elem as f64).collect())
    }
    
    field wheel_speed_bias() -> FieldResult<Vec<i32>> {
        Ok(self.0.wheel_speed_bias.iter().map(|&elem| elem as i32).collect())
    }
    
    field cos_sun_mag_align_thresh() -> FieldResult<f64> {
        Ok(self.0.cos_sun_mag_align_thresh as f64)
    }
    
    field unload_ang_thresh() -> FieldResult<f64> {
        Ok(self.0.unload_ang_thresh as f64)
    }
    
    field q_sat() -> FieldResult<f64> {
        Ok(self.0.q_sat as f64)
    }
    
    field raw_trq_max() -> FieldResult<f64> {
        Ok(self.0.raw_trq_max as f64)
    }
    
    field rws_motor_current() -> FieldResult<Vec<i32>> {
        Ok(self.0.rws_motor_current.iter().map(|&elem| elem as i32).collect())
    }
    
    field raw_motor_temp() -> FieldResult<i32> {
        Ok(self.0.raw_motor_temp as i32)
    }

});
