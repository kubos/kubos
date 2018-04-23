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
// Integration tests for the Kubos MAI400 API
//
// Note: The I2C commands are needed when the MAI is connected to the stack via
//       an AIM module. If no AIM is present, they should be removed

extern crate i2c_linux;
extern crate mai400_api;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_stream;
extern crate slog_term;

use i2c_linux::I2c;
use mai400_api::*;
use slog::{Drain, Logger};
use std::fs::File;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

const RV_POS_ECI: [f32; 3] = [1.1, 2.2, 3.3];
const RV_VEL_ECI: [f32; 3] = [4.4, 5.5, 6.6];
const RV_EPOCH: u32 = 1198800018;

fn i2c_cmds() {
    // Make sure the power line that goes to the MAI is turned on

    let mut i2c = I2c::from_path("/dev/i2c-1").unwrap();
    i2c.smbus_set_slave_address(0x1F, false).unwrap();

    i2c.i2c_write_block_data(0x03, &[0xF0]).unwrap();
    i2c.i2c_write_block_data(0x01, &[0x0E]).unwrap();
}

fn set_gps_time(mai: &MAI400, logger: &Logger) -> u8 {
    let gps_time = 1198800018;
    // Set GPS time to Jan 01, 2018
    let result = mai.set_gps_time(gps_time);

    if result.is_err() {
        error!(
            logger,
            "[Set GPS Time] Failed to send SetGPSTime command: {}",
            result.unwrap_err()
        );
        return 1;
    }

    let (telem, _imu, _irehs) = mai.get_message().unwrap();

    let rc = if let Some(std) = telem {
        // last command matches + gps time matches-ish
        if std.last_command == 0x44 {
            if std.gps_time >= gps_time {
                info!(logger, "[Set GPS Time] Test completed successfully");
                0
            } else {
                error!(
                    logger,
                    "[Set GPS Time] GPS time mismatch. Expected: {}, Found: {}",
                    gps_time,
                    std.gps_time
                );
                1
            }
        } else {
            error!(
                logger,
                "[Set GPS Time] Last command mismatch. Expected: {:X}, Found: {:X}",
                0x44,
                std.last_command
            );
            info!(logger, "gps_time: {}", std.gps_time);
            info!(logger, "time_subsec: {}", std.time_subsec);
            info!(logger, "cmd_valid_cntr: {}", std.cmd_valid_cntr);
            info!(logger, "cmd_invalid_cntr: {}", std.cmd_invalid_cntr);
            info!(
                logger,
                "cmd_invalid_chksum_cntr: {}",
                std.cmd_invalid_chksum_cntr
            );
            info!(logger, "last_command: {:X}", std.last_command);
            info!(logger, "acs_mode: {}", std.acs_mode);
            1
        }

    } else {
        error!(
            logger,
            "[Set GPS Time] Failed to read after sending command"
        );
        1
    };

    return rc;
}

fn set_mode(mai: &MAI400, logger: &Logger) -> u8 {
    // Mode = Test mode
    let mode = 0;

    let result = mai.set_mode(0, [0; 4]);

    if result.is_err() {
        error!(
            logger,
            "[Set Mode] Failed to send SetAcsMode command: {}",
            result.unwrap_err()
        );
        return 1;
    }

    let (telem, _imu, _irehs) = mai.get_message().unwrap();

    let rc = if let Some(std) = telem {
        if std.last_command == 0x0 {
            if std.acs_mode == mode {
                info!(logger, "[Set Mode] Test completed successfully");
                0
            } else {
                error!(
                    logger,
                    "[Set Mode] Mode mismatch. Expected: {}, Found: {}",
                    mode,
                    std.acs_mode
                );
                1
            }
        } else {
            error!(
                logger,
                "[Set Mode] Last command mismatch. Expected: {:X}, Found: {:X}",
                0x0,
                std.last_command
            );
            1
        }
    } else {
        error!(logger, "[Set Mode] Failed to read after sending command");
        1
    };

    return rc;
}

