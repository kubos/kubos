extern crate nums_as_bytes;
extern crate radio_api;
extern crate serde_json;
extern crate serial;

#[macro_use]
extern crate nom;

mod duplex_d2;
// mod comms;
pub mod commands;
mod messages;
// mod file;
// mod command;
// mod serial_comm;

pub use duplex_d2::DuplexD2;
// pub use serial_comm::SerialConnection;
// pub use command::Command;
