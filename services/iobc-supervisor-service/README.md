# ISIS iOBC Supervisor Service

Kubos Service for interacting with the ISIS-OBC Supervisor

# Configuration

The service must be configured in `/home/system/etc/config.toml` with the following fields:

- `[iobc-supervisor-service.addr]`

    - `ip` - Specifies the service's IP address
    - `port` - Specifies the port on which the service will be listening for UDP packets

For example:

```toml
[iobc-supervisor-service.addr]
ip = "0.0.0.0"
port = 8006
```

# Starting the Service

The service should be started automatically by its init script, but may also be started manually:

```bash
$ iobc-supervisor-service
Kubos antenna systems service started
Listening on: 0.0.0.0:8006
```

If no config file is specified, then the service will look at `/home/system/etc/config.toml`.
An alternative config file may be specified on the command line at run time:

```bash
$ iobc-supervisor-service -c config.toml
```

# Available Fields

```json
query {
    ping: "pong",
    supervisor: {
        version: {
            dummy,
            spiCommandStatus,
            indexOfSubsystem,
            majorVersion,
            minorVersion,
            patchVersion,
            gitHeadVersion,
            serialNumber,
            compileInformation,
            clockSpeed,
            codeType,
            crc
        },
        housekeeping: {
            dummy,
            spiCommandStatus,
            enableStatus: {
                powerObc,
                powerRtc,
                supervisorMode,
                busyRtc,
                powerOffRtc
            },
            supervisorUptime,
            iobcUptime,
            iobcResetCount,
            adcData,
            adcUpdateFlag,
            crc8
        }
    }
}

mutation {
    reset,
    emergencyReset,
    powercycle
}
```