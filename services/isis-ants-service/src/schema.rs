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

use model::*;
use juniper::Context as JuniperContext;
use juniper::FieldResult;

/// Context used to pass global data into Juniper queries
pub struct Context {
    pub subsystem: Subsystem,
}

impl JuniperContext for Context {}

graphql_union!(Telemetry: () |&self| {
	description: "Test"
	instance_resolvers: |&_| {
		&TelemetryNominal => match *self { Telemetry::Nominal(ref n) => Some(n), _ => None},
		&TelemetryDebug => match *self { Telemetry::Debug(ref d) => Some(d), _ => None},
	}
});

graphql_union!(TestResults: () |&self| {
	description: "Test"
	instance_resolvers: |&_| {
		&IntegrationTestResults => match *self { TestResults::Integration(ref i) => Some(i), _ => None},
		&HardwareTestResults => match *self { TestResults::Hardware(ref h) => Some(h), _ => None},
	}
});

pub struct QueryRoot;

/// Base GraphQL query model
graphql_object!(QueryRoot: Context as "Query" |&self| {
    
    //----- Test Query -----//
    field ping() -> FieldResult<String>
    {
    	Ok(String::from("Pong"))
    }
    
    //----- general queries ----//
    field ack() -> FieldResult<AckCommand>
    {
    	// Future development: figure out how Rust lifetimes work and persist the
    	// last mutation run between requests
    	Ok(AckCommand::None)
    }

    field power(&executor) -> FieldResult<GetPowerResponse>
        as "Antenna System Power State"
    {
        Ok(executor.context().subsystem.get_power()?)
    }
    
    field config() -> FieldResult<String>
    {
    	//TODO: Should this be something???
    	Ok(String::from("Default"))
    }
    
    field errors(&executor) -> FieldResult<Vec<String>>
    {
    	match executor.context().subsystem.errors.try_borrow() {
    		Ok(master_vec) => Ok(master_vec.clone()),
    		_ => Ok(vec!["Error: Failed to borrow master errors vector".to_owned()])
    	}
    }
    
    field telemetry(&executor, telem: TelemetryType) -> FieldResult<Telemetry>
    {
    	match telem {
    		TelemetryType::Nominal => Ok(Telemetry::Nominal(executor.context().subsystem.get_telemetry_nominal().unwrap())),
    		TelemetryType::Debug => Ok(Telemetry::Debug(executor.context().subsystem.get_telemetry_debug().unwrap()))
    	}
    }
    
    field test_results(&executor) -> FieldResult<IntegrationTestResults> {
    	Ok(executor.context().subsystem.get_test_results()?)
    }

	//------- Deployable-specific queries ------//
	
	field arm_status(&executor) -> FieldResult<ArmStatus>
	{
		Ok(executor.context().subsystem.get_arm_status()?)
	}
	
	field deployment_status(&executor) -> FieldResult<GetDeployResponse>
	{
		Ok(executor.context().subsystem.get_deploy_status()?)
	}
});

pub struct MutationRoot;

/// Base GraphQL mutation model
graphql_object!(MutationRoot: Context as "Mutation" |&self| {
	
	field errors(&executor) -> FieldResult<Vec<String>>
    {
    	match executor.context().subsystem.errors.try_borrow() {
    		Ok(master_vec) => Ok(master_vec.clone()),
    		_ => Ok(vec!["Error: Failed to borrow master errors vector".to_owned()])
    	}
    }

    field noop(&executor) -> FieldResult<NoopResponse>
    {
    	Ok(executor.context().subsystem.noop()?)
    }

    field control_power(&executor, state: PowerState) -> FieldResult<ControlPowerResponse>
    {
    	Ok(executor.context().subsystem.control_power(state)?)
    }
    
    field configure_hardware(&executor, config: ConfigureController) -> FieldResult<ConfigureHardwareResponse>
    {
    	Ok(executor.context().subsystem.configure_hardware(config)?)
    }
    
    field test_hardware(&executor, test: TestType) -> FieldResult<TestResults> 
    {
    	match test {
    		TestType::Integration => Ok(TestResults::Integration(executor.context().subsystem.integration_test().unwrap())),
    		TestType::Hardware => Ok(TestResults::Hardware(HardwareTestResults { success: true, data: String::from("Not Implemented")}))
    	}
    }
    
    field issue_raw_command(&executor, command: String, rx_len = 0: i32) -> FieldResult<RawCommandResponse>
    {
    	Ok(executor.context().subsystem.passthrough(command, rx_len)?)
    }
    
    //------- Deployable-specific mutations ------//
    
    field arm(&executor, state: ArmState) -> FieldResult<ArmResponse>
    {
    	Ok(executor.context().subsystem.arm(state)?)
    }
    
    field deploy(&executor, ant = (DeployType::All): DeployType, force = false: bool, time: i32) -> FieldResult<DeployResponse>
    {
    	Ok(executor.context().subsystem.deploy(ant, force, time)?)
    }
    
});
