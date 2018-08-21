extern crate file_protocol;
extern crate kubos_system;
#[macro_use]
extern crate log;
extern crate simplelog;

use kubos_system::Config as ServiceConfig;
use simplelog::*;
use std::thread;

use file_protocol::{CborProtocol, FileProtocol, Message};

fn recv_loop(config: ServiceConfig) -> Result<(), String> {
    // TODO: Make configurable
    let c_protocol = CborProtocol::new(config.hosturl());

    loop {
        // Listen on UDP port
        let (peer, message) = match c_protocol.recv_message_peer()? {
            (peer, Some(data)) => (peer, data),
            _ => continue,
        };

        // Break the processing work off into its own thread so we can
        // listen for requests from other clients
        thread::spawn(move || {
            // Set up the file system processor with the reply socket information
            // TODO: Opening a second local port might be a terrible plan. Though it is kind of what TCP does
            let f_protocol = FileProtocol::new(format!("{}", peer.ip()).to_owned(), peer.port());

            // Parse it into a known message type and process
            // TODO: Convert the various failures/unwraps to nice error printing
            if let Ok(message) = f_protocol.on_message(message) {
                match message {
                    Message::SyncChunks(hash, num_chunks) => {
                        // A client has notified us of a file we should be prepared to receive
                        f_protocol.store_meta(&hash, num_chunks).unwrap();
                    }
                    Message::NAK(hash, missing_chunks) => {
                        // Some number of chunks weren't received by the client.
                        // Resend them
                        if let Some(chunks) = missing_chunks {
                            f_protocol.do_upload(&hash, &chunks).unwrap();
                        }
                    }
                    Message::ReqReceive(channel_id, hash, path, mode) => {
                        // The client wants to send us a file.
                        // Go listen for the chunks.
                        // Note: Won't return until we've received all of them.
                        // (so could potentially never return)
                        f_protocol.sync_and_send(&hash, None).unwrap();

                        match f_protocol.local_export(&hash, &path, mode) {
                            Ok(()) => {
                                f_protocol.send_success(channel_id).unwrap();
                            }
                            Err(error) => {
                                f_protocol.send_failure(channel_id, &error).unwrap();
                            }
                        }
                    }
                    _ => {
                        // Whatever action was needed for this particular message
                        // was already taken care of by `on_message()`
                    }
                }
            } else {
                // Q: Do we want to do any kind of error handling,
                //    or just move on to the next message?
            }
        });
    }
}

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
    ]).unwrap();

    let config = ServiceConfig::new("file-transfer-service");

    info!("Starting file transfer service");

    match recv_loop(config) {
        Ok(()) => warn!("Service listener loop exited successfully?"),
        Err(err) => error!("Service listener exited early: {}", err),
    }
}
