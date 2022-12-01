use failure::{bail, Error};
use getopts::Options;
use kubos_app::*;
use log::*;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Error> {
    logging_setup!("rust-mission-app")?;
    let args: Vec<String> = ::std::env::args().collect();
    let mut opts = Options::new();

    // Standard app args:
    // This option will be processed by the system-api crate when a service query is run
    opts.optflagopt(
        "c",
        "config",
        "System config file which should be used",
        "CONFIG",
    );
    opts.optflag("h", "help", "Print this help menu");
    // App-specific args:
    opts.optflagopt("s", "cmd_string", "Subcommand", "CMD_STR");
    opts.optflagopt("t", "cmd_sleep", "Safe-mode sleep time", "CMD_INT");

    // Parse the command args
    let matches = match opts.parse(args) {
        Ok(r) => r,
        Err(f) => panic!("{}", f.to_string()),
    };

    // Check for subcommand to run
    if let Some(subcommand) = matches.opt_str("s") {
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
                            name,
                            version,
                            author
                        }
                    }
                }"#;

                match query(
                    &ServiceConfig::new("app-service")?,
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
    } else {
        // If there's no subcommand, we'll just go ahead and collect telemetry
        let monitor_service = ServiceConfig::new("monitor-service")?;
        let telemetry_service = ServiceConfig::new("telemetry-service")?;

        // Get the amount of memory currently available on the OBC
        let request = "{memInfo{available}}";
        let response = match query(&monitor_service, request, Some(Duration::from_secs(1))) {
            Ok(msg) => msg,
            Err(err) => {
                error!("Monitor service query failed: {}", err);
                bail!("Monitor service query failed: {}", err);
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

            match query(&telemetry_service, &request, Some(Duration::from_secs(1))) {
                Ok(msg) => {
                    let success = msg
                        .get("insert")
                        .and_then(|data| data.get("success").and_then(|val| val.as_bool()));

                    if success == Some(true) {
                        info!("Current memory value saved to database");
                    } else {
                        match msg.get("errors") {
                            Some(errors) => {
                                error!("Failed to save value to database: {}", errors);
                                bail!("Failed to save value to database: {}", errors);
                            }
                            None => {
                                error!("Failed to save value to database");
                                bail!("Failed to save value to database");
                            }
                        };
                    }
                }
                Err(err) => {
                    error!("Telemetry service mutation failed: {}", err);
                    bail!("Telemetry service mutation failed: {}", err);
                }
            }
        }
    }

    Ok(())
}
