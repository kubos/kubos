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

use mai400::*;
use messages::{MessageHeader, SYNC};
use super::*;


#[test]
fn get_message_bad() {
    let mock = mock_new!();

    let mai = MAI400 { conn: Connection { stream: Box::new(mock) } };

    assert_eq!(mai.get_message().unwrap_err(), MAIError::GenericError);
}


#[test]
fn get_message_unknown() {
    let mock = mock_new!();

    let mut raw: [u8; 201] = [0; 201];
    // Header
    raw[0] = 0x90;
    raw[1] = 0xEB;
    raw[2] = 1;
    raw[3] = 0;
    raw[4] = 0xFF; // Bad message ID
    raw[5] = 0x40;

    // Checksum
    raw[199] = 0x54;
    raw[200] = 0x73;

    mock.read.return_value(Ok(raw.to_vec()));

    let mai = MAI400 { conn: Connection { stream: Box::new(mock) } };

    assert_eq!(
        mai.get_message().unwrap_err(),
        MAIError::UnknownMessage { id: 0xFF }
    );
}

#[test]
fn get_message_good_stdtelem() {
    let mock = mock_new!();

    let expected = Response::StdTelem(StandardTelemetry {
        hdr: MessageHeader {
            sync: SYNC,
            data_len: 1,
            msg_id: 1,
            addr: 0x40,
        },
        crc: 0xE9FA,
        ..Default::default()
    });

    let mut raw: [u8; 201] = [0; 201];
    // Header
    raw[0] = 0x90;
    raw[1] = 0xEB;
    raw[2] = 1;
    raw[3] = 0;
    raw[4] = 1;
    raw[5] = 0x40;

    // Checksum
    raw[199] = 0xFA;
    raw[200] = 0xE9;

    mock.read.return_value(Ok(raw.to_vec()));

    let mai = MAI400 { conn: Connection { stream: Box::new(mock) } };

    let result = mai.get_message().unwrap();

    assert_eq!(result, expected);
}

#[test]
fn get_message_good_rawimu() {
    let mock = mock_new!();

    let expected = Response::IMU(RawIMU {
        hdr: MessageHeader {
            sync: SYNC,
            data_len: 13,
            msg_id: 3,
            addr: 0x40,
        },
        crc: 0xF21A,
        ..Default::default()
    });

    let mut raw: [u8; 21] = [0; 21];
    // Header
    raw[0] = 0x90;
    raw[1] = 0xEB;
    raw[2] = 0x0D;
    raw[3] = 0;
    raw[4] = 3;
    raw[5] = 0x40;

    // Checksum
    raw[19] = 0x1A;
    raw[20] = 0xF2;

    mock.read.return_value(Ok(raw.to_vec()));

    let mai = MAI400 { conn: Connection { stream: Box::new(mock) } };

    let result = mai.get_message().unwrap();

    assert_eq!(result, expected);
}

#[test]
fn get_message_good_irehs() {
    let mock = mock_new!();

    let expected = Response::IREHS(IREHSTelemetry {
        hdr: MessageHeader {
            sync: SYNC,
            data_len: 48,
            msg_id: 2,
            addr: 0x40,
        },
        crc: 0xD1FA,
        ..Default::default()
    });

    let mut raw: [u8; 56] = [0; 56];
    // Header
    raw[0] = 0x90;
    raw[1] = 0xEB;
    raw[2] = 0x30;
    raw[3] = 0;
    raw[4] = 2;
    raw[5] = 0x40;

    // Checksum
    raw[54] = 0xFA;
    raw[55] = 0xD1;

    mock.read.return_value(Ok(raw.to_vec()));

    let mai = MAI400 { conn: Connection { stream: Box::new(mock) } };

    let result = mai.get_message().unwrap();

    assert_eq!(result, expected);
}

#[test]
fn get_message_good_config() {
    let mock = mock_new!();

    let expected = Response::Config(ConfigInfo {
        hdr: MessageHeader {
            sync: SYNC,
            data_len: 20,
            msg_id: 6,
            addr: 0x40,
        },
        ehs_type: EHSType::Internal,
        st_type: StarTracker::MAISextant,
        crc: 0x4336,
        ..Default::default()
    });

    let mut raw: [u8; 20] = [0; 20];
    // Header
    raw[0] = 0x90;
    raw[1] = 0xEB;
    raw[2] = 0x14;
    raw[3] = 0;
    raw[4] = 6;
    raw[5] = 0x40;

    // Checksum
    raw[18] = 0x36;
    raw[19] = 0x43;

    mock.read.return_value(Ok(raw.to_vec()));

    let mai = MAI400 { conn: Connection { stream: Box::new(mock) } };

    let result = mai.get_message().unwrap();

    assert_eq!(result, expected);
}
