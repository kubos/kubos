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

use std::fs;
use std::path::Path;
use std::thread;
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
    let mut fixture = AppServiceFixture::setup();
    setup_apps(&fixture.registry_dir.path());

    fixture.start_service(true);
    thread::sleep(Duration::from_secs(1));
    fixture.teardown();
}

#[test]
fn onboot_cleanup() {
    let mut fixture = AppServiceFixture::setup();
    setup_apps(&fixture.registry_dir.path());
    
    let control_str = format!("{}/{}/{}", fixture.registry_dir.path().to_string_lossy(), "1-2-3-4-5", "1.0.0");
    let control = Path::new(&control_str);
    assert!(control.exists());
    
    let app_str = format!("{}/{}/{}", fixture.registry_dir.path().to_string_lossy(), "a-b-c-d-e", "0.0.3");
    let app_dir = Path::new(&app_str);
    assert!(app_dir.exists());
    
    assert!(fs::remove_file(app_dir.join("app3")).is_ok());

    fixture.start_service(true);
    
    thread::sleep(Duration::from_millis(200));
    
    fixture.teardown();
    
    assert!(!app_dir.exists());
    assert!(control.exists());
}