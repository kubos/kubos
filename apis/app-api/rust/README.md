# Rust Application API

The Rust application API is meant to simplify development of KubOS mission applications in Rust.

It provides the following functionality:

- Initializes logging
- Provides a helper function for sending GraphQL requests to services

# Examples

```
use failure::{bail, Error};
use kubos_app::*;
use log::*;

fn main() -> Result<(), Error> {
    // Initialize logging
    logging_setup!("my-rust-app");
    
    // GraphQL request to turn on the radio
    let request = r#"mutation {
            power(state: ON) {
                success
            }
        }"#;

    // Send the GraphQL request to the radio sevice
    match query(&ServiceConfig::new("radio-service")?, request, Some(Duration::from_secs(1))) {
        Err(error) => bail!("Failed to communicate with radio service: {}", error),
        Ok(data) => {
            // Parse the response to verify that the operation was
            // successful
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
```