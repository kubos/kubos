use std::net::{SocketAddr, UdpSocket};
use serde_cbor::de;

pub struct Protocol {
    pub handle: UdpSocket,
    recv_handle: fn(&str),
}

impl Protocol {
    pub fn new(bind_port: u16, recv_handle: fn(&str)) -> Self {
        Self {
            handle: UdpSocket::bind(format!("127.0.0.1:{}", bind_port)).unwrap(),
            recv_handle,
        }
    }

    pub fn send_message(&self, message: &[u8], host: &str, port: u16) -> Result<(), String> {
        // TODO: If paused, just queue up the message
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

    pub fn resume(&self) -> Result<(), String> {
        // if !paused {return;}
        // paused = false
        // while !paused && Some(write_queue.next()) {
        //   pull chunks out of the queue and write them
        // }
        unimplemented!();
    }

    // Not actually used by anything in the original Lua?
    pub fn send_pause(&self, host: &str, port: u16) -> Result<(), String> {
        println!("-> pause");
        let dest: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
        let mut payload = vec![1];
        self.handle.send_to(&payload, &dest).unwrap();
        Ok(())
    }

    // Not actually used by anything in the original Lua?
    pub fn send_resume(&self, host: &str, port: u16) -> Result<(), String> {
        println!("-> resume");
        let dest: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
        let mut payload = vec![2];
        self.handle.send_to(&payload, &dest).unwrap();
        Ok(())
    }

    // Called when a message is received over UDP?
    // It's somehow a wrapper around UDP `recv_start`???
    pub fn recv_start(&self, _err: &str, data: &[u8]) -> Result<(), String> {
        // TODO: error processing?
        if data.len() == 0 {
            return Ok(());
        }

        match data[0] {
            0 => {
                let message: String = de::from_slice(&data[1..])
                    .map_err(|err| format!("Failed to parse data: {:?}", err))?;
                println!("<- {}", message);
                // TODO: This somehow corresponds to `main`s on_message...wat.
                (self.recv_handle)(&message);
            }
            1 => {
                println!("<- puase");
                //TODO: paused = true
            }
            2 => {
                println!("<- resume");
                // TODO: This might need to be a channel message/signal
                self.resume();
            }
            x => eprintln!("Ignoring unknown control frame: {}", x),
        }

        Ok(())
    }
}
