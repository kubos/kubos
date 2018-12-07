use errors::*;
use std::sync::{Arc, Mutex};

/// Generic telemetry collected by the communication service.
#[derive(Default, GraphQLObject)]
pub struct CommsTelemetry {
    /// Errors that have occured within the communication service.
    pub errors: Vec<String>,
    /// Number of packets successfully uplinked.
    pub packets_up: i32,
    /// Number of packets downlinked.
    pub packets_down: i32,
}

/// Enum used to differentiate types of telemetry collected by the communication service.
pub enum TelemType {
    /// Packets up
    Up,
    /// Packets down
    Down,
}

// Function used to obtain a mutex lock and update communication service errors.
pub fn log_error(data: &Arc<Mutex<CommsTelemetry>>, error: String) -> CommsResult<()> {
    match data.lock() {
        Ok(mut telem) => {
            telem.errors.push(error);
            Ok(())
        }
        Err(_) => Err(CommsServiceError::MutexPoisoned.into()),
    }
}

// Function used to obtain a mutex lock and update communcation service telemetry.
pub fn log_telemetry(data: &Arc<Mutex<CommsTelemetry>>, telem_type: TelemType) -> CommsResult<()> {
    match data.lock() {
        Ok(mut telem) => {
            match telem_type {
                TelemType::Up => telem.packets_up += 1,
                TelemType::Down => telem.packets_down += 1,
            };
            Ok(())
        }
        Err(_) => Err(CommsServiceError::MutexPoisoned.into()),
    }
}
