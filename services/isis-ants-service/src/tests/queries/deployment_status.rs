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

#[test]
fn deploy_status_bad() {
    let mut mock = mock_new!();
    mock.state = false;

    let service = service_new!(mock);

    let query = r#"
        {
            deploymentStatus {
                 ant1Active,
                 ant1NotDeployed,
                 ant1StoppedTime,
                 ant2Active,
                 ant2NotDeployed,
                 ant2StoppedTime,
                 ant3Active,
                 ant3NotDeployed,
                 ant3StoppedTime,
                 ant4Active,
                 ant4NotDeployed,
                 ant4StoppedTime,
                 status,
                 sysArmed,
                 sysBurnActive,
                 sysIgnoreDeploy,
            }
        }"#;

    let expected = json!({
            "deploymentStatus": {
                 "ant1Active": false,
                 "ant1NotDeployed": false,
                 "ant1StoppedTime": false,
                 "ant2Active": false,
                 "ant2NotDeployed": false,
                 "ant2StoppedTime": false,
                 "ant3Active": false,
                 "ant3NotDeployed": false,
                 "ant3StoppedTime": false,
                 "ant4Active": false,
                 "ant4NotDeployed": false,
                 "ant4StoppedTime": false,
                 "sysArmed": false,
                 "status": "ERROR",
                 "sysBurnActive": false,
                 "sysIgnoreDeploy": false,
            },
    });

    test!(service, query, expected);
}

#[test]
fn deploy_status_deployed() {
    let mut mock = mock_new!();

    mock.deploy_status = DeployStatus {
        sys_burn_active: false,
        sys_ignore_deploy: false,
        sys_armed: false,
        ant_1_not_deployed: false,
        ant_1_stopped_time: false,
        ant_1_active: false,
        ant_2_not_deployed: false,
        ant_2_stopped_time: false,
        ant_2_active: false,
        ant_3_not_deployed: false,
        ant_3_stopped_time: false,
        ant_3_active: false,
        ant_4_not_deployed: false,
        ant_4_stopped_time: false,
        ant_4_active: false,
    };

    let service = service_new!(mock);

    let query = r#"
        {
            deploymentStatus {
                 ant1Active,
                 ant1NotDeployed,
                 ant1StoppedTime,
                 ant2Active,
                 ant2NotDeployed,
                 ant2StoppedTime,
                 ant3Active,
                 ant3NotDeployed,
                 ant3StoppedTime,
                 ant4Active,
                 ant4NotDeployed,
                 ant4StoppedTime,
                 status,
                 sysArmed,
                 sysBurnActive,
                 sysIgnoreDeploy,
            }
        }"#;

    let expected = json!({
            "deploymentStatus": {
                 "ant1Active": false,
                 "ant1NotDeployed": false,
                 "ant1StoppedTime": false,
                 "ant2Active": false,
                 "ant2NotDeployed": false,
                 "ant2StoppedTime": false,
                 "ant3Active": false,
                 "ant3NotDeployed": false,
                 "ant3StoppedTime": false,
                 "ant4Active": false,
                 "ant4NotDeployed": false,
                 "ant4StoppedTime": false,
                 "status": "DEPLOYED",
                 "sysArmed": false,
                 "sysBurnActive": false,
                 "sysIgnoreDeploy": false,
            }
    });

    test!(service, query, expected);
}

#[test]
fn deploy_status_deployed_2_antennas() {
    let mut mock = mock_new!();

    mock.deploy_status = DeployStatus {
        ant_1_not_deployed: false,
        ant_2_not_deployed: false,
        ant_3_not_deployed: false,
        ant_4_not_deployed: false,
        ..Default::default()
    };

    let config = r#"
            [isis-ants-service.addr]
            ip = "127.0.0.1"
            port = 9999"#;
    let service = Service::new(
        Config::new_from_str("isis-ants-service", config).unwrap(),
        Subsystem {
            ants: Arc::new(Mutex::new(Box::new(mock))),
            controller: Arc::new(RwLock::new(ConfigureController::Primary)),
            errors: Arc::new(RwLock::new(vec![])),
            last_cmd: Arc::new(RwLock::new(AckCommand::None)),
            count: 2,
        },
        QueryRoot,
        MutationRoot,
    );

    let query = r#"
        {
            deploymentStatus {
                 status,
            }
        }"#;

    let expected = json!({
            "deploymentStatus": {
                 "status": "DEPLOYED",
            }
    });

    test!(service, query, expected);
}

