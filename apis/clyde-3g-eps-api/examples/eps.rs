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

use clyde_3g_eps_api::*;
use rust_i2c::*;
use std::thread;
use std::time::Duration;

macro_rules! print_result {
    ($p:expr, $r:expr) => {
        match $r {
            Ok(v) => println!("{} - {:#?}", $p, v),
            Err(e) => println!("{} Err - {:#?}", $p, e),
        }
    };
}

macro_rules! dump_mother_telem {
    (
        $eps:expr,
        $($type:ident,)+
    ) => {
        $(
            print_result!(
                format!("{:?}", MotherboardTelemetry::Type::$type),
                $eps.get_motherboard_telemetry(MotherboardTelemetry::Type::$type)
            );
            thread::sleep(Duration::from_millis(100));
        )+
    };
}

macro_rules! dump_daughter_telem {
    (
        $eps:expr,
        $($type:ident,)+
    ) => {
        $(
            print_result!(
                format!("{:?}", DaughterboardTelemetry::Type::$type),
                $eps.get_daughterboard_telemetry(DaughterboardTelemetry::Type::$type)
            );
            thread::sleep(Duration::from_millis(100));
        )+
    };
}

macro_rules! dump_reset_telem {
    (
        $eps:expr,
        $($type:ident,)+
    ) => {
        $(
            print_result!(
                format!("{:?}", ResetTelemetry::Type::$type),
                $eps.get_reset_telemetry(ResetTelemetry::Type::$type)
            );
            thread::sleep(Duration::from_millis(100));
        )+
    };
}

pub fn main() {
    let eps = Eps::new(Connection::from_path("/dev/i2c-1", 0x2B));

    print_result!("Version Info", eps.get_version_info());
    thread::sleep(Duration::from_millis(100));

    print_result!("Board Status", eps.get_board_status());
    thread::sleep(Duration::from_millis(100));

    print_result!("Checksum", eps.get_checksum());
    thread::sleep(Duration::from_millis(100));

    print_result!("Last Error", eps.get_last_error());
    thread::sleep(Duration::from_millis(100));

    print_result!("Watchdog Period", eps.get_comms_watchdog_period());
    thread::sleep(Duration::from_millis(100));

    dump_mother_telem!(
        eps,
        VoltageFeedingBcr1,
        CurrentBcr1Sa1a,
        CurrentBcr1Sa1b,
        ArrayTempSa1a,
        ArrayTempSa1b,
        SunDetectorSa1a,
        SunDetectorSa1b,
        VoltageFeedingBcr2,
        CurrentBcr2Sa2a,
        CurrentBcr2Sa2b,
        ArrayTempSa2a,
        ArrayTempSa2b,
        SunDetectorSa2a,
        SunDetectorSa2b,
        VoltageFeedingBcr3,
        CurrentBcr3Sa3a,
        CurrentBcr3Sa3b,
        ArrayTempSa3a,
        ArrayTempSa3b,
        SunDetectorSa3a,
        SunDetectorSa3b,
        BcrOutputCurrent,
        BcrOutputVoltage,
        CurrentDraw3V3,
        CurrentDraw5V,
        OutputCurrent12V,
        OutputVoltage12V,
        OutputCurrentBattery,
        OutputVoltageBattery,
        OutputCurrent5V,
        OutputVoltage5V,
        OutputCurrent33V,
        OutputVoltage33V,
        OutputVoltageSwitch1,
        OutputCurrentSwitch1,
        OutputVoltageSwitch2,
        OutputCurrentSwitch2,
        OutputVoltageSwitch3,
        OutputCurrentSwitch3,
        OutputVoltageSwitch4,
        OutputCurrentSwitch4,
        OutputVoltageSwitch5,
        OutputCurrentSwitch5,
        OutputVoltageSwitch6,
        OutputCurrentSwitch6,
        OutputVoltageSwitch7,
        OutputCurrentSwitch7,
        OutputVoltageSwitch8,
        OutputCurrentSwitch8,
        OutputVoltageSwitch9,
        OutputCurrentSwitch9,
        OutputVoltageSwitch10,
        OutputCurrentSwitch10,
        BoardTemperature,
    );

    dump_daughter_telem!(
        eps,
        VoltageFeedingBcr4,
        CurrentBcr4Sa4a,
        CurrentBcr4Sa4b,
        ArrayTempSa4a,
        ArrayTempSa4b,
        SunDetectorSa4a,
        SunDetectorSa4b,
        VoltageFeedingBcr5,
        CurrentBcr5Sa5a,
        CurrentBcr5Sa5b,
        ArrayTempSa5a,
        ArrayTempSa5b,
        SunDetectorSa5a,
        SunDetectorSa5b,
        VoltageFeedingBcr6,
        CurrentBcr6Sa6a,
        CurrentBcr6Sa6b,
        ArrayTempSa6a,
        ArrayTempSa6b,
        SunDetectorSa6a,
        SunDetectorSa6b,
        VoltageFeedingBcr7,
        CurrentBcr7Sa7a,
        CurrentBcr7Sa7b,
        ArrayTempSa7a,
        ArrayTempSa7b,
        SunDetectorSa7a,
        SunDetectorSa7b,
        VoltageFeedingBcr8,
        CurrentBcr8Sa8a,
        CurrentBcr8Sa8b,
        ArrayTempSa8a,
        ArrayTempSa8b,
        SunDetectorSa8a,
        SunDetectorSa8b,
        VoltageFeedingBcr9,
        CurrentBcr9Sa9a,
        CurrentBcr9Sa9b,
        ArrayTempSa9a,
        ArrayTempSa9b,
        SunDetectorSa9a,
        SunDetectorSa9b,
        BoardTemperature,
    );

    dump_reset_telem!(eps, BrownOut, AutomaticSoftware, Manual, Watchdog,);
}
