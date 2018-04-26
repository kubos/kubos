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

use kubos_service;
use juniper::FieldResult;
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
    // {
    //     power {
    //         state: PowerState,
    //         uptime: Int
    //     }
    // }            
    field power(&executor) -> FieldResult<GetPowerResponse>
    {
        Ok(executor.context().subsystem().get_power()?)
    }
    
    // Get the current configuration of the system
    //
    // {
    //     config: "Not Implemented"
    // }
    field config(&executor) -> FieldResult<String>
    {
        Ok(String::from("Not Implemented"))
    }

    // Get current telemetry information for the system
    //
    // {
    //     //TODO
    // }
    field telemetry(&executor) -> FieldResult<Telemetry>
    {
        Ok(executor.context().subsystem().get_telemetry()?)
    }
    
    // Get the test results of the last run test
    //
    // Note: For this service, this actually just fetches the nominal
    // and debug telemetry of the system, since there is no actual
    // built-in test
    //
    // {
    //     testResults{
    //         success,
    //         telemetryNominal{...},
    //         telemetryDebug{...}
    //     }
    // }
    field test_results(&executor) -> FieldResult<IntegrationTestResults> {
        Ok(executor.context().subsystem().get_test_results()?)
    }
    
    field mode(&executor) -> FieldResult<Mode> {
        Ok(executor.context().subsystem().get_mode()?)
    }
        
    field orientation(&executor) -> FieldResult<String> {
        Ok(String::from("Not Implemented"))
    }
    
    field spin(&executor) -> FieldResult<Spin> {
        Ok(executor.context().subsystem().get_spin()?)
    }
});

pub struct MutationRoot;

/// Base GraphQL mutation model
graphql_object!(MutationRoot: Context as "Mutation" |&self| {
    
    // Get all errors encountered while processing this GraphQL request
    //
    // Note: This will only return errors thrown by fields which have
    // already been processed for this request, so it is recommended that
    // this field be specified last.
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
    // mutation {
    //     noop {
    //         errors: String,
    //         success: Boolean
    //    }
    // }
    field noop(&executor) -> FieldResult<GenericResponse>
    {
        executor.context().subsystem().last_cmd.set(AckCommand::Noop);
        Ok(executor.context().subsystem().noop()?)
    }

    // Control the power state of the system
    //
    // state: Power state the system should be changed to
    //   Note: The only valid input for this service is RESET
    //
    // mutation {
    //     controlPower(state: PowerState) {
    //         errors: String,
    //         success: Boolean,
    //         power: PowerState
    //     }
    // }
    field control_power(&executor, state: PowerState) -> FieldResult<ControlPowerResponse>
    {
        executor.context().subsystem().last_cmd.set(AckCommand::ControlPower);
        Ok(executor.context().subsystem().control_power(state)?)
    }
    
    // Configure the system
    //
    // config: Set which microcontroller future commands should be issued from
    //
    // mutation {
    //     configureHardware(config: ConfigureController) {
    //         errors: String,
    //         success: Boolean,
    //         config: ConfigureController
    //    }
    // }
    field configure_hardware(&executor) -> FieldResult<String>
    {
        executor.context().subsystem().last_cmd.set(AckCommand::ConfigureHardware);
        Ok(String::from("Not Implemented"))
    }
    
    // Run a system self-test
    //
    // test: Type of self-test to perform
    //
    // mutation {
    //     testHardware(test: TestType) {
    //         ... on IntegrationTestRsults {
    //             errors: String,
    //             success: Boolean,
    //             telemetryNominal{...},
    //             telemetryDebug{...}
    //         }
    //         ... on HardwareTestResults {
    //             errors: "Not Implemented",
    //             success: true,
    //             data: Empty
    //         }
    //    }
    // }
    field test_hardware(&executor, test: TestType) -> FieldResult<TestResults> 
    {
        executor.context().subsystem().last_cmd.set(AckCommand::TestHardware);
        match test {
            TestType::Integration => Ok(TestResults::Integration(executor.context().subsystem().get_test_results().unwrap())),
            TestType::Hardware => Ok(TestResults::Hardware(HardwareTestResults { errors: "Not Implemented".to_owned(), success: true, data: "".to_owned()}))
        }
    }
    
    // Pass a custom command through to the system
    //
    // command: String containing the hex values to be sent (ex. "C3")
    //   It will be converted to a byte array before transfer.
    // rxLen: Number of response bytes to read
    //
    // mutation {
    //     issueRawCommand(command: String, rx_len: Int) {
    //         errors: String,
    //         success: Boolean,
    //         response: String
    //     }
    // }
    // TODO: Maybe allow us to wait for a response?
    field issue_raw_command(&executor, command: String) -> FieldResult<GenericResponse>
    {
        executor.context().subsystem().last_cmd.set(AckCommand::IssueRawCommand);
        Ok(executor.context().subsystem().passthrough(command)?)
    }
    
    field set_mode(&executor, mode: Mode, qbi_cmd = {vec![0,0,0,0]}: Vec<i32>, sun_angle_enable = false: bool, sun_rot_angle = 0.0: f64) -> FieldResult<GenericResponse> {
        executor.context().subsystem().last_cmd.set(AckCommand::SetMode);
        match mode {
            Mode::NormalSun | Mode::LatLongSun => Ok(executor.context().subsystem().set_mode_sun(mode as u8, sun_angle_enable as i16, sun_rot_angle as f32)?),
            _ => Ok(executor.context().subsystem().set_mode(mode as u8, qbi_cmd)?),
        }
    }
    
    field update(&executor, gps_time: Option<i32>, rv: Option<RVInput>) -> FieldResult<GenericResponse> {
        executor.context().subsystem().last_cmd.set(AckCommand::Update);
        Ok(executor.context().subsystem().update(gps_time, rv)?)
    }
    
});
