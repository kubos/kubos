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

extern crate glob;

use std::env;
use std::process::Command;

use glob::glob;

/// Performs all the setup, build and configuration neccesary
/// to compile and link a C-based yotta module under Cargo
pub fn build_module() {
    let kubos_target = determine_target();
    do_build(&kubos_target);
    setup_libraries(&kubos_target);
}

/// Retrieve the kubos target from the environment
/// variable CARGO_KUBOS_TARGET. This variable *should*
/// get set when the `cargo kubos` command is run.
fn determine_target() -> String {
    match env::var("CARGO_KUBOS_TARGET") {
        Ok(val) => val,
        Err(e) => panic!("Error retrieving cargo-kubos target: {}", e),
    }
}

/// Performs the actual `kubos build` process
///
/// We also do a `kubos clean` beforehand because we
/// aren't building in a way that Cargo is aware of
/// so it isn't able to clean up for us
fn do_build(target: &str) {
    // Link in all system kubos modules
    if !Command::new("kubos")
        .args(&["link", "-a"])
        .status()
        .unwrap()
        .success()
    {
        panic!("Failed to run kubos link")
    }

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
        .args(&["-t", target, "build"])
        .status()
        .unwrap()
        .success()
    {
        panic!("Failed to run kubos build")
    }
}

// The default behavior of yotta is to create
// multiple static libraries which are linked into
// the final executable. We will copy all of the static
// libraries created by yotta into the folder where
// rustc can link against them.
fn setup_libraries(target: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let lib_glob = format!(
        "{}/build/{}/**/*.a",
        env::current_dir().unwrap().display(),
        target
    );

    // Inform Cargo -> Rustc about which folder our static
    // libraries will be in
    println!("cargo:rustc-link-search=native={}", out_dir);

    // The default behavior of rustc-link-lib=static
    // is to look for a library named lib{name}.a
    // We will copy each static lib into the right
    // location (OUT_DIR) with the correct name.
    for entry in glob(&lib_glob).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let out_path = format!(
                    "{}/lib{}.a",
                    out_dir,
                    path.file_stem().unwrap().to_str().unwrap()
                );
                if !Command::new("cp")
                    .args(&[path.to_str().unwrap(), &out_path])
                    .status()
                    .unwrap()
                    .success()
                {
                    panic!("Failed to copy lib {}", path.display())
                } else {
                    // Inform Cargo -> Rust about this library file
                    println!(
                        "cargo:rustc-link-lib=static={}",
                        path.file_stem().unwrap().to_str().unwrap()
                    );
                }
            }
            Err(e) => println!("bad path {:?}", e),
        }
    }
}
