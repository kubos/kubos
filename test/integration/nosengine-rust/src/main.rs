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

use nosengine_rust::*;
use utils::testing::TestCommand;

pub fn main() {
    callback_test();

    request_message_test();

    expect_error();

    spi_test();
}

fn callback_test() {
    println!("callback_test");

    let sim = TestCommand::new(
        "nos_engine_server_standalone",
        vec!["-f", "sim_config1.json"],
    );
    sim.spawn();

    let bus = client::Bus::new("testbus", "tcp://127.0.0.1:12001").unwrap();
    let node3 = client::DataNode::new(&bus, "node3").unwrap();
    let node4 = client::DataNode::new(&bus, "node4").unwrap();

    extern "C" fn callback(_data_node: *mut ffi::DataNodeHandle, msg_ptr: *mut ffi::MessageHandle) {
        println!("Received message in callback: {:?}", unsafe {
            client::Message::get_contents_from_ptr(msg_ptr)
        });
    }

    node4.set_message_callback(callback);

    node3.send_message("node4", &[1u8, 2, 3, 4, 5]).unwrap();
}

fn request_message_test() {
    println!("request_message_test");

    let sim = TestCommand::new(
        "nos_engine_server_standalone",
        vec!["-f", "sim_config2.json"],
    );
    sim.spawn();

    let bus = client::Bus::new("testbus", "tcp://127.0.0.2:12001").unwrap();
    let node5 = client::DataNode::new(&bus, "node5").unwrap();
    let node6 = client::DataNode::new(&bus, "node6").unwrap();

    extern "C" fn callback(node_ptr: *mut ffi::DataNodeHandle, msg_ptr: *mut ffi::MessageHandle) {
        client::DataNode::send_reply_message_ptr(node_ptr, msg_ptr, &[5u8, 6, 7, 8]);
    }

    node6.set_message_callback(callback);

    let response = node5
        .send_request_message("node6", &[1u8, 2, 3, 4])
        .unwrap();
    assert_eq!(response.get_contents(), &[5u8, 6, 7, 8]);
}

fn expect_error() {
    println!("expect_error");

    let sim = TestCommand::new(
        "nos_engine_server_standalone",
        vec!["-f", "sim_config3.json"],
    );
    sim.spawn();

    let bus = client::Bus::new("testbus", "tcp://127.0.0.3:12001").unwrap();
    let node7 = client::DataNode::new(&bus, "node7").unwrap();
    let node8 = client::DataNode::new(&bus, "node8").unwrap();

    extern "C" fn callback(node_ptr: *mut ffi::DataNodeHandle, msg_ptr: *mut ffi::MessageHandle) {
        client::DataNode::send_reply_message_ptr(node_ptr, msg_ptr, &[5u8, 6, 7, 8]);
    }

    node8.set_message_callback(callback);

    let response = node7.send_request_message("nowhere", &[1u8, 2, 3, 4]);
    match response {
        Err(client::NosError::NosEngineError {
            error_code: _,
            description: _,
        }) => {}
        _ => panic!("Expected invalid destination error."),
    }
}

fn spi_test() {
    println!("spi_test");

    let sim = TestCommand::new(
        "nos_engine_server_standalone",
        vec!["-f", "sim_config4.json"],
    );
    sim.spawn();

    use client::spi::*;
    use ffi::spi;
    use std::slice;

    extern "C" fn callback(dir: spi::SPIDirection, buffer: *mut u8, len: usize) -> usize {
        let data = unsafe { slice::from_raw_parts_mut(buffer, len) };
        match dir {
            spi::SPIDirection::Write => {
                println!("Wrote {} bytes: {:?}", len, data);
                len
            }
            spi::SPIDirection::Read => {
                println!("Reading {} bytes.", len);
                for i in 0..len {
                    data[i] = i as u8;
                }
                len
            }
        }
    }

    let master = SPIMaster::new("tcp://127.0.0.4:12001", "spia").unwrap();
    let _slave = SPISlave::new(1u8, "tcp://127.0.0.4:12001", "spia", callback).unwrap();

    master.chip_select(1u8);
    master.write(&[1u8, 2u8, 3u8, 4u8]).unwrap();
    let data = master.read(4).unwrap();
    assert_eq!(data, &[0u8, 1, 2, 3]);
}
