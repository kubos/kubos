/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::messages::{parse_ack_or_nak, parse_u32, File, GeoRecord, Message, StateOfHealth};
use radio_api::{Connection, RadioResult};

/// Structure for interacting with Duplex-D2 Radio API
pub struct DuplexD2 {
    conn: Connection,
}

impl DuplexD2 {
    /// Constructor for DuplexD2 structure
    pub fn new(conn: Connection) -> DuplexD2 {
        DuplexD2 { conn }
    }

    /// Retrieves a record of information regarding modem functioning.
    pub fn get_state_of_health_for_modem(&self) -> RadioResult<StateOfHealth> {
        self.conn.write(b"GUGETSOH")?;
        self.conn.read(StateOfHealth::parse)
    }

    /// Retrieves a count of files that have been received by the modem and await retrieval by the
    /// FP.
    pub fn get_uploaded_file_count(&self) -> RadioResult<u32> {
        self.conn.write(b"GUGETUFC")?;
        self.conn.read(parse_u32)
    }

    /// Retrieves a count of messages that have been received by the modem and await retrieval by
    /// the FP.
    pub fn get_uploaded_message_count(&self) -> RadioResult<u32> {
        self.conn.write(b"GUGETUMC")?;
        self.conn.read(parse_u32)
    }

    /// Retrieves a count of messages that have been received by the modem and await retrieval by
    /// the FP.
    pub fn get_download_file_count(&self) -> RadioResult<u32> {
        self.conn.write(b"GUGETDFC")?;
        self.conn.read(parse_u32)
    }

    /// Retrieves an estimate of the modem’s latitude and longitude coordinates at the time of the
    /// last connection.
    pub fn get_geolocation_position_estimate(&self) -> RadioResult<GeoRecord> {
        self.conn.write(b"GUGETGEO")?;
        self.conn.read(GeoRecord::parse)
    }

    /// Retrieves the next file in the upload queue.  File is then ACKed and deleted from queue.
    pub fn get_uploaded_file(&self) -> RadioResult<File> {
        self.conn.write(b"GUGET_UF")?;
        let result = self.conn.read(File::parse)?;
        self.conn.write(b"GU\x06")?;
        Ok(result)
    }

    /// Retrieves the next message in the upload queue.  Message is then ACKed and deleted.
    pub fn get_uploaded_message(&self) -> RadioResult<Message> {
        self.conn.write(b"GUGET_UM")?;
        let result = self.conn.read(Message::parse)?;
        self.conn.write(b"GU\x06")?;
        Ok(result)
    }

    /// Deletes all files in the modem download queue. Returns number of files deleted.
    pub fn delete_download_files(&self) -> RadioResult<u32> {
        self.conn.write(b"GUDLTQDF")?;
        self.conn.read(parse_u32)
    }

    /// Deletes all files in the modem upload queue. Returns number of files deleted.
    pub fn delete_uploaded_files(&self) -> RadioResult<u32> {
        self.conn.write(b"GUDLTQUF")?;
        self.conn.read(parse_u32)
    }

    /// Deletes all messages in the modem upload queue. Returns number of messages deleted.
    pub fn delete_uploaded_messages(&self) -> RadioResult<u32> {
        self.conn.write(b"GUDLTQUM")?;
        self.conn.read(parse_u32)
    }

    /// Put a new file on the download queue, returns true if accepted by EyeStar-D2.
    pub fn put_download_file(&self, file: &File) -> RadioResult<bool> {
        self.conn.write(b"GUPUT_DF")?;
        self.conn.read(parse_ack_or_nak)?;
        self.conn.write(&file.encode())?;
        self.conn.read(parse_ack_or_nak)
    }

    /// Check if modem is powered and if it is able to respond to commands.
    pub fn get_alive(&self) -> RadioResult<bool> {
        self.conn.write(b"GUGETALV")?;
        self.conn.read(parse_ack_or_nak)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use radio_api::{Connection, RadioResult, Stream};

    struct TestStream {
        data: Vec<u8>,
    }

    impl Stream for TestStream {
        fn write(&self, _: &[u8]) -> RadioResult<()> {
            Ok(())
        }

        fn read(&self) -> RadioResult<Vec<u8>> {
            Ok(self.data.clone())
        }
    }

    fn test_connection(data: Vec<u8>) -> Connection {
        Connection::new(Box::new(TestStream { data }))
    }

    #[test]
    fn test_get_uploaded_file_count() {
        let radio = DuplexD2::new(test_connection(b"GU\x00\x00\x00\x37".to_vec()));
        assert_eq!(55, radio.get_uploaded_file_count().unwrap());
    }
}
