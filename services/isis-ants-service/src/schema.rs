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

graphql_object!(GetPowerResponse: () |&self| {
	field state() -> FieldResult<PowerState> {
		Ok(self.state.clone())
	}
	
	field uptime() -> FieldResult<i32> {
		Ok(self.uptime as i32)
	}
});

graphql_object!(GetDeployResponse: () |&self| {
	field deploy_status() -> FieldResult<DeploymentStatus> {
		Ok(self.deploy_status.clone())
	}
		
    field sys_burn_active() -> FieldResult<bool> {
        Ok(self.details.sys_burn_active)
    }

    field sys_ignore_deploy() -> FieldResult<bool> {
        Ok(self.details.sys_ignore_deploy)
    }

    field sys_armed() -> FieldResult<bool> {
        Ok(self.details.sys_armed)
    }

    field ant_1_not_deployed() -> FieldResult<bool> {
        Ok(self.details.ant_1_not_deployed)
    }

    field ant_1_stopped_time() -> FieldResult<bool> {
        Ok(self.details.ant_1_stopped_time)
    }

    field ant_1_active() -> FieldResult<bool> {
        Ok(self.details.ant_1_active)
    }

    field ant_2_not_deployed() -> FieldResult<bool> {
        Ok(self.details.ant_2_not_deployed)
    }

    field ant_2_stopped_time() -> FieldResult<bool> {
        Ok(self.details.ant_2_stopped_time)
    }

    field ant_2_active() -> FieldResult<bool> {
        Ok(self.details.ant_2_active)
    }

    field ant_3_not_deployed() -> FieldResult<bool> {
        Ok(self.details.ant_3_not_deployed)
    }

    field ant_3_stopped_time() -> FieldResult<bool> {
        Ok(self.details.ant_3_stopped_time)
    }

    field ant_3_active() -> FieldResult<bool> {
        Ok(self.details.ant_3_active)
    }

    field ant_4_not_deployed() -> FieldResult<bool> {
        Ok(self.details.ant_4_not_deployed)
    }

    field ant_4_stopped_time() -> FieldResult<bool> {
        Ok(self.details.ant_4_stopped_time)
    }

    field ant_4_active() -> FieldResult<bool> {
        Ok(self.details.ant_4_active)
    }

});

graphql_object!(TelemetryNominal: () |&self| {
    field raw_temp() -> FieldResult<i32> {
    	Ok(self.0.raw_temp as i32)
    }
    
	field uptime() -> FieldResult<i32> {
		Ok(self.0.uptime as i32)
	}
    
    field sys_burn_active() -> FieldResult<bool> {
        Ok(self.0.deploy_status.sys_burn_active)
    }

    field sys_ignore_deploy() -> FieldResult<bool> {
        Ok(self.0.deploy_status.sys_ignore_deploy)
    }

    field sys_armed() -> FieldResult<bool> {
        Ok(self.0.deploy_status.sys_armed)
    }

    field ant_1_not_deployed() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_1_not_deployed)
    }

    field ant_1_stopped_time() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_1_stopped_time)
    }

    field ant_1_active() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_1_active)
    }

    field ant_2_not_deployed() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_2_not_deployed)
    }

    field ant_2_stopped_time() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_2_stopped_time)
    }

    field ant_2_active() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_2_active)
    }

    field ant_3_not_deployed() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_3_not_deployed)
    }

    field ant_3_stopped_time() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_3_stopped_time)
    }

    field ant_3_active() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_3_active)
    }

    field ant_4_not_deployed() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_4_not_deployed)
    }

    field ant_4_stopped_time() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_4_stopped_time)
    }

    field ant_4_active() -> FieldResult<bool> {
        Ok(self.0.deploy_status.ant_4_active)
    }

});

graphql_object!(TelemetryDebug: () |&self| {
    field ant_1_activation_count() -> FieldResult<i32> {
        Ok(self.ant1.act_count as i32)
    }
    
    field ant_1_activation_time() -> FieldResult<i32> {
        Ok(self.ant1.act_time as i32)
    }
    
    field ant_2_activation_count() -> FieldResult<i32> {
        Ok(self.ant2.act_count as i32)
    }
    
    field ant_2_activation_time() -> FieldResult<i32> {
        Ok(self.ant2.act_time as i32)
    }
    
    field ant_3_activation_count() -> FieldResult<i32> {
        Ok(self.ant3.act_count as i32)
    }
    
    field ant_3_activation_time() -> FieldResult<i32> {
        Ok(self.ant3.act_time as i32)
    }
    
    field ant_4_activation_count() -> FieldResult<i32> {
        Ok(self.ant4.act_count as i32)
    }
    
    field ant_4_activation_time() -> FieldResult<i32> {
        Ok(self.ant4.act_time as i32)
    }
});

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
    
    field errors(&executor) -> FieldResult<String>
    {
    	Ok(executor.context().subsystem.errors.borrow().clone())
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
	
	field errors(&executor) -> FieldResult<String>
    {
    	Ok(executor.context().subsystem.errors.borrow().clone())
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
