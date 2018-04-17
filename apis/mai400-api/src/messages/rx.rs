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

use nom::*;
use super::*;

/// Standard telemetry packet sent by the MAI-400 every 250ms
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StandardTelemetry {
    /// Message Header
    pub hdr: MessageHeader,
    /// Rotating variable indicator
    pub tlm_counter: u8,
    /// UTC Time in Seconds
    pub gps_time: u32,
    /// 4 Hz Subsecond counter
    pub time_subsec: u8,
    /// Valid Command Counter
    pub cmd_valid_cntr: u16,
    /// Invalid Command Counter. Number of commands which did not pass command verification.
    pub cmd_invalid_cntr: u16,
    /// Invalid Command CRC Counter. Number of command messages received with invalid checksums
    pub cmd_invalid_chksum_cntr: u16,
    /// Last valid CCT command received
    pub last_command: u8,
    /// Commanded ACS Mode
    pub acs_mode: u8,
    /// Raw sun sensor outputs
    pub css: [u16; 6],
    /// Whether the device is eclipsed or not
    pub eclipse_flag: u8,
    /// Unit Sun Vector in Body Frame
    pub sun_vec_b: [i16; 3],
    /// Magnetometer Reading (inc. bias and gain)
    pub i_b_field_meas: [i16; 3],
    /// Rate of Change of Magnetic field Vector in Body Frame
    pub bd: [f32; 3],
    /// RWS Commanded Wheel Speed, lsb: 1 RPM
    pub rws_speed_cmd: [i16; 3],
    /// RWS Wheel Speeds, lsb: 1 RPM
    pub rws_speed_tach: [i16; 3],
    /// Commanded Wheel Torque computed by ADACS (mNm)
    pub rwa_torque_cmd: [f32; 3],
    /// RWS Torque Command to wheel
    pub gc_rwa_torque_cmd: [i8; 3],
    /// Torque Coil Command computed by ADACS (Am<sup>2</sup>)
    pub torque_coil_cmd: [f32; 3],
    /// Torque Coil Command (lsb)
    pub gc_torque_coil_cmd: [i8; 3],
    /// Commanded orbit-to-body quaternion
    pub qbo_cmd: [i32; 4],
    /// Current Estimated Orbit-to-Body Quaternion
    pub qbo_hat: [i32; 4],
    /// Angle to Go in radians
    pub angle_to_go: f32,
    /// Error between command and estimate Qbo
    pub q_error: [i32; 4],
    /// Body rate in body frame (rad/sec).
    pub omega_b: [f32; 3],
    /// Rotating variable A. Use [RotatingTelemetry](struct.RotatingTelemetry.html) struct if interaction is needed
    pub rotating_variable_a: u32,
    /// Rotating variable B. Use [RotatingTelemetry](struct.RotatingTelemetry.html) struct if interaction is needed
    pub rotating_variable_b: u32,
    /// Rotating variable C. Use [RotatingTelemetry](struct.RotatingTelemetry.html) struct if interaction is needed
    pub rotating_variable_c: u32,
    /// Nadir vectors in Body
    pub nb: [i32; 3],
    /// Nadir vectors in ECI frame
    pub neci: [i32; 3],
    /// Message checksum
    pub crc: u16,
}

impl StandardTelemetry {
    /// Constructor. Converts a raw data array received from the MAI-400 into a usable structure
    pub fn new(msg: &[u8]) -> Self {
        standardtelem(msg).unwrap().1
    }
}

