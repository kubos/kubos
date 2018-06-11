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
extern crate kubos_app;
extern crate kubos_system;
#[macro_use]
extern crate serde_json;

use std::env;
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::panic;
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use std::{thread, thread::JoinHandle};

const DUMMY_APP_SRC: &'static str = r#"
#!/bin/bash
if [ "$1" = "--metadata" ]; then
    echo name = \"dummy\"
    echo version = \"0.0.1\"
    echo author = \"user\"
else
    echo uuid = \"$KUBOS_APP_UUID\"
    echo run_level = \"$KUBOS_APP_RUN_LEVEL\"
fi
"#;

const DUMMY_APP_TOML: &'static str = r#"
active = true
run_level = "OnBoot"

[app]
uuid = "a-b-c-d-e"
pid = 0
path = "{}/a-b-c-d-e/0.0.1/dummy-app"

[app.metadata]
name = "dummy"
version = "0.0.1"
author = "user"
"#;

struct AppServiceFixture {
    registry_dir: PathBuf,
    config_toml: PathBuf,
    join_handle: Option<JoinHandle<()>>,
    sender: Option<Sender<bool>>
}

impl AppServiceFixture {
    fn setup() -> Self {
        let mut registry_dir = env::temp_dir();
        registry_dir.push("apps");

        if !registry_dir.exists() {
            assert!(fs::create_dir_all(registry_dir.clone()).is_ok());
        }

        let mut config_toml = env::temp_dir();
        config_toml.push("config.toml");

        let mut toml = fs::File::create(config_toml.clone()).unwrap();
        toml.write_all(format!(r#"
            [app-service]
            registry-dir = "{}"
            [app-service.addr]
            ip = "0.0.0.0"
            port = 9999"#, registry_dir.to_str().unwrap()).as_bytes())
            .expect("Failed to write config.toml");

        Self {
            registry_dir: registry_dir,
            config_toml: config_toml.clone(),
            join_handle: None,
            sender: None
        }
    }

    fn start_service(&mut self)
    {
        let mut app_service = env::current_exe().unwrap();
        app_service.pop();
        app_service.set_file_name("kubos-app-service");

        let (tx, rx): (Sender<bool>, Receiver<bool>) = channel();
        let config_toml = self.config_toml.clone();

        let handle = thread::spawn(move || {
            let mut service_proc = Command::new(app_service)
                .arg("-c")
                .arg(config_toml.to_str().unwrap())
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            let mut run = true;
            while run {
                thread::sleep(Duration::from_millis(100));
                if let Ok(_) = rx.try_recv() {
                    service_proc.kill().unwrap();
                    run = false;
                }
            }
        });

        // Give the process a bit to actually start
        thread::sleep(Duration::from_millis(100));
        self.join_handle = Some(handle);
        self.sender = Some(tx);
    }

    fn setup_dummy_bin(&self, bin_dest: &Path) {
        let mut file = fs::File::create(bin_dest.clone()).unwrap();
        if !file.write_all(DUMMY_APP_SRC.as_bytes()).is_ok() {
            panic!("failed to write app script");
        }

        let mut perms = file.metadata().unwrap().permissions();
        perms.set_mode(0o755);

        if !file.set_permissions(perms).is_ok() {
            panic!("failed to change permissions of app script");
        }
    }

    fn setup_dummy_toml(&self, toml_dest: &Path) {
        let mut file = fs::File::create(toml_dest.clone()).unwrap();
        if !file.write_all(DUMMY_APP_TOML.as_bytes()).is_ok() {
            panic!("failed to write app TOML");
        }
    }

    fn teardown(&mut self) {
        if self.sender.is_some() {
            self.sender.take().unwrap().send(true).unwrap();
        }
        if self.join_handle.is_some() {
            self.join_handle.take().unwrap().join().unwrap();
        }

        if self.registry_dir.exists() {
            assert!(fs::remove_dir_all(self.registry_dir.clone()).is_ok());
        }

    }
}


#[test]
fn register_app() {
    let mut fixture = AppServiceFixture::setup();
    let mut app_bin = env::temp_dir();
    app_bin.push("dummy-app");

    fixture.setup_dummy_bin(&app_bin);

    let register_query = format!(r#"mutation {{
        register(path: "{}") {{
            active, runLevel, app {{
                uuid, name, version, author, pid, path
            }}
        }}
    }}"#, app_bin.to_str().unwrap());

    let apps_query = r#"{ apps { active, runLevel, app {
        uuid, name, version, author, pid, path
    } } }"#;

    fn assert_expected(v: &serde_json::Value) {
        assert_eq!(v["active"], json!(true));
        assert_eq!(v["runLevel"], json!("OnCommand"));
        assert!(v["app"].is_object());
        assert_eq!(v["app"]["name"], json!("dummy"));
        assert_eq!(v["app"]["version"], json!("0.0.1"));
        assert_eq!(v["app"]["author"], json!("user"));
    }

    fixture.start_service();

    let result = panic::catch_unwind(|| {
        println!("{}", register_query);
        let result = kubos_system::query("localhost:9999", &register_query,
                                        Some(Duration::from_secs(2)));
        assert!(result.is_ok(), "{:?}", result.err());
        assert_expected(&result.unwrap()["register"]);

        let result = kubos_system::query("localhost:9999", &apps_query,
                                        Some(Duration::from_secs(5)));
        assert!(result.is_ok());
        assert_expected(&result.unwrap()["apps"][0]);

        /*let start_result = registry.start_app(&entry.app.uuid, RunLevel::OnBoot);
        assert!(start_result.is_ok(), format!("error starting app: {}", start_result.unwrap_err()));
        assert!(start_result.unwrap() > 0); // pid*/
    });
    fixture.teardown();
    assert!(result.is_ok());
}

#[test]
fn discover_app() {
    let mut fixture = AppServiceFixture::setup();
    let mut parent_dir = fixture.registry_dir.clone();
    parent_dir.push("a-b-c-d-e");
    parent_dir.push("0.0.1");
    assert!(fs::create_dir_all(parent_dir.clone()).is_ok());

    let mut app_bin = parent_dir.clone();
    app_bin.push("dummy-app");

    fixture.setup_dummy_bin(&app_bin);

    let mut app_toml = parent_dir.clone();
    app_toml.push("app.toml");

    fixture.setup_dummy_toml(&app_toml);
    fixture.start_service();

    let result = panic::catch_unwind(|| {
        let apps_query = "{ apps { active, runLevel, app { uuid, name, version, author } } }";
        let result = kubos_system::query("localhost:9999", &apps_query,
                                        Some(Duration::from_secs(5)));
        assert!(result.is_ok());

        let apps = &result.unwrap()["apps"];
        assert!(apps.is_array());
        assert_eq!(apps.as_array().unwrap().len(), 1);

        let v = &apps.as_array().unwrap()[0];
        assert_eq!(v["active"], json!(true));
        assert_eq!(v["runLevel"], json!("OnBoot"));
        assert!(v["app"].is_object());
        assert_eq!(v["app"]["uuid"], json!("a-b-c-d-e"));
        assert_eq!(v["app"]["name"], json!("dummy"));
        assert_eq!(v["app"]["version"], json!("0.0.1"));
        assert_eq!(v["app"]["author"], json!("user"));
    });

    fixture.teardown();
    assert!(result.is_ok());
}
