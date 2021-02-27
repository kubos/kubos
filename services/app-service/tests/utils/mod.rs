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

use failure::{bail, Error};
use kubos_app::ServiceConfig;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use std::{env, fs};
use std::{thread, thread::JoinHandle};

use tempfile::TempDir;

pub struct MockAppBuilder {
    _name: String,
    _bin: Option<String>,
    _version: Option<String>,
    _author: Option<String>,
    _active: Option<bool>,
    _config: Option<String>,
}

impl MockAppBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            _name: String::from(name),
            _bin: None,
            _version: None,
            _author: None,
            _active: None,
            _config: None,
        }
    }

    pub fn bin(&mut self, bin_name: &str) -> &mut Self {
        self._bin = Some(String::from(bin_name));
        self
    }

    pub fn version(&mut self, version: &str) -> &mut Self {
        self._version = Some(String::from(version));
        self
    }

    pub fn author(&mut self, author: &str) -> &mut Self {
        self._author = Some(String::from(author));
        self
    }

    pub fn active(&mut self, active: bool) -> &mut Self {
        self._active = Some(active);
        self
    }

    pub fn config(&mut self, config: &str) -> &mut Self {
        self._config = Some(String::from(config));
        self
    }

    pub fn toml(&self, registry_dir: &str) -> String {
        format!(
            r#"
            active_version = {active}

            [app]
            executable = "{dir}/{name}/{version}/{bin}"
            name = "{name}"
            version = "{version}"
            author = "{author}"
            config = "{config}"
            "#,
            name = self._name,
            dir = registry_dir,
            active = self._active.unwrap_or(true),
            version = self._version.as_ref().unwrap_or(&String::from("0.0.1")),
            author = self._author.as_ref().unwrap_or(&String::from("unknown")),
            bin = self._bin.as_ref().unwrap_or(&self._name),
            config = self
                ._config
                .as_ref()
                .unwrap_or(&String::from("/etc/kubos-config.toml")),
        )
    }

    pub fn src(&self) -> String {
        format!(
            r#"#!/bin/bash
            if [ "$1" = "--metadata" ]; then
                echo name = \"{name}\"
                echo version = \"{version}\"
                echo author = \"{author}\"
            else
                echo "Hello, World!"
            fi
            "#,
            name = self._name,
            version = self._version.as_ref().unwrap_or(&String::from("0.0.1")),
            author = self._author.as_ref().unwrap_or(&String::from("user")),
        )
    }

    pub fn install(&self, registry_dir: &Path) {
        let mut parent_dir = registry_dir.to_path_buf().clone();
        parent_dir.push(&self._name);
        parent_dir.push(self._version.as_ref().unwrap_or(&String::from("0.0.1")));
        assert!(fs::create_dir_all(parent_dir.clone()).is_ok());

        let mut app_bin = parent_dir.clone();
        app_bin.push(self._bin.as_ref().unwrap_or(&self._name));

        self.generate_bin(&app_bin);

        let mut app_toml = parent_dir.clone();
        app_toml.push("app.toml");

        self.generate_toml(&registry_dir, &app_toml);
    }

    pub fn generate_bin(&self, bin_dest: &Path) {
        let mut file = fs::File::create(bin_dest).unwrap();
        if file.write_all(self.src().as_bytes()).is_err() {
            panic!("failed to write app script");
        }

        let mut perms = file.metadata().unwrap().permissions();
        perms.set_mode(0o755);

        if file.set_permissions(perms).is_err() {
            panic!("failed to change permissions of app script");
        }
    }

    pub fn generate_toml(&self, registry_dir: &Path, toml_dest: &Path) {
        let mut file = fs::File::create(toml_dest).unwrap();
        let toml = self.toml(registry_dir.to_str().unwrap());
        if file.write_all(toml.as_bytes()).is_err() {
            panic!("Failed to write app TOML");
        }
    }
}

