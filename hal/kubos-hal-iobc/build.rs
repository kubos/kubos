/// A custom build.rs for building out the kubos-hal-iobc
/// yotta module and exporting it as a Rust crate

extern crate kubos_build_helper;

fn main() {
    kubos_build_helper::build_module("kubos-hal-iobc");
}
