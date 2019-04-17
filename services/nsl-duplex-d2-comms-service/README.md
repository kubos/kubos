# NSL Duplex Communications Service

This communications service provides communications functionality over the NSL
Duplex radio.

## Running 

This service expects a service configuration file in either the default location
or to be passed in at runtime like so `cargo run -- -c config.toml`.

The service expects the following sections and settings to be present:

```toml
[nsl-duplex-comms-service] -- Service specific configuration
bus = "/dev/ttyUSB0" -- Serial bus to use for talking with the radio
[nsl-duplex-comms-service.addr] -- GraphQL configuration
ip = "127.0.0.1" -- IP to bind GraphQL server to
port = 8012 -- Port to listen on for GraphQL queries
[nsl-duplex-comms-service.comms] -- Communications service configuration
max_num_handlers = 10 -- Maximum number of concurrent message handlers
downlink_ports = [14011] -- Ports to listen for local traffic on
timeout = 1 -- Timeout when listening for packet response
ground_port = 14020 -- Port to send ground communications on
ground_ip = "192.168.0.1" -- IP to expect ground communications from
satellite_ip = "0.0.0.0" -- IP to bind Communications service listener to
```

When the service has started correctly it will display output like so:

```
2019-01-18T13:00:32.512854973-06:00 INFO  nsl_dupex_d2_comms_service - NSL Duplex Communications Service starting on /dev/ttyUSB0
2019-01-18T13:00:32.513053752-06:00 INFO comms_service::service - Communication service started
2019-01-18T13:00:32.513855-06:00 INFO kubos_service::service - Listening on: 127.0.0.1:8080
```
