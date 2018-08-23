extern crate file_protocol;
extern crate kubos_system;
#[macro_use]
extern crate log;
extern crate cbor_protocol;
extern crate simplelog;

use kubos_system::Config as ServiceConfig;
use simplelog::*;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

// use cbor_protocol::Protocol as CborProtocol;
use file_protocol::{FileProtocol, Message, Role};

// We need this in this lib.rs file so we can build integration tests

pub fn recv_loop(config: ServiceConfig) -> Result<(), String> {
    // TODO: Make configurable
    //let c_protocol = CborProtocol::new(config.hosturl());
    loop {
        let socket = match UdpSocket::bind("127.0.0.1:7000") {
            Ok(s) => s,
            Err(e) => panic!("Couldn't bind to socket {}", e),
        };

        let mut buf = [0; 4096];
        let (num_bytes, source) = match socket.peek_from(&mut buf) {
            Ok((num_bytes, source)) => {
                println!("peeked {} {:?}", num_bytes, source);
                (num_bytes, source)
            }
            Err(e) => panic!("No data received {}", e),
        };

        //let peer = c_protocol.peek_peer()?;

        let f_protocol = FileProtocol::new_from_socket(
            socket.try_clone().expect("no clone"),
            format!("{}", source.ip()).to_owned(),
            source.port(),
            Role::Server,
        );

        // let f_protocol = FileProtocol::new(format!("{}", source.ip()).to_owned(), source.port(), Role::Server);

        match f_protocol.message_engine(None, Duration::from_secs(1), false) {
            Ok(Some(Message::SyncChunks(_, _))) => {
                match f_protocol.message_engine(None, Duration::from_secs(1), false) {
                    Ok(Some(Message::ReqReceive(channel, hash, path, mode))) => {
                        f_protocol.message_engine(None, Duration::from_secs(5), true);
                        match f_protocol.local_export(&hash, &path, mode) {
                            Ok(_) => f_protocol.send_success(channel),
                            Err(e) => {
                                f_protocol.send_failure(channel, &e);
                                Ok(())
                            }
                        }
                    }
                    _ => {
                        f_protocol.message_engine(None, Duration::from_secs(5), true);
                        Ok(())
                    }
                }
            }
            _ => {
                f_protocol.message_engine(None, Duration::from_secs(5), true);
                Ok(())
            }
        };
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
    //         // if let Ok(message) = f_protocol.on_message(message) {
    //         //     match message {
    //         //         Message::ReqReceive(channel_id, hash, path, mode) => {
    //         //             // The client wants to send us a file.
    //         //             // Go listen for the chunks.
    //         //             // Note: Won't return until we've received all of them.
    //         //             // (so could potentially never return)
    //         //             f_protocol.sync_and_send(&hash, None).unwrap();

    //         //             match f_protocol.local_export(&hash, &path, mode) {
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
    //         //             // was already taken care of by `on_message()`
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
//             if let Ok(message) = f_protocol.on_message(message) {
//                 match message {
//                     Message::SyncChunks(hash, num_chunks) => {
//                         // A client has notified us of a file we should be prepared to receive
//                         f_protocol.store_meta(&hash, num_chunks).unwrap();
//                     }
//                     Message::NAK(hash, missing_chunks) => {
//                         // Some number of chunks weren't received by the client.
//                         // Resend them
//                         if let Some(chunks) = missing_chunks {
//                             f_protocol.do_upload(&hash, &chunks).unwrap();
//                         }
//                     }
//                     Message::ReqReceive(channel_id, hash, path, mode) => {
//                         // The client wants to send us a file.
//                         // Go listen for the chunks.
//                         // Note: Won't return until we've received all of them.
//                         // (so could potentially never return)
//                         f_protocol.sync_and_send(&hash, None).unwrap();

//                         match f_protocol.local_export(&hash, &path, mode) {
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
//                         // was already taken care of by `on_message()`
//                     }
//                 }
//             } else {
//                 // Q: Do we want to do any kind of error handling,
//                 //    or just move on to the next message?
//             }
//         });
//     }
// }
