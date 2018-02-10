pub trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

impl AsBytes for u32 {
    fn as_bytes(&self) -> Vec<u8> {
        let mut v = Vec::new();
        let mut udata = *self;
        loop {
            v.insert(0, (udata & 0xFF) as u8);
            udata = udata >> 8;
            if udata == 0 {
                break;
            }
        }
        v
    }
}

impl AsBytes for u64 {
    fn as_bytes(&self) -> Vec<u8> {
        let mut v = Vec::new();
        let mut udata = *self;
        loop {
            v.insert(0, (udata & 0xFF) as u8);
            udata = udata >> 8;
            if udata == 0 {
                break;
            }
        }
        v
    }
}
