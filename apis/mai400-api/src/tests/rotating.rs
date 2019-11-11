/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(clippy::float_cmp)]

use super::*;

#[test]
fn update_0() {
    let input = StandardTelemetry {
        tlm_counter: 0,
        rotating_variable_a: 0x3F8CCCCD,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x40533333,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.b_field_igrf[0], 1.1);
    assert_eq!(rotating.b_field_igrf[1], 2.2);
    assert_eq!(rotating.b_field_igrf[2], 3.3);
}

#[test]
fn update_1() {
    let input = StandardTelemetry {
        tlm_counter: 1,
        rotating_variable_a: 0x3F8CCCCD,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x40533333,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.sun_vec_eph[0], 1.1);
    assert_eq!(rotating.sun_vec_eph[1], 2.2);
    assert_eq!(rotating.sun_vec_eph[2], 3.3);
}

#[test]
fn update_2() {
    let input = StandardTelemetry {
        tlm_counter: 2,
        rotating_variable_a: 0x3F8CCCCD,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x40533333,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.sc_pos_eci[0], 1.1);
    assert_eq!(rotating.sc_pos_eci[1], 2.2);
    assert_eq!(rotating.sc_pos_eci[2], 3.3);
}

#[test]
fn update_3() {
    let input = StandardTelemetry {
        tlm_counter: 3,
        rotating_variable_a: 0x3F8CCCCD,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x40533333,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.sc_vel_eci[0], 1.1);
    assert_eq!(rotating.sc_vel_eci[1], 2.2);
    assert_eq!(rotating.sc_vel_eci[2], 3.3);
}

#[test]
fn update_4() {
    let input = StandardTelemetry {
        tlm_counter: 4,
        rotating_variable_a: 0x3F8CCCCD,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x40533333,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.kepler_elem.semi_major_axis, 1.1);
    assert_eq!(rotating.kepler_elem.eccentricity, 2.2);
    assert_eq!(rotating.kepler_elem.inclination, 3.3);
}

#[test]
fn update_5() {
    let input = StandardTelemetry {
        tlm_counter: 5,
        rotating_variable_a: 0x3F8CCCCD,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x40533333,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.kepler_elem.raan, 1.1);
    assert_eq!(rotating.kepler_elem.arg_parigee, 2.2);
    assert_eq!(rotating.kepler_elem.true_anomoly, 3.3);
}

#[test]
fn update_6() {
    let input = StandardTelemetry {
        tlm_counter: 6,
        rotating_variable_a: 0x3F8CCCCD,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x40533333,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.k_bdot[0], 1.1);
    assert_eq!(rotating.k_bdot[1], 2.2);
    assert_eq!(rotating.k_bdot[2], 3.3);
}

#[test]
fn update_7() {
    let input = StandardTelemetry {
        tlm_counter: 7,
        rotating_variable_a: 0x3F8CCCCD,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x40533333,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.kp[0], 1.1);
    assert_eq!(rotating.kp[1], 2.2);
    assert_eq!(rotating.kp[2], 3.3);
}

#[test]
fn update_8() {
    let input = StandardTelemetry {
        tlm_counter: 8,
        rotating_variable_a: 0x3F8CCCCD,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x40533333,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.kd[0], 1.1);
    assert_eq!(rotating.kd[1], 2.2);
    assert_eq!(rotating.kd[2], 3.3);
}

#[test]
fn update_9() {
    let input = StandardTelemetry {
        tlm_counter: 9,
        rotating_variable_a: 0x3F8CCCCD,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x40533333,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.k_unload[0], 1.1);
    assert_eq!(rotating.k_unload[1], 2.2);
    assert_eq!(rotating.k_unload[2], 3.3);
}

#[test]
fn update_10() {
    let input = StandardTelemetry {
        tlm_counter: 10,
        rotating_variable_a: 0x03040102,
        rotating_variable_b: 0x07080506,
        rotating_variable_c: 0x0B0C090A,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.css_bias[0], 0x0102);
    assert_eq!(rotating.css_bias[3], 0x0304);
    assert_eq!(rotating.css_bias[1], 0x0506);
    assert_eq!(rotating.css_bias[4], 0x0708);
    assert_eq!(rotating.css_bias[2], 0x090A);
    assert_eq!(rotating.css_bias[5], 0x0B0C);
}

#[test]
fn update_11() {
    let input = StandardTelemetry {
        tlm_counter: 11,
        rotating_variable_a: 0x03040102,
        rotating_variable_b: 0x07080506,
        rotating_variable_c: 0x0B0C090A,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.mag_bias[0], 0x0102);
    assert_eq!(rotating.rws_volt, 0x0304);
    assert_eq!(rotating.mag_bias[1], 0x0506);
    assert_eq!(rotating.rws_press, 0x0708);
    assert_eq!(rotating.mag_bias[2], 0x090A);
}

