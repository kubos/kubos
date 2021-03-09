//
// Copyright (C) 2019 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use clap::{App, AppSettings, Arg, SubCommand};
use failure::bail;
use file_protocol::{FileProtocol, FileProtocolConfig, State};
use log::{error, info};
use simplelog::*;
use std::path::Path;
use std::time::Duration;

#[allow(clippy::too_many_arguments)]
fn upload(
    protocol_instance: FileProtocol,
    source_path: &str,
    target_path: &str,
) -> Result<(), failure::Error> {
    info!(
        "Uploading local:{} to remote:{}",
        &source_path, &target_path
    );

    // Copy file to upload to temp storage. Calculate the hash and chunk info
    let (hash, num_chunks, mode) = protocol_instance.initialize_file(&source_path)?;

    // Generate channel id for transaction
    let channel = protocol_instance.generate_channel()?;

    // Tell our destination the hash and number of chunks to expect
    protocol_instance.send_metadata(channel, &hash, num_chunks)?;

    // Send export command for file
    protocol_instance.send_export(channel, &hash, &target_path, mode)?;

    // Start the engine to send the file data chunks
    protocol_instance.message_engine(
        |d| protocol_instance.recv(Some(d)),
        Duration::from_secs(2),
        &State::Transmitting,
    )?;
    Ok(())
}

fn download(
    protocol_instance: FileProtocol,
    source_path: &str,
    target_path: &str,
) -> Result<(), failure::Error> {
    info!(
        "Downloading remote: {} to local: {}",
        source_path, target_path
    );

    // Generate channel id for transaction
    let channel = protocol_instance.generate_channel()?;

    // Send our file request to the remote addr and verify that it's
    // going to be able to send it
    protocol_instance.send_import(channel, source_path)?;

    // Wait for the request reply.
    // Note/TODO: We don't use a timeout here because we don't know how long it will
    // take the server to prepare the file we've requested.
    // Larger files (> 100MB) can take over a minute to process.
    let reply = match protocol_instance.recv(None) {
        Ok(message) => message,
        Err(error) => bail!("Failed to import file: {}", error),
    };

    let state = protocol_instance.process_message(
        reply,
        &State::StartReceive {
            path: target_path.to_string(),
        },
    )?;

    protocol_instance.message_engine(
        |d| protocol_instance.recv(Some(d)),
        Duration::from_secs(2),
        &state,
    )?;
    Ok(())
}

