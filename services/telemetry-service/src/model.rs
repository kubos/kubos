use kubos_telemetry_db::Database;
use udp::*;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

pub struct Subsystem {
    pub database: Arc<Mutex<Database>>,
}

impl Subsystem {
    pub fn new(database: Database) -> Self {
        let db = Arc::new(Mutex::new(database));
        let udp = DirectUdp::new(db.clone());
        spawn(move || udp.start());

        Subsystem { database: db }
    }
}
