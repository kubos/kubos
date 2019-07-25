# Shell Service

The shell service is used to provide shell access and commanding from mission
operations to the OBC.
It may also be used between a developer’s system and the OBC when in a development or
testing environment.

The shell service provides shell functionality by implementing the shell protocol.
The shell protocol is UDP-based which means a connection is required between the OBC
and ground segment capable of transferring UDP packets.
This should be established using a standard network connection or by using an
instance of the communications service.

# Configuration

The service can be configured in the `/home/system/etc/config.toml` with the following fields:

```toml
[shell-service.addr]
# IP to bind GraphQL server to
ip = "127.0.0.1"
# Port to listen on for GraphQL queries
port = 8090
```

# Running the Service

The service should be started automatically by its init script, but may also be started manually:

```bash
$ shell-service
Listening on: 127.0.0.1:8089
```

If no config file is specified, then the service will look at `/home/system/etc/config.toml`.
An alternative config file may be specified on the command line at run time:

```bash
$ shell-service -c config.toml
```