extern crate cbor_protocol;
extern crate file_protocol;
extern crate kubos_system;
#[macro_use]
extern crate log;
extern crate simplelog;

use kubos_system::Config as ServiceConfig;
use simplelog::*;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

use cbor_protocol::Protocol as CborProtocol;
use file_protocol::{FileProtocol, Message, Role};

// We need this in this lib.rs file so we can build integration tests

pub fn recv_loop(config: ServiceConfig) -> Result<(), String> {
    let host = config.hosturl();
    let c_protocol = CborProtocol::new(host.clone());

    let mut host_parts = host.split(':').map(|val| val.to_owned());
    let host_ip = host_parts.next().unwrap();

    let prefix = match config.get("storage_dir") {
        Some(val) => val.as_str().and_then(|str| Some(str.to_owned())),
        None => None,
    };

    loop {
        // Listen on UDP port
        let (peer, message) = match c_protocol.recv_message_peer().map_err(|err| match err {
            Some(err) => err,
            None => "Failed to receive message".to_owned(),
        })? {
            (peer, Some(data)) => (peer, data),
            _ => continue,
        };

        let prefix_ref = prefix.clone();
        let host_ref = host_ip.clone();

        // Break the processing work off into its own thread so we can
        // listen for requests from other clients
        thread::spawn(move || {
            // Set up the file system processor with the reply socket information
            // TODO: Opening a second local port might be a terrible plan. Though it is kind of what TCP does
            let f_protocol =
                FileProtocol::new(&host_ref, &format!("{}", peer), Role::Server, prefix_ref);

            // Parse it into a known message type and process
            // TODO: Convert the various failures/unwraps to nice error printing
            if let Ok(message) = f_protocol.process_message(message, None, None) {
                // TODO: Probably nothing
            } else {
                // Q: Do we want to do any kind of error handling,
                //    or just move on to the next message?
            }
        });
    }

    return Ok(());

    // loop {
    //     // Listen on UDP port

    //     let peer =  c_protocol.peek_peer()?;

    //     info!("spawn thread for {:?}", peer);

    //     // Break the processing work off into its own thread so we can
    //     // listen for requests from other clients
    //     thread::spawn(move || {
    //         // Set up the file system processor with the reply socket information
    //         // TODO: Opening a second local port might be a terrible plan. Though it is kind of what TCP does
    //         let f_protocol = FileProtocol::new(format!("{}", peer.ip()).to_owned(), peer.port(), Role::Server);

    //         f_protocol.message_engine(None, true);

    //         // // Parse it into a known message type and process
    //         // // TODO: Convert the various failures/unwraps to nice error printing
    //         // if let Ok(message) = f_protocol.process_message(message) {
    //         //     match message {
    //         //         Message::ReqReceive(channel_id, hash, path, mode) => {
    //         //             // The client wants to send us a file.
    //         //             // Go listen for the chunks.
    //         //             // Note: Won't return until we've received all of them.
    //         //             // (so could potentially never return)
    //         //             f_protocol.sync_and_send(&hash, None).unwrap();

    //         //             match f_protocol.finalize_file(&hash, &path, mode) {
    //         //                 Ok(()) => {
    //         //                     f_protocol.send_success(channel_id).unwrap();
    //         //                 }
    //         //                 Err(error) => {
    //         //                     f_protocol.send_failure(channel_id, &error).unwrap();
    //         //                 }
    //         //             }
    //         //         }
    //         //         _ => {
    //         //             // Whatever action was needed for this particular message
    //         //             // was already taken care of by `process_message()`
    //         //         }
    //         //     }
    //         // } else {
    //         //     // Q: Do we want to do any kind of error handling,
    //         //     //    or just move on to the next message?
    //         // }
    //     });
    // }
}

// pub fn recv_loop(config: ServiceConfig) -> Result<(), String> {
//     let c_protocol = CborProtocol::new(config.hosturl());

//     loop {
//         // Listen on UDP port
//         let (peer, message) = match c_protocol.recv_message_peer()? {
//             (peer, Some(data)) => (peer, data),
//             _ => continue,
//         };

//         // Break the processing work off into its own thread so we can
//         // listen for requests from other clients
//         thread::spawn(move || {
//             // Set up the file system processor with the reply socket information
//             // TODO: Opening a second local port might be a terrible plan. Though it is kind of what TCP does
//             let f_protocol = FileProtocol::new(format!("{}", peer.ip()).to_owned(), peer.port());

//             // Parse it into a known message type and process
//             // TODO: Convert the various failures/unwraps to nice error printing
//             if let Ok(message) = f_protocol.process_message(message) {
//                 match message {
//                     Message::Metadata(hash, num_chunks) => {
//                         // A client has notified us of a file we should be prepared to receive
//                         f_protocol.store_meta(&hash, num_chunks).unwrap();
//                     }
//                     Message::NAK(hash, missing_chunks) => {
//                         // Some number of chunks weren't received by the client.
//                         // Resend them
//                         if let Some(chunks) = missing_chunks {
//                             f_protocol.send_chunks(&hash, &chunks).unwrap();
//                         }
//                     }
//                     Message::ReqReceive(channel_id, hash, path, mode) => {
//                         // The client wants to send us a file.
//                         // Go listen for the chunks.
//                         // Note: Won't return until we've received all of them.
//                         // (so could potentially never return)
//                         f_protocol.sync_and_send(&hash, None).unwrap();

//                         match f_protocol.finalize_file(&hash, &path, mode) {
//                             Ok(()) => {
//                                 f_protocol.send_success(channel_id).unwrap();
//                             }
//                             Err(error) => {
//                                 f_protocol.send_failure(channel_id, &error).unwrap();
//                             }
//                         }
//                     }
//                     _ => {
//                         // Whatever action was needed for this particular message
//                         // was already taken care of by `process_message()`
//                     }
//                 }
//             } else {
//                 // Q: Do we want to do any kind of error handling,
//                 //    or just move on to the next message?
//             }
//         });
//     }
// }
