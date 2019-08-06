# MAI-400 Service

Kubos Service for interacting with an [Adcole Maryland Aerospace MAI-400](https://www.adcolemai.com/adacs)

# Configuration

The service can be configured in the `/home/system/etc/config.toml` with the following fields:

```toml
[mai400-service.addr]
ip = "127.0.0.1"
port = 8082
```

Where `ip` specifies the service's IP address, and `port` specifies the port which UDP requests should be sent to.

# Starting the Service

The service should be started automatically by its init script, but may also be started manually:

```bash
$ mai400-service
Kubos MAI-400 service started
Listening on: 10.63.1.20:8082
```

If no config file is specified, then the service will look at `/home/system/etc/config.toml`.
An alternative config file may be specified on the command line at run time:

```bash
$ mai400-service -c config.toml
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

Get the current configuration of the system

```json
{
    config: "Not Implemented"
}
```

## Telemetry

Get current telemetry information for the system

```json
{
    telemetry{
        nominal{
            gpsTime: i32,
            timeSubsec: i32,
            cmdValidCntr: i32,
            cmdInvalidCntr: i32,
            cmdInvalidChksumCntr: i32,
            lastCommand: i32,
            acsMode: i32,
            css: [i32; 6],
            eclipseFlag: i32,
            sunVecB: [i32; 3],
            iBFieldMeas: [i32; 3],
            bd: [f64; 3],
            rwsSpeedCmd: [i32; 3],
            rwsSpeedTach: [i32; 3],
            rwaTorqueCmd: [f64; 3],
            gcRwaTorqueCmd: [i32; 3],
            torqueCoilCmd: [f64; 3],
            gcTorqueCoilCmd: [i32; 3],
            qboCmd: [i32; 4],
            qboHat: [i32; 4],
            angleToGo: f64,
            qError: [i32; 4],
            omegaB: [f64; 3],
            nb: [i32; 3],
            neci: [i32; 3],
        },
        debug{
            irehs{
                thermopilesA: [i32; 4],
                thermopilesB: [i32; 4],
                tempA: [i32; 4],
                tempB: [i32; 4],
                dipAngleA: i32,
                dipAngleB: i32,
                solutionDegraded: [Vec<String>; 8],
                thermopileStructA{
                    dipAngle: i32,
                    earthLimb: {
                        adc: i32,
                       temp: i32,
                       errors: bool,
                       flags: Vec<String>,
                       },
                    earthRef: {...},
                    spaceRef: {...},
                    wideFov: {...},
                thermopileStructB{...}
            },
            rawImu{
                accel: [i32; 3],
                gyro: [i32; 3],
                gyroTemp: i32,
            },
            rotating{
                bFieldIgrf: [f64; 3],
                sunVecEph: [f64; 3],
                scPosEci: [f64; 3],
                scVelEci: [f64; 3],
                keplerElem{
                    semiMajorAxis: f32,
                    eccentricity: f32,
                    inclination: f32,
                    raan: f32,
                    argParigee: f32,
                    trueAnomoly: f32,
                },
                kBdot: [f64; 3],
                kp: [f64; 3],
                kd: [f64; 3],
                kUnload: [f64; 3],
                cssBias: [i32; 6],
                magBias: [i32; 3],
                rwsVolt: i32,
                rwsPress: i32,
                attDetMode: i32,
                rwsResetCntr: [i32; 3],
                sunMagAligned: i32,
                minorVersion: i32,
                maiSn: i32,
                orbitPropMode: i32,
                acsOpMode: i32,
                procResetCntr: i32,
                majorVersion: i32,
                adsOpMode: i32,
                cssGain: [f64; 6],
                magGain: [f64; 3],
                orbitEpoch: i32,
                trueAnomolyEpoch: f64,
                orbitEpochNext: i32,
                scPosEciEpoch: [f64; 3],
                scVelEciEpoch: [f64; 3],
                qbXWheelSpeed: i32,
                qbXFilterGain: f64,
                qbXDipoleGain: f64,
                dipoleGain: [f64; 3],
                wheelSpeedBias: [i32; 3],
                cosSunMagAlignThresh: f64,
                unloadAngThresh: f64,
                qSat: f64,
                rwaTrqMax: f64,
                rwsMotorCurrent: [i32; 3],
                rwsMotorTemp: i32,
            }
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

## System Mode

Get the current mode of the system

```json
{
    mode: Mode
}
```

## System Orientation

Get the last reported orientation of the system

```json
{
    orientation: "Not Implemented"
}
```

## System Spin

Get the last reported spin values of the system
Note: The spin values are automatically updated every six seconds

```json
{
    spin{
        x: f64,
        y: f64,
        z: f64
    }
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

```json
mutation {
    configureHardware: "Not Implemented"
}
```

## System Self-Test

Run a system self-test

- test: Type of self-test to perform

```json
mutation {
    testHardware(test: TestType) {
        ... on IntegrationTestRsults {
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

- command: String containing the hex values to be sent (ex. "C3").
         It will be converted to a byte array before transfer.

```json
mutation {
    issueRawCommand(command: String) {
        errors: String,
        success: Boolean,
        response: String
    }
}
```

## Set ADC Mode

Set the attitude control mode

- mode: Control mode to change to
- qbiCmd: Optional array of four values needed for Qinertial and Qtable mode
- sunAngleEnable: Optional. Specifies whether the sun rotating angle should be updated when
                using Normal-Sun or LatLong-Sun mode
sunRotAngle: Optional. The sun rotating angle for use in Normal-Sun and LatLong-Sun mode

```json
mutation {
    setMode(mode: Mode, qbiCmd: Vec<i32>, sunAngleEnable: bool, sunRotAngle: f64) {
        errors: String,
        success: Boolean,
        response: String
    }
}
```

## Set System Values

Update system values

- gpsTime: Optional. If specified, updates the system's ADACS clock
- rv: Optional. If specified, updates the orbital position and velocity at epoch.
    The argument has the following sub-fields:
        - eciPos: Vector containing the new X, Y, and Z ECI position values
        - eciVel: Vector containing the new X, Y, and Z ECI velocity values
        - timeEpoch: GPS time at which the eciPos and eciVel values will go into effect

```json
mutation {
    update(gps_time: Option<i32>,
        rv: Option<{eciPos: [f64; 3], eciVel: [f64; 3], timeEpoch: i32}>) {
        errors: String,
        success: Boolean,
    }
}
```