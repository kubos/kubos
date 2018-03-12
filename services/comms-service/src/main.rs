extern crate nsl_duplex_d2;

use nsl_duplex_d2::DuplexD2;
use nsl_duplex_d2::serial_connection;
use nsl_duplex_d2::File;

fn main() {
    let radio = DuplexD2::new(serial_connection());

    let count = radio.get_uploaded_file_count().unwrap();
    println!("Count {:?}", count);

    if count != 0 {
        let file = radio.get_uploaded_file().unwrap();
        println!("File name: {}", file.name);
        println!("File data: {:?}", file.body);
    }

    println!("Sending file");
    let send_file = File {
        name: String::from("test.txt"),
        body: b"Hello from kubos\n".to_vec(),
    };
    let res = radio.put_download_file(&send_file).unwrap();
    println!("Result {}", res);
}
