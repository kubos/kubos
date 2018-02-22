extern crate nums_as_bytes;
extern crate radio_api;
extern crate serde_json;
extern crate serial;

#[macro_use]
extern crate nom;

mod duplex_d2;
mod commands;
mod messages;
mod serial_comm;

pub use duplex_d2::DuplexD2;
pub use commands::*;
pub use serial_comm::SerialConnection;
