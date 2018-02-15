extern crate nums_as_bytes;
extern crate radio_api;
extern crate serde_json;
extern crate serial;

mod duplex_d2;
mod comms;
mod state_of_health_record;
mod file;
mod message;
mod command;
mod serial_comm;

pub use duplex_d2::DuplexD2;
pub use serial_comm::SerialConnection;
