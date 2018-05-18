use codecs::*;
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub struct SocketInfo {
    pub sender: Sender<udp::UdpData>,
    pub handle: JoinHandle<()>,
}

pub struct Transport {
    sockets: HashMap<u16, SocketInfo>,
    /// Used by the transport to receive data from the socket threads
    receiver: Receiver<udp::UdpData>,
    /// Used by the socket threads to send data to the transport
    sender: Sender<udp::UdpData>,
}

impl Transport {
    pub fn new() -> Self {
        let (tx, rx) = channel::<udp::UdpData>();
        Self {
            sockets: HashMap::new(),
            receiver: rx,
            sender: tx,
        }
    }

    // Attempts to grab socket for destination port from socket map
    // Creates a new socket for port if one doesn't exist already
    fn init_socket(&mut self, dest_port: u16) -> Result<(), String> {
        let socket = match UdpSocket::bind("127.0.0.1:0") {
            Ok(s) => s,
            Err(e) => return Err(format!("Failed to bind UdpSocket {:?}", e)),
        };

        if let Err(e) = socket.set_write_timeout(Some(Duration::from_millis(100))) {
            return Err(format!("Failed to set udp write timeout {:?}", e));
        }

        if let Err(e) = socket.set_read_timeout(Some(Duration::from_millis(100))) {
            return Err(format!("Failed to set udp read timeout {:?}", e));
        }

        let (socket_tx, socket_rx) = channel::<udp::UdpData>();
        let transport_tx = self.sender.clone();
        let handle = thread::spawn(move || loop {
            loop {
                let dest = SocketAddr::from(([127, 0, 0, 1], 7000));
                let mut buffer = vec![0u8; 4096];
                if let Ok((amount, _source)) = socket.recv_from(&mut buffer) {
                    transport_tx
                        .send(udp::UdpData {
                            source: 7000,
                            dest: 7000,
                            data: buffer[0..amount].to_vec(),
                            checksum: false,
                        })
                        .unwrap();
                }

                if let Ok(data) = socket_rx.recv() {
                    if let Err(e) = socket.send_to(&data.data, &dest) {
                        warn!("Failed socket.send_to {:?}", e);
                    }
                }
            }
        });

        let info = SocketInfo {
            sender: socket_tx,
            handle: handle,
        };

        self.sockets.insert(dest_port, info);

        Ok(())
    }

    fn grab_socket(&mut self, dest_port: u16) -> Result<&SocketInfo, String> {
        if !self.sockets.contains_key(&dest_port) {
            self.init_socket(dest_port)?;
        };

        match self.sockets.get(&dest_port) {
            Some(ref socket) => Ok(socket),
            None => Err(format!("Failed to locate socket {}", dest_port)),
        }
    }

    pub fn read(&mut self, dest_port: u16) -> Result<Option<udp::UdpData>, String> {
        let _ = self.grab_socket(dest_port)?;

        info!("-udp-recv-");

        match self.receiver.try_recv() {
            Ok(data) => Ok(Some(data)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => {
                Err(format!("Socket channel {:?} disconnected", dest_port))
            }
        }
    }

    pub fn write(&mut self, data: udp::UdpData, dest_port: u16) -> Result<(), String> {
        let socket = self.grab_socket(dest_port)?;

        if let Err(e) = socket.sender.send(data) {
            return Err(format!(
                "Failed to send udp channel msg {}:{:?}",
                dest_port, e
            ));
        }

        Ok(())
    }
}
