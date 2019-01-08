//
// Copyright (C) 2019 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Communications service interface functions
//
// This file contains all the functions needed in order for the communications service framework to
// be able to communicate with the "radio"

use serial;
use serial::prelude::*;
use std::cell::RefCell;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use super::*;

// Maximum number of bytes to attempt to read at one time
const MAX_READ: usize = 48;
const TIMEOUT: Duration = Duration::from_millis(100);


// Initialize the serial bus connection for reading and writing from/to the "radio"
pub fn serial_init(bus: &str) -> ServiceResult<Arc<Mutex<RefCell<serial::SystemPort>>>> {
    let settings = serial::PortSettings {
        baud_rate: serial::Baud115200,
        char_size: serial::Bits8,
        parity: serial::ParityNone,
        stop_bits: serial::Stop1,
        flow_control: serial::FlowNone,
    };

    let mut port = serial::open(bus)?;

    port.configure(&settings)?;
    port.set_timeout(TIMEOUT)?;
    
    // Wrap the port in a mutex so that multiple threads can access it
    let conn = Arc::new(Mutex::new(RefCell::new(port)));
    
    Ok(conn)
}

// The read function that the comms service read thread will call to wait for messages from the
// "radio"
//
// Returns once a message has been received
pub fn read(conn: Arc<Mutex<RefCell<serial::SystemPort>>>) -> ServiceResult<Vec<u8>> {
    loop {
        // Note: These brackets force the program to release the serial port's mutex so that any
        // threads waiting on it in order to perform a write may do so
        {
            // Take ownership of the serial port
            let conn = match conn.lock() {
                Ok(val) => val,
                Err(e) => {
                    error!("Failed to take mutex: {:?}", e);
                    panic!();
                }
            };
            let mut conn = conn.try_borrow_mut()?;

            // Loop until either a full message has been received or a non-timeout error has occured
            //
            // Note: This program was written for the Beaglebone Black. The BBB UART driver
            // (8250_omap.c) has a peculiar behavior where it will only read, at most, 48 bytes at
            // a time before triggering an interrupt and returning the bytes to the `read` caller.
            // As a result, we'll continue to make `read` calls until either a) we read less than
            // 48 bytes in one go, or b) the read call returns a timeout
            let mut packet = vec![];
            loop {
                let mut buffer: Vec<u8> = vec![0; MAX_READ];
                match conn.read(buffer.as_mut_slice()) {
                    Ok(num) => {
                        buffer.resize(num, 0);
                        packet.append(&mut buffer);

                        debug!("Read {} bytes from radio", packet.len());

                        if num < MAX_READ {
                            return Ok(packet);
                        }
                    }
                    Err(ref err) => match err.kind() {
                        ::std::io::ErrorKind::TimedOut => {
                            if packet.len() > 0 {
                                return Ok(packet);
                            } else {
                                break;
                            }
                        }
                        other => bail!("Radio read failed: {:?}", other),
                    },
                };
            }
        }

        // Sleep for a moment so that other threads have the chance to grab the serial port mutex
        thread::sleep(Duration::from_millis(10));
    }
}

// The write function that the comms service will use to write messages to the "radio"
//
// This function may be called from either a message handler thread or from a downlink endpoint
pub fn write(conn: Arc<Mutex<RefCell<serial::SystemPort>>>, msg: &[u8]) -> ServiceResult<()> {
    let conn = match conn.lock() {
        Ok(val) => val,
        Err(e) => bail!("Failed to take mutex: {:?}", e),
    };
    let mut conn = conn.try_borrow_mut()?;

    conn.write(msg).and_then(|num| {
        debug!("Wrote {} bytes to radio", num);
        Ok(())
    })?;

    Ok(())
}