named!(standardtelem(&[u8]) -> StandardTelemetry,
    do_parse!(
        sync: le_u16 >>
        data_len: le_u16 >>
        msg_id: le_u8 >>
        addr: le_u8 >>
        tlm_counter: le_u8 >>
        gps_time: le_u32 >>
        time_subsec: le_u8 >>
        cmd_valid_cntr: le_u16 >>
        cmd_invalid_cntr: le_u16 >>
        cmd_invalid_chksum_cntr: le_u16 >>
        last_command: le_u8 >>
        acs_mode: le_u8 >>
        css_0: le_u16 >>
        css_1: le_u16 >>
        css_2: le_u16 >>
        css_3: le_u16 >>
        css_4: le_u16 >>
        css_5: le_u16 >>
        eclipse_flag: le_u8 >>
        sun_vec_b_0: le_i16 >>
        sun_vec_b_1: le_i16 >>
        sun_vec_b_2: le_i16 >>
        i_b_field_meas_0: le_i16 >>
        i_b_field_meas_1: le_i16 >>
        i_b_field_meas_2: le_i16 >>
        bd_0: le_f32 >>
        bd_1: le_f32 >>
        bd_2: le_f32 >>
        rws_speed_cmd_0: le_i16 >>
        rws_speed_cmd_1: le_i16 >>
        rws_speed_cmd_2: le_i16 >>
        rws_speed_tach_0: le_i16 >>
        rws_speed_tach_1: le_i16 >>
        rws_speed_tach_2: le_i16 >>
        rwa_torque_cmd_0: le_f32 >>
        rwa_torque_cmd_1: le_f32 >>
        rwa_torque_cmd_2: le_f32 >>
        gc_rwa_torque_cmd_0: le_i8 >>
        gc_rwa_torque_cmd_1: le_i8 >>
        gc_rwa_torque_cmd_2: le_i8 >>
        torque_coil_cmd_0: le_f32 >>
        torque_coil_cmd_1: le_f32 >>
        torque_coil_cmd_2: le_f32 >>
        gc_torque_coil_cmd_0: le_i8 >>
        gc_torque_coil_cmd_1: le_i8 >>
        gc_torque_coil_cmd_2: le_i8 >>
        qbo_cmd_0: le_i32 >>
        qbo_cmd_1: le_i32 >>
        qbo_cmd_2: le_i32 >>
        qbo_cmd_3: le_i32 >>
        qbo_hat_0: le_i32 >>
        qbo_hat_1: le_i32 >>
        qbo_hat_2: le_i32 >>
        qbo_hat_3: le_i32 >>
        angle_to_go: le_f32 >>
        q_error_0: le_i32 >>
        q_error_1: le_i32 >>
        q_error_2: le_i32 >>
        q_error_3: le_i32 >>
        omega_b_0: le_f32 >>
        omega_b_1: le_f32 >>
        omega_b_2: le_f32 >>
        rotating_variable_a: le_u32 >>
        rotating_variable_b: le_u32 >>
        rotating_variable_c: le_u32 >>
        nb_0: le_i32 >>
        nb_1: le_i32 >>
        nb_2: le_i32 >>
        neci_0: le_i32 >>
        neci_1: le_i32 >>
        neci_2: le_i32 >>
        crc: le_u16 >>
        (StandardTelemetry{
            hdr: MessageHeader {
                sync, 
                data_len, 
                msg_id, 
                addr
            },
            tlm_counter,
            gps_time,
            time_subsec,
            cmd_valid_cntr,
            cmd_invalid_cntr,
            cmd_invalid_chksum_cntr,
            last_command,
            acs_mode,
            css: [
                css_0,
                css_1,
                css_2,
                css_3,
                css_4,
                css_5
            ],
            eclipse_flag,
            sun_vec_b: [
                sun_vec_b_0,
                sun_vec_b_1,
                sun_vec_b_2
            ],
            i_b_field_meas: [
                i_b_field_meas_0,
                i_b_field_meas_1,
                i_b_field_meas_2
            ],
            bd: [
                bd_0,
                bd_1,
                bd_2
            ],
            rws_speed_cmd: [
                rws_speed_cmd_0,
                rws_speed_cmd_1,
                rws_speed_cmd_2
            ],
            rws_speed_tach: [
                rws_speed_tach_0,
                rws_speed_tach_1,
                rws_speed_tach_2
            ],
            rwa_torque_cmd: [
                rwa_torque_cmd_0,
                rwa_torque_cmd_1,
                rwa_torque_cmd_2
            ],
            gc_rwa_torque_cmd: [
                gc_rwa_torque_cmd_0,
                gc_rwa_torque_cmd_1,
                gc_rwa_torque_cmd_2
            ],
            torque_coil_cmd: [
                torque_coil_cmd_0,
                torque_coil_cmd_1,
                torque_coil_cmd_2
            ],
            gc_torque_coil_cmd: [
                gc_torque_coil_cmd_0,
                gc_torque_coil_cmd_1,
                gc_torque_coil_cmd_2
            ],
            qbo_cmd: [
                qbo_cmd_0,
                qbo_cmd_1,
                qbo_cmd_2,
                qbo_cmd_3
            ],
            qbo_hat: [
                qbo_hat_0,
                qbo_hat_1,
                qbo_hat_2,
                qbo_hat_3
            ],
            angle_to_go,
            q_error: [
                q_error_0,
                q_error_1,
                q_error_2,
                q_error_3
            ],
            omega_b: [
                omega_b_0,
                omega_b_1,
                omega_b_2
            ],
            rotating_variable_a,
            rotating_variable_b,
            rotating_variable_c,
            nb: [
                nb_0,
                nb_1,
                nb_2
            ],
            neci: [
                neci_0,
                neci_1,
                neci_2
            ],
            crc
        })
    )
);

