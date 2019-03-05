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

//! Service queries

use crate::models::*;
use crate::schema::Context;
use juniper::FieldResult;

/// Telemetry query structure
pub struct Telemetry;

graphql_object!(Telemetry: Context as "telemetry" |&self| {
    // Fetch telemetry data for the motherboard.
    // All returned values are automatically converted from their original raw data.
    // Refer to Table 11-7 of the EPS' User Manual for more information.
    //
    // telemetry {
    //         motherboard {
    //             VoltageFeedingBcr1: f64,
    //             CurrentBcr1Sa1a: f64,
    //             CurrentBcr1Sa1b: f64,
    //             ArrayTempSa1a: f64,
    //             ArrayTempSa1b: f64,
    //             SunDetectorSa1a: f64,
    //             SunDetectorSa1b: f64,
    //             VoltageFeedingBcr2: f64,
    //             CurrentBcr2Sa2a: f64,
    //             CurrentBcr2Sa2b: f64,
    //             ArrayTempSa2a: f64,
    //             ArrayTempSa2b: f64,
    //             SunDetectorSa2a: f64,
    //             SunDetectorSa2b: f64,
    //             VoltageFeedingBcr3: f64,
    //             CurrentBcr3Sa3a: f64,
    //             CurrentBcr3Sa3b: f64,
    //             ArrayTempSa3a: f64,
    //             ArrayTempSa3b: f64,
    //             SunDetectorSa3a: f64,
    //             SunDetectorSa3b: f64,
    //             BcrOutputCurrent: f64,
    //             BcrOutputVoltage: f64,
    //             CurrentDraw3V3: f64,
    //             CurrentDraw5V: f64,
    //             OutputCurrent12V: f64,
    //             OutputVoltage12V: f64,
    //             OutputCurrentBattery: f64,
    //             OutputVoltageBattery: f64,
    //             OutputCurrent5v: f64,
    //             OutputVoltage5v: f64,
    //             OutputCurrent33v: f64,
    //             OutputVoltage33v: f64,
    //             OutputVoltageSwitch1: f64,
    //             OutputCurrentSwitch1: f64,
    //             OutputVoltageSwitch2: f64,
    //             OutputCurrentSwitch2: f64,
    //             OutputVoltageSwitch3: f64,
    //             OutputCurrentSwitch3: f64,
    //             OutputVoltageSwitch4: f64,
    //             OutputCurrentSwitch4: f64,
    //             OutputVoltageSwitch5: f64,
    //             OutputCurrentSwitch5: f64,
    //             OutputVoltageSwitch6: f64,
    //             OutputCurrentSwitch6: f64,
    //             OutputVoltageSwitch7: f64,
    //             OutputCurrentSwitch7: f64,
    //             OutputVoltageSwitch8: f64,
    //             OutputCurrentSwitch8: f64,
    //             OutputVoltageSwitch9: f64,
    //             OutputCurrentSwitch9: f64,
    //             OutputVoltageSwitch10: f64,
    //             OutputCurrentSwitch10: f64,
    //             BoardTemperature: f64,
    //      }
    // }
    field motherboard() -> motherboard_telemetry::Telemetry
        as "Motherboard Telemetry"
    {
        motherboard_telemetry::Telemetry {}
    }

    // Fetch telemetry data for the daughterboard.
    // All returned values are automatically converted from their original raw data.
    // Refer to Table 11-8 of the EPS' User Manual for more information.
    //
    // telemetry {
    //         daughterboard {
    //             VoltageFeedingBcr4: f64: f64,
    //             CurrentBcr4Sa4a: f64,
    //             CurrentBcr4Sa4b: f64,
    //             ArrayTempSa4a: f64,
    //             ArrayTempSa4b: f64,
    //             SunDetectorSa4a: f64,
    //             SunDetectorSa4b: f64,
    //             VoltageFeedingBcr5: f64,
    //             CurrentBcr5Sa5a: f64,
    //             CurrentBcr5Sa5b: f64,
    //             ArrayTempSa5a: f64,
    //             ArrayTempSa5b: f64,
    //             SunDetectorSa5a: f64,
    //             SunDetectorSa5b: f64,
    //             VoltageFeedingBcr6: f64,
    //             CurrentBcr6Sa6a: f64,
    //             CurrentBcr6Sa6b: f64,
    //             ArrayTempSa6a: f64,
    //             ArrayTempSa6b: f64,
    //             SunDetectorSa6a: f64,
    //             SunDetectorSa6b: f64,
    //             VoltageFeedingBcr7: f64,
    //             CurrentBcr7Sa7a: f64,
    //             CurrentBcr7Sa7b: f64,
    //             ArrayTempSa7a: f64,
    //             ArrayTempSa7b: f64,
    //             SunDetectorSa7a: f64,
    //             SunDetectorSa7b: f64,
    //             VoltageFeedingBcr8: f64,
    //             CurrentBcr8Sa8a: f64,
    //             CurrentBcr8Sa8b: f64,
    //             ArrayTempSa8a: f64,
    //             ArrayTempSa8b: f64,
    //             SunDetectorSa8a: f64,
    //             SunDetectorSa8b: f64,
    //             VoltageFeedingBcr9: f64,
    //             CurrentBcr9Sa9a: f64,
    //             CurrentBcr9Sa9b: f64,
    //             ArrayTempSa9a: f64,
    //             ArrayTempSa9b: f64,
    //             SunDetectorSa9a: f64,
    //             SunDetectorSa9b: f64,
    //             BoardTemperature: f64,
    //        }
    // }
    field daughterboard() -> daughterboard_telemetry::Telemetry
        as "Daughterboard Telemetry"
    {
        daughterboard_telemetry::Telemetry {}
    }

    // Get the number of board resets, by category
    //
    // telemetry {
    //         reset {
    //             brownOut {
    //                 motherboard: i32,
    //                 daughterboard: i32,
    //            },
    //             automaticSoftware {
    //                 motherboard: i32,
    //                 daughterboard: i32,
    //            },
    //             manual {
    //                 motherboard: i32,
    //                 daughterboard: i32,
    //            },
    //             watchdog {
    //                 motherboard: i32,
    //                 daughterboard: i32,
    //            },
    //        }
    // }
    field reset() -> reset_telemetry::Telemetry
        as "Reset Telemetry"
    {
        reset_telemetry::Telemetry {}
    }

    // Fetch the current watchdog timeout period, in minutes
    //
    // telemetry {
    //         watchdogPeriod: u8,
    // }
    field watchdog_period(&executor) -> FieldResult<i32>
        as "Current watchdog period in minutes"
    {
        Ok(i32::from(executor.context().subsystem().get_comms_watchdog_period()?))
    }

    // Get the version information for the EPS motherboard and daughterboard (if accesssible)
    //
    // telemetry {
    //     motherboard {
    //         revision: i32,
    //         firmware_number: i32
    //     },
    //     daughterboard {
    //         revision: i32,
    //         firmware_number: i32
    //     }
    // }
    field version(&executor) -> FieldResult<version::VersionData>
        as "Hardware version information"
    {
        Ok(executor.context().subsystem().get_version()?)
    }

    // Fetch the last error which was encountered by the system while executing a command
    //
    // telemetry {
    //         lastEpsError {
    //             motherboard: last_error::Error,
    //             daughterboard: last_error::Error,
    //        }
    // }
    field last_eps_error(&executor) -> FieldResult<last_error::Data>
        as "Last EPS error reported"
    {
        Ok(executor.context().subsystem().get_last_eps_error()?)
    }

    // Check the status of the motherboard and daughterboard
    //
    // telemetry {
    //         boardStatus {
    //             motherboard: board_status::Status,
    //             daugherboard: board_status::Status
    //        }
    // }
    field board_status(&executor) -> FieldResult<board_status::BoardData>
        as "EPS board status"
    {
        Ok(executor.context().subsystem().get_board_status()?)
    }
});

