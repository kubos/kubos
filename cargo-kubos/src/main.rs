//
// Copyright (C) 2017 Kubos Corporation
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

extern crate getopts;

use getopts::Options;
use std::process::{exit, Command, Stdio};
use std::env;

/// Take a kubos target and convert it
/// to a Rust/Clang target triplet
fn target_converter(kubos_target: &str) -> String {
    match kubos_target {
        "x86-linux-native" => String::from("x86_64-unknown-linux-gnu"),
        "kubos-linux-beaglebone-gcc" => String::from("arm-unknown-linux-gnueabihf"),
        "kubos-linux-pumpkin-mbm2-gcc" => String::from("arm-unknown-linux-gnueabihf"),
        "kubos-linux-isis-gcc" => String::from("arm-unknown-linux-gnueabi"),
        _ => {
            panic!(
                "Target not supported for cargo/yotta builds {}",
                kubos_target
            )
        }
    }
}

/// Perform `cargo build` using the proper
/// Rust/Clang target triplet
fn cargo_build(target: &str) {
    let status = Command::new("cargo")
        .args(&["build", "--target", target, "-vv"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap();

    // Attempt to exit in a way which
    // honors the subprocess exit code
    if status.success() {
        exit(1)
    } else {
        exit(status.code().unwrap());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("b", "build", "builds module");
    opts.optopt("t", "target", "sets (kubos) build target", "NAME");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("b") {
        let k_target = match matches.opt_str("t") {
            Some(t) => t,
            None => String::from("x86-linux-native"),
        };
        let c_target = target_converter(&k_target);
        env::set_var("CARGO_KUBOS_TARGET", &k_target);
        cargo_build(&c_target);
    } else {
        println!("cargo-kubos is a helper utility for building");
        println!("kubos/yotta based modules under Cargo. It is");
        println!("primarily used when building crates which either");
        println!("contain a yotta module or depend on one.");
        println!("");
        println!("    -b, --build   : builds current module");
        println!("    -t, --target  : sets kubos target");
    }
}
