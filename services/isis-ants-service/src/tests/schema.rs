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

//use double;
use std::cell::RefCell;
use isis_ants_api::*;
use model::*;
//use objects::*;
use super::*;
use schema::*;

use juniper::{execute, RootNode, Value, Variables};

#[test]
fn ping() {
    let mock = mock_new!();

    let context = Context {
        subsystem: Subsystem {
            ants: Box::new(mock),
            errors: RefCell::new(vec![]),
            count: 4,
        },
    };

    let schema = RootNode::new(schema::QueryRoot, schema::MutationRoot);

    let query = r#"
        {
            ping
        }"#;

    assert_eq!(
        execute(query, None, &schema, &Variables::new(), &context),
        Ok((
            Value::object(
                vec![("ping", Value::string("pong"))].into_iter().collect(),
            ),
            vec![],
        ))
    );
}

#[test]
fn ack() {
    let mock = mock_new!();

    let context = Context {
        subsystem: Subsystem {
            ants: Box::new(mock),
            errors: RefCell::new(vec![]),
            count: 4,
        },
    };

    let schema = RootNode::new(schema::QueryRoot, schema::MutationRoot);

    let query = r#"
        {
            ack
        }"#;

    assert_eq!(
        execute(query, None, &schema, &Variables::new(), &context),
        Ok((
            Value::object(
                vec![("ack", Value::string("NONE"))].into_iter().collect(),
            ),
            vec![],
        ))
    );
}

//TODO: Errors

#[test]
fn power() {
    let mock = mock_new!();

    mock.get_uptime.return_value(Ok(10));

    let context = Context {
        subsystem: Subsystem {
            ants: Box::new(mock),
            errors: RefCell::new(vec![]),
            count: 4,
        },
    };

    let schema = RootNode::new(schema::QueryRoot, schema::MutationRoot);

    let query = r#"
        {
            power {
                state,
                uptime
            }
        }"#;

    assert_eq!(
        execute(query, None, &schema, &Variables::new(), &context),
        Ok((
            Value::object(
                vec![
                    (
                        "power",
                        Value::object(
                            vec![
                                ("state", Value::string("ON")),
                                ("uptime", Value::int(10)),
                            ].into_iter()
                                .collect(),
                        )
                    ),
                ].into_iter()
                    .collect(),
            ),
            vec![],
        ))
    );
}

#[test]
fn config() {
    let mock = mock_new!();

    let context = Context {
        subsystem: Subsystem {
            ants: Box::new(mock),
            errors: RefCell::new(vec![]),
            count: 4,
        },
    };

    let schema = RootNode::new(schema::QueryRoot, schema::MutationRoot);

    let query = r#"
        {
            config
        }"#;

    assert_eq!(
        execute(query, None, &schema, &Variables::new(), &context),
        Ok((
            Value::object(
                vec![("power", Value::string("Not Implemented"))]
                    .into_iter()
                    .collect(),
            ),
            vec![],
        ))
    );
}

//TODO: telemetry. Is the union getting broken up?

#[test]
fn test_results() {
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

    mock.get_activation_count.return_value_for(
        (KANTSAnt::Ant1),
        Ok(1),
    );
    mock.get_activation_time.return_value_for(
        (KANTSAnt::Ant1),
        Ok(11),
    );
    mock.get_activation_count.return_value_for(
        (KANTSAnt::Ant2),
        Ok(2),
    );
    mock.get_activation_time.return_value_for(
        (KANTSAnt::Ant2),
        Ok(22),
    );
    mock.get_activation_count.return_value_for(
        (KANTSAnt::Ant3),
        Ok(3),
    );
    mock.get_activation_time.return_value_for(
        (KANTSAnt::Ant3),
        Ok(33),
    );
    mock.get_activation_count.return_value_for(
        (KANTSAnt::Ant4),
        Ok(4),
    );
    mock.get_activation_time.return_value_for(
        (KANTSAnt::Ant4),
        Ok(44),
    );

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

    let context = Context {
        subsystem: Subsystem {
            ants: Box::new(mock),
            errors: RefCell::new(vec![]),
            count: 4,
        },
    };

    let schema = RootNode::new(schema::QueryRoot, schema::MutationRoot);

    let query = r#"
        {
            testResults {
                success,
                telemetryNominal {
                     rawTemp,
                     uptime,
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
                },
                telemetryDebug {
                     ant1ActivationCount,
                     ant1ActivationTime,
                     ant2ActivationCount,
                     ant2ActivationTime,
                     ant3ActivationCount,
                     ant3ActivationTime,
                     ant4ActivationCount,
                     ant4ActivationTime,
                }
            }
        }"#;

    assert_eq!(
        execute(query, None, &schema, &Variables::new(), &context),
        Ok((
            Value::object(
                vec![
                    (
                        "testResults",
                        Value::object(
                            vec![
                                ("success", Value::boolean(true)),
                                (
                                    "telemtryNominal",
                                    Value::object(
                                        vec![
                                            ("rawtemp", Value::int(15)),
                                            ("uptime", Value::int(35)),
                                            ("sysBurnActive", Value::boolean(false)),
                                        ].into_iter()
                                            .collect(),
                                    )
                                    //TODO: remaining return fields
                                ),
                            ].into_iter()
                                .collect(),
                        )
                    ),
                ].into_iter()
                    .collect(),
            ),
            vec![],
        ))
    );
}
