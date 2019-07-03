/*
 * Copyright (C) 2019 Kubos Corporation
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

// Test Rust project to help exercise all of the possible app framework behavior

use failure::{bail, Error};
use getopts::Options;
use kubos_app::*;

struct MyApp;

impl AppHandler for MyApp {
    fn on_boot(&self, _args: Vec<String>) -> Result<(), Error> {
        // Test that we can access a file which was packaged with this binary
        let contents = ::std::fs::read_to_string("testfile")?;

        assert_eq!(contents, "test string");

        Ok(())
    }

    fn on_command(&self, args: Vec<String>) -> Result<(), Error> {
        let mut success = false;
        
        if args.is_empty() {
            // Test using a custom config file
            let service = ServiceConfig::new("test-service");
            if service.hosturl() == "123.4.5.6:7890" {
                success = true;
            } else {
                bail!("Service URL mismatch: {}", service.hosturl());
            }
        }

        // Test passing through args to this underlying app logic
        let mut opts = Options::new();
        opts.optflag("f", "", "Test flag");
        opts.optopt("t", "test", "Test arg", "TEST");
        opts.optflag("h", "help", "Print this help menu");

        let matches = match opts.parse(&args) {
            Ok(r) => r,
            Err(f) => panic!(f.to_string()),
        };

        if matches.opt_present("f") {
            success = true;
        }

        if matches.opt_str("t") == Some("test".to_owned()) {
            success = true;
        }

        // Check for a positional arg
        if !matches.free.is_empty() && matches.free[0] == "pos" {
            success = true;
        }

        if success {
            Ok(())
        } else {
            bail!("Did not receive any valid arguments");
        }
    }
}

fn main() -> Result<(), Error> {
    let app = MyApp;
    app_main!(&app)?;

    Ok(())
}