fn set_mode_sun(mai: &MAI400, logger: &Logger) -> u8 {
    // Mode = Normal-sun mode
    let mode = 7;

    let result = mai.set_mode_sun(7, 0, 15.0);

    if result.is_err() {
        error!(
            logger,
            "[Set Mode (Sun)] Failed to send SetAcsMode command: {}",
            result.unwrap_err()
        );
        return 1;
    }

    let (telem, _imu, _irehs) = mai.get_message().unwrap();

    let rc = if let Some(std) = telem {

        if std.last_command == 0x0 {
            if std.acs_mode == mode {
                info!(logger, "[Set Mode (Sun)] Test completed successfully");
                0
            } else {
                error!(
                    logger,
                    "[Set Mode (Sun)] Mode mismatch. Expected: {}, Found: {}",
                    mode,
                    std.acs_mode
                );
                1
            }
        } else {
            error!(
                logger,
                "[Set Mode (Sun)] Last command mismatch. Expected: {:X}, Found: {:X}",
                0x0,
                std.acs_mode
            );
            1
        }
    } else {
        error!(
            logger,
            "[Set Mode (Sun)] Failed to read after sending command"
        );
        1
    };

    return rc;
}

fn set_rv(mai: &MAI400, logger: &Logger) -> u8 {

    // TODO: Set real values
    let result = mai.set_rv(RV_POS_ECI, RV_VEL_ECI, RV_EPOCH);

    if result.is_err() {
        error!(
            logger,
            "[Set RV] Failed to send SetRV command: {}",
            result.unwrap_err()
        );
        return 1;
    }

    let (telem, _imu, _irehs) = mai.get_message().unwrap();

    let rc = if let Some(std) = telem {

        if std.last_command == 0x41 {
            // The other telemetry values we can check are part of the rotating variable set,
            // so we'll need to wait to vrify them until we've read in all of the rotating variables.
            // Report successful for now.
            0
        } else {
            error!(
                logger,
                "[Set RV] Last command mismatch. Expected: {:X}, Found: {:X}",
                0x41,
                std.acs_mode
            );
            1
        }
    } else {
        error!(logger, "[Set RV] Failed to read after sending command");
        1
    };
    return rc;
}

fn passthrough(mai: &MAI400, logger: &Logger) -> u8 {

    let msg_id = 0x5A;

    let mut array = [0; 40];
    array[0] = 0x90; // SYNC byte 1
    array[1] = 0xEB; // SYNC byte 2
    array[2] = msg_id; // Request Reset command
    array[38] = 0xD5; // CRC byte 1
    array[39] = 0x01; // CRC byte 2

    let result = mai.passthrough(&array);

    if result.is_err() {
        error!(
            logger,
            "[Passthrough] Failed to send passthrough packet: {}",
            result.unwrap_err()
        );
        return 1;
    }

    let (telem, _imu, _irehs) = mai.get_message().unwrap();

    let rc = if let Some(std) = telem {

        if std.last_command == msg_id {
            info!(logger, "[Passthrough] Test completed successfully");
            0
        } else {
            error!(
                logger,
                "[Passthrough] Last command mismatch. Expected: {:X}, Found: {:X}",
                msg_id,
                std.acs_mode
            );
            1
        }
    } else {
        error!(logger, "[Passthrough] Failed to read after sending command");
        1
    };

    return rc;
}

fn reset(mai: &MAI400, logger: &Logger) -> u8 {
    let result = mai.reset();

    if result.is_err() {
        error!(
            logger,
            "[Reset] Failed to send Reset commands: {}",
            result.unwrap_err()
        );
        return 1;
    }

    let (telem, _imu, _irehs) = mai.get_message().unwrap();

    let rc = if let Some(std) = telem {
        let cmds = std.cmd_valid_cntr + std.cmd_invalid_cntr + std.cmd_invalid_chksum_cntr;
        if cmds == 0 {
            info!(logger, "[Reset] Test completed successfully");
            0
        } else {
            error!(
                logger,
                "[Reset] System reporting a non-zero number of received commands: {}",
                cmds
            );
            1
        }
    } else {
        error!(logger, "[Reset] Failed to read after sending command");
        1
    };

    return rc;
}

