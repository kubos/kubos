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

extern crate isis_trxvu;

use isis_trxvu::Trxvu;

pub fn main() {
    let radio = Trxvu::new();

    println!("Getting transmit telemetry");
    let tx_telemetry = radio.get_tx_telemetry().unwrap();
    println!("{:?}", tx_telemetry);

    println!("Getting receive telemetry");
    let rx_telemetry = radio.get_rx_telemetry().unwrap();
    println!("{:?}", rx_telemetry);
}
