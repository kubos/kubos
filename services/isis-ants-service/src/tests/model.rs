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
//use isis_ants_api::*;
use model::*;
use std::cell::RefCell;

#[test]
fn get_arm_status_good_armed() {
    let mock = mock_new!();

    let deploy = DeployStatus {
        sys_armed: true,
        ..Default::default()
    };

    mock.get_deploy.return_value(Ok(deploy));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_arm_status().unwrap();

    assert_eq!(result, ArmStatus::Armed);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn get_arm_status_good_disarmed() {
    let mock = mock_new!();

    let deploy = DeployStatus {
        sys_armed: false,
        ..Default::default()
    };

    mock.get_deploy.return_value(Ok(deploy));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_arm_status().unwrap();

    assert_eq!(result, ArmStatus::Disarmed);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn get_arm_status_bad() {
    let mock = mock_new!();

    mock.get_deploy
        .return_value(Err(AntsError::GenericError.into()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_arm_status().unwrap();

    assert_eq!(result, ArmStatus::Disarmed);
    assert_eq!(
        sub.errors.borrow().clone(),
        vec!["get_deploy: Generic error"]
    );
}

//TODO: get_deploy_status. Test w/ variable # antennas
#[test]
fn get_deploy_status_good_deployed() {
    let mock = mock_new!();

    let expected = DeployStatus {
        ant_1_not_deployed: false,
        ant_2_not_deployed: false,
        ant_3_not_deployed: false,
        ant_4_not_deployed: false,
        ..Default::default()
    };

    mock.get_deploy.return_value(Ok(expected.clone()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_deploy_status().unwrap();

    assert_eq!(result.status, DeploymentStatus::Deployed);
    assert_eq!(result.details, expected);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn get_deploy_status_good_deployed_2antennas() {
    let mock = mock_new!();

    let expected = DeployStatus {
        ant_1_not_deployed: false,
        ant_2_not_deployed: false,
        ant_3_not_deployed: true,
        ant_4_not_deployed: true,
        ..Default::default()
    };

    mock.get_deploy.return_value(Ok(expected.clone()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 2,
    };

    let result = sub.get_deploy_status().unwrap();

    assert_eq!(result.status, DeploymentStatus::Deployed);
    assert_eq!(result.details, expected);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn get_deploy_status_good_partial() {
    let mock = mock_new!();

    let expected = DeployStatus {
        ant_1_not_deployed: false,
        ant_2_not_deployed: true,
        ant_3_not_deployed: true,
        ant_4_not_deployed: false,
        ..Default::default()
    };

    mock.get_deploy.return_value(Ok(expected.clone()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_deploy_status().unwrap();

    assert_eq!(result.status, DeploymentStatus::Partial);
    assert_eq!(result.details, expected);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn get_deploy_status_good_stowed() {
    let mock = mock_new!();

    let expected = DeployStatus {
        ant_1_not_deployed: true,
        ant_2_not_deployed: true,
        ant_3_not_deployed: true,
        ant_4_not_deployed: true,
        ..Default::default()
    };

    mock.get_deploy.return_value(Ok(expected.clone()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_deploy_status().unwrap();

    assert_eq!(result.status, DeploymentStatus::Stowed);
    assert_eq!(result.details, expected);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn get_deploy_status_good_inprogess() {
    let mock = mock_new!();

    let expected = DeployStatus {
        ant_1_active: true,
        ..Default::default()
    };

    mock.get_deploy.return_value(Ok(expected.clone()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_deploy_status().unwrap();

    assert_eq!(result.status, DeploymentStatus::InProgress);
    assert_eq!(result.details, expected);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn get_deploy_status_good_error() {
    let mock = mock_new!();

    let expected = DeployStatus {
        ant_1_not_deployed: true,
        ant_1_stopped_time: true,
        ..Default::default()
    };

    mock.get_deploy.return_value(Ok(expected.clone()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_deploy_status().unwrap();

    assert_eq!(result.status, DeploymentStatus::Error);
    assert_eq!(result.details, expected);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn get_deploy_status_bad() {
    let mock = mock_new!();

    mock.get_deploy
        .return_value(Err(AntsError::GenericError.into()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_deploy_status().unwrap();

    assert_eq!(result.status, DeploymentStatus::Error);
    assert_eq!(result.details, DeployStatus::default());
    assert_eq!(
        sub.errors.borrow().clone(),
        vec!["get_deploy: Generic error"]
    );
}

#[test]
fn get_power_good_on() {
    let mock = mock_new!();

    mock.get_uptime.return_value(Ok(10));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_power().unwrap();

    assert_eq!(result.state, PowerState::On);
    assert_eq!(result.uptime, 10);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn get_power_good_off() {
    let mock = mock_new!();

    mock.get_uptime.return_value(Ok(0));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_power().unwrap();

    assert_eq!(result.state, PowerState::Off);
    assert_eq!(result.uptime, 0);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn get_power_bad() {
    let mock = mock_new!();

    mock.get_uptime
        .return_value(Err(AntsError::GenericError.into()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_power().unwrap();

    assert_eq!(result.state, PowerState::Off);
    assert_eq!(result.uptime, 0);
    assert_eq!(
        sub.errors.borrow().clone(),
        vec!["get_uptime: Generic error"]
    );
}

#[test]
fn get_telemetry_debug_good() {
    let mock = mock_new!();

    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant1, Ok(1));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant1, Ok(11));
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant2, Ok(2));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant2, Ok(22));
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant3, Ok(3));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant3, Ok(33));
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant4, Ok(4));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant4, Ok(44));

    let expected = TelemetryDebug {
        ant1: AntennaStats {
            act_count: 1,
            act_time: 11,
        },
        ant2: AntennaStats {
            act_count: 2,
            act_time: 22,
        },
        ant3: AntennaStats {
            act_count: 3,
            act_time: 33,
        },
        ant4: AntennaStats {
            act_count: 4,
            act_time: 44,
        },
    };

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_telemetry_debug().unwrap();

    assert_eq!(result, expected);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn get_telemetry_debug_bad() {
    let mock = mock_new!();

    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant1, Ok(1));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant1, Ok(11));
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant2, Ok(2));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant2, Ok(22));
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant3, Ok(3));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant3, Ok(33));
    // Leaving out Ant4 to throw 2 ConfigErrors

    let expected = TelemetryDebug {
        ant1: AntennaStats {
            act_count: 1,
            act_time: 11,
        },
        ant2: AntennaStats {
            act_count: 2,
            act_time: 22,
        },
        ant3: AntennaStats {
            act_count: 3,
            act_time: 33,
        },
        ant4: AntennaStats {
            act_count: 0,
            act_time: 0,
        },
    };

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_telemetry_debug().unwrap();

    assert_eq!(result, expected);
    assert_eq!(
        sub.errors.borrow().clone(),
        vec![
            "get_activation_count: Configuration error",
            "get_activation_time: Configuration error",
        ]
    );
}

#[test]
fn get_telemetry_nominal_good() {
    let mock = mock_new!();

    let expected = AntsTelemetry {
        raw_temp: 15,
        uptime: 35,
        deploy_status: DeployStatus {
            sys_armed: true,
            ant_1_active: true,
            ant_4_not_deployed: false,
            ..Default::default()
        },
    };

    mock.get_system_telemetry.return_value(Ok(expected.clone()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_telemetry_nominal().unwrap();

    assert_eq!(result, TelemetryNominal(expected));
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn get_telemetry_nominal_bad() {
    let mock = mock_new!();

    let expected = AntsTelemetry::default();

    mock.get_system_telemetry
        .return_value(Err(AntsError::GenericError.into()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_telemetry_nominal().unwrap();

    assert_eq!(result, TelemetryNominal(expected));
    assert_eq!(
        sub.errors.borrow().clone(),
        vec!["get_system_telemetry: Generic error"]
    );
}

//Note: get_test_results() is a wrapper for integration_test(), so there
//are no explicit integration_test() unit tests
#[test]
fn get_test_results_good() {
    let mock = mock_new!();

    let nominal = AntsTelemetry {
        raw_temp: 15,
        uptime: 35,
        deploy_status: DeployStatus {
            sys_armed: true,
            ant_1_active: true,
            ant_4_not_deployed: false,
            ..Default::default()
        },
    };
    mock.get_system_telemetry.return_value(Ok(nominal.clone()));

    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant1, Ok(1));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant1, Ok(11));
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant2, Ok(2));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant2, Ok(22));
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant3, Ok(3));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant3, Ok(33));
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant4, Ok(4));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant4, Ok(44));

    let debug = TelemetryDebug {
        ant1: AntennaStats {
            act_count: 1,
            act_time: 11,
        },
        ant2: AntennaStats {
            act_count: 2,
            act_time: 22,
        },
        ant3: AntennaStats {
            act_count: 3,
            act_time: 33,
        },
        ant4: AntennaStats {
            act_count: 4,
            act_time: 44,
        },
    };

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_test_results().unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(result.telemetry_nominal, TelemetryNominal(nominal));
    assert_eq!(result.telemetry_debug, debug);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn get_test_results_bad_nominal() {
    let mock = mock_new!();

    let nominal = AntsTelemetry::default();
    mock.get_system_telemetry
        .return_value(Err(AntsError::GenericError.into()));

    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant1, Ok(1));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant1, Ok(11));
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant2, Ok(2));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant2, Ok(22));
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant3, Ok(3));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant3, Ok(33));
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant4, Ok(4));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant4, Ok(44));

    let debug = TelemetryDebug {
        ant1: AntennaStats {
            act_count: 1,
            act_time: 11,
        },
        ant2: AntennaStats {
            act_count: 2,
            act_time: 22,
        },
        ant3: AntennaStats {
            act_count: 3,
            act_time: 33,
        },
        ant4: AntennaStats {
            act_count: 4,
            act_time: 44,
        },
    };

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_test_results().unwrap();

    assert_eq!(result.errors, "Nominal: Generic error;".to_owned());
    assert_eq!(result.success, false);
    assert_eq!(result.telemetry_nominal, TelemetryNominal(nominal));
    assert_eq!(result.telemetry_debug, debug);
    assert_eq!(
        sub.errors.borrow().clone(),
        vec!["get_system_telemetry: Generic error"]
    );
}

#[test]
fn get_test_results_bad_debug() {
    let mock = mock_new!();

    let nominal = AntsTelemetry {
        raw_temp: 15,
        uptime: 35,
        deploy_status: DeployStatus {
            sys_armed: true,
            ant_1_active: true,
            ant_4_not_deployed: false,
            ..Default::default()
        },
    };
    mock.get_system_telemetry.return_value(Ok(nominal.clone()));

    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant1, Ok(1));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant1, Ok(11));
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant2, Ok(2));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant2, Ok(22));
    //Excluding Ant3 for error testing
    mock.get_activation_count
        .return_value_for(KANTSAnt::Ant4, Ok(4));
    mock.get_activation_time
        .return_value_for(KANTSAnt::Ant4, Ok(44));

    let debug = TelemetryDebug {
        ant1: AntennaStats {
            act_count: 1,
            act_time: 11,
        },
        ant2: AntennaStats {
            act_count: 2,
            act_time: 22,
        },
        ant3: AntennaStats {
            act_count: 0,
            act_time: 0,
        },
        ant4: AntennaStats {
            act_count: 4,
            act_time: 44,
        },
    };

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.get_test_results().unwrap();

    assert_eq!(
        result.errors,
        "Debug: get_activation_count: Configuration error, get_activation_time: Configuration error"
            .to_owned()
    );
    assert_eq!(result.success, false);
    assert_eq!(result.telemetry_nominal, TelemetryNominal(nominal));
    assert_eq!(result.telemetry_debug, debug);
    assert_eq!(
        sub.errors.borrow().clone(),
        vec![
            "get_test_results(debug): get_activation_count: Configuration error, get_activation_time: Configuration error",
        ]
    );
}

