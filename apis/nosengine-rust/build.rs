extern crate cmake;

fn main() {
    // This runs the CMake file in nos3_c_interface.
    // See the comments in that file for an explanation of why it's necessary.
    // The important thing is that it doesn't actually build anything, so all I
    // have to do is run this one command.
    cmake::build("nosengine_c_interface");
}
