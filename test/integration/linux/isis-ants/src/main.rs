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

use isis_ants_api::*;
use log::*;
use std::fs::File;
use std::sync::Mutex;

fn arm(ants: &AntS) -> u8 {
    match ants.arm() {
        Ok(()) => {
            info!("[Arm Test] Test completed successfully");
            0
        }
        Err(err) => {
            error!("[Arm Test] Failed to arm AntS: {}", err);
            1
        }
    }
}

fn disarm(ants: &AntS) -> u8 {
    match ants.disarm() {
        Ok(()) => {
            info!("[Disarm Test] Test completed successfully");
            0
        }
        Err(err) => {
            error!("[Disarm Test] Failed to disarm AntS: {}", err);
            1
        }
    }
}

fn configure(ants: &AntS) -> u8 {
    match ants.configure(KANTSController::Secondary) {
        Ok(()) => {
            info!("[Configure Test] Test completed successfully");
            0
        }
        Err(err) => {
            error!("[Configure Test] Failed to configure AntS: {}", err);
            1
        }
    }
}

fn deploy(ants: &AntS) -> u8 {
    match ants.deploy(KANTSAnt::Ant3, false, 10) {
        Ok(()) => {
            info!("[Deploy Test] Test completed successfully");
            0
        }
        Err(err) => {
            error!("[Deploy Test] Failed to deploy antenna 3: {}", err);
            1
        }
    }
}

fn deploy_override(ants: &AntS) -> u8 {
    match ants.deploy(KANTSAnt::Ant1, true, 10) {
        Ok(()) => {
            info!("[Deploy Override Test] Test completed successfully");
            0
        }
        Err(err) => {
            error!("[Deploy Override Test] Failed to deploy antenna 1: {}", err);
            1
        }
    }
}

fn auto_deploy(ants: &AntS) -> u8 {
    match ants.auto_deploy(2) {
        Ok(()) => {
            info!("[Auto-Deploy Test] Test completed successfully");
            0
        }
        Err(err) => {
            error!("[Auto-Deploy Test] Failed to auto-deploy antennas: {}", err);
            1
        }
    }
}

fn cancel_deploy(ants: &AntS) -> u8 {
    match ants.cancel_deploy() {
        Ok(()) => {
            info!("[Disarm Test] Test completed successfully");
            0
        }
        Err(err) => {
            error!("[Disarm Test] Failed to cancel AntS deployment: {}", err);
            1
        }
    }
}

fn passthrough(ants: &AntS) -> u8 {
    let tx: [u8; 1] = [0xC3];
    let mut rx: [u8; 2] = [0; 2];

    match ants.passthrough(&tx, &mut rx) {
        Ok(()) => {
            info!("[Passthrough Test] Result: {:?}", rx);
            0
        }
        Err(err) => {
            error!(
                "[Passthrough Test] Failed to read AntS deployment status: {}",
                err
            );
            1
        }
    }
}

fn reset(ants: &AntS) -> u8 {
    match ants.reset() {
        Ok(()) => {
            info!("[Reset Test] Test completed successfully");
            0
        }
        Err(err) => {
            error!("[Reset Test] Failed to reset AntS: {}", err);
            1
        }
    }
}

fn get_deploy(ants: &AntS) -> u8 {
    let deploy = match ants.get_deploy() {
        Ok(result) => result,
        Err(err) => {
            error!(
                "[Passthrough Test] Failed to read AntS deployment status: {}",
                err
            );
            return 1;
        }
    };

    info!("Antenna deployment status:");
    info!("    sys_burn_active: {}", deploy.sys_burn_active);
    info!("    sys_ignore_deploy: {}", deploy.sys_ignore_deploy);
    info!("    sys_armed: {}", deploy.sys_armed);
    info!("    ant_1_not_deployed: {}", deploy.ant_1_not_deployed);
    info!("    ant_1_stopped_time: {}", deploy.ant_1_stopped_time);
    info!("    ant_1_active: {}", deploy.ant_1_active);
    info!("    ant_2_not_deployed: {}", deploy.ant_2_not_deployed);
    info!("    ant_2_stopped_time: {}", deploy.ant_2_stopped_time);
    info!("    ant_2_active: {}", deploy.ant_2_active);
    info!("    ant_3_not_deployed: {}", deploy.ant_3_not_deployed);
    info!("    ant_3_stopped_time: {}", deploy.ant_3_stopped_time);
    info!("    ant_3_active: {}", deploy.ant_3_active);
    info!("    ant_4_not_deployed: {}", deploy.ant_4_not_deployed);
    info!("    ant_4_stopped_time: {}", deploy.ant_4_stopped_time);
    info!("    ant_4_active: {}", deploy.ant_4_active);

    if !deploy.sys_armed {
        error!("[Deploy Status Test] AntS not reporting as armed");
        return 1;
    }

    info!("[Deploy Status Test] Test completed successfully");
    0
}

