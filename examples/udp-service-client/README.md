# Example Kubos Service Client App

This application creates a connection with a Kubos service at a requested IP address, sends a GraphQL request, and then prints the response.

## Configuration

The app takes in a single argument specifying a configuration TOML file to be used to control the UDP connection and query to be sent.

The TOML file should have the following format:

```
query_file = "query.txt"

[host]
ip = "10.0.2.15"
port = 8000

[service]
ip = "10.63.1.20"
port = 8082
```

Where
    - `query_file` specifies the file containing the GraphQL request to be sent
    - `[host]` specifies the IP address and port the client should use to send and listen from
    - `[service]` specifies the IP address and port of the service to query
    
## Running

To build and run the application, run the following command from this folder:

`cargo run -- [path-to-config-toml]`

For example:

`cargo run -- config.toml`

## Example Files

This folder contains two example files: 
    - `query.txt` - Contains an example GraphQL query
    - `config.toml` - Contains an example configuration TOML file