pub struct AppServiceFixture {
    pub registry_dir: TempDir,
    pub addr: String,
    config_toml: PathBuf,
    join_handle: Option<JoinHandle<()>>,
    sender: Option<Sender<bool>>,
}

impl AppServiceFixture {
    fn service_port() -> Result<u16, Error> {
        use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
        let port = {
            let loopback = Ipv4Addr::new(127, 0, 0, 1);
            // Every now and again we get port collisions, so we'll retry a couple times to set up
            // a socket for our service
            let mut counter = 0;
            let listener = loop {
                let socket = SocketAddrV4::new(loopback, 0);

                match TcpListener::bind(socket) {
                    Ok(obj) => break obj,
                    Err(error) => {
                        if counter == 2 {
                            bail!("Failed to bind socket: {:?}", error);
                        }
                    }
                }

                counter += 1;
            };
            listener.local_addr()?.port()
        };
        Ok(port)
    }

    pub fn setup() -> Self {
        let registry_dir = TempDir::new().expect("Failed to create registry dir");

        let mut config_toml = registry_dir.path().to_path_buf().clone();
        config_toml.push("config.toml");

        let mut toml = fs::File::create(config_toml.clone()).unwrap();
        let port = Self::service_port().unwrap_or(9999);

        println!(
            "Registry dir: {}, Port: {}",
            registry_dir.path().to_str().unwrap(),
            port
        );

        toml.write_all(
            format!(
                r#"
            [app-service]
            registry-dir = "{}"
            [app-service.addr]
            ip = "127.0.0.1"
            port = {}"#,
                registry_dir.path().to_str().unwrap(),
                port
            )
            .as_bytes(),
        )
        .expect("Failed to write config.toml");

        Self {
            registry_dir,
            addr: format!("127.0.0.1:{}", port),
            config_toml: config_toml.clone(),
            join_handle: None,
            sender: None,
        }
    }

    pub fn start_service(&mut self) {
        let mut app_service = env::current_exe().unwrap();
        app_service.pop();
        app_service.set_file_name("kubos-app-service");

        let (tx, rx): (Sender<bool>, Receiver<bool>) = channel();
        let config_toml = self.config_toml.clone();

        let handle = thread::spawn(move || {
            let mut service = Command::new(app_service);

            service
                .arg("-c")
                .arg(config_toml.to_str().unwrap())
                .stdin(Stdio::piped())
                .stdout(Stdio::piped());

            let mut service_proc = service.spawn().unwrap();

            thread::sleep(Duration::from_millis(100));
            match service_proc.try_wait() {
                Ok(Some(status)) => panic!("Service exited early: {}", status),
                Ok(None) => {}
                Err(err) => panic!("Failed to wait for service: {}", err),
            }

            let mut run = true;
            while run {
                thread::sleep(Duration::from_millis(100));
                if rx.try_recv().is_ok() {
                    service_proc.kill().unwrap();
                    run = false;
                }
            }
        });

        // Give the process a bit to actually start
        thread::sleep(Duration::from_millis(1000));
        self.join_handle = Some(handle);
        self.sender = Some(tx);
    }
}

impl Drop for AppServiceFixture {
    fn drop(&mut self) {
        if self.sender.is_some() {
            self.sender.take().unwrap().send(true).unwrap();
        }
        if self.join_handle.is_some() {
            self.join_handle.take().unwrap().join().unwrap();
        }
    }
}

pub fn send_query(config: ServiceConfig, query: &str) -> serde_json::Value {
    let client = reqwest::Client::new();

    let uri = format!("http://{}", config.hosturl().unwrap());

    let mut map = ::std::collections::HashMap::new();
    map.insert("query", query);

    let response: serde_json::Value = client
        .post(&uri)
        .json(&map)
        .send()
        .expect("Couldn't send request")
        .json()
        .expect("Couldn't deserialize response");

    if let Some(data) = response.get("data") {
        data.clone()
    } else {
        panic!("Bad query response: {:?}", response);
    }
}
