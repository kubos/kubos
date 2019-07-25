# Rust Application API

The Rust application API is meant to simplify development of KubOS mission applications in Rust.

It provides the following functionality:

- Initializes logging
- Parses and triggers desired run level
- Provides a helper function for sending GraphQL requests to services

# Examples

```
use failure::{bail, Error};
use kubos_app::*;
use std::time::Duration;

struct MyApp;

impl AppHandler for MyApp {
  fn on_boot(&self, _args: Vec<String>) -> Result<(), Error> {
    println!("OnBoot logic");

    let request = r#"mutation {
            power(state: ON) {
                success
            }
        }"#;

    match query(&ServiceConfig::new("radio-service")?, request, Some(Duration::from_secs(1))) {
        Err(error) => bail!("Failed to communicate with radio service: {}", error),
        Ok(data) => {
            if let Some(success) = data.get("power")
                .and_then(|power| power.get("success"))
            {
                match success.as_bool() {
                    Some(true) => println!("Successfully turned on radio"),
                    Some(false) => eprintln!("Failed to turn on radio"),
                    None => eprintln!("Failed to fetch radio power state")
                }
            } else {
                bail!("Failed to fetch radio power state");
            }
        }
    }

    Ok(())
  }
  fn on_command(&self, _args: Vec<String>) -> Result<(), Error> {
    println!("OnCommand logic");
    Ok(())
  }
}

fn main() -> Result<(), Error> {
    let app = MyApp { };
    app_main!(&app)?;
    Ok(())
}
```