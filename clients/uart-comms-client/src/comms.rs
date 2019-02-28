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

use super::*;

use serial;
use serial::prelude::*;
use std::io::prelude::*;

const MAX_READ: usize = 4096;
const TIMEOUT: Duration = Duration::from_millis(2000);

// Initialize the serial bus connection for reading and writing from the "radio"
pub fn serial_init(bus: &str) -> ClientResult<serial::SystemPort> {
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

    Ok(port)
}

pub fn write(port: &mut serial::SystemPort, data: &[u8]) -> ClientResult<()> {
    let num = port.write(data)?;
    if num < data.len() {
        bail!(
            "Failed to write all bytes of data. Wrote {} of {} bytes",
            num,
            data.len()
        );
    }

    Ok(())
}

pub fn read(port: &mut serial::SystemPort) -> ClientResult<Vec<u8>> {
    let mut buffer: Vec<u8> = vec![0; MAX_READ];
    match port.read(buffer.as_mut_slice()) {
        Ok(num) => {
            buffer.resize(num, 0);
        }
        Err(ref err) => match err.kind() {
            ::std::io::ErrorKind::TimedOut => {
                bail!("Timed out waiting for response");
            }
            other => bail!("Failed to read response: {:?}", other),
        },
    };

    Ok(buffer)
}
