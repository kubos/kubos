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

extern crate isis_ants_api;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_stream;
extern crate slog_term;

use isis_ants_api::{AntS, KANTSAnt, KANTSController, KI2CNum};
use slog::{Drain, Logger};
use std::fs::File;
use std::sync::Mutex;

fn arm(ants: &AntS, logger: &Logger) -> u8 {
    match ants.arm() {
        Ok(()) => {
            info!(logger, "[Arm Test] Test completed successfully");
            return 0;
        }
        Err(err) => {
            error!(logger, "[Arm Test] Failed to arm AntS: {}", err);
            return 1;
        }
    }
}

fn disarm(ants: &AntS, logger: &Logger) -> u8 {
    match ants.disarm() {
        Ok(()) => {
            info!(logger, "[Disarm Test] Test completed successfully");
            return 0;
        }
        Err(err) => {
            error!(logger, "[Disarm Test] Failed to disarm AntS: {}", err);
            return 1;
        }
    }
}

fn configure(ants: &AntS, logger: &Logger) -> u8 {
    match ants.configure(KANTSController::Secondary) {
        Ok(()) => {
            info!(logger, "[Configure Test] Test completed successfully");
            return 0;
        }
        Err(err) => {
            error!(logger, "[Configure Test] Failed to configure AntS: {}", err);
            return 1;
        }
    }
}

fn deploy(ants: &AntS, logger: &Logger) -> u8 {
    match ants.deploy(KANTSAnt::Ant3, false, 1) {
        Ok(()) => {
            info!(logger, "[Deploy Test] Test completed successfully");
            return 0;
        }
        Err(err) => {
            error!(logger, "[Deploy Test] Failed to deploy antenna 3: {}", err);
            return 1;
        }
    }
}

fn deploy_override(ants: &AntS, logger: &Logger) -> u8 {
    match ants.deploy(KANTSAnt::Ant1, true, 1) {
        Ok(()) => {
            info!(logger, "[Deploy Override Test] Test completed successfully");
            return 0;
        }
        Err(err) => {
            error!(
                logger,
                "[Deploy Override Test] Failed to deploy antenna 1: {}", err
            );
            return 1;
        }
    }
}

fn auto_deploy(ants: &AntS, logger: &Logger) -> u8 {
    match ants.auto_deploy(2) {
        Ok(()) => {
            info!(logger, "[Auto-Deploy Test] Test completed successfully");
            return 0;
        }
        Err(err) => {
            error!(
                logger,
                "[Auto-Deploy Test] Failed to auto-deploy antennas: {}", err
            );
            return 1;
        }
    }
}

fn cancel_deploy(ants: &AntS, logger: &Logger) -> u8 {
    match ants.cancel_deploy() {
        Ok(()) => {
            info!(logger, "[Disarm Test] Test completed successfully");
            return 0;
        }
        Err(err) => {
            error!(
                logger,
                "[Disarm Test] Failed to cancel AntS deployment: {}", err
            );
            return 1;
        }
    }
}

fn passthrough(ants: &AntS, logger: &Logger) -> u8 {
    let tx: [u8; 1] = [0xC3];
    let mut rx: [u8; 2] = [0; 2];

    match ants.passthrough(&tx, &mut rx) {
        Ok(()) => {
            info!(logger, "[Passthrough Test] Result: {:?}", rx);
            return 0;
        }
        Err(err) => {
            error!(
                logger,
                "[Passthrough Test] Failed to read AntS deployment status: {}", err
            );
            return 1;
        }
    }
}

fn reset(ants: &AntS, logger: &Logger) -> u8 {
    match ants.reset() {
        Ok(()) => {
            info!(logger, "[Reset Test] Test completed successfully");
            return 0;
        }
        Err(err) => {
            error!(logger, "[Reset Test] Failed to reset AntS: {}", err);
            return 1;
        }
    }
}

