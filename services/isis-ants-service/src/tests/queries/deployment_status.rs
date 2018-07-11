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
    let mock = mock_new!();

    mock.get_deploy.return_value(Err(AntsError::GenericError));

    let service = service_new!(mock);

    let query = r#"
        {
            deploymentStatus {
                 status,
                 sysBurnActive,
                 sysIgnoreDeploy,
                 sysArmed,
                 ant1NotDeployed,
                 ant1StoppedTime,
                 ant1Active,
                 ant2NotDeployed,
                 ant2StoppedTime,
                 ant2Active,
                 ant3NotDeployed,
                 ant3StoppedTime,
                 ant3Active,
                 ant4NotDeployed,
                 ant4StoppedTime,
                 ant4Active
            }
        }"#;

    let expected = json!({
            "deploymentStatus": {
                 "status": "ERROR",
                     "sysBurnActive": false,
                     "sysIgnoreDeploy": false,
                     "sysArmed": false,
                     "ant1NotDeployed": false,
                     "ant1StoppedTime": false,
                     "ant1Active": false,
                     "ant2NotDeployed": false,
                     "ant2StoppedTime": false,
                     "ant2Active": false,
                     "ant3NotDeployed": false,
                     "ant3StoppedTime": false,
                     "ant3Active": false,
                     "ant4NotDeployed": false,
                     "ant4StoppedTime": false,
                     "ant4Active": false
                },
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn deploy_status_deployed() {
    let mock = mock_new!();

    let deploy_status = DeployStatus {
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
    mock.get_deploy.return_value(Ok(deploy_status));

    let service = service_new!(mock);

    let query = r#"
        {
            deploymentStatus {
                 status,
                 sysBurnActive,
                 sysIgnoreDeploy,
                 sysArmed,
                 ant1NotDeployed,
                 ant1StoppedTime,
                 ant1Active,
                 ant2NotDeployed,
                 ant2StoppedTime,
                 ant2Active,
                 ant3NotDeployed,
                 ant3StoppedTime,
                 ant3Active,
                 ant4NotDeployed,
                 ant4StoppedTime,
                 ant4Active
            }
        }"#;

    let expected = json!({
            "deploymentStatus": {
                 "status": "DEPLOYED",
                 "sysBurnActive": false,
                 "sysIgnoreDeploy": false,
                 "sysArmed": false,
                 "ant1NotDeployed": false,
                 "ant1StoppedTime": false,
                 "ant1Active": false,
                 "ant2NotDeployed": false,
                 "ant2StoppedTime": false,
                 "ant2Active": false,
                 "ant3NotDeployed": false,
                 "ant3StoppedTime": false,
                 "ant3Active": false,
                 "ant4NotDeployed": false,
                 "ant4StoppedTime": false,
                 "ant4Active": false
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn deploy_status_deployed_2_antennas() {
    let mock = mock_new!();

    let deploy_status = DeployStatus {
        ant_1_not_deployed: false,
        ant_2_not_deployed: false,
        ant_3_not_deployed: false,
        ant_4_not_deployed: false,
        ..Default::default()
    };
    mock.get_deploy.return_value(Ok(deploy_status));

    let service = Service::new(
        Config::new("isis-ants-service"),
        Subsystem {
            ants: Box::new(mock),
            errors: RefCell::new(vec![]),
            last_cmd: Cell::new(AckCommand::None),
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

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn deploy_status_stowed() {
    let mock = mock_new!();

    let deploy_status = DeployStatus {
        ant_1_not_deployed: true,
        ant_2_not_deployed: true,
        ant_3_not_deployed: true,
        ant_4_not_deployed: true,
        ..Default::default()
    };
    mock.get_deploy.return_value(Ok(deploy_status));

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

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn deploy_status_stowed_2_antennas() {
    let mock = mock_new!();

    let deploy_status = DeployStatus {
        ant_1_not_deployed: true,
        ant_2_not_deployed: true,
        ant_3_not_deployed: false,
        ant_4_not_deployed: false,
        ..Default::default()
    };
    mock.get_deploy.return_value(Ok(deploy_status));

    let service = Service::new(
        Config::new("isis-ants-service"),
        Subsystem {
            ants: Box::new(mock),
            errors: RefCell::new(vec![]),
            last_cmd: Cell::new(AckCommand::None),
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

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn deploy_status_partial() {
    let mock = mock_new!();

    let deploy_status = DeployStatus {
        ant_1_not_deployed: true,
        ant_2_not_deployed: false,
        ant_3_not_deployed: true,
        ant_4_not_deployed: false,
        ..Default::default()
    };
    mock.get_deploy.return_value(Ok(deploy_status));

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

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn deploy_status_inprogress_1() {
    let mock = mock_new!();

    let deploy_status = DeployStatus {
        ant_1_active: true,
        ..Default::default()
    };
    mock.get_deploy.return_value(Ok(deploy_status.clone()));

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

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn deploy_status_inprogress_2() {
    let mock = mock_new!();

    let deploy_status = DeployStatus {
        ant_2_active: true,
        ..Default::default()
    };
    mock.get_deploy.return_value(Ok(deploy_status.clone()));

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

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn deploy_status_inprogress_3() {
    let mock = mock_new!();

    let deploy_status = DeployStatus {
        ant_3_active: true,
        ..Default::default()
    };
    mock.get_deploy.return_value(Ok(deploy_status.clone()));

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

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn deploy_status_inprogress_4() {
    let mock = mock_new!();

    let deploy_status = DeployStatus {
        ant_4_active: true,
        ..Default::default()
    };
    mock.get_deploy.return_value(Ok(deploy_status.clone()));

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

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}
