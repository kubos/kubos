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

use juniper::FieldResult;
use kubos_service;
use model::*;
use objects::*;

type Context = kubos_service::Context<Subsystem>;

pub struct QueryRoot;

/// Base GraphQL query model
graphql_object!(QueryRoot: Context as "Query" |&self| {

    // Test query to verify service is running without attempting
    // to communicate with the underlying subsystem
    //
    // {
    //     ping: "pong"
    // }
    field ping() -> FieldResult<String>
    {
        Ok(String::from("pong"))
    }

    //----- Generic Queries -----//

    // Get the last run mutation
    //
    // {
    //     ack: AckCommand
    // }
    field ack(&executor) -> FieldResult<AckCommand>
    {
        Ok(executor.context().subsystem().last_cmd.get())
    }

    // Get all errors encountered since the last time this field was queried
    //
    // {
    //     errors: [String]
    // }
    field errors(&executor) -> FieldResult<Vec<String>>
    {
        // TODO: Get the status of the read thread
        //executor.context().subsystem().get_read_health();

        match executor.context().subsystem().errors.try_borrow_mut() {
            Ok(mut master_vec) => {
                let current = master_vec.clone();
                master_vec.clear();
                master_vec.shrink_to_fit();
                Ok(current)
            },
            _ => Ok(vec!["Error: Failed to borrow master errors vector".to_owned()])
        }
    }

    // Get the current power state and uptime of the system
    //
    // TODO: Figure out what to do with uptime. It isn't returned by the device at all...
    //
    // {
    //     power {
    //         state: PowerState,
    //         uptime: Int
    //     }
    // }
    field power(&executor) -> FieldResult<String>
    {
        Ok(String::from("Not Implemented"))
    }

    // Get the current configuration of the system
    //
    // TODO: Return the version log info? Stretch goal: implement the LOGLIST command
    //
    // {
    //     config: "Not Implemented"
    // }
    field config(&executor) -> FieldResult<String>
    {
        Ok(String::from("Not Implemented"))
    }
    
    // TODO: Check for system errors
    // Stretch goal: Implement RXSTATUS
    field system_status(&executor) -> FieldResult<String>
    {
        Ok(String::from("Not Implemented"))
    }

    // TODO:
    // - Time - yes/no
    // - position - yes/no
    // - velocity - yes/no
    // - confidence? (ex. FINESTEERING)
    // - Potentially change the yes/no's to a gradient
    field lock_status(&executor) -> FieldResult<String>
    {
        Ok(String::from("Not Implemented"))
    }

    // TODO: 
    // - Locked GPS time
    // - Locked position
    // - Locked velocity
    // - System time lock occurred
    field lock_info(&executor) -> FieldResult<String>
    {
        Ok(String::from("Not Implemented"))
    }

    // TODO:
    // Nominal - 
    // - System status
    // - Lock status
    // - Lock info
    // - Power status
    // Debug -
    // - Maybe - System config info (LOGLIST?)
    field telemetry(&executor) -> FieldResult<String>
    {
        Ok(String::from("Not Implemented"))
    }

    field test_results(&executor) -> FieldResult<String> {
        Ok(String::from("Not Implemented"))
    }
});

pub struct MutationRoot;

/// Base GraphQL mutation model
graphql_object!(MutationRoot: Context as "Mutation" |&self| {
    // Get all errors encountered while processing this GraphQL request
    //
    // Note: This will only return errors thrown by fields which have
    // already been processed, so it is recommended that this field be specified last.
    //
    // mutation {
    //     errors: [String]
    // }
    field errors(&executor) -> FieldResult<Vec<String>>
    {
        match executor.context().subsystem().errors.try_borrow() {
            Ok(master_vec) => Ok(master_vec.clone()),
            _ => Ok(vec!["Error: Failed to borrow master errors vector".to_owned()])
        }
    }

    // Execute a trivial command against the system
    //
    // TODO: Get/Read FW version
    //
    // mutation {
    //     noop {
    //         errors: String,
    //         success: Boolean
    //    }
    // }
    field noop(&executor) -> FieldResult<String>
    {
        executor.context().subsystem().last_cmd.set(AckCommand::Noop);
        //Ok(executor.context().subsystem().noop()?)
        Ok(String::from("Not Implemented"))
    }

    // Control the power state of the system
    //
    // Note: Power control of the GPS device will be done by the GPSRM service
    //
    // mutation {
    //     controlPower: "Not Implemented"
    // }
    field control_power(&executor) -> FieldResult<String>
    {
        executor.context().subsystem().last_cmd.set(AckCommand::ControlPower);
        Ok(String::from("Not Implemented"))
    }

    // Configure the system
    //
    // TODO: Enum - Start logging new type, stop logging an existing type, stop logging everything
    //
    // mutation {
    //     configureHardware: "Not Implemented"
    // }
    field configure_hardware(&executor) -> FieldResult<String>
    {
        executor.context().subsystem().last_cmd.set(AckCommand::ConfigureHardware);
        Ok(String::from("Not Implemented"))
    }

    // Run a system self-test
    //
    // TODO: minimum - No-op command. Stretch goal: implement RXSTATUS and the "built-in status tests"
    //
    field test_hardware(&executor) -> FieldResult<String>
    {
        executor.context().subsystem().last_cmd.set(AckCommand::TestHardware);
        Ok(String::from("Not Implemented"))
    }

    // Pass a custom command through to the system
    //
    // command: String containing the hex values to be sent (ex. "C3")
    //          It will be converted to a byte array before transfer.
    //
    // mutation {
    //     issueRawCommand(command: String) {
    //         errors: String,
    //         success: Boolean,
    //         response: String
    //     }
    // }
    field issue_raw_command(&executor) -> FieldResult<String>
    {
        executor.context().subsystem().last_cmd.set(AckCommand::IssueRawCommand);
        //Ok(executor.context().subsystem().passthrough(command)?)
        Ok(String::from("Not Implemented"))
    }
});
