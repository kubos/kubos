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
use failure::Error;
use gomspace_p31u_api::*;
use kubos_service::{process_errors, push_err, run};
use log::info;
use std::str;
use std::sync::{Arc, Mutex, RwLock};

use crate::objects::*;

/// Model for service's subsystem
#[derive(Clone)]
pub struct Subsystem{
    //pub eps: Box<GsEps>,
    pub eps: Arc<Mutex<Box<dyn GsEps>>>,
    pub errors: Arc<RwLock<Vec<String>>>,
    pub last_cmd: Arc<RwLock<AckCommand>>,
}

impl Subsystem {
    /// Creates new Subsystem structure instance
    /// Code initializing subsystems communications
    /// would likely be placed here
    pub fn new(bus: &str, addr: u8, wd_timeout:u32) -> EpsResult<Subsystem> {
        // Initialize config

        //let eps = Box::new(Eps::new(bus, addr, wd_timeout)?);
        let eps: Arc<Mutex<Box<dyn GsEps>>> = Arc::new(Mutex::new(Box::new(Eps::new(
            bus, addr, wd_timeout
        )?)));
        
        info!("Kubos GomSpace EPS systems service started");

        Ok(Subsystem {
           eps,            
           errors: Arc::new(RwLock::new(vec![])),
           last_cmd: Arc::new(RwLock::new(AckCommand::None)),           
        })       
    }

