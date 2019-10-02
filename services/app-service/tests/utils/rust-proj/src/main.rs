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
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Error> {
    
    logging_setup!("rust-proj")?;
    
    let mut success = false;
    
    let args: Vec<String> = ::std::env::args().collect();

    if args.is_empty() {
        // Test using a custom config file
        let service = ServiceConfig::new("test-service")?;
        if service.hosturl() == Some("123.4.5.6:7890".to_owned()) {
            success = true;
        } else {
            bail!("Service URL mismatch: {:?}", service.hosturl());
        }
        
        // Test that we can access a file which was packaged with this binary
        let contents = ::std::fs::read_to_string("testfile")?;

        assert_eq!(contents, "test string");
    }
    
    let program = args[0].clone();
    
    let mut opts = Options::new();
    // Standard app args:
    // This option will be processed by the system-api crate when a service query is run
    opts.optflagopt(
        "c",
        "config",
        "System config file which should be used",
        "CONFIG",
    );
    opts.optflag("h", "help", "Print this help menu");
    // Test-specific args:
    opts.optflag("f", "", "Test flag");
    opts.optopt("t", "test", "Test arg", "TEST");
    opts.optflag("l", "", "Long running test");

    let matches = match opts.parse(&args[1..]) {
        Ok(r) => r,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(&brief));
        return Ok(());
    }

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
    
    if matches.opt_present("l") {
        // We want to test the app service's ability to track an application which doesn't
        // immediately return.
        // Future work: Add option to return a non-zero RC
        thread::sleep(Duration::from_secs(2));
        success = true;
    }

    if success {
        Ok(())
    } else {
        bail!("Did not receive any valid arguments");
    }
}
