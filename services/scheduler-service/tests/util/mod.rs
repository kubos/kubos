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

use std::thread;
use std::time::Duration;
use utils::testing::{service_query, TestService};

pub fn spawn_scheduler(ip: &str, port: u16, schedules_dir: &str) -> TestService {
    let config = format!(
        r#"
        [scheduler-service]
        schedules_dir = "{}"
        "#,
        schedules_dir
    );

    let mut scheduler_service = TestService::new("scheduler-service", ip, port);
    scheduler_service.config(&config);
    scheduler_service.build();
    scheduler_service.spawn();

    thread::sleep(Duration::from_millis(1000));

    scheduler_service
}

pub fn register_schedule(name: &str, path: &str, ip: &str, port: u16) -> serde_json::Value {
    let mutation = format!(
        r#"mutation {{ register(name: "{}", path: "{}") {{ errors, success }} }}"#,
        name, path
    );

    service_query(&mutation, ip, port)
}

pub fn activate_schedule(name: &str, ip: &str, port: u16) -> serde_json::Value {
    let mutation = format!(
        r#"mutation {{ activate(name: "{}") {{ errors, success }} }}"#,
        name
    );

    service_query(&mutation, ip, port)
}

pub fn remove_schedule(name: &str, ip: &str, port: u16) -> serde_json::Value {
    let mutation = format!(
        r#"mutation {{ remove(name: "{}") {{ errors, success }} }}"#,
        name
    );

    service_query(&mutation, ip, port)
}
