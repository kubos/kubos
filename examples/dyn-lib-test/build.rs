use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // let out_dir = env::var("OUT_DIR").unwrap();

    println!("cargo:rustc-link-search=/lib");
    println!("cargo:rustc-link-lib=dylib=gtk-3");
}
