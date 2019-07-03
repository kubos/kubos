# Serial Communications Service

This example communications service provides communications functionality over a
serial link and gives a template for how to structure a communications service project.

KISS framing is implemented to preserve message integrity over the serial link.

A more detailed tutorial on how to create a new communications service can be found
[here](https://docs.kubos.com/latest/tutorials/comms-service.html)

## Running

This service expects a service configuration file in either the default location
or to be passed in at runtime like so `cargo run -- -c config.toml`.

The service expects the following sections and settings to be present:

```toml
[serial-comms-service] -- Service specific configuration
bus = "/dev/ttyUSB0" -- Serial bus to use
[serial-comms-service.addr] -- GraphQL configuration
ip = "127.0.0.1" -- IP to bind GraphQL server to
port = 8012 -- Port to listen on for GraphQL queries
[serial-comms-service.comms] -- Communications service configuration
max_num_handlers = 10 -- Maximum number of concurrent message handlers
downlink_ports = [14011] -- Ports to listen for local traffic on
timeout = 1 -- Timeout when listening for packet response
ip = "0.0.0.0" -- IP to bind Communications service listener to
```

When the service has started correctly it will display output like so:

```
2019-01-18T13:00:32.512854973-06:00 INFO serial_comms_service - Serial Communications Service starting on /dev/ttyUSB0
2019-01-18T13:00:32.513053752-06:00 INFO comms_service::service - Communication service started
2019-01-18T13:00:32.513855-06:00 INFO kubos_service::service - Listening on: 127.0.0.1:8080
```