/// Raw accelerometer and gyroscope data
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RawIMU {
    /// Message Header
    pub hdr: MessageHeader,
    /// Accelerometer (X, Y, Z)  (3.9 mg/lsb)
    pub accel: [i16; 3],
    /// Gyroscope (X, Y, Z) (8.75 mdps/lsb)
    pub gyro: [i16; 3],
    /// Gyroscope temperature (-1C/lsb)
    pub gyro_temp: u8,
    /// Message checksum
    pub crc: u16,
}

impl RawIMU {
    /// Constructor. Converts a raw data array received from the MAI-400 into a usable structure
    pub fn new(msg: &[u8]) -> Self {
        raw_imu(msg).unwrap().1
    }
}

named!(raw_imu(&[u8]) -> RawIMU,
    do_parse!(
        sync: le_u16 >>
        data_len: le_u16 >>
        msg_id: le_u8 >>
        addr: le_u8 >>
        accel_x: le_i16 >>
        accel_y: le_i16 >>
        accel_z: le_i16 >>
        gyro_x: le_i16 >>
        gyro_y: le_i16 >>
        gyro_z: le_i16 >>
        gyro_temp: le_u8 >>
        crc: le_u16 >>
        (RawIMU {
                hdr: MessageHeader {
                    sync, 
                    data_len, 
                    msg_id, 
                    addr
                },
                accel: [accel_x ,accel_y, accel_z],
                gyro: [gyro_x, gyro_y, gyro_z],
                gyro_temp,
                crc
        })
    )
);

/// IR Earth Horizon Sensor telemetry data
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IREHSTelemetry {
    /// Message Header
    pub hdr: MessageHeader,
    /// IREHS A Thermopile Sensors 12-bit ADC (0.8059 mV per lsb)
    /// [earth limb, earth reference, space reference, wide FOV]
    pub thermopiles_a: [u16; 4],
    /// IREHS B Thermopile Sensors 12-bit ADC (0.8059 mV per lsb)
    /// [earth limb, earth reference, space reference, wide FOV]
    pub thermopiles_b: [u16; 4],
    /// Thermistor Temp 12-bit ADC for A (0.8059 mV per lsb)
    /// [earth limb, earth reference, space reference, wide FOV]
    pub temp_a: [u16; 4],
    /// Thermistor Temp 12-bit ADC for B (0.8059 mV per lsb)
    /// [earth limb, earth reference, space reference, wide FOV]
    pub temp_b: [u16; 4],
    /// Calculated dip angle of Earth limb for A in degrees
    pub dip_angle_a: u32,
    /// Calculated dip angle of Earth limb for B in degrees
    pub dip_angle_b: u32,
    /// Degradation codes for thermopiles
    /// [{A}, {B}]
    pub solution_degraded: [ThermopileFlags; 8],
    /// Message checksum
    pub crc: u16,
}

impl IREHSTelemetry {
    /// Constructor. Converts a raw data array received from the MAI-400 into a usable structure
    pub fn new(msg: &[u8]) -> Self {
        irehs_telem(msg).unwrap().1
    }
}

