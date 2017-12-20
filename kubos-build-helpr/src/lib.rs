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

use std::env;
use std::process::Command;

/// Performs all the setup, build and configuration neccesary
/// to compile and link a C-based yotta module under Cargo
pub fn build_module(module_name: &str) {
    let kubos_target = determine_target();
    do_build(&kubos_target);
    setup_environment(module_name, &kubos_target);
}


/// Retrieve the kubos target from the environment
/// variable CARGO_KUBOS_TARGET. This variable *should*
/// get set when the `cargo kubos` command is run.
fn determine_target() -> String {
    match env::var("CARGO_KUBOS_TARGET") {
        Ok(val) => String::from(val),
        Err(e) => panic!("Error retrieving cargo-kubos target: {}", e),
    }
}

/// Performs the actual `kubos build` process
///
/// We also do a `kubos clean` beforehand because we
/// aren't building in a way that Cargo is aware of
/// so it isn't able to clean up for us
fn do_build(target: &str) {
    // Can we attach this to cargo clean?
    if !Command::new("kubos")
        .args(&["clean"])
        .status()
        .unwrap()
        .success()
    {
        panic!("Failed to run kubos clean")
    }

    // Build with target set explicitly
    if !Command::new("kubos")
        .args(&["-t", target.as_ref(), "build"])
        .status()
        .unwrap()
        .success()
    {
        panic!("Failed to run kubos build")
    }
}

/// Perform any neccesary post-build environment setup
fn setup_environment(module_name: &str, target: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let lib_path = format!("build/{}/source/{}.a", target, module_name);
    let out_path = format!("{}/lib{}.a", out_dir, module_name);


    // The default behavior of rustc-link-lib=static
    // is to look for a library named lib{name}.a
    // We also need to move the library into the OUT_DIR
    // so that the module building can find it
    if !Command::new("cp")
        .args(&[&lib_path, &out_path])
        .status()
        .unwrap()
        .success()
    {
        panic!("Failed to copy kubos lib")
    }

    // Inform Cargo -> Rust about where our library file is
    // and how to link against it
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static={}", module_name);
}
