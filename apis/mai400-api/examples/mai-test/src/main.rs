extern crate mai400_api;
extern crate serial;

use mai400_api::*;
use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};

enum Message {
    Terminate,
}

fn read_loop(receiver: Arc<Mutex<mpsc::Receiver<Message>>>) {
    let connection = Connection::new("/dev/ttyS5".to_owned());
    let mai = MAI400::new(connection);

    let mut std = StandardTelemetry::default();
    let mut config = ConfigInfo::default();
    let mut imu = RawIMU::default();
    let mut irehs = IREHSTelemetry::default();

    let mut rotating = RotatingTelemetry::default();

    loop {
        let msg = mai.get_message().unwrap();
        match msg {
            Response::StdTelem(telem) => {
                rotating.update(&telem);
                std = telem.clone();
                println!("Got StdTelem");
            }
            Response::Config(cfg) => {
                config = cfg.clone();
                println!("Got config");
            }
            Response::IMU(telem) => {
                imu = telem.clone();
                println!("Got RawIMU");
            }
            Response::IREHS(telem) => {
                irehs = telem.clone();
                println!("Got IREHS");
            }
        }

        let message = receiver.lock().unwrap().recv().unwrap();
        match message {
            Message::Terminate => {
                println!("Killing thread");

                // Print config
                println!("\nCurrent config:\n-----------------------");
                println!("model: {}", config.model);
                println!("serial: {}", config.serial);
                println!("major: {}", config.major);
                println!("minor: {}", config.minor);
                println!("build: {}", config.build);
                println!("n_ehs: {}", config.n_ehs);
                println!("ehs_type: {:?}", config.ehs_type);
                println!("n_st: {}", config.n_st);
                println!("st_type: {:?}", config.st_type);
                println!("crc: {}", config.crc);

                // Print standard telem
                println!("\nStandard Telemetry:\n-----------------------");
                println!("tlm_counter: {}", std.tlm_counter);
                println!("gps_time: {}", std.gps_time);
                println!("time_subsec: {}", std.time_subsec);
                println!("cmd_valid_cntr: {}", std.cmd_valid_cntr);
                println!("cmd_invalid_cntr: {}", std.cmd_invalid_cntr);
                println!("cmd_invalid_chksum_cntr: {}", std.cmd_invalid_chksum_cntr);
                println!("last_command: {}", std.last_command);
                println!("acs_mode: {}", std.acs_mode);
                println!("css_0: {}", std.css[0]);
                println!("css_1: {}", std.css[1]);
                println!("css_2: {}", std.css[2]);
                println!("css_3: {}", std.css[3]);
                println!("css_4: {}", std.css[4]);
                println!("css_5: {}", std.css[5]);
                println!("eclipse_flag: {}", std.eclipse_flag);
                println!("sun_vec_b_0: {}", std.sun_vec_b[0]);
                println!("sun_vec_b_1: {}", std.sun_vec_b[1]);
                println!("sun_vec_b_2: {}", std.sun_vec_b[2]);
                println!("i_b_field_meas_0: {}", std.i_b_field_meas[0]);
                println!("i_b_field_meas_1: {}", std.i_b_field_meas[1]);
                println!("i_b_field_meas_2: {}", std.i_b_field_meas[2]);
                println!("bd_0: {}", std.bd[0]);
                println!("bd_1: {}", std.bd[1]);
                println!("bd_2: {}", std.bd[2]);
                println!("rws_speed_cmd_0: {}", std.rws_speed_cmd[0]);
                println!("rws_speed_cmd_1: {}", std.rws_speed_cmd[1]);
                println!("rws_speed_cmd_2: {}", std.rws_speed_cmd[2]);
                println!("rws_speed_tach_0: {}", std.rws_speed_tach[0]);
                println!("rws_speed_tach_1: {}", std.rws_speed_tach[1]);
                println!("rws_speed_tach_2: {}", std.rws_speed_tach[2]);
                println!("rwa_torque_cmd_0: {}", std.rwa_torque_cmd[0]);
                println!("rwa_torque_cmd_1: {}", std.rwa_torque_cmd[1]);
                println!("rwa_torque_cmd_2: {}", std.rwa_torque_cmd[2]);
                println!("gc_rwa_torque_cmd_0: {}", std.gc_rwa_torque_cmd[0]);
                println!("gc_rwa_torque_cmd_1: {}", std.gc_rwa_torque_cmd[1]);
                println!("gc_rwa_torque_cmd_2: {}", std.gc_rwa_torque_cmd[2]);
                println!("torque_coil_cmd_0: {}", std.torque_coil_cmd[0]);
                println!("torque_coil_cmd_1: {}", std.torque_coil_cmd[1]);
                println!("torque_coil_cmd_2: {}", std.torque_coil_cmd[2]);
                println!("gc_torque_coil_cmd_0: {}", std.gc_torque_coil_cmd[0]);
                println!("gc_torque_coil_cmd_1: {}", std.gc_torque_coil_cmd[1]);
                println!("gc_torque_coil_cmd_2: {}", std.gc_torque_coil_cmd[2]);
                println!("qbo_cmd_0: {}", std.qbo_cmd[0]);
                println!("qbo_cmd_1: {}", std.qbo_cmd[1]);
                println!("qbo_cmd_2: {}", std.qbo_cmd[2]);
                println!("qbo_cmd_3: {}", std.qbo_cmd[3]);
                println!("qbo_hat_0: {}", std.qbo_hat[0]);
                println!("qbo_hat_1: {}", std.qbo_hat[1]);
                println!("qbo_hat_2: {}", std.qbo_hat[2]);
                println!("qbo_hat_3: {}", std.qbo_hat[3]);
                println!("angle_to_go: {}", std.angle_to_go);
                println!("q_error_0: {}", std.q_error[0]);
                println!("q_error_1: {}", std.q_error[1]);
                println!("q_error_2: {}", std.q_error[2]);
                println!("q_error_3: {}", std.q_error[3]);
                println!("omega_b_0: {}", std.omega_b[0]);
                println!("omega_b_1: {}", std.omega_b[1]);
                println!("omega_b_2: {}", std.omega_b[2]);
                println!("nb_0: {}", std.nb[0]);
                println!("nb_1: {}", std.nb[1]);
                println!("nb_2: {}", std.nb[2]);
                println!("neci_0: {}", std.neci[0]);
                println!("neci_1: {}", std.neci[1]);
                println!("neci_2: {}", std.neci[2]);
                println!("crc: {}", std.crc);

                // Print rotating
                println!("\nRotating variables:\n-----------------------");
                print!("b_field_igrf:");
                for elem in rotating.b_field_igrf.iter() {
                    print!(" {}", elem);
                }
                print!("\nsun_vec_eph:");
                for elem in rotating.sun_vec_eph.iter() {
                    print!(" {}", elem);
                }
                print!("\nsc_pos_eci:");
                for elem in rotating.sc_pos_eci.iter() {
                    print!(" {}", elem);
                }
                print!("\nsc_vel_eci:");
                for elem in rotating.sc_vel_eci.iter() {
                    print!(" {}", elem);
                }
                println!("\nkepler_elem:");
                println!(
                    "    semi_major_axis: {}",
                    rotating.kepler_elem.semi_major_axis
                );
                println!("    eccentricity: {}", rotating.kepler_elem.eccentricity);
                println!("    inclination: {}", rotating.kepler_elem.inclination);
                println!("    raan: {}", rotating.kepler_elem.raan);
                println!("    arg_parigee: {}", rotating.kepler_elem.arg_parigee);
                println!("    true_anomoly: {}", rotating.kepler_elem.true_anomoly);
                print!("k_bdot:");
                for elem in rotating.k_bdot.iter() {
                    print!(" {}", elem);
                }
                print!("\nkp:");
                for elem in rotating.kp.iter() {
                    print!(" {}", elem);
                }
                print!("\nkd:");
                for elem in rotating.kd.iter() {
                    print!(" {}", elem);
                }
                print!("\nk_unload:");
                for elem in rotating.k_unload.iter() {
                    print!(" {}", elem);
                }
                print!("\ncss_bias:");
                for elem in rotating.css_bias.iter() {
                    print!(" {}", elem);
                }
                print!("\nmag_bias:");
                for elem in rotating.mag_bias.iter() {
                    print!(" {}", elem);
                }
                println!("\nrws_volt: {}", rotating.rws_volt);
                println!("rws_press: {}", rotating.rws_press);
                println!("att_det_mode: {}", rotating.att_det_mode);
                print!("rws_reset_cntr:");
                for elem in rotating.rws_reset_cntr.iter() {
                    print!(" {}", elem);
                }
                println!("\nsun_mag_aligned: {}", rotating.sun_mag_aligned);
                println!("minor_version: {}", rotating.minor_version);
                println!("mai_sn: {}", rotating.mai_sn);
                println!("orbit_prop_mode: {}", rotating.orbit_prop_mode);
                println!("acs_op_mode: {}", rotating.acs_op_mode);
                println!("proc_reset_cntr: {}", rotating.proc_reset_cntr);
                println!("major_version: {}", rotating.major_version);
                println!("ads_op_mode: {}", rotating.ads_op_mode);
                print!("css_gain:");
                for elem in rotating.css_gain.iter() {
                    print!(" {}", elem);
                }
                print!("\nmag_gain:");
                for elem in rotating.mag_gain.iter() {
                    print!(" {}", elem);
                }
                println!("\norbit_epoch: {}", rotating.orbit_epoch);
                println!("true_anomoly_epoch: {}", rotating.true_anomoly_epoch);
                println!("orbit_epoch_next: {}", rotating.orbit_epoch_next);
                print!("sc_pos_eci_epoch:");
                for elem in rotating.sc_pos_eci_epoch.iter() {
                    print!(" {}", elem);
                }
                print!("\nsc_vel_eci_epoch:");
                for elem in rotating.sc_vel_eci_epoch.iter() {
                    print!(" {}", elem);
                }
                println!("\nqb_x_wheel_speed: {}", rotating.qb_x_wheel_speed);
                println!("qb_x_filter_gain: {}", rotating.qb_x_filter_gain);
                println!("qb_x_dipole_gain: {}", rotating.qb_x_dipole_gain);
                print!("dipole_gain:");
                for elem in rotating.dipole_gain.iter() {
                    print!(" {}", elem);
                }
                print!("\nwheel_speed_bias:");
                for elem in rotating.wheel_speed_bias.iter() {
                    print!(" {}", elem);
                }
                println!(
                    "\ncos_sun_mag_align_thresh: {}",
                    rotating.cos_sun_mag_align_thresh
                );
                println!("unload_ang_thresh: {}", rotating.unload_ang_thresh);
                println!("q_sat: {}", rotating.q_sat);
                println!("raw_trq_max: {}", rotating.raw_trq_max);
                print!("rws_motor_current:");
                for elem in rotating.rws_motor_current.iter() {
                    print!(" {}", elem);
                }
                println!("\nraw_motor_temp: {}", rotating.raw_motor_temp);

                // Print raw IMU
                println!("\nRaw IMU:\n-----------------------");
                println!("accel: {} {} {}", imu.accel[0], imu.accel[1], imu.accel[2]);
                println!("gyro: {} {} {}", imu.gyro[0], imu.gyro[1], imu.gyro[2]);
                println!("gyro_temp: {}", imu.gyro_temp);
                println!("crc: {}", imu.crc);

                // Print IREHS telem
                println!("\nIREHS telemetry:\n-----------------------");
                println!("thermopiles_a:");
                for elem in irehs.thermopiles_a.iter() {
                    print!(" {}", elem);
                }
                println!("\nthermopiles_b:");
                for elem in irehs.thermopiles_b.iter() {
                    print!(" {}", elem);
                }
                println!("\ntemp_a:");
                for elem in irehs.temp_a.iter() {
                    print!(" {}", elem);
                }
                println!("\ntemp_b:");
                for elem in irehs.temp_b.iter() {
                    print!(" {}", elem);
                }
                println!("\ndip_angle_a: {}", irehs.dip_angle_a);
                println!("dip_angle_b: {}", irehs.dip_angle_b);
                print!("solution_degraded:");
                for elem in irehs.solution_degraded.iter() {
                    print!(" {:?}", elem);
                }
                println!("\ncrc: {}\n", irehs.crc);

                break;
            }
        }
    }
}

