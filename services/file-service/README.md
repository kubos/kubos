# Kubos File Transfer Service

The file transfer service is used to transfer files between the mission operations
center and the OBC.
It may also be used to transfer files between a developer’s system and the OBC when
in a development environment.

# Configuration

The service can be configured in the `/home/system/etc/config.toml` with the following fields:

```toml
[file-service]
# The directory which should be used for temporary storage of file
# chunks. Note: The directory will be created if it does not
# already exist.
storage_dir = "file-transfer"
# The length of time, in seconds, for which the service should wait
# for new messages from the client once a file protocol transaction
# has been started
timeout = 2
# Each file is broken up into equally sized chunks prior to
# transfer. This option specifies the size of those chunks in bytes.
chunk_size = 4096
# The number of times the protocol waits for a new message before
# ending the transaction.
hold_count = 5

[file-service.addr]
# IP to bind GraphQL server to
ip = "127.0.0.1"
# Port to listen on for GraphQL queries
port = 8090
```

# Running the Service

The service should be started automatically by its init script, but may also be started manually:

```bash
$ file-service
Listening on: 127.0.0.1:8089
```

If no config file is specified, then the service will look at `/home/system/etc/config.toml`.
An alternative config file may be specified on the command line at run time:

```bash
$ file-service -c config.toml
```