named!(irehs_telem(&[u8]) -> IREHSTelemetry,
    do_parse!(
        sync: le_u16 >>
        data_len: le_u16 >>
        msg_id: le_u8 >>
        addr: le_u8 >>
        thermopiles_a_earth_limb: le_u16 >>
        thermopiles_a_earth_ref: le_u16 >>
        thermopiles_a_space_ref: le_u16 >>
        thermopiles_a_wide_fov: le_u16 >>
        thermopiles_b_earth_limb: le_u16 >>
        thermopiles_b_earth_ref: le_u16 >>
        thermopiles_b_space_ref: le_u16 >>
        thermopiles_b_wide_fov: le_u16 >>
        temp_a_earth_limb: le_u16 >>
        temp_a_earth_ref: le_u16 >>
        temp_a_space_ref: le_u16 >>
        temp_a_wide_fov: le_u16 >>
        temp_b_earth_limb: le_u16 >>
        temp_b_earth_ref: le_u16 >>
        temp_b_space_ref: le_u16 >>
        temp_b_wide_fov: le_u16 >>
        dip_angle_a: le_u32 >>
        dip_angle_b: le_u32 >>
        solution_degraded_earth_limb_a: le_u8 >>
        solution_degraded_earth_ref_a: le_u8 >>
        solution_degraded_space_ref_a: le_u8 >>
        solution_degraded_wide_fov_a: le_u8 >>
        solution_degraded_earth_limb_b: le_u8 >>
        solution_degraded_earth_ref_b: le_u8 >>
        solution_degraded_space_ref_b: le_u8 >>
        solution_degraded_wide_fov_b: le_u8 >>
        crc: le_u16 >>
        (IREHSTelemetry {
                hdr: MessageHeader {
                    sync, 
                    data_len, 
                    msg_id, 
                    addr
                },
                thermopiles_a: [
                    thermopiles_a_earth_limb,
                    thermopiles_a_earth_ref,
                    thermopiles_a_space_ref,
                    thermopiles_a_wide_fov
                ],
                thermopiles_b: [
                    thermopiles_b_earth_limb,
                    thermopiles_b_earth_ref,
                    thermopiles_b_space_ref,
                    thermopiles_b_wide_fov
                ],
                temp_a: [
                    temp_a_earth_limb,
                    temp_a_earth_ref,
                    temp_a_space_ref,
                    temp_a_wide_fov
                ],
                temp_b: [
                    temp_b_earth_limb,
                    temp_b_earth_ref,
                    temp_b_space_ref,
                    temp_b_wide_fov
                ],
                dip_angle_a,
                dip_angle_b,
                solution_degraded: [
                    ThermopileFlags::from_bits_truncate(solution_degraded_earth_limb_a),
                    ThermopileFlags::from_bits_truncate(solution_degraded_earth_ref_a),
                    ThermopileFlags::from_bits_truncate(solution_degraded_space_ref_a),
                    ThermopileFlags::from_bits_truncate(solution_degraded_wide_fov_a),
                    ThermopileFlags::from_bits_truncate(solution_degraded_earth_limb_b),
                    ThermopileFlags::from_bits_truncate(solution_degraded_earth_ref_b),
                    ThermopileFlags::from_bits_truncate(solution_degraded_space_ref_b),
                    ThermopileFlags::from_bits_truncate(solution_degraded_wide_fov_b)
                ],
                crc,
        })
    )
);

bitflags! {
    /// Thermopile error flags
    #[derive(Default)]
    pub struct ThermopileFlags: u8 {
        /// Dip angle exceeded senser system limit
        const DIP_ANGLE_LIMIT = 0x01;
        /// Sun in FOV using sun ephemeris
        const SUN_IN_EPHEMERIS = 0x02;
        /// Thermopile is saturated
        const THERMOPILE_SAT = 0x04;
        /// No communications
        const NO_COMM = 0x08;
        /// Earth reference is bad
        const BAD_EARTH_REF = 0x10;
        /// Using auxiliary wide FOV sensor
        const AUX_WIDE_FOV = 0x20;
    }
}

/// ADCS configuration information
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConfigInfo {
    /// Message header
    pub hdr: MessageHeader,
    /// ADACS model number (should be `400`)
    pub model: u16,
    /// Device serial number
    pub serial: u16,
    /// Firmware major version number
    pub major: u8,
    /// Firmware minor version number
    pub minor: u8,
    /// Firmware build number
    pub build: u16,
    /// Number of earth horizon sensors
    pub n_ehs: u8,
    /// Type of earth horizon sensors
    pub ehs_type: EHSType,
    /// Number of star trackers
    pub n_st: u8,
    /// Type of star trackers
    pub st_type: StarTracker,
    /// Message checksum
    pub crc: u16,
}

