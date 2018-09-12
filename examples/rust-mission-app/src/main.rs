extern crate getopts;
#[macro_use]
extern crate kubos_app;

use getopts::Options;
use kubos_app::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

struct MyApp;

impl AppHandler for MyApp {
    fn on_boot(&self, args: Vec<String>) {
        let mut opts = Options::new();
        opts.optflag("v", "verbose", "Enable verbose output");
        opts.optflagopt("l", "log", "Log file path", "LOG_FILE");

        // Parse the command args (skip the first arg with the application name)
        let matches = match opts.parse(&args[1..]) {
            Ok(r) => r,
            Err(f) => panic!(f.to_string()),
        };

        // Get the path to use for logging
        let log_path = matches
            .opt_str("l")
            .unwrap_or("/home/kubos/test-output".to_owned());
        println!("Log file set to: {}", log_path);

        // Set up the log file
        let mut log_file = OpenOptions::new().append(true).open(log_path).unwrap();

        writeln!(log_file, "OnBoot logic called").unwrap();

        // Check for our other command line argument
        if matches.opt_present("v") {
            writeln!(log_file, "Verbose output enabled").unwrap();
        }
    }
    fn on_command(&self, _args: Vec<String>) {
        fs::write("/home/kubos/test-output", "OnCommand logic\r\n").unwrap();
    }
}

fn main() {
    let app = MyApp;
    app_main!(&app);
}
