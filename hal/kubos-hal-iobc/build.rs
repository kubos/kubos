/// A custom build.rs for building out the kubos-hal-iobc
/// yotta module and exporting it as a Rust crate

use std::process::Command;

/// The majority of the code here is hard-coded boilerplate
/// and should be moved out into another kubos-build crate
fn main() {
    // Can we attach this to cargo clean?
    Command::new("kubos").args(&["clean"]).status().unwrap();

    // This should be dependent on the architecture
    // of the current cargo target/build configuration
    Command::new("kubos")
        .args(&["-t", "x86-linux-native", "build"])
        .status()
        .unwrap();

    // The default behavior of rustc-link-lib=static
    // is to look for a library named lib{name}.a
    Command::new("cp")
        .args(
            &[
                "build/x86-linux-native/source/kubos-hal-iobc.a",
                "build/x86-linux-native/source/libkubos-hal-iobc.a",
            ],
        )
        .status()
        .unwrap();

    // We need to figure out how to infer this information from
    // the information in the ./build folder
    println!("cargo:rustc-link-search=native=build/x86-linux-native/source");
    println!("cargo:rustc-link-lib=static=kubos-hal-iobc");
}
