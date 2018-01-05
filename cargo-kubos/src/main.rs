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
                "Target '{}' not supported for cargo/yotta builds\
                 \nCurrently supported targets are:\
                 \nx86-linux-native\
                 \nkubos-linux-beaglebone-gcc\
                 \nkubos-linux-pumpkin-mbm2-gcc\
                 \nkubos-linux-isis-gcc",
                kubos_target
            )
        }
    }
}

/// Perform `cargo 'command'` using the proper Rust/Clang target triplet
fn cargo_command(target: String, command: String, mut extra_params: Vec<String>) {
    let mut params = vec![command, String::from("--target"), target];
    params.append(&mut extra_params);

    let status = Command::new("cargo")
        .args(&params)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap();

    // Attempt to exit in a way which
    // honors the subprocess exit code
    if status.success() {
        exit(1)
    }
    exit(status.code().unwrap());
}

/// Displays usage message
fn print_usage(opts: Options) {
    let brief = "cargo-kubos is a helper utility for running \
        Cargo commands with a Kubos target attached.\nIt is \
        used when building/running/testing crates which either\
        contain a yotta module or depend on one. \
        \n\nUsage:\
        \n\tcargo kubos -c [cargo command] [options] -- [cargo options]
        \n\tcargo kubos -c build -t x86-linux-native -- -vv";
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.reqopt("c", "command", "cargo command to run", "COMMAND");
    opts.optopt("t", "target", "sets (Kubos) target", "NAME");
    opts.optflag("h", "help", "Displays help");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("Error - {}\n", f);
            print_usage(opts);
            return;
        }
    };

    // Collect extra parametrs
    let extra_params = if !matches.free.is_empty() {
        let mut params = matches.free.clone();
        // remove extra kubos pamatere
        params.retain(|x| x != "kubos");
        params
    } else {
        Vec::new()
    };

    if matches.opt_present("h") {
        print_usage(opts);
    } else {
        let k_target = match matches.opt_str("t") {
            Some(t) => t,
            None => String::from("x86-linux-native"),
        };
        let command = matches.opt_str("c").unwrap();
        let c_target = target_converter(&k_target);
        env::set_var("CARGO_KUBOS_TARGET", &k_target);
        cargo_command(c_target, command, extra_params);
    }
}