/// Top-level query root structure
pub struct Root;

/// Base GraphQL query
graphql_object!(Root: Context as "Query" |&self| {

    // Test query to verify service is running without
    // attempting to communicate with hardware
    //
    // {
    //    ping: "pong"
    // }
    field ping() -> FieldResult<String>
        as "Test service query"
    {
        Ok(String::from("pong"))
    }

    // Get the last mutation run
    //
    // {
    //    ack: subsystem::Mutations
    // }
    field ack(&executor) -> FieldResult<subsystem::Mutations>
        as "Last run mutation"
    {
        let last_cmd = executor.context().subsystem().last_mutation.read()?;
        Ok(*last_cmd)
    }

    // Get all errors encountered since the last time
    // this field was queried
    //
    // {
    //    errors: [String]
    // }
    field errors(&executor) -> FieldResult<Vec<String>>
        as "Last errors encountered"
    {
        Ok(executor.context().subsystem().get_errors()?)
    }

    // Get the system power status
    //
    // {
    //    power {
    //        motherboard: PowerState,
    //      daughterboard: PowerState,
    //    }
    // }
    field power(&executor) -> FieldResult<GetPowerResponse>
        as "Last errors encountered"
    {
        Ok(executor.context().subsystem().get_power()?)
    }

    // Get telemetry from the EPS
    // telemetry {
    //         motherboard {
    //             ...
    //      },
    //         daughterboard {
    //             ...
    //        },
    //         reset {
    //             brownOut {
    //                 motherboard: i32,
    //                 daughterboard: i32,
    //            },
    //             automaticSoftware {
    //                 motherboard: i32,
    //                 daughterboard: i32,
    //            },
    //             manual {
    //                 motherboard: i32,
    //                 daughterboard: i32,
    //            },
    //             watchdog {
    //                 motherboard: i32,
    //                 daughterboard: i32,
    //            },
    //        },
    //         watchdogPeriod: u8,
    //         version {
    //             motherboard {
    //                 revision: i32,
    //                 firmwareNumber: i32,
    //            },
    //             daughterboard {
    //                 revision: i32,
    //                 firmwareNumber: i32,
    //            },
    //        },
    //         lastEpsError {
    //             motherboard: last_error::Error,
    //             daughterboard: last_error::Error,
    //        },
    //         boardStatus {
    //             motherboard: board_status::Status,
    //             daugherboard: board_status::Status
    //        }
    // }
    field telemetry(&executor) -> FieldResult<Telemetry>
    {
        Ok(Telemetry)
    }
});
