/// A custom build.rs for building out the yotta
/// module and exporting it as a Rust crate.

fn main() {
    kubos_build_helper::build_module();
}