fn cleanup(protocol_instance: FileProtocol, hash: Option<String>) -> Result<(), failure::Error> {
    match &hash {
        Some(s) => info!("Requesting remote cleanup of temp storage for hash {}", s),
        None => info!("Requesting remote cleanup of all temp storage"),
    }

    // Generate channel ID for transaction
    let channel = protocol_instance.generate_channel()?;

    // Send our cleanup request to the remote addr and verify that it's
    // going to be able to send it
    protocol_instance.send_cleanup(channel, hash)?;

    Ok(())
}

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default()).unwrap()
    ])
    .unwrap();

    info!("Starting file transfer client");

    let args = App::new("File transfer client")
        .subcommand(
            SubCommand::with_name("upload")
                .about("Initiates upload of local file")
                .arg(
                    Arg::with_name("source_path")
                        .help("Local file path to upload")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("target_path")
                        .help("Destination path on remote target")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("download")
                .about("Requests download of remote file")
                .arg(
                    Arg::with_name("source_path")
                        .help("Remote file path to download")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("target_path")
                        .help("Local destination path")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("cleanup")
                .about("Requests cleanup of remote temporary storage")
                .arg(
                    Arg::with_name("hash")
                        .help("Specific file storage to clean up")
                        .takes_value(true),
                ),
        )
        .arg(
            Arg::with_name("host_ip")
                .help("IP address of the local host to use")
                .long("host-ip")
                .short("-h")
                .takes_value(true)
                .default_value("0.0.0.0"),
        )
        .arg(
            Arg::with_name("host_port")
                .help("UDP port that the file transfer service will send responses to")
                .long("host-port")
                .short("-P")
                .takes_value(true)
                .default_value("8080"),
        )
        .arg(
            Arg::with_name("remote_ip")
                .help("IP address of the file transfer service to connect to")
                .long("remote-ip")
                .short("-r")
                .takes_value(true)
                .default_value("0.0.0.0"),
        )
        .arg(
            Arg::with_name("remote_port")
                .help("UDP port of the file transfer service to connect to")
                .long("remote-port")
                .short("-p")
                .takes_value(true)
                .default_value("8040"),
        )
        .arg(
            Arg::with_name("storage_prefix")
                .help("Folder name used for transfer storage")
                .long("storage-prefix")
                .short("-s")
                .takes_value(true)
                .default_value("file-storage"),
        )
        .arg(
            Arg::with_name("transfer_chunk_size")
                .help("Chunk size used for transfer chunking")
                .long("transfer-chunk-size")
                .short("-c")
                .takes_value(true)
                .default_value("1024"),
        )
        .arg(
            Arg::with_name("hash_chunk_size")
                .help("Chunk size used when hashing for file storage")
                .long("hash-chunk-size")
                .takes_value(true)
                .default_value("2048"),
        )
        .arg(
            Arg::with_name("hold_count")
                .help("Internal hold counter controlling retry length")
                .long("hold-count")
                .short("-t")
                .takes_value(true)
                .default_value("6"),
        )
        .arg(
            Arg::with_name("inter_chunk_delay")
                .help("Delay (in milliseconds) between each chunk transmission")
                .long("inter-chunk-delay")
                .short("-d")
                .takes_value(true)
                .default_value("1"),
        )
        .arg(
            Arg::with_name("max_chunks_transmit")
                .help("Maximum number of chunks to transmit in one go")
                .long("max-chunks-transmit")
                .short("-m")
                .takes_value(true),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .get_matches();

    let host_ip = args.value_of("host_ip").unwrap();
    let host_port: u16 = args.value_of("host_port").unwrap().parse().unwrap();
    let remote_addr = format!(
        "{}:{}",
        args.value_of("remote_ip").unwrap(),
        args.value_of("remote_port").unwrap()
    );
    let transfer_chunk_size: usize = args
        .value_of("transfer_chunk_size")
        .unwrap()
        .parse()
        .unwrap();
    let hash_chunk_size: usize = args.value_of("hash_chunk_size").unwrap().parse().unwrap();
    let hold_count: u16 = args.value_of("hold_count").unwrap().parse().unwrap();
    let storage_prefix = args.value_of("storage_prefix").unwrap().to_string();
    let inter_chunk_delay: u64 = args.value_of("inter_chunk_delay").unwrap().parse().unwrap();
    let max_chunks_transmit: Option<u32> = if args.is_present("max_chunks_transmit") {
        Some(
            args.value_of("max_chunks_transmit")
                .unwrap()
                .parse()
                .unwrap(),
        )
    } else {
        None
    };

    let protocol_config = FileProtocolConfig::new(
        Some(storage_prefix),
        transfer_chunk_size,
        hold_count,
        inter_chunk_delay,
        max_chunks_transmit,
        hash_chunk_size,
    );
    let protocol_instance = FileProtocol::new(
        &format!("{}:{}", host_ip, host_port),
        &remote_addr,
        protocol_config,
    );

    let result = match args.subcommand_name() {
        Some("upload") => {
            let upload_args = args.subcommand_matches("upload").unwrap();
            let source_path = upload_args.value_of("source_path").unwrap();
            let target_path = match upload_args.value_of("target_path") {
                Some(path) => path.to_owned(),
                None => Path::new(&source_path)
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
            };

            upload(protocol_instance, &source_path, &target_path)
        }
        Some("download") => {
            let download_args = args.subcommand_matches("download").unwrap();
            let source_path = download_args.value_of("source_path").unwrap();
            let target_path = match download_args.value_of("target_path") {
                Some(path) => path.to_owned(),
                None => Path::new(&source_path)
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
            };

            download(protocol_instance, &source_path, &target_path)
        }
        Some("cleanup") => {
            let hash = args
                .subcommand_matches("cleanup")
                .unwrap()
                .value_of("hash")
                .to_owned()
                .map(|v| v.to_owned());
            cleanup(protocol_instance, hash)
        }
        _ => panic!("Invalid command"),
    };

    if let Err(err) = result {
        error!("Operation failed: {}", err);
        std::process::exit(1);
    } else {
        info!("Operation successful");
    }
}
