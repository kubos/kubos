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

extern crate tempfile;

use serde_json::Value;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::Write;
use std::process;
use std::process::{Command, Stdio};
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tempfile::tempdir;
use warp::{self, Buf, Filter};

pub struct TestCommand {
    command: String,
    args: Vec<&'static str>,
    child_handle: RefCell<Box<Option<process::Child>>>,
}

impl TestCommand {
    pub fn new(command: &str, args: Vec<&'static str>) -> TestCommand {
        TestCommand {
            command: String::from(command),
            args,
            child_handle: RefCell::new(Box::new(None)),
        }
    }

    /// Ask Cargo to run the command.
    /// This is *not* a blocking function. The command is
    /// spawned in the background, allowing the test
    /// to continue on.
    pub fn spawn(&self) {
        let child = Command::new(&self.command)
            .args(&self.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start");

        let mut child_handle = self.child_handle.borrow_mut();
        *child_handle = Box::new(Some(child));
    }
}

/// This structure allows the creation of an instance
/// of an actual service/binary crate for use in
/// integration tests within the same crate.
pub struct TestService {
    config_path: String,
    // Keep around so the temp file stays alive
    _config_file: File,
    // Keep around so the temp dir stays alive
    _tmp_dir: tempfile::TempDir,
    name: String,
    child_handle: RefCell<Box<Option<process::Child>>>,
}

impl TestService {
    /// Create config for TestService and return basic struct
    pub fn new(name: &str, ip: &str, port: u16) -> TestService {
        let mut config = Vec::new();
        writeln!(&mut config, "[{}.addr]", name).unwrap();
        writeln!(&mut config, "ip = \"{}\"", ip).unwrap();
        writeln!(&mut config, "port = {}", port).unwrap();
        let config_str = String::from_utf8(config).unwrap();

        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.toml");
        let mut config_file = File::create(config_path.clone()).unwrap();
        writeln!(config_file, "{}", config_str).unwrap();

        TestService {
            config_path: config_path.to_str().unwrap().to_owned(),
            _config_file: config_file,
            _tmp_dir: dir,
            name: String::from(name),
            child_handle: RefCell::new(Box::new(None)),
        }
    }

    /// Appends additional configuration data to service's config
    pub fn config(&mut self, config_data: &str) {
        self._config_file.seek(SeekFrom::End(0)).unwrap();
        self._config_file.write_all(config_data.as_bytes()).unwrap();
    }

    /// Ask Cargo to build the service binary.
    /// This is a *blocking* function. We know when it returns
    /// that the service is ready to be run.
    pub fn build(&self) {
        Command::new("cargo")
            .arg("build")
            .arg("--package")
            .arg(&self.name)
            .output()
            .expect("Failed to build service");
    }

    /// Ask Cargo to run the service binary.
    /// This is *not* a blocking function. The service is
    /// spawned in the background, allowing the test
    /// to continue on.
    pub fn spawn(&self) {
        let child = Command::new("cargo")
            .arg("run")
            .arg("--package")
            .arg(self.name.clone())
            .arg("--")
            .arg("-c")
            .arg(self.config_path.clone())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start");

        let mut child_handle = self.child_handle.borrow_mut();
        *child_handle = Box::new(Some(child));
    }

    /// Kill the running process.
    pub fn kill(&self) {
        let mut borrowed_child = self.child_handle.borrow_mut();
        if let Some(mut handle) = borrowed_child.take() {
            handle.kill().unwrap();
        }
    }
}

/// Implement custom drop functionality which
/// will retrieve handle to child process and kill it.
impl Drop for TestService {
    fn drop(&mut self) {
        let mut borrowed_child = self.child_handle.borrow_mut();
        if let Some(mut handle) = borrowed_child.take() {
            handle.kill().unwrap();
        }
    }
}

pub fn service_query(query: &str, ip: &str, port: u16) -> Value {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(100))
        .build()
        .unwrap();
    let mut map = ::std::collections::HashMap::new();
    map.insert("query", query);
    for _ in 0..5 {
        if let Ok(mut result) = client
            .post(&format!("http://{}:{}", ip, port))
            .json(&map)
            .send()
        {
            return serde_json::from_str(&result.text().unwrap()).unwrap();
        }
        thread::sleep(Duration::from_millis(100));
    }

    panic!("Service query failed - {}:{}", ip, port);
}

pub struct ServiceListener {
    requests: Arc<Mutex<VecDeque<String>>>,
}

impl ServiceListener {
    /// Spawns a new dummy service listener
    /// This listener will just listen for posts and save off
    /// the request bodies for examination in tests
    pub fn spawn(_ip: &str, port: u16) -> ServiceListener {
        let requests = Arc::new(Mutex::new(VecDeque::<String>::new()));

        let req_handle = requests.clone();

        let listener = warp::post2()
            .and(warp::any())
            .and(warp::body::concat().and_then(|body: warp::body::FullBody| {
                std::str::from_utf8(body.bytes())
                    .map(String::from)
                    .map_err(warp::reject::custom)
            }))
            .map(move |body: String| {
                req_handle.lock().unwrap().push_back(body);
                "hi"
            });

        thread::spawn(move || warp::serve(listener).run(([127, 0, 0, 1], port)));

        ServiceListener {
            requests,
        }
    }

    /// Pops a request body from the collected queue
    /// These are popped off in FIFO fashion
    pub fn get_request(&self) -> Option<String> {
        self.requests.lock().unwrap().pop_front()
    }
}
