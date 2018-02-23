extern crate nsl_duplex_d2;

use nsl_duplex_d2::{get_file, get_file_count};
use nsl_duplex_d2::DuplexD2;
use nsl_duplex_d2::SerialConnection;

fn main() {
    let duplex = DuplexD2::new(Box::new(SerialConnection));

    let count = duplex.send_command(&get_file_count()).unwrap();
    println!("Count {:?}", count);

    let file = duplex.send_command(&get_file()).unwrap();
    println!("File name: {}", file.name);
    println!("File data: {:?}", file.body);
}
