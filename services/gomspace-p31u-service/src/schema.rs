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

    /// Verify service is running without communicating with the underlying subsystem

    field ping() -> FieldResult<String>{
        Ok(String::from("pong"))    
    }

    //----- Generic Queries -----//

    /// Get the last run mutation

    field ack(&executor) -> FieldResult<AckCommand>
    {
        let last_cmd = executor.context().subsystem().last_cmd.read()?;
        Ok(*last_cmd)
    }

    /// Get all errors encountered since the last time this field was queried

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

    /// Ping the EPS system via i2c
    field testping(&executor) -> FieldResult<GenericResponse>
    {
       Ok(executor.context().subsystem().eps_ping()?)
    }

    /// Query the EPS housekeeping data
    field epshk(&executor) -> FieldResult<SchEpsHk>
    {
        Ok(executor.context().subsystem().eps_get_housekeeping()?)
    }

    /// Query EPS system configuration      
    field epssystemconfig(&executor) -> FieldResult<SchEpsSystemConfig>
    {
        Ok(executor.context().subsystem().eps_get_system_config()?)
    }

    /// Query EPS battery configuration
    field epsbattconfig(&executor) -> FieldResult<SchEpsBatteryConfig>
    {
        Ok(executor.context().subsystem().eps_get_battery_config()?)
    }

    /// Query EPS heatter
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
    
    /// Reset the EPS
    field reset(&executor) -> FieldResult<GenericResponse>{
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::Reboot;
        Ok(executor.context().subsystem().eps_reset()?)
    }

    /// Reboot the EPS
    field reboot(&executor) -> FieldResult<GenericResponse>{
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::Reboot;
        Ok(executor.context().subsystem().eps_reboot()?)
    }

    /// Set system configuration
    field set_system_config(&executor,
                ppt_mode: i32,
                battheater_mode: i32,
                battheater_low:i32,
                battheater_high:i32,
                output_normal_value:Vec<i32>,
                output_safe_value:Vec<i32>,
                output_initial_on_delay:Vec<i32>,
                output_initial_off_delay:Vec<i32>,
                vboost_settings:Vec<i32>) -> FieldResult<GenericResponse>{
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::SetSystemConfig;
        Ok(executor.context().subsystem().eps_set_system_config(   
                ppt_mode,
                battheater_mode,
                battheater_low,
                battheater_high,
                output_normal_value,
                output_safe_value,
                output_initial_on_delay,
                output_initial_off_delay,
                vboost_settings
        )?)
    }

    /// Set battery configuration
    field set_battery_config(&executor,
            batt_maxvoltage: i32,
            batt_safevoltage: i32,
            batt_criticalvoltage: i32,
            batt_normalvoltage: i32) -> FieldResult<GenericResponse>{
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::SetBatteryConfig;
        Ok(executor.context().subsystem().eps_set_battery_config(
            batt_maxvoltage,
            batt_safevoltage,
            batt_criticalvoltage,
            batt_normalvoltage
        )?)
    }

    /// Save battery configuration
    field save_battery_config(&executor) -> FieldResult<GenericResponse>
    {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::SaveBattConfig;
        Ok(executor.context().subsystem().eps_save_battery_config()?)
    }

    /// Reset the system configuration to user default
    field reset_system_config(&executor) -> FieldResult<GenericResponse>
    {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::SystemConfigReset;
        Ok(executor.context().subsystem().eps_reset_system_config()?)
    }

    /// Reset the battery configuration to user default
    field reset_battery_config(&executor) -> FieldResult<GenericResponse>
    {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::BatteryConfigReset;        
        Ok(executor.context().subsystem().eps_reset_battery_config()?)
    }

    /// Reset all counters including watchdogs and reboot counters to 0 
    field reset_counter(&executor) -> FieldResult<GenericResponse>
    {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::ResetCounters;    
        Ok(executor.context().subsystem().eps_reset_counters()?)
    }

    /// Set a single EPS output
    field eps_set_single_output(&executor, channel:i32, value:i32,delay:i32) -> FieldResult<(GenericResponse)>
    {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::SetEpsChannels;
        Ok(executor.context().subsystem().eps_set_single_output(channel,value,delay)?)
    }

    /// Set EPS MPPT value for mode 2 (Non-Auto mode)
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

    /// EPS set output
    field eps_set_output(&executor, mask:i32) -> FieldResult<GenericResponse>
    {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::EpsHeaterToggle;
        Ok(executor.context().subsystem().eps_set_output(mask)?)
    }

    ///Kick watchdog
    field eps_watchdog_kick(&executor) -> FieldResult<GenericResponse>
    {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::EpsWatchDog;
        Ok(executor.context().subsystem().eps_watchdog_kick()?)
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