fn get_deploy(ants: &AntS, logger: &Logger) -> u8 {
    let deploy = match ants.get_deploy() {
        Ok(result) => result,
        Err(err) => {
            error!(
                logger,
                "[Passthrough Test] Failed to read AntS deployment status: {}", err
            );
            return 1;
        }
    };

    info!(logger, "Antenna deployment status:");
    info!(logger, "    sys_burn_active: {}", deploy.sys_burn_active);
    info!(
        logger,
        "    sys_ignore_deploy: {}", deploy.sys_ignore_deploy
    );
    info!(logger, "    sys_armed: {}", deploy.sys_armed);
    info!(
        logger,
        "    ant_1_not_deployed: {}", deploy.ant_1_not_deployed
    );
    info!(
        logger,
        "    ant_1_stopped_time: {}", deploy.ant_1_stopped_time
    );
    info!(logger, "    ant_1_active: {}", deploy.ant_1_active);
    info!(
        logger,
        "    ant_2_not_deployed: {}", deploy.ant_2_not_deployed
    );
    info!(
        logger,
        "    ant_2_stopped_time: {}", deploy.ant_2_stopped_time
    );
    info!(logger, "    ant_2_active: {}", deploy.ant_2_active);
    info!(
        logger,
        "    ant_3_not_deployed: {}", deploy.ant_3_not_deployed
    );
    info!(
        logger,
        "    ant_3_stopped_time: {}", deploy.ant_3_stopped_time
    );
    info!(logger, "    ant_3_active: {}", deploy.ant_3_active);
    info!(
        logger,
        "    ant_4_not_deployed: {}", deploy.ant_4_not_deployed
    );
    info!(
        logger,
        "    ant_4_stopped_time: {}", deploy.ant_4_stopped_time
    );
    info!(logger, "    ant_4_active: {}", deploy.ant_4_active);

    if !deploy.sys_armed {
        error!(logger, "[Deploy Status Test] AntS not reporting as armed");
        return 1;
    }

    info!(logger, "[Deploy Status Test] Test completed successfully");
    return 0;
}

fn get_sys_telem(ants: &AntS, logger: &Logger) -> u8 {
    let sys_telem = match ants.get_system_telemetry() {
        Ok(result) => result,
        Err(err) => {
            error!(
                logger,
                "[Passthrough Test] Failed to read AntS deployment status: {}", err
            );
            return 1;
        }
    };

    info!(logger, "Antenna system telemetry:");
    info!(logger, "    raw_temp: {}", sys_telem.raw_temp);
    info!(logger, "    deploy_status:");
    info!(
        logger,
        "        sys_burn_active: {}", sys_telem.deploy_status.sys_burn_active
    );
    info!(
        logger,
        "        sys_ignore_deploy: {}", sys_telem.deploy_status.sys_ignore_deploy
    );
    info!(
        logger,
        "        sys_armed: {}", sys_telem.deploy_status.sys_armed
    );
    info!(
        logger,
        "        ant_1_not_deployed: {}", sys_telem.deploy_status.ant_1_not_deployed
    );
    info!(
        logger,
        "        ant_1_stopped_time: {}", sys_telem.deploy_status.ant_1_stopped_time
    );
    info!(
        logger,
        "        ant_1_active: {}", sys_telem.deploy_status.ant_1_active
    );
    info!(
        logger,
        "        ant_2_not_deployed: {}", sys_telem.deploy_status.ant_2_not_deployed
    );
    info!(
        logger,
        "        ant_2_stopped_time: {}", sys_telem.deploy_status.ant_2_stopped_time
    );
    info!(
        logger,
        "        ant_2_active: {}", sys_telem.deploy_status.ant_2_active
    );
    info!(
        logger,
        "        ant_3_not_deployed: {}", sys_telem.deploy_status.ant_3_not_deployed
    );
    info!(
        logger,
        "        ant_3_stopped_time: {}", sys_telem.deploy_status.ant_3_stopped_time
    );
    info!(
        logger,
        "        ant_3_active: {}", sys_telem.deploy_status.ant_3_active
    );
    info!(
        logger,
        "        ant_4_not_deployed: {}", sys_telem.deploy_status.ant_4_not_deployed
    );
    info!(
        logger,
        "        ant_4_stopped_time: {}", sys_telem.deploy_status.ant_4_stopped_time
    );
    info!(
        logger,
        "        ant_4_active: {}", sys_telem.deploy_status.ant_4_active
    );
    info!(logger, "    uptime: {}", sys_telem.uptime);

    info!(
        logger,
        "[System Telemetry Test] Test completed successfully"
    );

    return 0;
}

fn get_act_counts(ants: &AntS, logger: &Logger) -> u8 {
    let act_count = match ants.get_activation_count(KANTSAnt::Ant1) {
        Ok(result) => result,
        Err(err) => {
            error!(
                logger,
                "[Activation Count Test] Failed to get antenna 1's activation count: {}", err
            );
            return 1;
        }
    };

    info!(logger, "Antenna 1 activation count: {}", act_count);

    let act_count = match ants.get_activation_count(KANTSAnt::Ant2) {
        Ok(result) => result,
        Err(err) => {
            error!(
                logger,
                "[Activation Count Test] Failed to get antenna 2's activation count: {}", err
            );
            return 1;
        }
    };

    info!(logger, "Antenna 2 activation count: {}", act_count);

    let act_count = match ants.get_activation_count(KANTSAnt::Ant3) {
        Ok(result) => result,
        Err(err) => {
            error!(
                logger,
                "[Activation Count Test] Failed to get antenna 3's activation count: {}", err
            );
            return 1;
        }
    };

    info!(logger, "Antenna 3 activation count: {}", act_count);

    let act_count = match ants.get_activation_count(KANTSAnt::Ant4) {
        Ok(result) => result,
        Err(err) => {
            error!(
                logger,
                "[Activation Count Test] Failed to get antenna 4's activation count: {}", err
            );
            return 1;
        }
    };

    info!(logger, "Antenna 4 activation count: {}", act_count);

    info!(
        logger,
        "[Activation Counts Test] Test completed successfully"
    );

    return 0;
}

