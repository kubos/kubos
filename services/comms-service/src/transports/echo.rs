use codecs;
use std::cell::RefCell;
use transports::Transport;

pub struct Echo {
    buffer: RefCell<Vec<codecs::udp::UdpData>>,
}

impl Echo {
    pub fn new() -> Self {
        info!("echo transport starting");
        Self {
            buffer: RefCell::new(vec![]),
        }
    }
}

impl Transport for Echo {
    fn read(&self) -> Result<Option<codecs::udp::UdpData>, String> {
        match self.buffer.borrow_mut().pop() {
            None => Ok(None),
            Some(d) => Ok(Some(codecs::udp::UdpData {
                source: d.dest,
                dest: d.source,
                data: d.data,
                checksum: d.checksum,
            })),
        }
    }

    fn write(&mut self, data: codecs::udp::UdpData) -> Result<(), String> {
        self.buffer.borrow_mut().push(data);
        Ok(())
    }
}
