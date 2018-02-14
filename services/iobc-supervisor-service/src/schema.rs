//
// Copyright (C) 2018 Kubos Corporation
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

extern crate kubos_hal_iobc;

use model::{Supervisor, SupervisorEnableStatus, SupervisorHousekeeping, SupervisorVersion};
use juniper::Context as JuniperContext;
use juniper::FieldResult;

/// Context used to pass global data into Juniper queries
pub struct Context {
    pub supervisor: Supervisor,
}

impl JuniperContext for Context {}

impl Context {
    /// Give us a reference to subsystem for passing
    /// along the Juniper chain
    pub fn get_supervisor(&self) -> &Supervisor {
        &self.supervisor
    }
}

/// GraphQL model annotations for SupervisorVersion
graphql_object!(SupervisorVersion: Context as "SupervisorVersion" |&self| {
    description: "Supervisor Version Information"
    // The GraphQL spec only defines an Integer and Float type
    // These wrapper functions convert our base types (u8 mostly)
    // into the more compatible i32. Same is done for
    // SupervisorEnableStatus and SupervisorHousekeeping

    field dummy() -> FieldResult<i32>
        as "Dummy Byte"
    {
        Ok(i32::from(self.0.dummy))
    }

    field spi_command_status() -> FieldResult<i32>
        as "SPI Command Status"
    {
        Ok(i32::from(self.0.spi_command_status))
    }

    field index_of_subsystem() -> FieldResult<i32>
        as "Index Of Subsystem"
    {
        Ok(i32::from(self.0.index_of_subsystem))
    }

    field major_version() -> FieldResult<i32>
        as "Major Version"
    {
        Ok(i32::from(self.0.major_version))
    }

    field minor_version() -> FieldResult<i32>
        as "Minor Version"
    {
        Ok(i32::from(self.0.minor_version))
    }

    field patch_version() -> FieldResult<i32>
        as "Patch Version"
    {
        Ok(i32::from(self.0.patch_version))
    }

    field git_head_version() -> FieldResult<i32>
        as "Git Head Version"
    {
        // There is no i32::from(u32)
        // So we have to cast like this
        Ok(self.0.git_head_version as i32)
    }

    field serial_number() -> FieldResult<i32>
        as "Serial Number"
    {
        Ok(i32::from(self.0.serial_number))
    }

    field compile_information() -> FieldResult<Vec<i32>>
        as "Compile Information"
    {
        Ok(self.0.compile_information.iter()
           .map(|x| i32::from(*x))
           .collect::<Vec<i32>>())
    }

    field clock_speed() -> FieldResult<i32>
        as "Clock Speed"
    {
        Ok(i32::from(self.0.clock_speed))
    }

    field code_type() -> FieldResult<i32>
        as "Code Type"
    {
        Ok(i32::from(self.0.code_type))
    }

    field crc() -> FieldResult<i32>
        as "CRC"
    {
        Ok(i32::from(self.0.crc))
    }
});

graphql_object!(SupervisorEnableStatus: Context as "SupervisorEnableStatus" |&self| {
    description: "Enable Status"

    field power_obc() -> FieldResult<i32>
        as "Power OBC"
    {
        Ok(i32::from(self.0.power_obc))
    }

    field power_rtc() -> FieldResult<i32>
        as "Power RTC"
    {
        Ok(i32::from(self.0.power_rtc))
    }

    field supervisor_mode() -> FieldResult<i32>
        as "Supervisor Mode"
    {
        Ok(i32::from(self.0.supervisor_mode))
    }

    field busy_rtc() -> FieldResult<i32>
        as "Busy RTC"
    {
        Ok(i32::from(self.0.busy_rtc))
    }

    field power_off_rtc() -> FieldResult<i32>
        as "Power Off RTC"
    {
        Ok(i32::from(self.0.power_off_rtc))
    }
});

graphql_object!(SupervisorHousekeeping: Context as "SupervisorHousekeeping" |&self| {
    description: "Supervisor Housekeeping"

    field dummy() -> FieldResult<i32>
        as "Dummy Byte"
    {
        Ok(i32::from(self.0.dummy))
    }

    field spi_command_status() -> FieldResult<i32>
        as "SPI Command Status"
    {
        Ok(i32::from(self.0.spi_command_status))
    }

    field enable_status() -> FieldResult<SupervisorEnableStatus>
        as "Enable Status"
    {
        Ok(SupervisorEnableStatus(self.0.enable_status))
    }

    field supervisor_uptime() -> FieldResult<i32>
        as "Supervisor Uptime"
    {
        Ok(self.0.supervisor_uptime as i32)
    }

    field iobc_uptime() -> FieldResult<i32>
        as "iOBC Uptime"
    {
        Ok(self.0.iobc_uptime as i32)
    }

    field iobc_reset_count() -> FieldResult<i32>
        as "iOBC Reset Count"
    {
        Ok(self.0.iobc_reset_count as i32)
    }

    field adc_data() -> FieldResult<Vec<i32>>
        as "ADC Data"
    {
        Ok(self.0.adc_data.iter()
           .map(|x| i32::from(*x))
        .collect::<Vec<i32>>())
    }

    field adc_update_flag() -> FieldResult<i32>
        as "ADC Update Flag"
    {
        Ok(i32::from(self.0.adc_update_flag))
    }

    field crc8() -> FieldResult<i32>
        as "CRC8"
    {
        Ok(i32::from(self.0.crc8))
    }
});

/// GraphQL model for Subsystem
graphql_object!(Supervisor: Context as "Supervisor" |&self| {
    description: "iOBC Supervisor"

    field version() -> FieldResult<SupervisorVersion>
        as "Supervisor Version Information"
    {
        Ok(self.version()?)
    }

    field housekeeping() -> FieldResult<SupervisorHousekeeping>
        as "Supervisor Housekeeping Information"
    {
        Ok(self.housekeeping()?)
    }
});

pub struct QueryRoot;

/// Base GraphQL query model
graphql_object!(QueryRoot : Context as "Query" |&self| {
    field supervisor(&executor) -> FieldResult<&Supervisor>
        as "Supervisor Query"
    {
        Ok(executor.context().get_supervisor())
    }
});

pub struct MutationRoot;

/// Base GraphQL mutation model
graphql_object!(MutationRoot : Context as "Mutation" |&self| {

    field reset(&executor) -> FieldResult<()>
        as "Reset Supervisor"
    {
        Ok(kubos_hal_iobc::supervisor_reset()?)
    }

    field emergency_reset(&executor) -> FieldResult<()>
        as "Supervisor Emergency Reset"
    {
        Ok(kubos_hal_iobc::supervisor_emergency_reset()?)
    }

    field powercycle(&executor) -> FieldResult<()>
        as "Supervisor Powercycle"
    {
        Ok(kubos_hal_iobc::supervisor_powercycle()?)
    }

});
