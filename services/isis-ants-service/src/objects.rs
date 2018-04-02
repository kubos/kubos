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
use model::*;

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