#[test]
fn deploy_status_stowed() {
    let mut mock = mock_new!();

    mock.deploy_status = DeployStatus {
        ant_1_not_deployed: true,
        ant_2_not_deployed: true,
        ant_3_not_deployed: true,
        ant_4_not_deployed: true,
        ..Default::default()
    };

    let service = service_new!(mock);

    let query = r#"
        {
            deploymentStatus {
                 status,
            }
        }"#;

    let expected = json!({
            "deploymentStatus": {
                 "status": "STOWED",
            }
    });

    test!(service, query, expected);
}

#[test]
fn deploy_status_stowed_2_antennas() {
    let mut mock = mock_new!();

    mock.deploy_status = DeployStatus {
        ant_1_not_deployed: true,
        ant_2_not_deployed: true,
        ant_3_not_deployed: false,
        ant_4_not_deployed: false,
        ..Default::default()
    };

    let config = r#"
            [isis-ants-service.addr]
            ip = "127.0.0.1"
            port = 9999"#;
    let service = Service::new(
        Config::new_from_str("isis-ants-service", config).unwrap(),
        Subsystem {
            ants: Arc::new(Mutex::new(Box::new(mock))),
            controller: Arc::new(RwLock::new(ConfigureController::Primary)),
            errors: Arc::new(RwLock::new(vec![])),
            last_cmd: Arc::new(RwLock::new(AckCommand::None)),
            count: 2,
        },
        QueryRoot,
        MutationRoot,
    );

    let query = r#"
        {
            deploymentStatus {
                 status,
            }
        }"#;

    let expected = json!({
            "deploymentStatus": {
                 "status": "STOWED",
            }
    });

    test!(service, query, expected);
}

#[test]
fn deploy_status_partial() {
    let mut mock = mock_new!();

    mock.deploy_status = DeployStatus {
        ant_1_not_deployed: true,
        ant_2_not_deployed: false,
        ant_3_not_deployed: true,
        ant_4_not_deployed: false,
        ..Default::default()
    };

    let service = service_new!(mock);

    let query = r#"
        {
            deploymentStatus {
                 status,
            }
        }"#;

    let expected = json!({
            "deploymentStatus": {
                 "status": "PARTIAL",
            }
    });

    test!(service, query, expected);
}

#[test]
fn deploy_status_inprogress_1() {
    let mut mock = mock_new!();

    mock.deploy_status = DeployStatus {
        ant_1_active: true,
        ..Default::default()
    };

    let service = service_new!(mock);

    let query = r#"
        {
            deploymentStatus {
                 status,
            }
        }"#;

    let expected = json!({
            "deploymentStatus": {
                 "status": "IN_PROGRESS",
            }
    });

    test!(service, query, expected);
}

#[test]
fn deploy_status_inprogress_2() {
    let mut mock = mock_new!();

    mock.deploy_status = DeployStatus {
        ant_2_active: true,
        ..Default::default()
    };

    let service = service_new!(mock);

    let query = r#"
        {
            deploymentStatus {
                 status,
            }
        }"#;

    let expected = json!({
            "deploymentStatus": {
                 "status": "IN_PROGRESS",
            }
    });

    test!(service, query, expected);
}

#[test]
fn deploy_status_inprogress_3() {
    let mut mock = mock_new!();

    mock.deploy_status = DeployStatus {
        ant_3_active: true,
        ..Default::default()
    };

    let service = service_new!(mock);

    let query = r#"
        {
            deploymentStatus {
                 status,
            }
        }"#;

    let expected = json!({
            "deploymentStatus": {
                 "status": "IN_PROGRESS",
            }
    });

    test!(service, query, expected);
}

#[test]
fn deploy_status_inprogress_4() {
    let mut mock = mock_new!();

    mock.deploy_status = DeployStatus {
        ant_4_active: true,
        ..Default::default()
    };

    let service = service_new!(mock);

    let query = r#"
        {
            deploymentStatus {
                 status,
            }
        }"#;

    let expected = json!({
            "deploymentStatus": {
                 "status": "IN_PROGRESS",
            }
    });

    test!(service, query, expected);
}