fn read(mai: &MAI400, logger: &Logger) -> u8 {

    let mut rc = 0;

    // Read loop test
    let main_exit = Arc::new(AtomicBool::new(false));
    let thread_exit = main_exit.clone();

    let (sender, receiver) = channel();
    let thread_logger = logger.clone();

    let handle = thread::spawn(move || read_loop(thread_exit, &thread_logger, sender));

    // Let read loop run for 10 seconds to ensure that we get all of the
    // rotating variable values
    thread::sleep(Duration::new(10, 0));

    // Kill the read thread
    main_exit.store(true, Ordering::Relaxed);

    let (std, imu, irehs, rotating) = receiver.recv().unwrap();

    handle.join().unwrap();

    if let Some(telem) = std {
        // Print standard telem
        info!(logger, "Standard Telemetry:");
        info!(logger, "tlm_counter: {}", telem.tlm_counter);
        info!(logger, "gps_time: {}", telem.gps_time);
        info!(logger, "time_subsec: {}", telem.time_subsec);
        info!(logger, "cmd_valid_cntr: {}", telem.cmd_valid_cntr);
        info!(logger, "cmd_invalid_cntr: {}", telem.cmd_invalid_cntr);
        info!(
            logger,
            "cmd_invalid_chksum_cntr: {}",
            telem.cmd_invalid_chksum_cntr
        );
        info!(logger, "last_command: {:X}", telem.last_command);
        info!(logger, "acs_mode: {}", telem.acs_mode);
        info!(logger, "css_0: {}", telem.css[0]);
        info!(logger, "css_1: {}", telem.css[1]);
        info!(logger, "css_2: {}", telem.css[2]);
        info!(logger, "css_3: {}", telem.css[3]);
        info!(logger, "css_4: {}", telem.css[4]);
        info!(logger, "css_5: {}", telem.css[5]);
        info!(logger, "eclipse_flag: {}", telem.eclipse_flag);
        info!(logger, "sun_vec_b_0: {}", telem.sun_vec_b[0]);
        info!(logger, "sun_vec_b_1: {}", telem.sun_vec_b[1]);
        info!(logger, "sun_vec_b_2: {}", telem.sun_vec_b[2]);
        info!(logger, "i_b_field_meas_0: {}", telem.i_b_field_meas[0]);
        info!(logger, "i_b_field_meas_1: {}", telem.i_b_field_meas[1]);
        info!(logger, "i_b_field_meas_2: {}", telem.i_b_field_meas[2]);
        info!(logger, "bd_0: {}", telem.bd[0]);
        info!(logger, "bd_1: {}", telem.bd[1]);
        info!(logger, "bd_2: {}", telem.bd[2]);
        info!(logger, "rws_speed_cmd_0: {}", telem.rws_speed_cmd[0]);
        info!(logger, "rws_speed_cmd_1: {}", telem.rws_speed_cmd[1]);
        info!(logger, "rws_speed_cmd_2: {}", telem.rws_speed_cmd[2]);
        info!(logger, "rws_speed_tach_0: {}", telem.rws_speed_tach[0]);
        info!(logger, "rws_speed_tach_1: {}", telem.rws_speed_tach[1]);
        info!(logger, "rws_speed_tach_2: {}", telem.rws_speed_tach[2]);
        info!(logger, "rwa_torque_cmd_0: {}", telem.rwa_torque_cmd[0]);
        info!(logger, "rwa_torque_cmd_1: {}", telem.rwa_torque_cmd[1]);
        info!(logger, "rwa_torque_cmd_2: {}", telem.rwa_torque_cmd[2]);
        info!(
            logger,
            "gc_rwa_torque_cmd_0: {}",
            telem.gc_rwa_torque_cmd[0]
        );
        info!(
            logger,
            "gc_rwa_torque_cmd_1: {}",
            telem.gc_rwa_torque_cmd[1]
        );
        info!(
            logger,
            "gc_rwa_torque_cmd_2: {}",
            telem.gc_rwa_torque_cmd[2]
        );
        info!(logger, "torque_coil_cmd_0: {}", telem.torque_coil_cmd[0]);
        info!(logger, "torque_coil_cmd_1: {}", telem.torque_coil_cmd[1]);
        info!(logger, "torque_coil_cmd_2: {}", telem.torque_coil_cmd[2]);
        info!(
            logger,
            "gc_torque_coil_cmd_0: {}",
            telem.gc_torque_coil_cmd[0]
        );
        info!(
            logger,
            "gc_torque_coil_cmd_1: {}",
            telem.gc_torque_coil_cmd[1]
        );
        info!(
            logger,
            "gc_torque_coil_cmd_2: {}",
            telem.gc_torque_coil_cmd[2]
        );
        info!(logger, "qbo_cmd_0: {}", telem.qbo_cmd[0]);
        info!(logger, "qbo_cmd_1: {}", telem.qbo_cmd[1]);
        info!(logger, "qbo_cmd_2: {}", telem.qbo_cmd[2]);
        info!(logger, "qbo_cmd_3: {}", telem.qbo_cmd[3]);
        info!(logger, "qbo_hat_0: {}", telem.qbo_hat[0]);
        info!(logger, "qbo_hat_1: {}", telem.qbo_hat[1]);
        info!(logger, "qbo_hat_2: {}", telem.qbo_hat[2]);
        info!(logger, "qbo_hat_3: {}", telem.qbo_hat[3]);
        info!(logger, "angle_to_go: {}", telem.angle_to_go);
        info!(logger, "q_error_0: {}", telem.q_error[0]);
        info!(logger, "q_error_1: {}", telem.q_error[1]);
        info!(logger, "q_error_2: {}", telem.q_error[2]);
        info!(logger, "q_error_3: {}", telem.q_error[3]);
        info!(logger, "omega_b_0: {}", telem.omega_b[0]);
        info!(logger, "omega_b_1: {}", telem.omega_b[1]);
        info!(logger, "omega_b_2: {}", telem.omega_b[2]);
        info!(logger, "nb_0: {}", telem.nb[0]);
        info!(logger, "nb_1: {}", telem.nb[1]);
        info!(logger, "nb_2: {}", telem.nb[2]);
        info!(logger, "neci_0: {}", telem.neci[0]);
        info!(logger, "neci_1: {}", telem.neci[1]);
        info!(logger, "neci_2: {}", telem.neci[2]);


        // Print rotating
        info!(logger, "Rotating variables:");
        let data: String = rotating
            .b_field_igrf
            .iter()
            .map(|x| format!(" {}", x))
            .collect();
        info!(logger, "b_field_igrf:{}", data);
        let data: String = rotating
            .sun_vec_eph
            .iter()
            .map(|x| format!(" {}", x))
            .collect();
        info!(logger, "sun_vec_eph:{}", data);
        let data: String = rotating
            .sc_pos_eci
            .iter()
            .map(|x| format!(" {}", x))
            .collect();
        info!(logger, "sc_pos_eci:{}", data);
        let data: String = rotating
            .sc_vel_eci
            .iter()
            .map(|x| format!(" {}", x))
            .collect();
        info!(logger, "sc_vel_eci:{}", data);
        info!(logger, "kepler_elem:");
        info!(
            logger,
            "    semi_major_axis: {}",
            rotating.kepler_elem.semi_major_axis
        );
        info!(
            logger,
            "    eccentricity: {}",
            rotating.kepler_elem.eccentricity
        );
        info!(
            logger,
            "    inclination: {}",
            rotating.kepler_elem.inclination
        );
        info!(logger, "    raan: {}", rotating.kepler_elem.raan);
        info!(
            logger,
            "    arg_parigee: {}",
            rotating.kepler_elem.arg_parigee
        );
        info!(
            logger,
            "    true_anomoly: {}",
            rotating.kepler_elem.true_anomoly
        );
        let data: String = rotating.k_bdot.iter().map(|x| format!(" {}", x)).collect();
        info!(logger, "k_bdot:{}", data);

        let data: String = rotating.kp.iter().map(|x| format!(" {}", x)).collect();
        info!(logger, "kp:{}", data);

        let data: String = rotating.kd.iter().map(|x| format!(" {}", x)).collect();
        info!(logger, "kd:{}", data);
        let data: String = rotating
            .k_unload
            .iter()
            .map(|x| format!(" {}", x))
            .collect();
        info!(logger, "k_unload:{}", data);

        let data: String = rotating
            .css_bias
            .iter()
            .map(|x| format!(" {}", x))
            .collect();
        info!(logger, "css_bias:{}", data);

        let data: String = rotating
            .mag_bias
            .iter()
            .map(|x| format!(" {}", x))
            .collect();
        info!(logger, "mag_bias:{}", data);

        let data: String = rotating
            .rws_reset_cntr
            .iter()
            .map(|x| format!(" {}", x))
            .collect();
        info!(logger, "rws_volt: {}", rotating.rws_volt);
        info!(logger, "rws_press: {}", rotating.rws_press);
        info!(logger, "att_det_mode: {}", rotating.att_det_mode);
        info!(logger, "rws_reset_cntr:{}", data);
        info!(logger, "sun_mag_aligned: {}", rotating.sun_mag_aligned);
        info!(logger, "minor_version: {}", rotating.minor_version);
        info!(logger, "mai_sn: {}", rotating.mai_sn);
        info!(logger, "orbit_prop_mode: {}", rotating.orbit_prop_mode);
        info!(logger, "acs_op_mode: {}", rotating.acs_op_mode);
        info!(logger, "proc_reset_cntr: {}", rotating.proc_reset_cntr);
        info!(logger, "major_version: {}", rotating.major_version);
        info!(logger, "ads_op_mode: {}", rotating.ads_op_mode);
        let data: String = rotating
            .css_gain
            .iter()
            .map(|x| format!(" {}", x))
            .collect();
        info!(logger, "css_gain:{}", data);

        let data: String = rotating
            .mag_gain
            .iter()
            .map(|x| format!(" {}", x))
            .collect();
        info!(logger, "mag_gain:{}", data);

        let data: String = rotating
            .sc_pos_eci_epoch
            .iter()
            .map(|x| format!(" {}", x))
            .collect();
        info!(logger, "orbit_epoch: {}", rotating.orbit_epoch);
        info!(
            logger,
            "true_anomoly_epoch: {}",
            rotating.true_anomoly_epoch
        );
        info!(logger, "orbit_epoch_next: {}", rotating.orbit_epoch_next);
        info!(logger, "sc_pos_eci_epoch:{}", data);

        let data: String = rotating
            .sc_vel_eci_epoch
            .iter()
            .map(|x| format!(" {}", x))
            .collect();
        info!(logger, "sc_vel_eci_epoch:{}", data);
        info!(logger, "qb_x_wheel_speed: {}", rotating.qb_x_wheel_speed);
        info!(logger, "qb_x_filter_gain: {}", rotating.qb_x_filter_gain);
        info!(logger, "qb_x_dipole_gain: {}", rotating.qb_x_dipole_gain);
        let data: String = rotating
            .dipole_gain
            .iter()
            .map(|x| format!(" {}", x))
            .collect();
        info!(logger, "dipole_gain:{}", data);
        let data: String = rotating
            .wheel_speed_bias
            .iter()
            .map(|x| format!(" {}", x))
            .collect();
        info!(logger, "wheel_speed_bias:{}", data);
        info!(
            logger,
            "cos_sun_mag_align_thresh: {}",
            rotating.cos_sun_mag_align_thresh
        );
        info!(logger, "unload_ang_thresh: {}", rotating.unload_ang_thresh);
        info!(logger, "q_sat: {}", rotating.q_sat);
        info!(logger, "raw_trq_max: {}", rotating.raw_trq_max);
        let data: String = rotating
            .rws_motor_current
            .iter()
            .map(|x| format!(" {}", x))
            .collect();
        info!(logger, "rws_motor_current:{}", data);
        info!(logger, "raw_motor_temp: {}", rotating.raw_motor_temp);

        // Finish up 'Set RV' integration test
        rc += if rotating.orbit_epoch_next == RV_EPOCH {
            if rotating.sc_pos_eci_epoch == RV_POS_ECI {
                if rotating.sc_vel_eci_epoch == RV_VEL_ECI {
                    info!(logger, "[Set RV] Test completed successfully");
                    0
                } else {
                    error!(
                        logger,
                        "[Set RV] ECI velocity mismatch. Expected: {:?}, Found: {:?}",
                        RV_VEL_ECI,
                        rotating.sc_vel_eci_epoch
                    );
                    1
                }
            } else {
                error!(
                    logger,
                    "[Set RV] ECI position mismatch. Expected: {:?}, Found: {:?}",
                    RV_POS_ECI,
                    rotating.sc_pos_eci_epoch
                );
                1
            }
        } else {
            error!(
                logger,
                "[Set RV] ECI EPOCH mismatch. Expected: {}, Found: {}",
                RV_EPOCH,
                rotating.orbit_epoch_next
            );
            1
        }
    } else {
        error!(
            logger,
            "[Read] Failed to read any standard telemetry packets"
        );
        error!(logger, "[Set RV] Unable to determine test results");
        rc = 1;
    }

    if let Some(telem) = imu {
        info!(logger, "Raw IMU:");
        info!(
            logger,
            "accel: {} {} {}",
            telem.accel[0],
            telem.accel[1],
            telem.accel[2]
        );
        info!(
            logger,
            "gyro: {} {} {}",
            telem.gyro[0],
            telem.gyro[1],
            telem.gyro[2]
        );
        info!(logger, "gyro_temp: {}", telem.gyro_temp);
    } else {
        error!(logger, "[Read] Failed to read any raw IMU packets");
        rc = 1;
    }

    if let Some(telem) = irehs {
        info!(logger, "IREHS telemetry:");
        let data: String = telem
            .thermopiles_a
            .iter()
            .map(|x| format!(" {}", x))
            .collect();
        info!(logger, "thermopiles_a:{}", data);
        let data: String = telem
            .thermopiles_b
            .iter()
            .map(|x| format!(" {}", x))
            .collect();
        info!(logger, "thermopiles_b:{}", data);
        let data: String = telem.temp_a.iter().map(|x| format!(" {}", x)).collect();
        info!(logger, "temp_a:{}", data);
        let data: String = telem.temp_b.iter().map(|x| format!(" {}", x)).collect();
        info!(logger, "temp_b:{}", data);
        info!(logger, "dip_angle_a: {}", telem.dip_angle_a);
        info!(logger, "dip_angle_b: {}", telem.dip_angle_b);
        let data: String = telem
            .solution_degraded
            .iter()
            .map(|x| format!(" {:?}", x))
            .collect();
        info!(logger, "solution_degraded:{}", data);
    } else {
        error!(logger, "[Read] Failed to read any IREHS telemetry packets");
        rc = 1;
    }

    if rc == 0 {
        info!(logger, "[Read] Test completed successfully");
    }

    return rc;
}

