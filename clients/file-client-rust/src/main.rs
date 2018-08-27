extern crate clap;
extern crate file_protocol;
#[macro_use]
extern crate log;
extern crate simplelog;

use clap::{App, Arg};
use simplelog::*;
use std::path::Path;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
    ]).unwrap();

    info!("Starting file transfer client");

    let args = App::new("File transfer client")
        .arg(
            Arg::with_name("operation")
                .index(1)
                .required(true)
                .possible_values(&["upload", "download"])
                .case_insensitive(true),
        )
        .arg(Arg::with_name("source_file").index(2).required(true))
        .arg(Arg::with_name("target_file").index(3))
        .arg(
            Arg::with_name("local_ip")
                .short("i")
                .takes_value(true)
                .default_value("0.0.0.0"),
        )
        .arg(
            Arg::with_name("remote_addr")
                .short("-a")
                .takes_value(true)
                .default_value("0.0.0.0:7000"),
        )
        .get_matches();

    // get upload vs download (required)
    let command = args.value_of("operation").unwrap();

    // get source file (required)
    let source_path = args.value_of("source_file").unwrap();

    // get target file. If not present, just copy the filename from the source path
    let target_path: String = match args.value_of("target_file") {
        Some(path) => path.to_owned(),
        None => Path::new(&source_path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned(),
    };

    let local_ip = args.value_of("local_ip").unwrap();
    let remote_addr = args.value_of("remote_addr").unwrap();

    let result = match command.as_ref() {
        "upload" => file_protocol::upload(local_ip, remote_addr, &source_path, &target_path, None),
        "download" => {
            file_protocol::download(local_ip, remote_addr, &source_path, &target_path, None)
        }
        // This shouldn't be possible, since we checked the string earlier
        _ => {
            error!("Unknown command given");
            return;
        }
    };

    if let Err(err) = result {
        error!("Operation failed: {}", err);
    } else {
        info!("Operation successful");
    }
}
