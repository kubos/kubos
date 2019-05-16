use failure::{bail, Error};
use getopts::Options;
use kubos_app::*;

struct MyApp;

impl AppHandler for MyApp {
    fn on_boot(&self, _args: Vec<String>) -> Result<(), Error> {
        Ok(())
    }

    fn on_command(&self, args: Vec<String>) -> Result<(), Error> {
        if args.is_empty() {
            bail!("No args given");
        }

        let mut opts = Options::new();
        opts.optflag("f", "", "Test flag");
        opts.optopt("t", "test", "Test arg", "TEST");
        opts.optflag("h", "help", "Print this help menu");

        let matches = match opts.parse(&args) {
            Ok(r) => r,
            Err(f) => panic!(f.to_string()),
        };

        let mut success = matches.opt_present("f");

        if matches.opt_str("t") == Some("test".to_owned()) {
            success = true;
        }

        // Check for a positional arg
        if !matches.free.is_empty() && matches.free[0] == "pos" {
            success = true;
        }

        if success {
            Ok(())
        } else {
            bail!("Did not receive any valid arguments");
        }
    }
}

fn main() -> Result<(), Error> {
    let app = MyApp;
    app_main!(&app)?;

    Ok(())
}
