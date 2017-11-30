pub struct Subsystem {
    pub power: bool
}

impl Subsystem {
    pub fn power(&self) -> bool {
        self.power
    }

    pub fn me(&self) -> &Subsystem {
        self
    }
}