impl ConfigInfo {
    /// Constructor. Converts a raw data array received from the MAI-400 into a usable structure
    pub fn new(msg: &[u8]) -> Self {
        configinfo(msg).unwrap().1
    }
}

named!(configinfo(&[u8]) -> ConfigInfo,
    do_parse!(
        sync: le_u16 >>
        data_len: le_u16 >>
        msg_id: le_u8 >>
        addr: le_u8 >>
        model: le_u16 >>
        serial: le_u16 >>
        major: le_u8 >>
        minor: le_u8 >>
        build: le_u16 >>
        n_ehs: le_u8 >> 
        ehs_type: le_u8 >>
        n_st: le_u8 >>
        st_type: le_u8 >>
        crc: le_u16 >>
        (ConfigInfo{
                hdr: MessageHeader {
                    sync, 
                    data_len, 
                    msg_id, 
                    addr
                }, 
                model, 
                serial, 
                major, 
                minor,
                build, 
                n_ehs,
                ehs_type: match ehs_type {
                    0 => EHSType::Internal,
                    _ => EHSType::External
                },
                n_st,
                st_type: match st_type {
                    0 => StarTracker::MAISextant,
                    _ => StarTracker::Vectronic,
                }, 
                crc})
    )
);

/// Messages sent by the MAI-400
#[derive(Debug, PartialEq)]
pub enum Response {
    /// Standard telemetry message
    StdTelem(StandardTelemetry),
    /// Raw IMU message
    IMU(RawIMU),
    /// IREHS telemetry message
    IREHS(IREHSTelemetry),
    /// Device configuration information
    Config(ConfigInfo),
}

/// Type of earth horizon sensor
#[derive(Clone, Debug, PartialEq)]
pub enum EHSType {
    /// EHS type has not been successfully fetched yet
    Unknown,
    /// Internal MAI IREHS
    Internal,
    /// External EHS
    External,
}

impl Default for EHSType {
    fn default() -> Self {
        EHSType::Unknown
    }
}

/// Type of star tracker
#[derive(Clone, Debug, PartialEq)]
pub enum StarTracker {
    /// Star tracker type has not been successfully fetched yet
    Unknown,
    /// MAI space sextant
    MAISextant,
    /// Vectronic VST41-M
    Vectronic,
}

impl Default for StarTracker {
    fn default() -> Self {
        StarTracker::Unknown
    }
}


