use codecs;
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use transports::Transport;

pub struct SocketInfo {
    pub sender: Sender<codecs::udp::UdpData>,
    pub handle: JoinHandle<()>,
}

pub struct Udp {
    sockets: HashMap<u16, SocketInfo>,
    /// Used by the transport to receive data from the socket threads
    receiver: Receiver<codecs::udp::UdpData>,
    /// Used by the socket threads to send data to the transport
    sender: Sender<codecs::udp::UdpData>,
}

impl Udp {
    pub fn new() -> Self {
        info!("udp transport starting");
        let (tx, rx) = channel::<codecs::udp::UdpData>();
        Self {
            sockets: HashMap::new(),
            receiver: rx,
            sender: tx,
        }
    }

    pub fn expose_ports(&mut self, ports: &[u16]) -> Result<(), String> {
        for p in ports {
            info!("udp exposing port {}", p);
            self.listen_socket(*p)?;
        }
        Ok(())
    }

    // Attempts to grab socket for destination port from socket map
    // Creates a new socket for port if one doesn't exist already
    fn listen_socket(&mut self, listen_port: u16) -> Result<(), String> {
        let (socket_tx, socket_rx) = channel::<codecs::udp::UdpData>();
        let transport_tx = self.sender.clone();
        let handle = thread::spawn(move || thread_loop(listen_port, 0, transport_tx, socket_rx));

        let info = SocketInfo {
            sender: socket_tx,
            handle: handle,
        };

        self.sockets.insert(listen_port, info);

        Ok(())
    }

    // Attempts to grab socket for destination port from socket map
    // Creates a new socket for port if one doesn't exist already
    fn init_socket(&mut self, dest_port: u16) -> Result<(), String> {
        let (socket_tx, socket_rx) = channel::<codecs::udp::UdpData>();
        let transport_tx = self.sender.clone();
        let handle = thread::spawn(move || thread_loop(0, dest_port, transport_tx, socket_rx));

        // let handle = thread::spawn(move || loop {
        //     let tx = transport_tx.clone();
        //     let dest = SocketAddr::from(([127, 0, 0, 1], dest_port));
        //     let mut buffer = vec![0u8; 4096];
        //     if let Ok((amount, _source)) = socket.recv_from(&mut buffer) {
        //         info!("udp received {} bytes from {}", amount, dest_port);
        //         let dat = codecs::udp::UdpData {
        //             source: socket_port,
        //             dest: dest_port,
        //             data: buffer[0..amount].to_vec(),
        //             checksum: false,
        //         };
        //         info!("udp sending {:?} from {}", dat, dest_port);
        //         tx.send(dat).unwrap();
        //     }

        //     if let Ok(data) = socket_rx.recv() {
        //         if let Err(e) = socket.send_to(&data.data, &dest) {
        //             warn!("udp failed socket.send_to {:?}", e);
        //         } else {
        //             info!("udp sent {} bytes to {}", data.data.len(), dest_port);
        //         }
        //     }
        // });

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
}

impl Transport for Udp {
    fn read(&self) -> Result<Option<codecs::udp::UdpData>, String> {
        match self.receiver.try_recv() {
            Ok(data) => {
                info!("got msg from channe {:?}", data);
                Ok(Some(data))
            }
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(String::from("udp receiver disconnected")),
        }
    }

    fn write(&mut self, data: codecs::udp::UdpData) -> Result<(), String> {
        let dest_port = data.dest;
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

fn thread_loop(
    source_port: u16,
    dest_port: u16,
    sender: Sender<codecs::udp::UdpData>,
    receiver: Receiver<codecs::udp::UdpData>,
) {
    let socket = match UdpSocket::bind(format!("127.0.0.1:{}", source_port)) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to bind UdpSocket {:?}", e);
            return;
        }
    };

    if let Err(e) = socket.set_write_timeout(Some(Duration::from_millis(100))) {
        error!("Failed to set udp write timeout {:?}", e);
        return;
    }

    if let Err(e) = socket.set_read_timeout(Some(Duration::from_millis(100))) {
        error!("Failed to set udp read timeout {:?}", e);
        return;
    }

    let mut dest_port = dest_port;
    let mut source_port = 0;

    loop {
        let mut buffer = vec![0u8; 4096];
        // info!("attempt to recv@{}", listen_port);
        if let Ok((amount, source)) = socket.recv_from(&mut buffer) {
            // info!("got packet {} --> {}", listen_port, source.port());
            dest_port = source.port();
            // info!("udp received {} bytes from {}", amount, source);
            let dat = codecs::udp::UdpData {
                //source: dest_port,
                //dest: source_port,
                source: dest_port,
                dest: source_port,
                data: buffer[0..amount].to_vec(),
                checksum: true,
            };
            // info!("udp sending {:?} from {}", dat, listen_port);
            sender.send(dat).unwrap();
        }

        // info!("attempt to read msg off channel");
        if let Ok(data) = receiver.try_recv() {
            source_port = data.source;
            if dest_port != 0 {
                let dest = SocketAddr::from(([127, 0, 0, 1], dest_port));

                if let Err(e) = socket.send_to(&data.data, &dest) {
                    warn!("udp failed socket.send_to {:?}", e);
                } else {
                    // info!("udp sent {} bytes to {}", data.data.len(), dest_port);
                }
            }
        }
        thread::sleep(Duration::from_millis(1000));
    }
}
