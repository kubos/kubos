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
    let radio = match Trxvu::new() {
        Ok(r) => r,
        Err(e) => {
            print!("Err {}", e);
            return;
        }
    };

    println!("Getting transmit telemetry");
    let tx_telemetry = match radio.get_tx_telemetry() {
        Ok(t) => t,
        Err(e) => panic!("Err {}", e.to_string()),
    };
    println!(
        "TX Telemetry\n\
         Inst RF Reflected {} dBm\n\
         Inst RF Forward   {} dBm\n\
         Supply Voltage    {} mA\n\
         Supply Current    {} C\n\
         Power Amp Temp   {} C\n\
         Oscillator Temp   {} C",
        tx_telemetry.inst_rf_reflected,
        tx_telemetry.inst_rf_forward,
        tx_telemetry.supply_voltage,
        tx_telemetry.supply_current,
        tx_telemetry.temp_power_amp,
        tx_telemetry.temp_oscillator
    );

    println!("Getting transmit state");
    let tx_state = match radio.get_tx_state() {
        Ok(t) => t,
        Err(e) => panic!("Err {}", e.to_string()),
    };
    println!("Tx State: {:?}", tx_state);

    println!("Getting receive telemetry");
    let rx_telemetry = match radio.get_rx_telemetry() {
        Ok(r) => r,
        Err(e) => panic!("Err {}", e.to_string()),
    };
    println!(
        "TX Telemetry\n\
         Inst Doppler Offset {} dBm\n\
         Inst Signal Strength   {} dBm\n\
         Supply Voltage    {} mA\n\
         Supply Current    {} C\n\
         Power Amp Temp   {} C\n\
         Oscillator Temp   {} C",
        rx_telemetry.inst_doppler_offset,
        rx_telemetry.inst_signal_strength,
        rx_telemetry.supply_voltage,
        rx_telemetry.supply_current,
        rx_telemetry.temp_power_amp,
        rx_telemetry.temp_oscillator
    );

    match radio.get_rx_uptime() {
        Ok(u) => println!("RX Uptime {}", u),
        Err(e) => panic!("Err: {}", e.to_string()),
    };
}
