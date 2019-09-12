# Kubos UART Comms Service Client

Example client "radio" over UART program

Wraps the user's data in a Space Packet and then sends it over the UART port
The example communication service, `uart-comms-service`, should be running and listening for
these messages.
The service will forward the message on to the requested destination port and then return the
response once the request has completed.

Note: Currently this client can only be used to send/receive GraphQL requests

Packets can be additionally encapsulated using the KISS protocol to simulate additional
radio-specific framing. Resulting packet is `KISS<space-packet<graphql-payload>>`.

## Running the Client

To build and run the client program, run the following command from this folder:

    cargo run -- -b {bus} -p {port} [-f {file} | -d {data}] [-k]
    
Required arguments:

- `-b {bus}` - UART bus to communicate over
- `-p {port}` - Message destination port
- `-f {file}` or `-d {data]` - Message to transmit
    - `file` - File to read message data from
    - `data` - Raw string to send as message
    
Optional arguments:

- `-k` - Packets should be encased with KISS framing