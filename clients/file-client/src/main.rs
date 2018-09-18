extern crate clap;
extern crate file_protocol;
#[macro_use]
extern crate log;
extern crate simplelog;

use clap::{App, Arg};
use file_protocol::{FileProtocol, State};
use simplelog::*;
use std::path::Path;
use std::time::Duration;

fn upload(
    host_ip: &str,
    remote_addr: &str,
    source_path: &str,
    target_path: &str,
    prefix: Option<String>,
) -> Result<(), String> {
    let f_protocol = FileProtocol::new(host_ip, remote_addr, prefix);

    info!(
        "Uploading local:{} to remote:{}",
        &source_path, &target_path
    );

    // Copy file to upload to temp storage. Calculate the hash and chunk info
    let (hash, num_chunks, mode) = f_protocol.initialize_file(&source_path)?;

    // Generate channel id for transaction
    let channel = f_protocol.generate_channel()?;

    // Tell our destination the hash and number of chunks to expect
    f_protocol.send_metadata(channel, &hash, num_chunks)?;

    // Send export command for file
    f_protocol.send_export(channel, &hash, &target_path, mode)?;

    // Start the engine to send the file data chunks
    Ok(f_protocol.message_engine(
        |d| f_protocol.recv(Some(d)),
        Duration::from_secs(2),
        State::Transmitting,
    )?)
}

fn download(
    host_ip: &str,
    remote_addr: &str,
    source_path: &str,
    target_path: &str,
    prefix: Option<String>,
) -> Result<(), String> {
    let f_protocol = FileProtocol::new(host_ip, remote_addr, prefix);

    info!(
        "Downloading remote: {} to local: {}",
        source_path, target_path
    );

    // Generate channel id for transaction
    let channel = f_protocol.generate_channel()?;

    // Send our file request to the remote addr and verify that it's
    // going to be able to send it
    f_protocol.send_import(channel, source_path)?;

    // Wait for the request reply.
    // Note/TODO: We don't use a timeout here because we don't know how long it will
    // take the server to prepare the file we've requested.
    // Larger files (> 100MB) can take over a minute to process.
    let reply = match f_protocol.recv(None) {
        Ok(Some(message)) => message,
        Ok(None) => return Err("Failed to import file".to_owned()),
        Err(error) => return Err(format!("Failed to import file: {}", error)),
    };

    let state = f_protocol.process_message(
        reply,
        State::StartReceive {
            path: target_path.to_string(),
        },
    )?;

    Ok(f_protocol.message_engine(|d| f_protocol.recv(Some(d)), Duration::from_secs(2), state)?)
}

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
            Arg::with_name("host_ip")
                .short("h")
                .takes_value(true)
                .default_value("0.0.0.0"),
        )
        .arg(
            Arg::with_name("remote_ip")
                .short("-r")
                .takes_value(true)
                .default_value("0.0.0.0"),
        )
        .arg(
            Arg::with_name("remote_port")
                .short("-p")
                .takes_value(true)
                .default_value("7000"),
        )
        .get_matches();

    // Get upload vs download (required)
    let command = args.value_of("operation").unwrap();

    // Get source file (required)
    let source_path = args.value_of("source_file").unwrap();

    // Get target file. If not present, just copy the filename from the source path
    let target_path: String = match args.value_of("target_file") {
        Some(path) => path.to_owned(),
        None => Path::new(&source_path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned(),
    };

    let host_ip = args.value_of("host_ip").unwrap();
    let remote_addr = format!(
        "{}:{}",
        args.value_of("remote_ip").unwrap(),
        args.value_of("remote_port").unwrap()
    );

    let result = match command.as_ref() {
        "upload" => upload(host_ip, &remote_addr, &source_path, &target_path, None),
        "download" => download(host_ip, &remote_addr, &source_path, &target_path, None),
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
