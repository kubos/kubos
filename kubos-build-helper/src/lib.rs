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

use glob::glob;
use std::process::Command;

/// Performs all the setup, build and configuration neccesary
/// to compile and link a C-based cmake project under Cargo
pub fn build_module() {
    let dst = cmake::Config::new("").build_target("").build();
    setup_libraries(dst.to_str().unwrap());
}

// The default behavior of cmake is to create
// multiple static libraries which are linked into
// the final executable. We will copy all of the static
// libraries created by cmake into the folder where
// rustc can link against them.
fn setup_libraries(out_dir: &str) {
    let lib_glob = format!("{}/build/**/*.a", out_dir);

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
                if Command::new("cp")
                    .args(&[path.to_str().unwrap(), &out_path])
                    .status()
                    .unwrap()
                    .success()
                {
                    // Inform Cargo -> Rust about this library file
                    println!(
                        "cargo:rustc-link-lib=static={}",
                        path.file_stem().unwrap().to_str().unwrap()
                    );
                } else {
                    panic!("Failed to copy lib {}", path.display())
                }
            }
            Err(e) => println!("bad path {:?}", e),
        }
    }
}
