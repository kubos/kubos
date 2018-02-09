extern crate nsl_duplex_d2;

use nsl_duplex_d2::DuplexD2;

fn main() {
    let duplex = DuplexD2::init();

    let count = duplex.get_uploaded_file_count().unwrap();
    println!("Count {}", count);
}
