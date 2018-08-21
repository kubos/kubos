extern crate file_protocol;
#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;

use file_protocol::{CborProtocol, FileProtocol, Message};

fn recv_loop() -> Result<(), String> {
    // TODO: Make configurable
    let c_protocol = CborProtocol::new(7000);

    loop {
        // Listen on UDP port
        let (peer, message) = match c_protocol.recv_message_peer()? {
            (peer, Some(data)) => (peer, data),
            _ => continue,
        };

        // TODO: thread::spawn()

        // Set up the file system processor with the reply socket information
        // TODO: Opening a second local port might be a terrible plan. Though it is kind of what TCP does
        let f_protocol = FileProtocol::new(format!("{}", peer.ip()).to_owned(), peer.port());

        // Parse it into a known message type and process
        // TODO: How to handle message parsing failures? Do we tell the client?
        // TODO: Handle multiple simultaneous clients
        match f_protocol.on_message(message)? {
            Message::Sync(hash) => {
                debug!("Sync({})", hash);
                //TODO: sync_and_send
            }
            Message::SyncChunks(hash, chunk_num) => {
                debug!("SyncChunks({}, {})", hash, chunk_num);
                //TODO: sync_and_send
            }
            Message::ReceiveChunk(hash) => {
                debug!("ReceiveChunk({})", hash);
                // A chunk was automatically stored. Do nothing
            }
            Message::ACK(hash) => {
                debug!("ACK({})", hash);
                // The client now has all of the needed chunks of a file.
                // Q: Do nothing?
            }
            Message::NAK(hash) => {
                debug!("NAK({})", hash);
                // TODO: Some number of chunks weren't received by the client.
                // Resend them
            }
            Message::ReqReceive(channel_id) => {
                debug!("ReqReceive({})", channel_id);
                // The client wants to send us a file
                // TODO: Let it. sync_and_send
            }
            Message::ReqTransmit(channel_id) => {
                debug!("ReqTransmit({})", channel_id);
                // The client wants us to send a file
                // We already did the underlying chunking and sent the success/failure message
                // Q: Do nothing...currently on_message takes care of everything
            }
            Message::SuccessReceive(channel_id) => {
                debug!("SuccessReceive({})", channel_id);
                // Q: I don't think the server will ever receive this.
                //    I think that the server only ever *sends* this.
            }
            Message::SuccessTransmit(channel_id, hash, chunk_num, mode) => {
                debug!(
                    "SuccessTransmit({}, {}, {}, {:?})",
                    channel_id, hash, chunk_num, mode
                );
                // Q: I don't think the server will ever receive this.
                //    I think that the server only ever *sends* this.
            }
            Message::Failure(channel_id, hash) => {
                debug!("Failure({}, {})", channel_id, hash);
                // Q: I don't think the server will ever receive this.
                //    I think that the server only ever *sends* this.
            }
        }
    }
}

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
    ]).unwrap();

    info!("Starting file transfer service");

    match recv_loop() {
        Ok(()) => warn!("Service listener loop exited successfully?"),
        Err(err) => error!("Service listener exited early: {}", err),
    }
}
