# Telemetry Database Service

Kubos Service for interacting with the telemetry database

# Configuration

The service can be configured in the `/home/system/etc/config.toml` with the following fields:

```
[telemetry-service]
database = "/var/lib/telemetry.db"

[telemetry-service.addr]
ip = "127.0.0.1"
port = 8089
```

Where `database` specifies the path to the telemetry database file, `ip` specifies the
service's IP address, and `port` specifies the port on which the service will be
listening for UDP packets.

# Starting the Service

The service should be started automatically by its init script, but may also be started manually:

```
$ telemetry-service
Listening on: 127.0.0.1:8089
```

If no config file is specified, then the service will look at `/home/system/etc/config.toml`.
An alternative config file may be specified on the command line at run time:

```bash
$ telemetry-service -c config.toml
```

# Panics

Attempts to grab database path from Configuration and will `panic!` if not found.
Attempts to connect to database at provided path and will `panic!` if connection fails.
Attempts to create telemetry table and will `panic!` if table creation fails.

# GraphQL Schema

```graphql
type Entry {
  timestamp: Integer!
  subsystem: String!
  parameter: String!
  value: Float!
}

query ping: "pong"
query telemetry(timestampGe: Integer, timestampLe: Integer, subsystem: String, parameter: String, parameters: [String]): Entry
query routedTelemetry(timestampGe: Integer, timestampLe: Integer, subsystem: String, parameter: String, parameters: [String], output: String!, compress: Boolean = true): String!

mutation insert(timestamp: Integer, subsystem: String!, parameter: String!, value: String!):{ success: Boolean!, errors: String! }
```

# Example Queries

## Select all attributes of all telemetry entries
```graphql
{
  telemetry {
    timestamp,
    subsystem,
    parameter,
    value
  }
}
```

## Select all attributes of all telemetry entries for the eps subsystem
```graphql
{
  telemetry(subsystem: "eps") {
    timestamp,
    subsystem,
    parameter,
    value
  }
}
```

## Select all attributes of all telemetry entries for the voltage parameter of the eps subsystem
```graphql
{
  telemetry(subsystem: "eps", parameter: "voltage") {
    timestamp,
    subsystem,
    parameter,
    value
  }
}
```

## Select all attributes of all telemetry entries for the voltage and current parameters of the eps subsystem
```graphql
{
  telemetry(subsystem: "eps", parameters: ["voltage", "current"]) {
    timestamp,
    subsystem,
    parameter,
    value
  }
}
```

## Select all attributes of all telemetry entries occurring between the timestamps 100 and 200
```graphql
{
  telemetry(timestampGe: 101, timestampLe: 199) {
    timestamp,
    subsystem,
    parameter,
    value
  }
}
```

## Select all attributes of all telemetry entries occurring at the timestamp 101
```graphql
{
  telemetry(timestampGe: 101, timestampLe: 101) {
    timestamp,
    subsystem,
    parameter,
    value
  }
}
```

## Select ten entries occurring on or after the timestamp 1008
```graphql
{
  telemetry(limit: 10, timestampGe: 1008) {
    timestamp,
    subsystem,
    parameter,
    value
  }
}
```

## Repeat the previous query, but route the output to compressed file `/home/system/recent_telem.tar.gz`
```graphql
{
  telemetry(limit: 10, timestampGe: 1008, output: "/home/system/recent_telem")
}
```

## Repeat the previous query, but route the output to uncompressed file `/home/system/recent_telem`
```graphql
{
  telemetry(limit: 10, timestampGe: 1008, output: "/home/system/recent_telem", compress: false)
}
```

# Example Mutations

## Insert a new entry, allowing the service to generate the timestamp
```graphql
mutation {
    insert(subsystem: "eps", parameter: "voltage", value: "4.0") {
        success,
        errors
    }
}
```

## Insert a new entry with a custom timestamp
```graphql
mutation {
    insert(timestamp: 533, subsystem: "eps", parameter: "voltage", value: "5.1") {
        success,
        errors
    }
}

```

## Delete all entries from the EPS subsystem occuring before timestamp 1003
```graphql
mutation {
    delete(subsystem: "eps", timestampLe: 1004) {
        success,
        errors,
        entriesDeleted
    }
}
```