# ISIS Antenna Systems Service

Kubos Service for interacting with an [ISIS Antenna System](https://www.isispace.nl/product-category/products/antenna-systems/)

# Configuration

The service must be configured in `/home/system/etc/config.toml` with the following fields:

- `[isis-ants-service.addr]`

    - `ip` - Specifies the service's IP address
    - `port` - Specifies the port on which the service will be listening for UDP packets

- `[isis-ants-service]`

    - `bus` - Specifies the I2C bus the antenna system is connected to
    - `primary` - Specifies the I2C address of the antenna system's primary microcontroller
    - `secondary` - Specifies the I2C address of the secondary microcontroller. If no secondary contoller is present, this value should be `"0x00"`.
    - `antennas` - Specifies the number of antennas present in the system. Expected value: 2 or 4.
    - `wd_timeout` - Specifies the interval at which the AntS watchdog should be automatically kicked. To disable automatic kicking, this value should be `0`.

For example:

```toml
[isis-ants-service.addr]
ip = "0.0.0.0"
port = 8006

[isis-ants-service]
bus = "/dev/i2c-0"
primary = "0x31"
secondary = "0x32"
antennas = 4
wd_timeout = 10
```

# Starting the Service

The service should be started automatically by its init script, but may also be started manually:

```bash
$ isis-ants-service
Kubos antenna systems service started
Listening on: 0.0.0.0:8006
```

If no config file is specified, then the service will look at `/home/system/etc/config.toml`.
An alternative config file may be specified on the command line at run time:

```bash
$ isis-ants-service -c config.toml
```

# Queries

## Ping

Test query to verify service is running without attempting
to communicate with the underlying subsystem

```json
{
    ping: "pong"
}
```

## ACK

Get the last run mutation

```json
{
    ack: AckCommand
}
```

## Errors

Get all errors encountered since the last time this field was queried

```json
{
    errors: [String]
}
```

## Power Status

Get the current power state and uptime of the system

```json
{
    power {
        state: PowerState,
        uptime: Int
    }
}
```

## Configuration

Get the current microcontroller which commands will be sent to

```json
{
    config: ConfigureController
}
```

## Telemetry

Get current telemetry information for the system

```json
{
    telemetry {
        nominal {
            rawTemp: Int,
            uptime: Int,
            sysBurnActive: Boolean,
            sysIgnoreDeploy: Boolean,
            sysArmed: Boolean,
            ant1NotDeployed: Boolean,
            ant1StoppedTime: Boolean,
            ant1Active: Boolean,
            ant2NotDeployed: Boolean,
            ant2StoppedTime: Boolean,
            ant2Active: Boolean,
            ant3NotDeployed: Boolean,
            ant3StoppedTime: Boolean,
            ant3Active: Boolean,
            ant4NotDeployed: Boolean,
            ant4StoppedTime: Boolean,
            ant4Active: Boolean
        },
           debug {
            ant1ActivationCount: Int,
            ant1ActivationTime: Int,
            ant2ActivationCount: Int,
            ant2ActivationTime: Int,
            ant3ActivationCount: Int,
            ant3ActivationTime: Int,
            ant4ActivationCount: Int,
            ant4ActivationTime: Int,
        }
    }
}
```

## Test Results

Get the test results of the last run test

Note: For this service, this actually just fetches the nominal
and debug telemetry of the system, since there is no actual
built-in test

```json
{
    testResults{
        success,
        telemetryNominal{...},
        telemetryDebug{...}
    }
}
```

## System Armed Status

Get the current armed/disarmed status of the system

```json
{
    armStatus: ArmStatus
}
```

## System deployment status

Get the current deployment status of the system

```json
{
    deploymentStatus {
        status: DeploymentStatus,
        sysBurnActive: Boolean,
        sysIgnoreDeploy: Boolean,
        sysArmed: Boolean,
        ant1NotDeployed: Boolean,
        ant1StoppedTime: Boolean,
        ant1Active: Boolean,
        ant2NotDeployed: Boolean,
        ant2StoppedTime: Boolean,
        ant2Active: Boolean,
        ant3NotDeployed: Boolean,
        ant3StoppedTime: Boolean,
        ant3Active: Boolean,
        ant4NotDeployed: Boolean,
        ant4StoppedTime: Boolean,
        ant4Active: Boolean
}
```

# Mutations


## Errors

Get all errors encountered while processing this GraphQL request

Note: This will only return errors thrown by fields which have
already been processed, so it is recommended that this field be specified last.

```json
mutation {
    errors: [String]
}
```

## No-Op

Execute a trivial command against the system

```json
mutation {
    noop {
        errors: String,
        success: Boolean
   }
}
```

## Set Power State

Control the power state of the system

- state: Power state the system should be changed to
  Note: The only valid input for this service is `RESET`

```json
mutation {
    controlPower(state: PowerState) {
        errors: String,
        success: Boolean,
        power: PowerState
    }
}
```

## Configuration

Configure the system

- config: Set which microcontroller future commands should be issued from

```json
mutation {
    configureHardware(config: ConfigureController) {
        errors: String,
        success: Boolean,
        config: ConfigureController
   }
}
```

## System Self-Test

Run a system self-test

- test: Type of self-test to perform

```json
mutation {
    testHardware(test: TestType) {
        ... on IntegrationTestResults {
            errors: String,
            success: Boolean,
            telemetryNominal{...},
            telemetryDebug{...}
        }
        ... on HardwareTestResults {
            errors: "Not Implemented",
            success: true,
            data: Empty
        }
   }
}
```

## Passthrough

Pass a custom command through to the system

- command: String containing the hex values to be sent (ex. "C3")
  It will be converted to a byte array before transfer.
- rxLen: Number of response bytes to read

```json
mutation {
    issueRawCommand(command: String, rx_len: Int) {
        errors: String,
        success: Boolean,
        response: String
    }
}
```

## Arm/Disarm

Arm/Disarm the system

- state: Armed/Disarmed state the system should be changed to

```json
mutation {
    arm(state: ArmState) {
        errors: String,
        success: Boolean
   }
```

## Deploy Antennas

Deploy antenna/s

- ant: (Default - All) Antenna to deploy
- force: (Default - false) Whether current deployment state should be ignored/overridden
- time: Maximum amount of time to spend attempting to deploy the antenna
  (for 'All', this is the amount of time spent for each antenna)

```json
mutation {
    deploy(ant: DeployType = DeployType::All, force: Boolean = false, time: Int) {
        errors: String,
        success: Boolean
   }
}
```