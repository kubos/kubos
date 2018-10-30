extern crate chrono;
#[macro_use]
extern crate failure;
extern crate getopts;
#[macro_use]
extern crate kubos_app;

use chrono::Utc;
use failure::Error;
use getopts::Options;
use kubos_app::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;

struct MyApp;

const BOOTFILE: &str = "/home/system/var/onboot-output";
const COMMANDFILE: &str = "/home/system/var/oncommand-output";

macro_rules! log {
    ($log_file:ident, $msg:expr) => {{
        writeln!($log_file, "{}: {}", Utc::now(), $msg).unwrap();
    }};
    ($log_file:ident, $msg:expr, $($arg:tt)*) => {{
        let message = format!($msg, $($arg)*);
        writeln!($log_file, "{}: {}", Utc::now(), message).unwrap();
    }};
}

impl AppHandler for MyApp {
    fn on_boot(&self, _args: Vec<String>) -> Result<(), Error> {
        let monitor_service = ServiceConfig::new("monitor-service");
        let telemetry_service = ServiceConfig::new("telemetry-service");

        loop {
            thread::sleep(Duration::from_secs(3));
            // Set up the log file
            let mut log_file = OpenOptions::new().create(true).append(true).open(BOOTFILE)?;
            log!(log_file, "OnBoot logic called");

            // Get the amount of memory currently available on the OBC
            let request = "{memInfo{available}}";
            let response = match query(
                monitor_service.clone(),
                request,
                Some(Duration::from_secs(1)),
            ) {
                Ok(msg) => msg,
                Err(err) => {
                    log!(log_file, "Monitor service query failed: {}", err);
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
                            log!(log_file, "Current memory value saved to database");
                        } else {
                            match msg.get("errors") {
                                Some(errors) => log!(log_file,
                                    "Failed to save value to database: {}",
                                    errors
                                ),
                                None => log!(log_file, "Failed to save value to database"),
                            };
                        }
                    }
                    Err(err) => {
                        log!(log_file, "Monitor service mutation failed: {}", err);
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

        // Set up the log file
        let mut log_file = OpenOptions::new().create(true).append(true).open(COMMANDFILE)?;

        log!(log_file, "OnCommand logic called");

        let subcommand = matches.opt_str("s").unwrap_or("".to_owned());

        match subcommand.as_ref() {
            "safemode" => {
                let time: u64 = match matches.opt_get("t") {
                    Ok(Some(val)) => val,
                    _ => {
                        log!(log_file, "Command Integer must be positive and non-zero");
                        bail!("Command Integer must be positive and non-zero");
                    }
                };

                log!(log_file, "Going into safemode for {} seconds", time);
                thread::sleep(Duration::from_secs(time));
                log!(log_file, "Resuming normal operations");
            }
            _ => {
                // Get a list of all the currently registered applications
                log!(log_file, "Querying for active applications");
                
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
                    Ok(msg) => log!(log_file, "App query result: {:?}", msg),
                    Err(err) => {
                        log!(log_file, "App service query failed: {}", err);
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