fn get_act_times(ants: &AntS, logger: &Logger) -> u8 {
    let act_time = match ants.get_activation_time(KANTSAnt::Ant1) {
        Ok(result) => result,
        Err(err) => {
            error!(
                logger,
                "[Activation Time Test] Failed to get antenna 1's activation time: {}", err
            );
            return 1;
        }
    };

    info!(logger, "Antenna 1 activation time: {}", act_time);

    let act_time = match ants.get_activation_time(KANTSAnt::Ant2) {
        Ok(result) => result,
        Err(err) => {
            error!(
                logger,
                "[Activation Time Test] Failed to get antenna 2's activation time: {}", err
            );
            return 1;
        }
    };

    info!(logger, "Antenna 2 activation time: {}", act_time);

    let act_time = match ants.get_activation_time(KANTSAnt::Ant3) {
        Ok(result) => result,
        Err(err) => {
            error!(
                logger,
                "[Activation Time Test] Failed to get antenna 3's activation time: {}", err
            );
            return 1;
        }
    };

    info!(logger, "Antenna 3 activation time: {}", act_time);

    let act_time = match ants.get_activation_time(KANTSAnt::Ant4) {
        Ok(result) => result,
        Err(err) => {
            error!(
                logger,
                "[Activation Time Test] Failed to get antenna 4's activation time: {}", err
            );
            return 1;
        }
    };

    info!(logger, "Antenna 4 activation time: {}", act_time);

    info!(
        logger,
        "[Activation Times Test] Test completed successfully"
    );

    return 0;
}

fn get_uptime(ants: &AntS, logger: &Logger) -> u8 {
    let uptime = match ants.get_uptime() {
        Ok(result) => result,
        Err(err) => {
            error!(
                logger,
                "[Passthrough Test] Failed to read AntS deployment status: {}", err
            );
            return 1;
        }
    };

    info!(logger, "System uptime: {}", uptime);
    info!(logger, "[Uptime Test] Test completed successfully");

    return 0;
}

pub fn main() {
    let mut error_count: u8 = 0;

    // Output warning and error messages to stderr
    let decorator = slog_term::PlainSyncDecorator::new(std::io::stderr());
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let console_drain = slog::LevelFilter(drain, slog::Level::Warning);

    // Output all messages to the log file
    let file = File::create("ants-rust-results.txt").expect("Couldn't open log file");
    let decorator = slog_term::PlainSyncDecorator::new(file);
    let file_drain = Mutex::new(slog_term::FullFormat::new(decorator).build()).fuse();

    // Combine the loggers so we only have to make one logging call per message
    let logger = Logger::root(slog::Duplicate(console_drain, file_drain).fuse(), o!());

    // Initialize connection with the antenna system
    let ants = match AntS::new(KI2CNum::KI2C1, 0x31, 0x32, 4, 10) {
        Ok(result) => result,
        Err(err) => {
            error!(logger, "Failed to init connection: {}", err);
            panic!("Unable to connect to AntS. Cancelling test");
        }
    };

    info!(logger, "ISIS AntS Integration Tests");

    error_count += arm(&ants, &logger);
    error_count += disarm(&ants, &logger);
    error_count += configure(&ants, &logger);

    // Prep for deployment tests
    if ants.arm().is_err() {
        error_count += 1;
        error!(logger, "Failed to arm AntS");
    }

    error_count += deploy(&ants, &logger);
    error_count += deploy_override(&ants, &logger);
    error_count += auto_deploy(&ants, &logger);
    error_count += cancel_deploy(&ants, &logger);
    error_count += passthrough(&ants, &logger);
    error_count += get_deploy(&ants, &logger);
    error_count += get_sys_telem(&ants, &logger);
    error_count += get_act_counts(&ants, &logger);
    error_count += get_act_times(&ants, &logger);
    error_count += get_uptime(&ants, &logger);

    error_count += reset(&ants, &logger);

    info!(logger, "ISIS AntS Integration Tests Complete");

    if error_count == 0 {
        // Since we don't have the logger set up to print to stdout,
        // we need a manual println call here
        println!("AntS tests completed successfully");
        info!(logger, "AntS tests completed successfully");
    } else {
        eprintln!("One or more AntS tests have failed. See ant-results.txt for info");
        warn!(logger, "One or more AntS tests have failed");
    }
}
