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

use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::process;
use std::process::{Command, Stdio};
use std::str;
use tempfile::tempdir;

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

    /// Ask Cargo to build the service binary.
    /// This is a *blocking* function. We know when it returns
    /// that the service is ready to be run.
    pub fn build(&self) {
        Command::new("cargo")
            .arg("build")
            .arg("--package")
            .arg(self.name.to_owned())
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
