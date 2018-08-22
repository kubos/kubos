extern crate file_protocol;
#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;

use std::env;
use std::path::Path;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
    ]).unwrap();

    info!("Starting file transfer client");

    let mut args = env::args();
    // Skip the first command arg
    args.next();

    // get upload vs download (required)
    let command = match args.next() {
        Some(ref cmd) if cmd == "upload" || cmd == "download" => cmd.clone(),
        _ => {
            error!("Missing first arg: 'upload' or 'download' must be specified");
            return;
        }
    };

    // get source file (required)
    let source_path = match args.next() {
        Some(path) => path,
        None => {
            error!("Missing source file path");
            return;
        }
    };

    // get target file. If not present, just copy the filename from the source path
    let target_path = match args.next() {
        Some(path) => path,
        None => Path::new(&source_path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned(),
    };

    let result = match command.as_ref() {
        "upload" => file_protocol::upload(&source_path, &target_path),
        // "download" => file_protocol::download(&source_path, &target_path),
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
