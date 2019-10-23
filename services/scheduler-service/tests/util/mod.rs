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

use std::cell::RefCell;
use std::io::Write;
use std::thread;
use std::time::Duration;
use tempfile::{NamedTempFile, TempDir};
use utils::testing::{service_query, TestService};

pub struct SchedulerFixture {
    service: RefCell<TestService>,
    ip: String,
    port: u16,
    _schedules_dir: TempDir,
    schedules_holder: RefCell<Vec<NamedTempFile>>,
}

#[allow(dead_code)]
impl SchedulerFixture {
    pub fn spawn(ip: &str, port: u16) -> SchedulerFixture {
        let schedules_dir = TempDir::new().unwrap();
        let schedules_dir_path = schedules_dir.path().to_str().unwrap();

        let config = format!(
            r#"
        [app-service.addr]
        ip = "{}"
        port = {}
        [scheduler-service]
        schedules_dir = "{}"
        "#,
            ip,
            (port + 1000),
            schedules_dir_path,
        );

        let mut scheduler_service = TestService::new("scheduler-service", ip, port);
        scheduler_service.config(&config);
        scheduler_service.build();
        scheduler_service.spawn();

        thread::sleep(Duration::from_millis(2000));

        SchedulerFixture {
            service: RefCell::new(scheduler_service),
            ip: ip.to_owned(),
            port,
            _schedules_dir: schedules_dir,
            schedules_holder: RefCell::new(vec![]),
        }
    }

    pub fn restart(&self) {
        let borrowed_service = self.service.borrow_mut();
        borrowed_service.kill();
        borrowed_service.spawn();
        thread::sleep(Duration::from_millis(1000));
    }

    pub fn create_task_list(&self, contents: Option<String>) -> String {
        let mut schedule = NamedTempFile::new().unwrap();

        if let Some(c) = contents {
            schedule.write_all(c.as_bytes()).unwrap();
        }

        let path = schedule.path().to_str().unwrap().to_owned();

        self.schedules_holder.borrow_mut().push(schedule);

        path
    }

    // Sends createMode mutation to service under test
    pub fn create_mode(&self, name: &str) -> serde_json::Value {
        let mutation = format!(
            r#"mutation {{ createMode(name: "{}") {{ errors, success }} }}"#,
            name
        );

        service_query(&mutation, &self.ip, self.port)
    }

    // Sends createMode mutation to service under test
    pub fn remove_mode(&self, name: &str) -> serde_json::Value {
        let mutation = format!(
            r#"mutation {{ removeMode(name: "{}") {{ errors, success }} }}"#,
            name
        );

        service_query(&mutation, &self.ip, self.port)
    }

    // Sends activateMode mutation to service under test
    pub fn activate_mode(&self, name: &str) -> serde_json::Value {
        let mutation = format!(
            r#"mutation {{ activateMode(name: "{}") {{ errors, success }} }}"#,
            name
        );

        service_query(&mutation, &self.ip, self.port)
    }

    pub fn activate_safe(&self) -> serde_json::Value {
        let mutation = format!(r#"mutation {{ safeMode {{ errors, success }} }}"#,);

        service_query(&mutation, &self.ip, self.port)
    }

    pub fn import_task_list(&self, name: &str, path: &str, mode: &str) -> serde_json::Value {
        let mutation = format!(
            r#"mutation {{ importTaskList(name: "{}", path: "{}", mode: "{}") {{ errors, success }} }}"#,
            name, path, mode
        );

        service_query(&mutation, &self.ip, self.port)
    }

    pub fn import_raw_task_list(&self, name: &str, mode: &str, json: &str) -> serde_json::Value {
        let mutation = format!(
            r#"mutation {{ importRawTaskList(name: "{}", mode: "{}", json: "{}") {{ errors, success }} }}"#,
            name, mode, json
        );

        service_query(&mutation, &self.ip, self.port)
    }

    pub fn remove_task_list(&self, name: &str, mode: &str) -> serde_json::Value {
        let mutation = format!(
            r#"mutation {{ removeTaskList(name: "{}", mode: "{}") {{ errors, success }} }}"#,
            name, mode
        );

        service_query(&mutation, &self.ip, self.port)
    }

    pub fn query(&self, query: &str) -> serde_json::Value {
        service_query(&query, &self.ip, self.port)
    }
}
