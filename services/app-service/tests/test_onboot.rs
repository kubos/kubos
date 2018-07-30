/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#![deny(warnings)]
extern crate kubos_app;
extern crate kubos_system;
extern crate serde_json;
extern crate tempfile;

use std::{env, thread};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

mod utils;
pub use utils::*;

fn setup_apps(registry_dir: &Path) {
    MockAppBuilder::new("app1", "a-b-c-d-e")
        .version("0.0.1")
        .active(false)
        .author("mham")
        .install(&registry_dir);
    MockAppBuilder::new("app1", "a-b-c-d-e")
        .version("0.0.2")
        .active(false)
        .install(&registry_dir);
    MockAppBuilder::new("app2", "f-g-h-i-j")
        .version("1.0.0")
        .active(false)
        .install(&registry_dir);
    MockAppBuilder::new("app2", "f-g-h-i-j")
        .version("1.0.1")
        .active(true)
        .install(&registry_dir);
    MockAppBuilder::new("app3", "a-b-c-d-e")
        .version("0.0.3")
        .active(true)
        .install(&registry_dir);
    MockAppBuilder::new("app4", "1-2-3-4-5")
        .version("1.0.0")
        .active(true)
        .author("user")
        .install(&registry_dir);
}

// This just tests that calling the app service with the '-b' parameter
// doesn't panic anywhere
#[test]
fn onboot_good() {
    let fixture = AppServiceFixture::setup();
    setup_apps(&fixture.registry_dir.path());

    let mut app_service = env::current_exe().unwrap();
    app_service.pop();
    app_service.set_file_name("kubos-app-service");

    let config_toml = fixture.config_toml.clone();

    let handle: thread::JoinHandle<_> = thread::spawn(move || {
        let mut cmd = Command::new(app_service)
            .arg("-c")
            .arg(config_toml.to_str().unwrap())
            .arg("-b")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        thread::sleep(Duration::from_millis(100));
        match cmd.try_wait() {
            Ok(Some(status)) => panic!("Command exited early: {}", status),
            Ok(None) => {}
            Err(err) => panic!("Failed to wait for command: {}", err),
        }

        thread::sleep(Duration::from_millis(1000));
        // Kill the app service that was started. Since it was spawned
        // as a command it'll just run forever if we let it.
        cmd.kill().unwrap();
    });

    assert!(handle.join().is_ok());
}
