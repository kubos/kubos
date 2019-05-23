# Local Communications Service

This communications service provides communications functionality over a local
network. This service could be used to communicate with services which are running
locally, or with a system which is accessible via the local network interfaces.

## Running

This service expects a service configuration file in either the default location
or to be passed in at runtime like so `cargo run -- -c config.toml`.

The service expects the following sections and settings to be present:

```toml
# Service specific configuration
[local-comms-service]
# IP address the gateway is bound to
gateway_ip = "127.0.0.1"
# Port the gateway is listening on
gateway_port = 13001
# IP to expect gateway communications on
listening_ip = "127.0.0.1"
# Port to listen on for gateway messages
listening_port = 13002

# GraphQL configuration
[local-comms-service.addr]
# IP to bind GraphQL server to
ip = "127.0.0.1"
# Port to listen on for GraphQL queries
port = 8012

# Communications service configuration
[local-comms-service.comms]
# Maximum number of concurrent message handlers
max_num_handlers = 10
# Ports to listen for local traffic on
downlink_ports = [14011]
#  Timeout when listening for packet response (milliseconds)
timeout = 1
# IP to bind Communications service listener to
ip = "127.0.0.1"
```

When the service has started correctly it will display output like so:

```
2019-05-23T09:46:10.216941589-05:00 INFO comms_service::service - Communication service started
```
