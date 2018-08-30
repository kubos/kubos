extern crate file_protocol;
#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;

use std::env;
use std::path::Path;
use std::time::Duration;
use std::thread;
use file_protocol::{messages, storage, FileProtocol, State};

fn upload(port: u16, source_path: &str, target_path: &str) -> Result<(), String> {
    let f_protocol = FileProtocol::new(String::from("127.0.0.1"), port);

    info!(
        "Uploading local:{} to remote:{}",
        &source_path, &target_path
    );
    // Copy file to upload to temp storage. Calculate the hash and chunk info
    // Q: What's `mode` for? `initialize_file` always returns 0. Looks like it should be file permissions
    let (hash, num_chunks, mode) = storage::initialize_file(&source_path)?;
    // Tell our destination the hash and number of chunks to expect
    f_protocol.send(messages::metadata(&hash, num_chunks).unwrap())?;
    // TODO: Remove this sleep - see below
    // There is currently a race condition where sync and export are both sent
    // quickly and the server processes them concurrently, but the folder
    // structure from sync isn't ready when export starts
    thread::sleep(Duration::from_millis(100));
    // Send export command for file
    f_protocol.send_export(&hash, &target_path, mode)?;
    // Start the engine
    Ok(f_protocol.message_engine(Duration::from_secs(2), State::Transmitting)?)
}

fn download(port: u16, source_path: &str, target_path: &str) -> Result<(), String> {
    let f_protocol = FileProtocol::new(String::from("127.0.0.1"), port);

    info!(
        "Downloading remote: {} to local: {}",
        source_path, target_path
    );

    // Send our file request to the remote addr and get the returned data
    f_protocol.send_import(source_path)?;

    Ok(f_protocol.message_engine(
        Duration::from_secs(2),
        State::StartReceive {
            path: target_path.to_string(),
        },
    )?)
}

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
        "upload" => upload(7000, &source_path, &target_path),
        "download" => download(7000, &source_path, &target_path),
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
