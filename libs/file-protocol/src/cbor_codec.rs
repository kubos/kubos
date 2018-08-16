use std::net::{SocketAddr, UdpSocket};

pub struct Protocol {
    pub handle: UdpSocket,
}

impl Protocol {
    pub fn new(bind_port: u16) -> Self {
        Self {
            handle: UdpSocket::bind(format!("127.0.0.1:{}", bind_port)).unwrap(),
        }
    }

    pub fn send_message(&self, message: &[u8], host: &str, port: u16) -> Result<(), String> {
        let dest: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
        // let mut e = Encoder::from_memory();
        //e.encode(&message).unwrap();
        //let mut payload = e.as_bytes().to_vec();
        let mut payload = vec![];
        payload.extend(message);
        payload.insert(0, 0);
        self.handle.send_to(&payload, &dest).unwrap();
        Ok(())
    }
}