fn read_loop(
    exit: Arc<AtomicBool>,
    logger: &Logger,
    sender: Sender<
        (Option<StandardTelemetry>,
         Option<RawIMU>,
         Option<IREHSTelemetry>,
         RotatingTelemetry),
    >,
) {
    let connection = Connection::new("/dev/ttyS5".to_owned());
    let mai = MAI400::new(connection);

    let mut std: Option<StandardTelemetry> = None;
    let mut imu: Option<RawIMU> = None;
    let mut irehs: Option<IREHSTelemetry> = None;
    let mut rotating = RotatingTelemetry::default();

    while !exit.load(Ordering::Relaxed) {
        let (new_std, new_imu, new_irehs) = mai.get_message().unwrap();

        if new_std.is_some() {
            std = new_std;
            rotating.update(&std.clone().unwrap());
        }
        if new_imu.is_some() {
            imu = new_imu;
        }
        if new_irehs.is_some() {
            irehs = new_irehs;
        }
    }

    sender.send((std, imu, irehs, rotating)).unwrap();
}

fn main() {

    let mut error_count: u8 = 0;

    // Output warning and error messages to stderr
    let decorator = slog_term::PlainSyncDecorator::new(std::io::stderr());
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let console_drain = slog::LevelFilter(drain, slog::Level::Warning);

    // Output all messages to the log file
    let file = File::create("mai400-results.txt").expect("Couldn't open log file");
    let decorator = slog_term::PlainSyncDecorator::new(file);
    let file_drain = Mutex::new(slog_term::FullFormat::new(decorator).build()).fuse();

    // Combine the loggers so we only have to make one logging call per message
    let logger = Logger::root(slog::Duplicate(console_drain, file_drain).fuse(), o!());

    // Make sure the MAI400 is powered
    i2c_cmds();

    // Give it a sec to make sure it's fully up
    thread::sleep(Duration::new(1, 0));

    // Initialize a connection with the device
    let connection = Connection::new("/dev/ttyS5".to_owned());
    let mai = MAI400::new(connection);

    info!(logger, "MAI400 Integration Tests");

    error_count += set_gps_time(&mai, &logger);
    error_count += set_mode(&mai, &logger);
    error_count += set_mode_sun(&mai, &logger);
    error_count += set_rv(&mai, &logger);
    error_count += passthrough(&mai, &logger);
    error_count += read(&mai, &logger);
    error_count += reset(&mai, &logger);

    info!(logger, "MAI400 Integration Tests Complete");

    if error_count == 0 {
        // Since we don't have the logger set up to print to stdout,
        // we need a manual println call here
        println!("MAI400 tests completed successfully");
        info!(logger, "MAI400 tests completed successfully");
    } else {
        eprintln!("One or more MAI400 tests have failed. See mai400-results.txt for info");
        warn!(logger, "One or more MAI400 tests have failed");
    }
}
