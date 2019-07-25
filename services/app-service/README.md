# Applications Service

The applications service is responsible for monitoring and managing all mission
applications for a system.

The service is capable of tracking multiple versions of each application, allowing
users to easily upgrade and rollback their mission applications when necessary.

Whenever a new application is registered with the service, its manifest file and all
other files in the specified directory are copied into the service’s application
registry.
By default, this registry is stored under `/home/system/kubos/apps`.

## Configuration

The service expects the following sections and settings to be present in the configuration file:

```toml
# GraphQL configuration
[app-service.addr]
# IP to bind GraphQL server to
ip = "127.0.0.1"
# Port to listen on for GraphQL queries
port = 8000

# General service configuration
[app-service]
registry-dir = "/home/system/kubos/apps"
```

## Running

This service expects a service configuration file in either the default location
or to be passed in at runtime like so `cargo run -- -c config.toml`.

## Schema

### Queries

```graphql
{
    ping: "pong",
    apps(name: String, version: String, active, Boolean) {
        [entry {
            active: Boolean!,
            app {
                name: String!,
                executable: String!,
                version: String!,
                author: String!
            }
        }]
    }
}
```

### Mutations

```graphql
mutation {
    register(path: String!) {
        success: Boolean!,
        errors: String!,
        entry {
            active: Boolean!,
            app {
                name: String!,
                executable: String!,
                version: String!,
                author: String!
            }
        }
    },
    uninstall(name: String!, version: String) {
        success: Boolean!,
        errors: String!,
    },
    setVersion(name: String!, version: String!) {
        success: Boolean!,
        errors: String!,
    },
    startApp(name: String!, runLevel: String!, config: String, args: [String] {
        success: Boolean!,
        errors: String!,
        pid: Int
    }
}
```