    //----- Functions for EPS Queries -----//
    pub fn eps_ping(&self) -> EpsResult<GenericResponse> {
        let result = run!(self.eps.lock().unwrap().ping(), self.errors);

        Ok(GenericResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }

    pub fn eps_reset(&self) -> EpsResult<GenericResponse> {
        let result = run!(self.eps.lock().unwrap().reset(), self.errors);

        Ok(GenericResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }


    pub fn eps_reboot(&self) -> EpsResult<GenericResponse> {
        let result = run!(self.eps.lock().unwrap().reboot(), self.errors);

        Ok(GenericResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }

    pub fn eps_save_battery_config(&self) -> EpsResult<GenericResponse> {
        let result = run!(self.eps.lock().unwrap().save_battery_config(), self.errors);

        Ok(GenericResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }


    pub fn eps_reset_system_config(&self) -> EpsResult<GenericResponse> {
        let result = run!(self.eps.lock().unwrap().reset_system_config(), self.errors);

        Ok(GenericResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }

    pub fn eps_reset_battery_config(&self) -> EpsResult<GenericResponse> {
        let result = run!(self.eps.lock().unwrap().reset_battery_config(), self.errors);

        Ok(GenericResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }

    pub fn eps_reset_counters(&self) -> EpsResult<GenericResponse> {
        let result = run!(self.eps.lock().unwrap().reset_counters(), self.errors);

        Ok(GenericResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }

    pub fn eps_get_housekeeping(&self) -> EpsResult<SchEpsHk> {
        println!("Get EPS HK");

        let result = run!(self.eps.lock().unwrap().get_housekeeping(), self.errors).unwrap_or_default();

        let epshk = SchEpsHk {
            vboost:result.vboost.iter().map(|x| *x as i32).collect(),
            vbatt:result.vbatt as i32,
            curin:result.curin.iter().map(|x| *x as i32).collect(),
            cursun:result.cursun as i32,
            cursys:result.cursys as i32,
            reserved1:result.reserved1 as i32,
            curout:result.curout.iter().map(|x| *x as i32).collect(),
            output:result.output.iter().map(|x| *x as i32).collect(),
            output_on_delta:result.output_on_delta.iter().map(|x| *x as i32).collect(),
            output_off_delta:result.output_off_delta.iter().map(|x| *x as i32).collect(),
            latchup:result.latchup.iter().map(|x| *x as i32).collect(),
            wdt_i2c_time_left:result.wdt_i2c_time_left as i32,
            wdt_gnd_time_left:result.wdt_gnd_time_left as i32,
            wdt_csp_pings_left:result.wdt_csp_pings_left.iter().map(|x| *x as i32).collect(),
            counter_wdt_i2c:result.counter_wdt_i2c as i32,
            counter_wdt_gnd:result.counter_wdt_gnd as i32,
            counter_wdt_csp:result.counter_wdt_csp.iter().map(|x| *x as i32).collect(),
            counter_boot:result.counter_boot as i32,
            temp:result.temp.into_iter().map(|x| *x as i32).collect(),
            boot_cause:result.boot_cause as i32,
            batt_mode:result.batt_mode as i32,
            ppt_mode:result.ppt_mode as i32,
           // reserved2: unsafe { cast::transmute(k_epshk.reserved2.map(|x| { x as i32 }).collet(),
            reserved2:result.reserved2 as i32,
        };
        Ok(epshk)
       // Ok(result)
    }

    pub fn eps_get_system_config(&self) -> EpsResult<SchEpsSystemConfig> {
        println!("Get System configuration");
        let result = run!(self.eps.lock().unwrap().get_system_config(), self.errors).unwrap_or_default();
        
        let sysconf = SchEpsSystemConfig {
            ppt_mode: result.ppt_mode as i32,
            battheater_mode:result.battheater_mode as i32,
            battheater_low:result.battheater_low as i32,
            battheater_high:result.battheater_high as i32,
            output_normal_value:result.output_normal_value.iter().map(|x| *x as i32).collect(),
            output_safe_value:result.output_safe_value.iter().map(|x| *x as i32).collect(),
            output_initial_on_delay:result.output_initial_on_delay.iter().map(|x| *x as i32).collect(),
            output_initial_off_delay:result.output_initial_off_delay.iter().map(|x| *x as i32).collect(),
            //vboost: vboost.to_vec().into_iter().map(|x| x as i32).collect(),
            vboost:result.vboost.iter().map(|x| *x as i32).collect(),
        };
                
        Ok(sysconf)
    }

    
    pub fn eps_get_battery_config(&self) -> EpsResult<SchEpsBatteryConfig> {
        println!("Get Battery configuration");
        let result = run!(self.eps.lock().unwrap().get_battery_config(), self.errors).unwrap_or_default();
       
        let epsbatconf = SchEpsBatteryConfig {
            batt_maxvoltage:result.batt_maxvoltage as i32,
            batt_safevoltage:result.batt_safevoltage as i32,
            batt_criticalvoltage:result.batt_criticalvoltage as i32,
            batt_normalvoltage:result.batt_normalvoltage as i32,
            reserved1:result.reserved1.iter().map(|x| *x as i32).collect(),
            reserved2:result.reserved2.iter().map(|x| *x as i32).collect(),
        };
        Ok(epsbatconf) 
    }


   
    pub fn eps_get_heater(&self) -> EpsResult<i32> {
        println!("Get Heater configuration");
        let result = run!(self.eps.lock().unwrap().get_heater(), self.errors).unwrap();

        Ok(result)
    }

    //----- Functions for EPS Mutations -----//

    /// Subsystem power state setter
    pub fn eps_set_single_output(&self, eps_channel:i32, eps_value:i32,eps_delay:i32) ->  EpsResult<(GenericResponse)> {
        let channel = eps_channel as u8;
        let value = eps_value as u8;
        let delay = eps_delay as u16;
        println!("Setting power state: Channel {} is set to {} with {}ms delay", channel, value,delay);


        let result = run!(self.eps.lock().unwrap().set_single_output(channel, value, delay), self.errors);

        Ok(GenericResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }

    /// Set MPPT voltage level 
    pub fn eps_set_input_value(&self, in1_voltage: i32,in2_voltage:i32,in3_voltage:i32) ->  EpsResult<(GenericResponse)> {
        let in1 = in1_voltage as u16;
        let in2 = in2_voltage as u16;
        let in3 = in3_voltage as u16;
        println!("Setting MPPT power state: Channel 1 is set to {},\n
                        Channel 2 is set to {},\n
                        Channel 3 is set to {}", in1, in2, in3);

        let result = run!(self.eps.lock().unwrap().set_input_value(in1, in2, in3), self.errors);


        Ok(GenericResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }


    /// Set MPPT mode
    pub fn eps_set_input_mode(&self, mode:i32) ->  EpsResult<(GenericResponse)> {
        let eps_mode = mode as u8;
        println!("Setting MPPT mode to mode{}", eps_mode);

        let result = run!(self.eps.lock().unwrap().set_input_mode(eps_mode), self.errors);

        Ok(GenericResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }

    /// Set Heater on/off
    pub fn eps_set_heater(&self, cmd:i32,heater:i32,mode:i32) ->  EpsResult<(GenericResponse)> {
        let heater_cmd = cmd as u8;
        let heater_select = heater as u8;
        let heater_mode = mode as u8;
        println!("Setting Heater {} to {}; mode{}.", heater_select, heater_cmd, heater_mode);

        let result = run!(self.eps.lock().unwrap().set_heater(heater_cmd,heater_select,heater_mode), self.errors);

        Ok(GenericResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }

    /// set_output: This function has been skipped for now. 

    //watchdog_kick
    pub fn eps_watchdog_kick(&self) -> EpsResult<GenericResponse> {
        let result = run!(self.eps.lock().unwrap().watchdog_kick(), self.errors);

        Ok(GenericResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }

    // watchdog_start
    pub fn eps_watchdog_start(&self) -> EpsResult<GenericResponse> {
        let result = run!(self.eps.lock().unwrap().watchdog_start(), self.errors);

        Ok(GenericResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }
    // watchdog_stop
    pub fn eps_watchdog_stop(&self) -> EpsResult<GenericResponse> {
        let result = run!(self.eps.lock().unwrap().watchdog_stop(), self.errors);

        Ok(GenericResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }

    pub fn passthrough(&self, command: String, rx_len: i32) -> EpsResult<RawCommandResponse> {
    // Convert the hex values in the string into actual hex values
    // Ex. "c3c2" -> [0xc3, 0xc2]
    let tx: Vec<u8> = command
        .as_bytes()
        .chunks(2)
        .map(|chunk| u8::from_str_radix(str::from_utf8(chunk).unwrap(), 16).unwrap())
        .collect();

    let mut rx: Vec<u8> = vec![0; rx_len as usize];

    let result = run!(
        self.eps
            .lock()
            .unwrap()
            .passthrough(tx.as_slice(), rx.as_mut_slice()),
        self.errors
    );

    // Convert the response hex values into a String for the GraphQL output
    // Note: This is in BIG ENDIAN format
    Ok(match result {
        Ok(_) => RawCommandResponse {
            success: true,
            errors: "".to_owned(),
            response: rx
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<String>(),
        },
        Err(err) => RawCommandResponse {
            success: false,
            errors: err,
            response: "".to_owned(),
        },
    })
    }
}
