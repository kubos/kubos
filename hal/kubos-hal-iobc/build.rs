/// A custom build.rs for building out the kubos-hal-iobc
/// yotta module and exporting it as a Rust crate

extern crate kubos_build_helpr;

/// The majority of the code here is hard-coded boilerplate
/// and should be moved out into another kubos-build crate
fn main() {
    kubos_build_helpr::build_module("kubos-hal-iobc");
}
