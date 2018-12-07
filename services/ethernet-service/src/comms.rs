use comms_service::{CommsConfig, CommsResult};
use std::net::UdpSocket;
use std::sync::Arc;

use super::CONFIG_PATH;

// Function to allow reading from a UDP socket.
pub fn read(socket: Arc<UdpSocket>) -> CommsResult<Vec<u8>> {
    let mut buf = [0; 4096];
    let (size, _) = socket.recv_from(&mut buf)?;
    Ok(buf[0..size].to_vec())
}

// Function to allow writing over a UDP socket.
pub fn write(socket: Arc<UdpSocket>, data: &[u8]) -> CommsResult<()> {
    let config = CommsConfig::new("ethernet-service", CONFIG_PATH.to_string());
    socket.send_to(
        data,
        (&*config.ground_ip, config.ground_port.unwrap_or_default()),
    )?;
    Ok(())
}
