//
// Copyright (C) 2017 Kubos Corporation
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

use crate::model::*;
use crate::objects::*;
use juniper::FieldResult;
use kubos_service;

type Context = kubos_service::Context<Subsystem>;


pub struct QueryRoot;
// Base GraphQL query model
graphql_object!(QueryRoot : Context as "Query" |&self| {
    

    //----- Test Queries -----//

    // Verify service is running without communicating with the underlying subsystem

    field testping() -> FieldResult<String>{
        Ok(String::from("pong"))    
    }

    //----- Generic Queries -----//

    // Get the last run mutation

    field ack(&executor) -> FieldResult<AckCommand>
    {
        let last_cmd = executor.context().subsystem().last_cmd.read()?;
        Ok(*last_cmd)
    }

    // Get all errors encountered since the last time this field was queried

    field errors(&executor) -> FieldResult<Vec<String>>
    {
        match executor.context().subsystem().errors.write() {
            Ok(mut master_vec) => {
                let current = master_vec.clone();
                master_vec.clear();
                master_vec.shrink_to_fit();
                Ok(current)
            },
            _ => Ok(vec!["Error: Failed to borrow master errors vector".to_owned()])
        }
    }


    //----- EPS Queries -----//

    field ping(&executor) -> FieldResult<GenericResponse>
    {
       Ok(executor.context().subsystem().eps_ping()?)
    }

    field reset(&executor) -> FieldResult<GenericResponse>
    {
       Ok(executor.context().subsystem().eps_reset()?)
    }

    field reboot(&executor) -> FieldResult<GenericResponse>
    {
       Ok(executor.context().subsystem().eps_reboot()?)
    }

    field battsave(&executor) -> FieldResult<GenericResponse>
    {
       Ok(executor.context().subsystem().eps_save_battery_config()?)
    }

    field systemreset(&executor) -> FieldResult<GenericResponse>
    {
       Ok(executor.context().subsystem().eps_reset_system_config()?)
    }

    field batteryreset(&executor) -> FieldResult<GenericResponse>
    {
       Ok(executor.context().subsystem().eps_reset_battery_config()?)
    }

    field batteryreset(&executor) -> FieldResult<GenericResponse>
    {
       Ok(executor.context().subsystem().eps_reset_battery_config()?)
    }

    field countreset(&executor) -> FieldResult<GenericResponse>
    {
       Ok(executor.context().subsystem().eps_reset_counters()?)
    }

    field epshk(&executor) -> FieldResult<SchEpsHk>
    {
        Ok(executor.context().subsystem().eps_get_housekeeping()?)
    }

    field epssystemconfig(&executor) -> FieldResult<SchEpsSystemConfig>
    {
        Ok(executor.context().subsystem().eps_get_system_config()?)
    }

    field epsbattconfig(&executor) -> FieldResult<SchEpsBatteryConfig>
    {
        Ok(executor.context().subsystem().eps_get_battery_config()?)
    }

    field epsheatter(&executor) -> FieldResult<i32>
    {
        Ok(executor.context().subsystem().eps_get_heater()?)
    }

});

pub struct MutationRoot;

// Base GraphQL mutation model
graphql_object!(MutationRoot : Context as "Mutation" |&self| {

    // Each field represents functionality available
    // through the GraphQL mutations
    field eps_set_single_output(&executor, channel:i32, value:i32,delay:i32) -> FieldResult<(GenericResponse)>
    {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::SetEpsChannels;
        Ok(executor.context().subsystem().eps_set_single_output(channel,value,delay)?)
    }

    field eps_set_input_value(&executor, in1_voltage: i32,in2_voltage:i32,in3_voltage:i32) -> FieldResult<(GenericResponse)>
    {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::EpsSetMPPTLevel;
        Ok(executor.context().subsystem().eps_set_input_value(in1_voltage,in2_voltage,in3_voltage)?)
    }


    /// Set EPS MPPT mode
    field eps_set_input_mode(&executor,mode:i32) -> FieldResult<GenericResponse>
    {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::EpsSetMPPTmode;
        Ok(executor.context().subsystem().eps_set_input_mode(mode)?)
    }

    /// set heater on/off
    field eps_set_heater(&executor,cmd:i32,heater:i32,mode:i32) -> FieldResult<GenericResponse>
    {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::EpsHeaterToggle;
        Ok(executor.context().subsystem().eps_set_heater(cmd,heater,mode)?)
    }

    ///Kick watchdog
    field eps_watchdog_kick(&executor) -> FieldResult<GenericResponse>
    {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::EpsWatchDog;
        Ok(executor.context().subsystem().eps_watchdog_kick()?)
    }

    ///start watchdog
    field eps_watchdog_start(&executor) -> FieldResult<GenericResponse>
    {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::Noop;
        Ok(executor.context().subsystem().eps_watchdog_start()?)
    }

    ///Stop watchdog
    field eps_watchdog_stop(&executor) -> FieldResult<GenericResponse>
    {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::Noop;
        Ok(executor.context().subsystem().eps_watchdog_stop()?)
    }

    // Pass a custom command through to the system
    //
    // command: String containing the hex values to be sent (ex. "C3")
    //   It will be converted to a byte array before transfer.
    // rxLen: Number of response bytes to read
    //

    field issue_raw_command(&executor, command: String, rx_len = 0: i32) -> FieldResult<RawCommandResponse>
    {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::IssueRawCommand;
        Ok(executor.context().subsystem().passthrough(command, rx_len)?)
    }
});
