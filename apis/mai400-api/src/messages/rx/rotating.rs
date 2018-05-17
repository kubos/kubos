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

use super::*;

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
    pub rwa_trq_max: f32,
    /// Reaction Wheel Motor Current (X, Y, Z) (A) (0.0003663003663 A/lsb)
    pub rws_motor_current: [u16; 3],
    /// RWS Motor Temperature (Temperature oC = rwsMotorTemp * 0.0402930 - 50)
    pub rws_motor_temp: i16,
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
    /// let mai = MAI400::new("/dev/ttyS5")?;
    ///
    /// let mut rotating = RotatingTelemetry::default();
    ///
    /// let (std, _imu, _irehs) = mai.get_message()?;
    /// if std.is_some() {
    /// 	rotating.update(&std.unwrap());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`MAIError`]: enum.MAIError.html
    ///
    /// [`selfTelemetry`]: struct.selfTelemetry.html
    pub fn update(&mut self, msg: &StandardTelemetry) {
        // Note: The documentation says the 3 MSB are used for version information,
        // so we want to isolate just the rotating counter bits
        match msg.tlm_counter & 0x1F {
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
                self.css_bias[0] = msg.rotating_variable_a as i16;
                self.css_bias[1] = msg.rotating_variable_b as i16;
                self.css_bias[2] = msg.rotating_variable_c as i16;
                self.css_bias[3] = msg.rotating_variable_a.wrapping_shr(16) as i16;
                self.css_bias[4] = msg.rotating_variable_b.wrapping_shr(16) as i16;
                self.css_bias[5] = msg.rotating_variable_c.wrapping_shr(16) as i16;
            }
            11 => {
                self.mag_bias[0] = msg.rotating_variable_a as i16;
                self.mag_bias[1] = msg.rotating_variable_b as i16;
                self.mag_bias[2] = msg.rotating_variable_c as i16;
                self.rws_volt = msg.rotating_variable_a.wrapping_shr(16) as i16;
                self.rws_press = msg.rotating_variable_b.wrapping_shr(16) as i16;
            }
            12 => {
                self.att_det_mode = msg.rotating_variable_a as u8;
                self.rws_reset_cntr[0] = msg.rotating_variable_a.wrapping_shr(8) as u8;
                self.sun_mag_aligned = msg.rotating_variable_a.wrapping_shr(16) as u8;
                self.minor_version = msg.rotating_variable_a.wrapping_shr(24) as u8;
                self.mai_sn = msg.rotating_variable_b as u8;
                self.rws_reset_cntr[1] = msg.rotating_variable_b.wrapping_shr(8) as u8;
                self.orbit_prop_mode = msg.rotating_variable_b.wrapping_shr(16) as u8;
                self.acs_op_mode = msg.rotating_variable_b.wrapping_shr(24) as u8;
                self.proc_reset_cntr = msg.rotating_variable_c as u8;
                self.rws_reset_cntr[2] = msg.rotating_variable_c.wrapping_shr(8) as u8;
                self.major_version = msg.rotating_variable_c.wrapping_shr(16) as u8;
                self.ads_op_mode = msg.rotating_variable_c.wrapping_shr(24) as u8;
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
                self.wheel_speed_bias[0] = msg.rotating_variable_a as i16;
                self.wheel_speed_bias[1] = msg.rotating_variable_b as i16;
                self.wheel_speed_bias[2] = msg.rotating_variable_c as i16;
            }
            22 => {
                self.cos_sun_mag_align_thresh = f32::from_bits(msg.rotating_variable_a);
                self.unload_ang_thresh = f32::from_bits(msg.rotating_variable_b);
                self.q_sat = f32::from_bits(msg.rotating_variable_c);
            }
            23 => {
                self.rwa_trq_max = f32::from_bits(msg.rotating_variable_a);
                self.rws_motor_current[0] = msg.rotating_variable_b as u16;
                self.rws_motor_current[1] = msg.rotating_variable_b.wrapping_shr(16) as u16;
                self.rws_motor_current[2] = msg.rotating_variable_c as u16;
                self.rws_motor_temp = msg.rotating_variable_c.wrapping_shr(16) as i16;
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