fn main() {
    println!("MAI400 Rust Test");

    let connection = Connection::new("/dev/ttyS5".to_owned());
    let mai = MAI400::new(connection);

    // Start read thread
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    thread::spawn(move || read_loop(receiver));

    // Test commands

    // Request config info
    mai.get_info().unwrap();

    // Set GPS time to Jan 01, 2018
    mai.set_gps_time(1198800018).unwrap();

    // Set RV
    // TODO: Get actual value
    // mai.set_rv(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1198800018).unwrap();

    // Set ACS mode
    // TODO: Get actual values
    // mai.set_mode(13, 1, -1, -3, 0).unwrap();

    // Passthrough
    // TODO: Get actual CRC value
    let mut array = [0; 8];
    array[0] = 0x90; // SYNC byte 1
    array[1] = 0xEB; // SYNC byte 2
    array[2] = 0x0; // Data_len byte 1
    array[3] = 0x0; // Data_len byte 2
    array[4] = 0x5A; // Msg_id
    array[5] = 0x0; // Addr
    array[6] = 0x00; // CRC byte 1
    array[7] = 0x00; // CRC byte 2
    mai.passthrough(&array).unwrap();

    // Let read loop run for 10 seconds
    thread::sleep(Duration::new(10, 0));

    // Kill the read thread
    sender.send(Message::Terminate).unwrap();

    println!("End of test");
}
