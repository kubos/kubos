# Rust Service Example

This is an example of a subsystem service implemented in Rust.

This service should be started with the included `config.toml` file, like so:

    $ cargo run --  -c config.toml

This service listens on http://0.0.0.0:8123/graphql for
graphql queries and mutations.
Queries are requests for state information (telemetry).
Mutations are equivalent to commands.

There is also a graphiql interface at http://0.0.0.0:8123/graphiql
for ease of development.

## Schema

The service has the following schema:

### Queries

    {
        subsystem {
            power: Boolean!,
            uptime: Int!,
            temperature: Int!
        }
    }
    
### Mutations

    {
        SetPower(power: Boolean!) {
            power: Boolean!
        },
        ResetUptime {
            uptime: Int!
        },
        CalibrateThermometer {
            temperature: Int!
        }
    }