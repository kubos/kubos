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
use std::thread;
use std::time::Duration;
use tempfile::{NamedTempFile, TempDir};
use utils::testing::{service_query, TestService};

pub struct SchedulerFixture {
    _service: TestService,
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
        [scheduler-service]
        schedules_dir = "{}"
        "#,
            schedules_dir_path
        );

        let mut scheduler_service = TestService::new("scheduler-service", ip, port);
        scheduler_service.config(&config);
        scheduler_service.build();
        scheduler_service.spawn();

        thread::sleep(Duration::from_millis(1000));

        SchedulerFixture {
            _service: scheduler_service,
            ip: ip.to_owned(),
            port,
            _schedules_dir: schedules_dir,
            schedules_holder: RefCell::new(vec![]),
        }
    }

    pub fn create(&self) -> String {
        let schedule = NamedTempFile::new().unwrap();
        let path = schedule.path().to_str().unwrap().to_owned();

        self.schedules_holder.borrow_mut().push(schedule);

        path
    }

    pub fn register(&self, name: &str, path: &str) -> serde_json::Value {
        let mutation = format!(
            r#"mutation {{ register(name: "{}", path: "{}") {{ errors, success }} }}"#,
            name, path
        );

        service_query(&mutation, &self.ip, self.port)
    }

    pub fn activate(&self, name: &str) -> serde_json::Value {
        let mutation = format!(
            r#"mutation {{ activate(name: "{}") {{ errors, success }} }}"#,
            name
        );

        service_query(&mutation, &self.ip, self.port)
    }

    pub fn remove(&self, name: &str) -> serde_json::Value {
        let mutation = format!(
            r#"mutation {{ remove(name: "{}") {{ errors, success }} }}"#,
            name
        );

        service_query(&mutation, &self.ip, self.port)
    }

    pub fn query(&self, query: &str) -> serde_json::Value {
        service_query(&query, &self.ip, self.port)
    }
}