fn get_sys_telem(ants: &AntS) -> u8 {
    let sys_telem = match ants.get_system_telemetry() {
        Ok(result) => result,
        Err(err) => {
            error!(
                "[Passthrough Test] Failed to read AntS deployment status: {}",
                err
            );
            return 1;
        }
    };

    info!("Antenna system telemetry:");
    info!("    raw_temp: {}", sys_telem.raw_temp);
    info!("    deploy_status:");
    info!(
        "        sys_burn_active: {}",
        sys_telem.deploy_status.sys_burn_active
    );
    info!(
        "        sys_ignore_deploy: {}",
        sys_telem.deploy_status.sys_ignore_deploy
    );
    info!("        sys_armed: {}", sys_telem.deploy_status.sys_armed);
    info!(
        "        ant_1_not_deployed: {}",
        sys_telem.deploy_status.ant_1_not_deployed
    );
    info!(
        "        ant_1_stopped_time: {}",
        sys_telem.deploy_status.ant_1_stopped_time
    );
    info!(
        "        ant_1_active: {}",
        sys_telem.deploy_status.ant_1_active
    );
    info!(
        "        ant_2_not_deployed: {}",
        sys_telem.deploy_status.ant_2_not_deployed
    );
    info!(
        "        ant_2_stopped_time: {}",
        sys_telem.deploy_status.ant_2_stopped_time
    );
    info!(
        "        ant_2_active: {}",
        sys_telem.deploy_status.ant_2_active
    );
    info!(
        "        ant_3_not_deployed: {}",
        sys_telem.deploy_status.ant_3_not_deployed
    );
    info!(
        "        ant_3_stopped_time: {}",
        sys_telem.deploy_status.ant_3_stopped_time
    );
    info!(
        "        ant_3_active: {}",
        sys_telem.deploy_status.ant_3_active
    );
    info!(
        "        ant_4_not_deployed: {}",
        sys_telem.deploy_status.ant_4_not_deployed
    );
    info!(
        "        ant_4_stopped_time: {}",
        sys_telem.deploy_status.ant_4_stopped_time
    );
    info!(
        "        ant_4_active: {}",
        sys_telem.deploy_status.ant_4_active
    );
    info!("    uptime: {}", sys_telem.uptime);

    info!("[System Telemetry Test] Test completed successfully");

    0
}

fn get_act_counts(ants: &AntS) -> u8 {
    let act_count = match ants.get_activation_count(KANTSAnt::Ant1) {
        Ok(result) => result,
        Err(err) => {
            error!(
                "[Activation Count Test] Failed to get antenna 1's activation count: {}",
                err
            );
            return 1;
        }
    };

    info!("Antenna 1 activation count: {}", act_count);

    let act_count = match ants.get_activation_count(KANTSAnt::Ant2) {
        Ok(result) => result,
        Err(err) => {
            error!(
                "[Activation Count Test] Failed to get antenna 2's activation count: {}",
                err
            );
            return 1;
        }
    };

    info!("Antenna 2 activation count: {}", act_count);

    let act_count = match ants.get_activation_count(KANTSAnt::Ant3) {
        Ok(result) => result,
        Err(err) => {
            error!(
                "[Activation Count Test] Failed to get antenna 3's activation count: {}",
                err
            );
            return 1;
        }
    };

    info!("Antenna 3 activation count: {}", act_count);

    let act_count = match ants.get_activation_count(KANTSAnt::Ant4) {
        Ok(result) => result,
        Err(err) => {
            error!(
                "[Activation Count Test] Failed to get antenna 4's activation count: {}",
                err
            );
            return 1;
        }
    };

    info!("Antenna 4 activation count: {}", act_count);

    info!("[Activation Counts Test] Test completed successfully");

    0
}

