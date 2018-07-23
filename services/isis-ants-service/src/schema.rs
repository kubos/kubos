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
        as "Antenna System Power State"
    {
        Ok(executor.context().subsystem().get_power()?)
    }

    // Get the current microcontroller which commands will be sent to
    //
    // {
    //     config: ConfigureController
    // }
    field config(&executor) -> FieldResult<ConfigureController>
    {
        // Future development: Once Rust lifetimes have been figured out,
        // this could be updated to return the controller previously set
        // with the 'configureHardware' mutation
        Ok(executor.context().subsystem().get_config()?)
    }

    // Get current telemetry information for the system
    //
    // {
    //     telemetry {
    //         nominal {
    //             rawTemp: Int,
    //             uptime: Int,
    //             sysBurnActive: Boolean,
    //             sysIgnoreDeploy: Boolean,
    //             sysArmed: Boolean,
    //             ant1NotDeployed: Boolean,
    //             ant1StoppedTime: Boolean,
    //             ant1Active: Boolean,
    //             ant2NotDeployed: Boolean,
    //             ant2StoppedTime: Boolean,
    //             ant2Active: Boolean,
    //             ant3NotDeployed: Boolean,
    //             ant3StoppedTime: Boolean,
    //             ant3Active: Boolean,
    //             ant4NotDeployed: Boolean,
    //             ant4StoppedTime: Boolean,
    //             ant4Active: Boolean
    //         },
    //     	   debug {
    //             ant1ActivationCount: Int,
    //             ant1ActivationTime: Int,
    //             ant2ActivationCount: Int,
    //             ant2ActivationTime: Int,
    //             ant3ActivationCount: Int,
    //             ant3ActivationTime: Int,
    //             ant4ActivationCount: Int,
    //             ant4ActivationTime: Int,
    //         }
    //     }
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

    //----- Deployable-specific Queries -----//

    // Get the current armed/disarmed status of the system
    //
    // {
    //     armStatus: ArmStatus
    // }
    field arm_status(&executor) -> FieldResult<ArmStatus>
    {
        Ok(executor.context().subsystem().get_arm_status()?)
    }

    // Get the current deployment status of the system
    //
    // {
    //     deploymentStatus {
    //         status: DeploymentStatus,
    //         sysBurnActive: Boolean,
    //         sysIgnoreDeploy: Boolean,
    //         sysArmed: Boolean,
    //         ant1NotDeployed: Boolean,
    //         ant1StoppedTime: Boolean,
    //         ant1Active: Boolean,
    //         ant2NotDeployed: Boolean,
    //         ant2StoppedTime: Boolean,
    //         ant2Active: Boolean,
    //         ant3NotDeployed: Boolean,
    //         ant3StoppedTime: Boolean,
    //         ant3Active: Boolean,
    //         ant4NotDeployed: Boolean,
    //         ant4StoppedTime: Boolean,
    //         ant4Active: Boolean
    // }
    field deployment_status(&executor) -> FieldResult<GetDeployResponse>
    {
        Ok(executor.context().subsystem().get_deploy_status()?)
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
    // mutation {
    //     noop {
    //         errors: String,
    //         success: Boolean
    //    }
    // }
    field noop(&executor) -> FieldResult<NoopResponse>
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
    field configure_hardware(&executor, config: ConfigureController) -> FieldResult<ConfigureHardwareResponse>
    {
        executor.context().subsystem().last_cmd.set(AckCommand::ConfigureHardware);
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
        executor.context().subsystem().last_cmd.set(AckCommand::TestHardware);

        match test {
            TestType::Integration => Ok(TestResults::Integration(executor.context().subsystem().integration_test().unwrap())),
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
    field issue_raw_command(&executor, command: String, rx_len = 0: i32) -> FieldResult<RawCommandResponse>
    {
        executor.context().subsystem().last_cmd.set(AckCommand::IssueRawCommand);
        Ok(executor.context().subsystem().passthrough(command, rx_len)?)
    }

    //----- Deployable-specific mutations -----//

    // Arm/Disarm the system
    //
    // state: Armed/Disarmed state the system should be changed to
    //
    // mutation {
    //     arm(state: ArmState) {
    //         errors: String,
    //         success: Boolean
    //    }
    // }
    field arm(&executor, state: ArmState) -> FieldResult<ArmResponse>
    {
        executor.context().subsystem().last_cmd.set(AckCommand::Arm);
        Ok(executor.context().subsystem().arm(state)?)
    }

    // Deploy antenna/s
    //
    // ant: (Default - All) Antenna to deploy
    // force: (Default - false) Whether current deployment state should be ignored/overridden
    // time: Maximum amount of time to spend attempting to deploy the antenna
    //   (for 'All', this is the amount of time spent for each antenna)
    //
    // mutation {
    //     deploy(ant: DeployType = DeployType::All, force: Boolean = false, time: Int) {
    //         errors: String,
    //         success: Boolean
    //    }
    // }
    field deploy(&executor, ant = (DeployType::All): DeployType, force = false: bool, time: i32) -> FieldResult<DeployResponse>
    {
        executor.context().subsystem().last_cmd.set(AckCommand::Deploy);
        Ok(executor.context().subsystem().deploy(ant, force, time)?)
    }

});
