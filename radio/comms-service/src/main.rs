extern crate nsl_duplex_d2;

use nsl_duplex_d2::DuplexD2;
use nsl_duplex_d2::SerialConnection;

fn main() {
    let duplex = DuplexD2::new(Box::new(SerialConnection));

    let count = duplex.get_uploaded_file_count().unwrap();
    println!("Count {}", count);
}