fn get_act_times(ants: &AntS) -> u8 {
    let act_time = match ants.get_activation_time(KANTSAnt::Ant1) {
        Ok(result) => result,
        Err(err) => {
            error!(
                "[Activation Time Test] Failed to get antenna 1's activation time: {}",
                err
            );
            return 1;
        }
    };

    info!("Antenna 1 activation time: {}", act_time);

    let act_time = match ants.get_activation_time(KANTSAnt::Ant2) {
        Ok(result) => result,
        Err(err) => {
            error!(
                "[Activation Time Test] Failed to get antenna 2's activation time: {}",
                err
            );
            return 1;
        }
    };

    info!("Antenna 2 activation time: {}", act_time);

    let act_time = match ants.get_activation_time(KANTSAnt::Ant3) {
        Ok(result) => result,
        Err(err) => {
            error!(
                "[Activation Time Test] Failed to get antenna 3's activation time: {}",
                err
            );
            return 1;
        }
    };

    info!("Antenna 3 activation time: {}", act_time);

    let act_time = match ants.get_activation_time(KANTSAnt::Ant4) {
        Ok(result) => result,
        Err(err) => {
            error!(
                "[Activation Time Test] Failed to get antenna 4's activation time: {}",
                err
            );
            return 1;
        }
    };

    info!("Antenna 4 activation time: {}", act_time);

    info!("[Activation Times Test] Test completed successfully");

    0
}

fn get_uptime(ants: &AntS) -> u8 {
    let uptime = match ants.get_uptime() {
        Ok(result) => result,
        Err(err) => {
            error!(
                "[Passthrough Test] Failed to read AntS deployment status: {}",
                err
            );
            return 1;
        }
    };

    info!("System uptime: {}", uptime);
    info!("[Uptime Test] Test completed successfully");

    0
}

fn watchdog_kick(ants: &AntS) -> u8 {
    match ants.watchdog_kick() {
        Ok(()) => {
            info!("[WD Test] Test completed successfully");
            0
        }
        Err(err) => {
            error!("[WD Test] Failed to kick WD: {}", err);
            1
        }
    }
}

pub fn main() {
    let mut error_count: u8 = 0;

    use log4rs::append::console::ConsoleAppender;
    use log4rs::encode::pattern::PatternEncoder;
    use log4rs_syslog::SyslogAppender;
    // Use custom PatternEncoder to avoid duplicate timestamps in logs.
    let syslog_encoder = Box::new(PatternEncoder::new("{m}"));
    // Set up logging which will be routed to syslog for processing
    let syslog = Box::new(
        SyslogAppender::builder()
            .encoder(syslog_encoder)
            .openlog(
                "isis-ants",
                log4rs_syslog::LogOption::LOG_PID | log4rs_syslog::LogOption::LOG_CONS,
                log4rs_syslog::Facility::User,
            )
            .build(),
    );

    // Set up logging which will be routed to stdout
    let stdout = Box::new(ConsoleAppender::builder().build());

    // Combine the loggers into one master config
    let config = log4rs::config::Config::builder()
        .appender(log4rs::config::Appender::builder().build("syslog", syslog))
        .appender(log4rs::config::Appender::builder().build("stdout", stdout))
        .build(
            log4rs::config::Root::builder()
                .appender("syslog")
                .appender("stdout")
                // Set the minimum logging level to record
                .build(log::LevelFilter::Debug),
        )
        .unwrap();

    // Start the logger
    log4rs::init_config(config).unwrap();

    // Initialize connection with the antenna system
    let ants = match AntS::new("/dev/i2c-0", 0x31, 0x32, 4, 10) {
        Ok(result) => result,
        Err(err) => {
            error!("Failed to init connection: {}", err);
            panic!("Unable to connect to AntS. Cancelling test");
        }
    };

    info!("ISIS AntS Integration Tests");

    error_count += arm(&ants);
    error_count += disarm(&ants);
    error_count += configure(&ants);

    // Prep for deployment tests
    if ants.arm().is_err() {
        error_count += 1;
        error!("Failed to arm AntS");
    }

    error_count += deploy(&ants);
    error_count += deploy_override(&ants);
    error_count += auto_deploy(&ants);
    error_count += cancel_deploy(&ants);
    error_count += passthrough(&ants);
    error_count += get_deploy(&ants);
    error_count += get_sys_telem(&ants);
    error_count += get_act_counts(&ants);
    error_count += get_act_times(&ants);
    error_count += get_uptime(&ants);

    error_count += reset(&ants);
    error_count += watchdog_kick(&ants);

    info!("ISIS AntS Integration Tests Complete");

    if error_count == 0 {
        // Since we don't have the logger set up to print to stdout,
        // we need a manual println call here
        println!("AntS tests completed successfully");
        info!("AntS tests completed successfully");
    } else {
        eprintln!("One or more AntS tests have failed. See ant-results.txt for info");
        warn!("One or more AntS tests have failed");
    }
}
