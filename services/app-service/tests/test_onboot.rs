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
 *
 * This test file tests solely that the full top-to-bottom process of finding and kicking off
 * registered applcations at boot time doesn't mysteriously break down anywhere.
 *
 * The majority of the onBoot-related test cases are in src/tests/registry_onboot.rs, since the
 * return status of the run_onboot() function can be directly examined and verified from there.
 */
#![deny(warnings)]
extern crate kubos_app;
extern crate kubos_system;
extern crate serde_json;
extern crate tempfile;

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
