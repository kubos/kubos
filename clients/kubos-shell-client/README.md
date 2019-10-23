# Kubos Shell Client

This client program can be used to test communication with the Kubos shell service.

## Running the Client

To build and run the client program, run the following command from this folder:

    cargo run --bin kubos-shell-client -- (start|run|list|join|kill) [config-options]
    
Required arguments:

- Operation to perform
    - `start` - Start a new shell session
    - `run -c "{command}"` - Runs a single remote command
        - `command` - Remote command and any required arguments. *Must* be enclosed in `"`s.
    - `list` - List existing shell sessions
    - `join -c {channel-id}` - Join an existing shell session
        - `channel-id` - Channel ID of existing shell session
    - `kill -c {channel-id} [-s {signal}]` - Kill an existing shell session
        - `channel-id` - Channel ID of existing shell session
        - `signal` - Signal number to be passed when killing the session

Optional arguments:

- `-i {remote IP}` - Default: `0.0.0.0`. IP address of the shell service to connect to.
- `-p {remote port}` - Default: `8050`. UDP port of the shell service to connect to.