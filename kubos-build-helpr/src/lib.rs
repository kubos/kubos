use std::env;
use std::process::Command;

/// Performs all the setup, build and configuration neccesary
/// to compile and link a C-based yotta module under Cargo
pub fn build_module(module_name: &str) {
    let kubos_target = determine_target();

    do_build(&kubos_target);

    setup_environment(module_name, &kubos_target);
}


/// Translates Cargo/Rust/Clang target triplet
/// into the proper target for `kubos build`
///
/// This bit might be better wrapped in a cargo
/// extension which takes a kubos target and then
/// determines the correct Rust triplet
fn determine_target() -> String {
    match env::var("TARGET").unwrap().as_ref() {
        // Default native vagrant target
        "x86_64-unknown-linux-gnu" => String::from("x86-linux-native"),
        // Currently set to the beaglebone black toolchain
        // We will eventually need to determine if we intend
        // to build with the pumpkin-mbm2 toolchain
        "arm-unknown-linux-gnueabidf" => String::from("kubos-linux-beaglebone-gcc"),
        // ISIS iOBC target
        "arm-unknown-linux-gnueabi" => String::from("kubos-linux-isis-gcc"),
        target => panic!("Target not supported for Kubos modules {}", target),
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