#[test]
fn update_12() {
    let input = StandardTelemetry {
        tlm_counter: 12,
        rotating_variable_a: 0x04030201,
        rotating_variable_b: 0x08070605,
        rotating_variable_c: 0x0C0B0A09,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.att_det_mode, 0x01);
    assert_eq!(rotating.rws_reset_cntr[0], 0x02);
    assert_eq!(rotating.sun_mag_aligned, 0x03);
    assert_eq!(rotating.minor_version, 0x04);
    assert_eq!(rotating.mai_sn, 0x05);
    assert_eq!(rotating.rws_reset_cntr[1], 0x06);
    assert_eq!(rotating.orbit_prop_mode, 0x07);
    assert_eq!(rotating.acs_op_mode, 0x08);
    assert_eq!(rotating.proc_reset_cntr, 0x09);
    assert_eq!(rotating.rws_reset_cntr[2], 0x0A);
    assert_eq!(rotating.major_version, 0x0B);
    assert_eq!(rotating.ads_op_mode, 0x0C);
}

#[test]
fn update_13() {
    let input = StandardTelemetry {
        tlm_counter: 13,
        rotating_variable_a: 0x3F8CCCCD,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x40533333,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.css_gain[0], 1.1);
    assert_eq!(rotating.css_gain[1], 2.2);
    assert_eq!(rotating.css_gain[2], 3.3);
}

#[test]
fn update_14() {
    let input = StandardTelemetry {
        tlm_counter: 14,
        rotating_variable_a: 0x3F8CCCCD,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x40533333,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.css_gain[3], 1.1);
    assert_eq!(rotating.css_gain[4], 2.2);
    assert_eq!(rotating.css_gain[5], 3.3);
}

#[test]
fn update_15() {
    let input = StandardTelemetry {
        tlm_counter: 15,
        rotating_variable_a: 0x3F8CCCCD,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x40533333,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.mag_gain[0], 1.1);
    assert_eq!(rotating.mag_gain[1], 2.2);
    assert_eq!(rotating.mag_gain[2], 3.3);
}

#[test]
fn update_16() {
    let input = StandardTelemetry {
        tlm_counter: 16,
        rotating_variable_a: 0x01020304,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x090A0B0C,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.orbit_epoch, 0x01020304);
    assert_eq!(rotating.true_anomoly_epoch, 2.2);
    assert_eq!(rotating.orbit_epoch_next, 0x090A0B0C);
}

#[test]
fn update_17() {
    let input = StandardTelemetry {
        tlm_counter: 17,
        rotating_variable_a: 0x3F8CCCCD,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x40533333,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.sc_pos_eci_epoch[0], 1.1);
    assert_eq!(rotating.sc_pos_eci_epoch[1], 2.2);
    assert_eq!(rotating.sc_pos_eci_epoch[2], 3.3);
}

#[test]
fn update_18() {
    let input = StandardTelemetry {
        tlm_counter: 18,
        rotating_variable_a: 0x3F8CCCCD,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x40533333,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.sc_vel_eci_epoch[0], 1.1);
    assert_eq!(rotating.sc_vel_eci_epoch[1], 2.2);
    assert_eq!(rotating.sc_vel_eci_epoch[2], 3.3);
}

#[test]
fn update_19() {
    let input = StandardTelemetry {
        tlm_counter: 19,
        rotating_variable_a: 0x01020304,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x40533333,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.qb_x_wheel_speed, 0x0102);
    assert_eq!(rotating.qb_x_filter_gain, 2.2);
    assert_eq!(rotating.qb_x_dipole_gain, 3.3);
}

#[test]
fn update_20() {
    let input = StandardTelemetry {
        tlm_counter: 20,
        rotating_variable_a: 0x3F8CCCCD,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x40533333,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.dipole_gain[0], 1.1);
    assert_eq!(rotating.dipole_gain[1], 2.2);
    assert_eq!(rotating.dipole_gain[2], 3.3);
}

#[test]
fn update_21() {
    let input = StandardTelemetry {
        tlm_counter: 21,
        rotating_variable_a: 0x03040102,
        rotating_variable_b: 0x07080506,
        rotating_variable_c: 0x0B0C090A,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.wheel_speed_bias[0], 0x0102);
    assert_eq!(rotating.wheel_speed_bias[1], 0x0506);
    assert_eq!(rotating.wheel_speed_bias[2], 0x090A);
}

#[test]
fn update_22() {
    let input = StandardTelemetry {
        tlm_counter: 22,
        rotating_variable_a: 0x3F8CCCCD,
        rotating_variable_b: 0x400CCCCD,
        rotating_variable_c: 0x40533333,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.cos_sun_mag_align_thresh, 1.1);
    assert_eq!(rotating.unload_ang_thresh, 2.2);
    assert_eq!(rotating.q_sat, 3.3);
}

#[test]
fn update_23() {
    let input = StandardTelemetry {
        tlm_counter: 23,
        rotating_variable_a: 0x3F8CCCCD,
        rotating_variable_b: 0x07080506,
        rotating_variable_c: 0x0B0C090A,
        ..Default::default()
    };

    let mut rotating = RotatingTelemetry::default();

    rotating.update(&input);

    assert_eq!(rotating.rwa_trq_max, 1.1);
    assert_eq!(rotating.rws_motor_current[0], 0x0506);
    assert_eq!(rotating.rws_motor_current[1], 0x0708);
    assert_eq!(rotating.rws_motor_current[2], 0x090A);
    assert_eq!(rotating.rws_motor_temp, 0x0B0C);
}
