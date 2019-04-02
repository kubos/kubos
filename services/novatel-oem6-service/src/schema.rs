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

use crate::model::*;
use crate::objects::*;
use juniper::FieldResult;
use kubos_service;

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
        let last_cmd = executor.context().subsystem().last_cmd.read()?;
        Ok(*last_cmd)
    }

    // Get all errors encountered since the last time this field was queried
    //
    // {
    //     errors: [String]
    // }
    field errors(&executor) -> FieldResult<Vec<String>>
    {
        executor.context().subsystem().get_errors();

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

    // Get the current power state of the system
    //
    // Note: `uptime` is included as an available field in order to conform to
    //       the Kubos Service Outline, but cannot be implemented for this device,
    //       so the value will be 1 if the device is on and 0 if the device is off
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
    // Stretch goal: implement the LOGLIST command
    //
    // {
    //     config: "Not Implemented"
    // }
    field config(&executor) -> FieldResult<String>
    {
        Ok(String::from("Not Implemented"))
    }

    // Get the test results of the last run test
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

    // Get the current system status and errors
    //
    // {
    //     systemStatus {
    //        errors: Vec<String>,
    //        status: Vec<String>
    //     }
    // }
    field system_status(&executor) -> FieldResult<SystemStatus>
    {
        Ok(executor.context().subsystem().get_system_status()?)
    }

    // Get current status of position information gathering
    //
    // {
    //     lockStatus {
    //         positionStatus: SolutionStatus,
    //         positionType: PosVelType,
    //         time {
    //             ms: Int,
    //             week: Int
    //         },
    //         timeStatus: RefTimeStatus,
    //         velocityStatus: SolutionStatus,
    //         velocityType: PosVelType
    //     }
    // }
    field lock_status(&executor) -> FieldResult<LockStatus>
    {
        Ok(executor.context().subsystem().get_lock_status()?)
    }

    // Get the last known good position information
    //
    // {
    //     lockInfo {
    //        position: Vec<Float>,
    //        time {
    //            ms: Int,
    //            week: Int
    //        },
    //        velocity: Vec<Float>
    //     }
    // }
    field lock_info(&executor) -> FieldResult<LockInfo>
    {
        Ok(executor.context().subsystem().get_lock_info()?)
    }

    // Get current telemetry information for the system
    //
    // {
    //     telemetry{
    //         debug {
    //             components: [{
    //                 bootVersion: String,
    //                 compType: Int,
    //                 compileDate: String,
    //                 compileTime: String, 
    //                 hwVersion: String,
    //                 model: String,
    //                 serialNum: String,
    //                 swVersion: String,
    //             }],
    //             numComponents: Int
    //         },
    //         nominal{
    //             lockInfo {...},
    //             lockStatus {...},
    //             systemStatus: {
    //                errors: Vec<String>,
    //                status: Vec<String>
    //             }
    //         }
    //     }
    // }
    field telemetry(&executor) -> FieldResult<Telemetry>
    {
        Ok(executor.context().subsystem().get_telemetry()?)
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
        executor.context().subsystem().get_errors();

        match executor.context().subsystem().errors.read() {
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
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::Noop;
        Ok(executor.context().subsystem().noop()?)
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
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::ControlPower;
        Ok(String::from("Not Implemented"))
    }

    // Configure the system
    //
    // config: Vector of configuration requests (ConfigStruct)
    //   - option: Configuration operation which should be performed
    //   - hold: For `LOG_*` requests, specifies whether this request should be excluded
    //           from removal by future 'UNLOG_ALL' requests.
    //           For `UNLOG_ALL` requests, specifies whether the 'hold' value in previous
    //           `LOG_*` requests should be ignored.
    //   - interval: Interval at which log messages should be generated.
    //               Note: Only applies to `LOG_POSITION_DATA` requests. Ignored otherwise
    //   - offset: Offset of interval at which log messages should be generated.
    //             Note: Only applies to `LOG_POSITION_DATA` requests. Ignored otherwise
    //
    // mutation {
    //     configureHardware(config: [{option: ConfigOption, hold: Boolean, interval: Float, offset: Float},...]) {
    //         config: String
    //         errors: String,
    //         success: Boolean,
    //     }
    // }
    field configure_hardware(
        &executor,
        config: Vec<ConfigStruct>,
    ) -> FieldResult<ConfigureHardwareResponse>
    {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::ConfigureHardware;
        Ok(executor.context().subsystem().configure_hardware(config)?)
    }

    // Run a system self-test
    //
    // test: Type of self-test to perform
    //
    // mutation {
    //     testHardware(test: TestType) {
    //         ... on IntegrationTestResults {
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
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::TestHardware;
        match test {
            TestType::Integration => Ok(TestResults::Integration(executor.context().subsystem()
                    .get_test_results().unwrap())),
            TestType::Hardware => Ok(TestResults::Hardware(HardwareTestResults {
                        errors: "Not Implemented".to_owned(), success: true, data: "".to_owned()}))
        }
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
    field issue_raw_command(&executor, command: String) -> FieldResult<GenericResponse>
    {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::IssueRawCommand;
        Ok(executor.context().subsystem().passthrough(command)?)
    }
});