/// Structure to contain all possible variables which can be returned
/// by the standard telemetry message's `rotating_variable` fields
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RotatingTelemetry {
    /// IGRF magnetic fields (X, Y, Z) (Tesla)
    pub b_field_igrf: [f32; 3],
    /// ECI Sun Vector from Ephemeris (X, Y, Z) (Unit)
    pub sun_vec_eph: [f32; 3],
    /// ECI Spacecraft Position (X, Y, Z) (km)
    pub sc_pos_eci: [f32; 3],
    /// ECI Spacecraft Velocity (X, Y, Z) (km)
    pub sc_vel_eci: [f32; 3],
    /// Keplerian elements
    pub kepler_elem: KeplerElem,
    /// Bdot Gain Acquisition Mode (X, Y, Z)
    pub k_bdot: [f32; 3],
    /// Proportional Gain Normal Mode (X, Y, Z)
    pub kp: [f32; 3],
    /// Derivative Gain Normal Mode (X, Y, Z)
    pub kd: [f32; 3],
    /// Unloading Gain Normal Mode (X, Y, Z)
    pub k_unload: [f32; 3],
    /// CSS{n} Bias (1, 2, 3, 4, 5, 6)
    pub css_bias: [i16; 6],
    /// MAG Bias (X, Y, Z)
    pub mag_bias: [i16; 3],
    /// RWS Bus Voltage (0.00483516483 v/lsb)
    pub rws_volt: i16,
    /// Reserved
    pub rws_press: i16,
    /// Attitude Determination Mode
    pub att_det_mode: u8,
    /// RWS Reset Counter (X, Y, Z)
    pub rws_reset_cntr: [u8; 3],
    /// Sun and Mag Field are aligned
    pub sun_mag_aligned: u8,
    /// Software Minor Version
    pub minor_version: u8,
    /// Software Unit Serial Number
    pub mai_sn: u8,
    /// Orbit Propagation Mode
    pub orbit_prop_mode: u8,
    /// ACS Mode in Operation
    pub acs_op_mode: u8,
    /// ADACS Processor Reset Counter
    pub proc_reset_cntr: u8,
    /// Software Major Version
    pub major_version: u8,
    /// ADS Mode in Operation
    pub ads_op_mode: u8,
    /// CSS{n} Gain (1, 2, 3, 4, 5, 6)
    pub css_gain: [f32; 6],
    /// Mag Gain (X, Y, Z)
    pub mag_gain: [f32; 3],
    /// Epoch of Current Orbit (GPS sec)
    pub orbit_epoch: u32,
    /// True Anomaly at Epoch – Kepler (deg)
    pub true_anomoly_epoch: f32,
    /// Epoch of Next Updated RV (GPS sec)
    pub orbit_epoch_next: u32,
    /// ECI Position at Next Epoch (X, Y, Z) (km)
    pub sc_pos_eci_epoch: [f32; 3],
    /// ECI Velocity at Next Epoch (X, Y, Z) (km/sec)
    pub sc_vel_eci_epoch: [f32; 3],
    /// QbX Wheel Speed Command (rpm)
    pub qb_x_wheel_speed: i16,
    /// QbX Filter Gain
    pub qb_x_filter_gain: f32,
    /// QbX Dipole Gain
    pub qb_x_dipole_gain: f32,
    /// Dipole Gain (X, Y, Z)
    pub dipole_gain: [f32; 3],
    /// Wheel Speed Bias (X, Y, Z) (rpm)
    pub wheel_speed_bias: [i16; 3],
    /// Cosine of Sun/Mag Align Threshold Angle
    pub cos_sun_mag_align_thresh: f32,
    /// Max AngleToGo for Unloading (rad)
    pub unload_ang_thresh: f32,
    /// Quaternion feedback saturation.
    pub q_sat: f32,
    /// Maximum RWA Torque (mNm)
    pub raw_trq_max: f32,
    /// Reaction Wheel Motor Current (X, Y, Z) (A) (0.0003663003663 A/lsb)
    pub rws_motor_current: [u16; 3],
    /// RWS Motor Temperature (Temperature oC = rwsMotorTemp * 0.0402930 - 50)
    pub raw_motor_temp: i16,
}

