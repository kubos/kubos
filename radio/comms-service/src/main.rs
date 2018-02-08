extern crate nsl_duplex_d2;

fn main() {
    let count = nsl_duplex_d2::comms::get_uploaded_file_count().unwrap();
    println!("File count {}", count);
}
