extern crate nsl_duplex_d2;

use nsl_duplex_d2::DuplexD2;
use nsl_duplex_d2::serial_connection;

fn main() {
    let radio = DuplexD2::new(serial_connection());

    let count = radio.get_file_count().unwrap();
    println!("Count {:?}", count);

    let file = radio.get_file().unwrap();
    println!("File name: {}", file.name);
    println!("File data: {:?}", file.body);
}