impl RotatingTelemetry {
    /// Extract the self variables from a standard telemetry message and update
    /// the appropriate corresponding fields in a [`selfTelemetry`] structure
    ///
    /// # Arguments
    ///
    /// * msg - Standard telemetry message to extract variables from
    /// * self - self variables structure to copy extracted data into
    ///
    /// # Errors
    ///
    /// If errors are encountered, the structure will not be updated
    ///
    /// # Examples
    ///
    /// ```
    /// # use mai400_api::*;
    /// # fn func() -> MAIResult<()> {
    /// # let connection = Connection::new("/dev/ttyS5".to_owned());
    /// let mai = MAI400::new(connection);
    ///
    /// let mut rotating = RotatingTelemetry::default();
    ///
    /// let msg = mai.get_message()?;
    /// match msg {
    ///     Response::StdTelem(telem) => {
    ///         rotating.update(&telem);
    ///     }
    ///     _ => {}
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`MAIError`]: enum.MAIError.html
    ///
    /// [`selfTelemetry`]: struct.selfTelemetry.html
    // TODO: verify the bit shifting
    // TODO: Doc says 3 MSB are used for version information. Need to extract
    pub fn update(&mut self, msg: &StandardTelemetry) {
        match msg.tlm_counter {
            0 => {
                self.b_field_igrf[0] = f32::from_bits(msg.rotating_variable_a);
                self.b_field_igrf[1] = f32::from_bits(msg.rotating_variable_b);
                self.b_field_igrf[2] = f32::from_bits(msg.rotating_variable_c);
            }
            1 => {
                self.sun_vec_eph[0] = f32::from_bits(msg.rotating_variable_a);
                self.sun_vec_eph[1] = f32::from_bits(msg.rotating_variable_b);
                self.sun_vec_eph[2] = f32::from_bits(msg.rotating_variable_c);
            }
            2 => {
                self.sc_pos_eci[0] = f32::from_bits(msg.rotating_variable_a);
                self.sc_pos_eci[1] = f32::from_bits(msg.rotating_variable_b);
                self.sc_pos_eci[2] = f32::from_bits(msg.rotating_variable_c);
            }
            3 => {
                self.sc_vel_eci[0] = f32::from_bits(msg.rotating_variable_a);
                self.sc_vel_eci[1] = f32::from_bits(msg.rotating_variable_b);
                self.sc_vel_eci[2] = f32::from_bits(msg.rotating_variable_c);
            }
            4 => {
                self.kepler_elem.semi_major_axis = f32::from_bits(msg.rotating_variable_a);
                self.kepler_elem.eccentricity = f32::from_bits(msg.rotating_variable_b);
                self.kepler_elem.inclination = f32::from_bits(msg.rotating_variable_c);
            }
            5 => {
                self.kepler_elem.raan = f32::from_bits(msg.rotating_variable_a);
                self.kepler_elem.arg_parigee = f32::from_bits(msg.rotating_variable_b);
                self.kepler_elem.true_anomoly = f32::from_bits(msg.rotating_variable_c);
            }
            6 => {
                self.k_bdot[0] = f32::from_bits(msg.rotating_variable_a);
                self.k_bdot[1] = f32::from_bits(msg.rotating_variable_b);
                self.k_bdot[2] = f32::from_bits(msg.rotating_variable_c);
            }
            7 => {
                self.kp[0] = f32::from_bits(msg.rotating_variable_a);
                self.kp[1] = f32::from_bits(msg.rotating_variable_b);
                self.kp[2] = f32::from_bits(msg.rotating_variable_c);
            }
            8 => {
                self.kd[0] = f32::from_bits(msg.rotating_variable_a);
                self.kd[1] = f32::from_bits(msg.rotating_variable_b);
                self.kd[2] = f32::from_bits(msg.rotating_variable_c);
            }
            9 => {
                self.k_unload[0] = f32::from_bits(msg.rotating_variable_a);
                self.k_unload[1] = f32::from_bits(msg.rotating_variable_b);
                self.k_unload[2] = f32::from_bits(msg.rotating_variable_c);
            }
            10 => {
                self.css_bias[0] = msg.rotating_variable_a.wrapping_shr(16) as i16;
                self.css_bias[1] = msg.rotating_variable_b.wrapping_shr(16) as i16;
                self.css_bias[2] = msg.rotating_variable_c.wrapping_shr(16) as i16;
                self.css_bias[3] = msg.rotating_variable_a as i16;
                self.css_bias[4] = msg.rotating_variable_b as i16;
                self.css_bias[5] = msg.rotating_variable_c as i16;
            }
            11 => {
                self.mag_bias[0] = msg.rotating_variable_a.wrapping_shr(16) as i16;
                self.mag_bias[1] = msg.rotating_variable_b.wrapping_shr(16) as i16;
                self.mag_bias[2] = msg.rotating_variable_c.wrapping_shr(16) as i16;
                self.rws_volt = msg.rotating_variable_a as i16;
                self.rws_press = msg.rotating_variable_b as i16;
            }
            12 => {
                self.att_det_mode = msg.rotating_variable_a.wrapping_shr(24) as u8;
                self.rws_reset_cntr[0] = msg.rotating_variable_a.wrapping_shr(16) as u8;
                self.sun_mag_aligned = msg.rotating_variable_a.wrapping_shr(8) as u8;
                self.minor_version = msg.rotating_variable_a as u8;
                self.mai_sn = msg.rotating_variable_b.wrapping_shr(24) as u8;
                self.rws_reset_cntr[1] = msg.rotating_variable_b.wrapping_shr(16) as u8;
                self.orbit_prop_mode = msg.rotating_variable_b.wrapping_shr(8) as u8;
                self.acs_op_mode = msg.rotating_variable_b as u8;
                self.proc_reset_cntr = msg.rotating_variable_c.wrapping_shr(24) as u8;
                self.rws_reset_cntr[2] = msg.rotating_variable_c.wrapping_shr(16) as u8;
                self.major_version = msg.rotating_variable_c.wrapping_shr(8) as u8;
                self.ads_op_mode = msg.rotating_variable_c as u8;
            }
            13 => {
                self.css_gain[0] = f32::from_bits(msg.rotating_variable_a);
                self.css_gain[1] = f32::from_bits(msg.rotating_variable_b);
                self.css_gain[2] = f32::from_bits(msg.rotating_variable_c);
            }
            14 => {
                self.css_gain[3] = f32::from_bits(msg.rotating_variable_a);
                self.css_gain[4] = f32::from_bits(msg.rotating_variable_b);
                self.css_gain[5] = f32::from_bits(msg.rotating_variable_c);
            }
            15 => {
                self.mag_gain[0] = f32::from_bits(msg.rotating_variable_a);
                self.mag_gain[1] = f32::from_bits(msg.rotating_variable_b);
                self.mag_gain[2] = f32::from_bits(msg.rotating_variable_c);
            }
            16 => {
                self.orbit_epoch = msg.rotating_variable_a as u32;
                self.true_anomoly_epoch = f32::from_bits(msg.rotating_variable_b);
                self.orbit_epoch_next = msg.rotating_variable_c as u32;
            }
            17 => {
                self.sc_pos_eci_epoch[0] = f32::from_bits(msg.rotating_variable_a);
                self.sc_pos_eci_epoch[1] = f32::from_bits(msg.rotating_variable_b);
                self.sc_pos_eci_epoch[2] = f32::from_bits(msg.rotating_variable_c);
            }
            18 => {
                self.sc_vel_eci_epoch[0] = f32::from_bits(msg.rotating_variable_a);
                self.sc_vel_eci_epoch[1] = f32::from_bits(msg.rotating_variable_b);
                self.sc_vel_eci_epoch[2] = f32::from_bits(msg.rotating_variable_c);
            }
            19 => {
                self.qb_x_wheel_speed = msg.rotating_variable_a.wrapping_shr(16) as i16;
                self.qb_x_filter_gain = f32::from_bits(msg.rotating_variable_b);
                self.qb_x_dipole_gain = f32::from_bits(msg.rotating_variable_c);
            }
            20 => {
                self.dipole_gain[0] = f32::from_bits(msg.rotating_variable_a);
                self.dipole_gain[1] = f32::from_bits(msg.rotating_variable_b);
                self.dipole_gain[2] = f32::from_bits(msg.rotating_variable_c);
            }
            21 => {
                self.wheel_speed_bias[0] = msg.rotating_variable_a.wrapping_shr(16) as i16;
                self.wheel_speed_bias[1] = msg.rotating_variable_b.wrapping_shr(16) as i16;
                self.wheel_speed_bias[2] = msg.rotating_variable_c.wrapping_shr(16) as i16;
            }
            22 => {
                self.cos_sun_mag_align_thresh = f32::from_bits(msg.rotating_variable_a);
                self.unload_ang_thresh = f32::from_bits(msg.rotating_variable_b);
                self.q_sat = f32::from_bits(msg.rotating_variable_c);
            }
            23 => {
                self.raw_trq_max = f32::from_bits(msg.rotating_variable_a);
                self.rws_motor_current[0] = msg.rotating_variable_b.wrapping_shr(16) as u16;
                self.rws_motor_current[1] = msg.rotating_variable_b as u16;
                self.rws_motor_current[2] = msg.rotating_variable_c.wrapping_shr(16) as u16;
                self.raw_motor_temp = msg.rotating_variable_c as i16;
            }
            _ => {}
        }
    }
}

/// Structure for keplarian elements returned in the standard telemetry message
#[derive(Clone, Debug, Default, PartialEq)]
pub struct KeplerElem {
    /// Semi major axis (km)
    pub semi_major_axis: f32,
    /// Eccentricity
    pub eccentricity: f32,
    /// Inclination (deg)
    pub inclination: f32,
    /// Right ascension of ascending node (deg)
    pub raan: f32,
    /// Argument of perigee (deg)
    pub arg_parigee: f32,
    /// True anomaly (deg)
    pub true_anomoly: f32,
}
