//
// Copyright (C) 2019 Kubos Corporation
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
fn ack_default() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_good_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let query = r#"{
            ack
        }"#;

    let expected = json!({
            "ack": "NONE"
    });

    test!(service, query, expected);
}

#[test]
fn ack_noop() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_good_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let mutation = r#"mutation {
            noop {
                success
            }
        }"#;

    request!(service, mutation);

    let query = r#"{
            ack
        }"#;

    let expected = json!({
            "ack": "NOOP"
    });

    test!(service, query, expected);
}

#[test]
fn ack_manual_reset() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_good_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let mutation = r#"mutation {
            manualReset {
                success
            }
        }"#;

    request!(service, mutation);

    let query = r#"{
            ack
        }"#;

    let expected = json!({
            "ack": "MANUAL_RESET"
    });

    test!(service, query, expected);
}

#[test]
fn ack_reset_watchdog() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_good_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let mutation = r#"mutation {
            resetWatchdog {
                success
            }
        }"#;

    request!(service, mutation);

    let query = r#"{
            ack
        }"#;

    let expected = json!({
            "ack": "RESET_WATCHDOG"
    });

    test!(service, query, expected);
}

#[test]
fn ack_set_watchdog_period() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_good_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let mutation = r#"mutation {
            setWatchdogPeriod(period: 20) {
                success
            }
        }"#;

    request!(service, mutation);

    let query = r#"{
            ack
        }"#;

    let expected = json!({
            "ack": "SET_WATCHDOG_PERIOD"
    });

    test!(service, query, expected);
}

#[test]
fn ack_issue_raw_command() {
    let config: Config = Default::default();
    let subsystem: Box<Subsystem> = Box::new(Subsystem::new(gen_mock_good_eps()).unwrap());
    let service = Service::new(config, subsystem, QueryRoot, MutationRoot);

    let mutation = r#"mutation {
            issueRawCommand(command: 4, data: [1, 2, 3, 4]) {
                success
            }
        }"#;

    request!(service, mutation);

    let query = r#"{
            ack
        }"#;

    let expected = json!({
            "ack": "RAW_COMMAND"
    });

    test!(service, query, expected);
}
