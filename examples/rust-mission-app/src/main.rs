#[macro_use]
extern crate failure;
extern crate getopts;
extern crate kubos_app;
#[macro_use]
extern crate log;

use failure::Error;
use getopts::Options;
use kubos_app::*;

use std::thread;
use std::time::Duration;

struct MyApp;

impl AppHandler for MyApp {
    fn on_boot(&self, _args: Vec<String>) -> Result<(), Error> {
        let monitor_service = ServiceConfig::new("monitor-service");
        let telemetry_service = ServiceConfig::new("telemetry-service");

        loop {
            thread::sleep(Duration::from_secs(3));
            info!("OnBoot logic called");

            // Get the amount of memory currently available on the OBC
            let request = "{memInfo{available}}";
            let response = match query(
                monitor_service.clone(),
                request,
                Some(Duration::from_secs(1)),
            ) {
                Ok(msg) => msg,
                Err(err) => {
                    info!("Monitor service query failed: {}", err);
                    continue;
                }
            };

            let memory = response.get("memInfo").and_then(|msg| msg.get("available"));

            // Save the amount to the telemetry database
            if let Some(mem) = memory {
                let request = format!(
                    r#"
                    mutation {{
                        insert(subsystem: "OBC", parameter: "available_mem", value: "{}") {{
                            success,
                            errors
                        }}
                    }}
                "#,
                    mem
                );

                match query(
                    telemetry_service.clone(),
                    &request,
                    Some(Duration::from_secs(1)),
                ) {
                    Ok(msg) => {
                        let success = msg.get("insert").and_then(|data| {
                            data.get("success").and_then(|val| {
                                val.as_bool()
                            })
                        });

                        if success == Some(true) {
                            info!("Current memory value saved to database");
                        } else {
                            match msg.get("errors") {
                                Some(errors) => info!(
                                    "Failed to save value to database: {}",
                                    errors
                                ),
                                None => info!("Failed to save value to database"),
                            };
                        }
                    }
                    Err(err) => {
                        info!("Monitor service mutation failed: {}", err);
                        continue;
                    }
                }
            }
        }
    }

    fn on_command(&self, args: Vec<String>) -> Result<(), Error> {
        let mut opts = Options::new();

        opts.optflagopt("r", "run", "Run level which should be executed", "RUN_LEVEL");
        opts.optflagopt("s", "cmd_string", "Subcommand", "CMD_STR");
        opts.optflagopt("t", "cmd_sleep", "Safe-mode sleep time", "CMD_INT");

        // Parse the command args (skip the first arg with the application name)
        let matches = match opts.parse(&args[1..]) {
            Ok(r) => r,
            Err(f) => panic!(f.to_string()),
        };

        info!("OnCommand logic called");

        let subcommand = matches.opt_str("s").unwrap_or("".to_owned());

        match subcommand.as_ref() {
            "safemode" => {
                let time: u64 = match matches.opt_get("t") {
                    Ok(Some(val)) => val,
                    _ => {
                        info!("Command Integer must be positive and non-zero");
                        bail!("Command Integer must be positive and non-zero");
                    }
                };

                info!("Going into safemode for {} seconds", time);
                thread::sleep(Duration::from_secs(time));
                info!("Resuming normal operations");
            }
            _ => {
                // Get a list of all the currently registered applications
                info!("Querying for active applications");
                
                let request = r#"{
                    apps {
                        active,
                        app {
                            uuid,
                            name,
                            version,
                            author
                        }
                    }
                }"#;
                
                match query(
                    ServiceConfig::new("app-service"),
                    request,
                    Some(Duration::from_secs(1)),
                ) {
                    Ok(msg) => info!("App query result: {:?}", msg),
                    Err(err) => {
                        info!("App service query failed: {}", err);
                        bail!("App service query failed: {}", err)
                    }
                }
            }
        }
        
        Ok(())
    }
}

fn main() -> Result<(), Error> {    
    let app = MyApp;
    app_main!(&app)?;
    
    Ok(())
}