#[test]
fn arm_good_arm() {
    let mock = mock_new!();

    mock.arm.return_value(Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.arm(ArmState::Arm).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn arm_error_arm() {
    let mock = mock_new!();

    mock.arm.return_value(Err(AntsError::GenericError.into()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.arm(ArmState::Arm).unwrap();

    assert_eq!(result.errors, "Generic error");
    assert_eq!(result.success, false);
    assert_eq!(sub.errors.borrow().clone(), vec!["arm: Generic error"]);
}

#[test]
fn arm_good_disarm() {
    let mock = mock_new!();

    mock.disarm.return_value(Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.arm(ArmState::Disarm).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn arm_error_disarm() {
    let mock = mock_new!();

    mock.disarm
        .return_value(Err(AntsError::GenericError.into()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.arm(ArmState::Disarm).unwrap();

    assert_eq!(result.errors, "Generic error");
    assert_eq!(result.success, false);
    assert_eq!(sub.errors.borrow().clone(), vec!["disarm: Generic error"]);
}

#[test]
fn configure_good_primary() {
    let mock = mock_new!();

    mock.configure
        .return_value_for(KANTSController::Primary, Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.configure_hardware(ConfigureController::Primary)
        .unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(result.config, ConfigureController::Primary);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn configure_good_secondary() {
    let mock = mock_new!();

    mock.configure
        .return_value_for(KANTSController::Secondary, Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.configure_hardware(ConfigureController::Secondary)
        .unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(result.config, ConfigureController::Secondary);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn configure_bad() {
    let mock = mock_new!();

    mock.configure
        .return_value(Err(AntsError::GenericError.into()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.configure_hardware(ConfigureController::Primary)
        .unwrap();

    assert_eq!(result.errors, "Generic error");
    assert_eq!(result.success, false);
    assert_eq!(result.config, ConfigureController::Primary);
    assert_eq!(
        sub.errors.borrow().clone(),
        vec!["configure: Generic error"]
    );
}

#[test]
fn control_power_good() {
    let mock = mock_new!();

    mock.reset.return_value(Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.control_power(PowerState::Reset).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(result.power, PowerState::Reset);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn control_power_bad() {
    let mock = mock_new!();

    mock.reset.return_value(Err(AntsError::GenericError.into()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.control_power(PowerState::Reset).unwrap();

    assert_eq!(result.errors, "Generic error");
    assert_eq!(result.success, false);
    assert_eq!(result.power, PowerState::Reset);
    assert_eq!(sub.errors.borrow().clone(), vec!["reset: Generic error"]);
}

#[test]
fn control_power_invalid() {
    let mock = mock_new!();

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.control_power(PowerState::On).unwrap();

    assert_eq!(result.errors, "Invalid power state".to_owned());
    assert_eq!(result.success, false);
    assert_eq!(result.power, PowerState::On);
}

#[test]
fn deploy_good_all() {
    let mock = mock_new!();

    mock.auto_deploy.return_value(Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::All, false, 5).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn deploy_good_ant1() {
    let mock = mock_new!();

    mock.deploy
        .return_value_for((KANTSAnt::Ant1, false, 5), Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::Antenna1, false, 5).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn deploy_good_ant1_override() {
    let mock = mock_new!();

    mock.deploy
        .return_value_for((KANTSAnt::Ant1, true, 5), Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::Antenna1, true, 5).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn deploy_good_ant2() {
    let mock = mock_new!();

    mock.deploy
        .return_value_for((KANTSAnt::Ant2, false, 5), Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::Antenna2, false, 5).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn deploy_good_ant2_override() {
    let mock = mock_new!();

    mock.deploy
        .return_value_for((KANTSAnt::Ant2, true, 5), Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::Antenna2, true, 5).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn deploy_good_ant3() {
    let mock = mock_new!();

    mock.deploy
        .return_value_for((KANTSAnt::Ant3, false, 5), Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::Antenna3, false, 5).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn deploy_good_ant3_override() {
    let mock = mock_new!();

    mock.deploy
        .return_value_for((KANTSAnt::Ant3, true, 5), Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::Antenna3, true, 5).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn deploy_good_ant4() {
    let mock = mock_new!();

    mock.deploy
        .return_value_for((KANTSAnt::Ant4, false, 5), Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::Antenna4, false, 5).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn deploy_good_ant4_override() {
    let mock = mock_new!();

    mock.deploy
        .return_value_for((KANTSAnt::Ant4, true, 5), Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::Antenna4, true, 5).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn deploy_bad() {
    let mock = mock_new!();

    mock.deploy
        .return_value(Err(AntsError::GenericError.into()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.deploy(DeployType::Antenna1, true, 5).unwrap();

    assert_eq!(result.errors, "Generic error");
    assert_eq!(result.success, false);
    assert_eq!(sub.errors.borrow().clone(), vec!["deploy: Generic error"]);
}

#[test]
fn noop_good() {
    let mock = mock_new!();

    mock.watchdog_kick.return_value(Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.noop().unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn noop_error() {
    let mock = mock_new!();

    mock.watchdog_kick
        .return_value(Err(AntsError::GenericError.into()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.noop().unwrap();

    assert_eq!(result.errors, "Generic error");
    assert_eq!(result.success, false);
    assert_eq!(
        sub.errors.borrow().clone(),
        vec!["watchdog_kick: Generic error"]
    );
}

#[test]
fn passthrough_good_noresponse() {
    let mock = mock_new!();

    mock.passthrough
        .return_value_for((vec![0xc3, 0xc2], vec![]), Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.passthrough("c3c2".to_owned(), 0).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn passthrough_good_withresponse() {
    let mock = mock_new!();

    mock.passthrough
        .return_value_for((vec![0xc3, 0xc2], vec![0x00, 0x01]), Ok(()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.passthrough("c3c2".to_owned(), 2).unwrap();

    assert_eq!(result.errors, "".to_owned());
    assert_eq!(result.success, true);
    assert_eq!(result.response, "0001".to_owned());
    assert_eq!(sub.errors.borrow().clone(), Vec::<String>::new());
}

#[test]
fn passthrough_error() {
    let mock = mock_new!();

    mock.passthrough
        .return_value(Err(AntsError::GenericError.into()));

    let sub = Subsystem {
        ants: Box::new(mock),
        errors: RefCell::new(vec![]),
        count: 4,
    };

    let result = sub.passthrough("c3c2".to_owned(), 2).unwrap();

    assert_eq!(result.errors, "Generic error");
    assert_eq!(result.success, false);
    assert_eq!(
        sub.errors.borrow().clone(),
        vec!["passthrough: Generic error"]
    );
}
