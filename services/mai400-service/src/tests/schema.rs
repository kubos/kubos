/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::*;
use kubos_service::{Config, Service};
use model::*;
use schema::*;
use serde_json;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tests::test_data::*;

macro_rules! wrap {
    ($result:ident) => {{
        json!({
                                        "msg": $result,
                                        "errs": ""
                                }).to_string()
    }};
}

#[test]
fn ping() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"{
            ping
        }"#;

    let expected = json!({
            "ping": "pong"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn ack_default() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"{
            ack
        }"#;

    let expected = json!({
            "ack": "NONE"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn ack_noop() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let noop = r#"mutation {
            noop {
                success
            }
        }"#;

    service.process(noop.to_owned());

    let query = r#"{
            ack
        }"#;

    let expected = json!({
            "ack": "NOOP"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn ack_control_power() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let mutation = r#"mutation {
            controlPower(state: RESET) {
                success
            }
        }"#;

    service.process(mutation.to_owned());

    let query = r#"{
            ack
        }"#;

    let expected = json!({
            "ack": "CONTROL_POWER"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn ack_configure_hardware() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let mutation = r#"mutation {
            configureHardware
        }"#;

    service.process(mutation.to_owned());

    let query = r#"{
            ack
        }"#;

    let expected = json!({
            "ack": "CONFIGURE_HARDWARE"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn ack_test_hardware() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let mutation = r#"mutation {
            testHardware(test: INTEGRATION){
            ... on IntegrationTestResults{
                errors
            }}
        }"#;

    service.process(mutation.to_owned());

    let query = r#"{
            ack
        }"#;

    let expected = json!({
            "ack": "TEST_HARDWARE"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn ack_issue_raw_command() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let mutation = r#"mutation {
            issueRawCommand(command: "90EB5AD501"){
                errors,
                success
            }
        }"#;

    service.process(mutation.to_owned());

    let query = r#"{
            ack
        }"#;

    let expected = json!({
            "ack": "ISSUE_RAW_COMMAND"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn ack_set_mode() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let mutation = r#"mutation {
            setMode(mode: RATE_NULLING) {
                errors,
                success
            }
        }"#;

    service.process(mutation.to_owned());

    let query = r#"{
            ack
        }"#;

    let expected = json!({
            "ack": "SET_MODE"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn ack_update() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let mutation = r#"mutation {
            update(gpsTime: 1198800018) {
                errors,
                success
            }
        }"#;

    service.process(mutation.to_owned());

    let query = r#"{
            ack
        }"#;

    let expected = json!({
            "ack": "UPDATE"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn query_errors_empty() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"{
            errors
        }"#;

    let expected = json!({
            "errors": []
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn query_errors_single() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let noop = r#"mutation {
            noop {
                success
            }
        }"#;

    service.process(noop.to_owned());

    let query = r#"{
            errors
        }"#;

    let expected = json!({
            "errors": ["Noop: Unable to communicate with MAI400"]
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn query_errors_multiple() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let noop = r#"mutation {
            noop {
                success
            }
        }"#;

    service.process(noop.to_owned());
    service.process(noop.to_owned());

    let query = r#"{
            errors
        }"#;

    let expected = json!({
            "errors": ["Noop: Unable to communicate with MAI400", "Noop: Unable to communicate with MAI400"]
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn query_errors_clear_after_query() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let noop = r#"mutation {
            noop {
                success
            }
        }"#;

    service.process(noop.to_owned());
    service.process(noop.to_owned());

    let query = r#"{
            errors
        }"#;

    service.process(query.to_owned());

    let expected = json!({
            "errors": []
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn get_power_on() {
    let mock = MockStream::default();

    let data = Arc::new(ReadData {
        std_telem: Mutex::new(STD),
        irehs_telem: Mutex::new(IREHS),
        imu: Mutex::new(IMU),
        rotating: Mutex::new(ROTATING),
    });

    let data_ref = data.clone();

    let service = service_new!(mock, data);

    thread::spawn(move || loop {
        {
            let mut local = data_ref.std_telem.lock().unwrap();
            local.tlm_counter += 1;
        }
        thread::sleep(Duration::from_millis(100));
    });

    let query = r#"{
            power{
                state,
                uptime
            }
        }"#;

    let expected = json!({
            "power": {
                "state": "ON",
                "uptime": 2
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn get_power_off() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"{
            power{
                state,
                uptime
            }
        }"#;

    let expected = json!({
            "power": {
                "state": "OFF",
                "uptime": 0
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn config() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"{
            config
        }"#;

    let expected = json!({
            "config": "Not Implemented"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

// telemetry: nominal, debug {rotating, irehs, imu}
#[test]
fn telemetry_nominal() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"{
            telemetry{
                nominal{
                    acsMode,
                    angleToGo,
                    bd,
                    cmdInvalidChksumCntr,
                    cmdInvalidCntr,
                    cmdValidCntr,
                    css,
                    eclipseFlag,
                    gcRwaTorqueCmd,
                    gcTorqueCoilCmd,
                    gpsTime,
                    iBFieldMeas,
                    lastCommand,
                    nb,
                    neci
                    omegaB,
                    qError,
                    qboCmd,
                    qboHat,
                    rwaTorqueCmd,
                    rwsSpeedCmd,
                    rwsSpeedTach,
                    sunVecB,
                    timeSubsec,
                    torqueCoilCmd,
                }
            }
        }"#;

    let expected = json!({
            "telemetry": {
                "nominal": {
                    "acsMode": "RATE_NULLING",
                    "angleToGo": 0.0,
                    "bd": [-0.0000017433042103220942, 1.2922178882490699e-7 ,-9.995264917961322e-7],
                    "cmdInvalidChksumCntr": 0,
                    "cmdInvalidCntr": 0,
                    "cmdValidCntr": 2,
                    "css": [0, 4, 1, 0, 0, 4],
                    "eclipseFlag": 1,
                    "gcRwaTorqueCmd": [0, 0, 0],
                    "gcTorqueCoilCmd": [127, -15, 118],
                    "gpsTime": 1198800019,
                    "iBFieldMeas": [-1369, 105, -785],
                    "lastCommand": 0x44,
                    "nb": [31769, 31769, 31769],
                    "neci": [-32754, -627, -627],
                    "omegaB": [0.0, 0.0, 0.0],
                    "qError": [0, 0, 0, 32767],
                    "qboCmd": [0, 0, 0, 32767],
                    "qboHat": [0, 0, 0, 32767],
                    "rwaTorqueCmd": [0.0, 0.0, 0.0],
                    "rwsSpeedCmd": [0, 0, 0],
                    "rwsSpeedTach": [0, 0, 0],
                    "sunVecB": [-32767, -32767, -32767],
                    "timeSubsec": 0,
                    "torqueCoilCmd": [0.1080000028014183, -0.012922178953886032, 0.09995264559984207]
                }
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn telemetry_debug_irehs() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"{
            telemetry{
                debug{
                    irehs{
                        dipAngleA,
                        dipAngleB,
                        solutionDegraded,
                        tempA,
                        tempB,
                        thermopileStructA{
                            dipAngle,
                            earthLimb{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                            earthRef{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                            spaceRef{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                            wideFov{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                        },
                        thermopileStructB{
                            dipAngle,
                            earthLimb{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                            earthRef{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                            spaceRef{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                            wideFov{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                        },
                        thermopilesA,
                        thermopilesB,
                    }
                }
            }
        }"#;

    let expected = json!({
            "telemetry": {
                "debug": {
                    "irehs": {
                        "dipAngleA": 0,
                        "dipAngleB": 0,
                        "solutionDegraded": [
                            ["NO_COMM"],
                            ["NO_COMM"],
                            ["DIP_ANGLE_LIMIT"],
                            ["DIP_ANGLE_LIMIT"],
                            ["NO_COMM"],
                            ["NO_COMM"],
                            ["DIP_ANGLE_LIMIT"],
                            ["DIP_ANGLE_LIMIT"],
                        ],
                        "tempA": [0, 0, 0, 0],
                        "tempB": [0, 0, 0, 0],
                        "thermopileStructA": {
                            "dipAngle": 0,
                            "earthLimb": {
                                "adc": 0,
                                "flags": ["NO_COMM"],
                                "errors": true,
                                "temp": 0
                            },
                            "earthRef": {
                                "adc": 0,
                                "flags": ["NO_COMM"],
                                "errors": true,
                                "temp": 0
                            },
                            "spaceRef": {
                                "adc": 0,
                                "flags": ["DIP_ANGLE_LIMIT"],
                                "errors": true,
                                "temp": 0
                            },
                            "wideFov": {
                                "adc": 0,
                                "flags": ["DIP_ANGLE_LIMIT"],
                                "errors": true,
                                "temp": 0
                            },
                        },
                        "thermopileStructB": {
                            "dipAngle": 0,
                            "earthLimb": {
                                "adc": 0,
                                "flags": ["NO_COMM"],
                                "errors": true,
                                "temp": 0
                            },
                            "earthRef": {
                                "adc": 0,
                                "flags": ["NO_COMM"],
                                "errors": true,
                                "temp": 0
                            },
                            "spaceRef": {
                                "adc": 0,
                                "flags": ["DIP_ANGLE_LIMIT"],
                                "errors": true,
                                "temp": 0
                            },
                            "wideFov": {
                                "adc": 0,
                                "flags": ["DIP_ANGLE_LIMIT"],
                                "errors": true,
                                "temp": 0
                            },
                        },
                        "thermopilesA": [0, 0, 0, 0],
                        "thermopilesB": [0, 0, 0, 0],
                    }
                }
            }
    });

    let result = service.process(query.to_owned());

    assert_eq!(result, wrap!(expected));

    let mut actual: serde_json::Value = serde_json::from_slice(&result.as_bytes()).unwrap();
    actual = actual["msg"].clone();

    // Make sure derived structure values matche original values from arrays

    let dip_angle_a = &actual["telemetry"]["debug"]["irehs"]["dipAngleA"];
    let dip_angle_b = &actual["telemetry"]["debug"]["irehs"]["dipAngleB"];
    let solution_degraded = &actual["telemetry"]["debug"]["irehs"]["solutionDegraded"];
    let temp_a = &actual["telemetry"]["debug"]["irehs"]["tempA"];
    let temp_b = &actual["telemetry"]["debug"]["irehs"]["tempB"];
    let thermopiles_a = &actual["telemetry"]["debug"]["irehs"]["thermopilesA"];
    let thermopiles_b = &actual["telemetry"]["debug"]["irehs"]["thermopilesB"];
    let struct_a = &actual["telemetry"]["debug"]["irehs"]["thermopileStructA"];
    let struct_b = &actual["telemetry"]["debug"]["irehs"]["thermopileStructB"];

    assert_eq!(&struct_a["dipAngle"], dip_angle_a);
    assert_eq!(struct_a["earthLimb"]["adc"], thermopiles_a[0]);
    assert_eq!(struct_a["earthLimb"]["flags"], solution_degraded[0]);
    assert_eq!(struct_a["earthLimb"]["temp"], temp_a[0]);
    assert_eq!(struct_a["earthRef"]["adc"], thermopiles_a[1]);
    assert_eq!(struct_a["earthRef"]["flags"], solution_degraded[1]);
    assert_eq!(struct_a["earthRef"]["temp"], temp_a[1]);
    assert_eq!(struct_a["spaceRef"]["adc"], thermopiles_a[2]);
    assert_eq!(struct_a["spaceRef"]["flags"], solution_degraded[2]);
    assert_eq!(struct_a["spaceRef"]["temp"], temp_a[2]);
    assert_eq!(struct_a["wideFov"]["adc"], thermopiles_a[3]);
    assert_eq!(struct_a["wideFov"]["flags"], solution_degraded[3]);
    assert_eq!(struct_a["wideFov"]["temp"], temp_a[3]);

    assert_eq!(&struct_b["dipAngle"], dip_angle_b);
    assert_eq!(struct_b["earthLimb"]["adc"], thermopiles_b[0]);
    assert_eq!(struct_b["earthLimb"]["flags"], solution_degraded[4]);
    assert_eq!(struct_b["earthLimb"]["temp"], temp_b[0]);
    assert_eq!(struct_b["earthRef"]["adc"], thermopiles_b[1]);
    assert_eq!(struct_b["earthRef"]["flags"], solution_degraded[5]);
    assert_eq!(struct_b["earthRef"]["temp"], temp_b[1]);
    assert_eq!(struct_b["spaceRef"]["adc"], thermopiles_b[2]);
    assert_eq!(struct_b["spaceRef"]["flags"], solution_degraded[6]);
    assert_eq!(struct_b["spaceRef"]["temp"], temp_b[2]);
    assert_eq!(struct_b["wideFov"]["adc"], thermopiles_b[3]);
    assert_eq!(struct_b["wideFov"]["flags"], solution_degraded[7]);
    assert_eq!(struct_b["wideFov"]["temp"], temp_b[3]);
}

#[test]
fn telemetry_debug_imu() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"{
            telemetry{
                debug{
                    rawImu{
                        accel,
                        gyro,
                        gyroTemp
                    }
                }
            }
        }"#;

    let expected = json!({
            "telemetry": {
                "debug": {
                    "rawImu": {
                        "accel": [1, -5, 272],
                        "gyro": [38, 29, 22],
                        "gyroTemp": 19,
                    }
                }
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn telemetry_debug_rotating() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"{
            telemetry{
                debug{
                    rotating{
                        acsOpMode,
                        adsOpMode,
                        attDetMode,
                        bFieldIgrf,
                        cosSunMagAlignThresh,
                        cssBias,
                        cssGain,
                        dipoleGain,
                        kBdot,
                        kUnload,
                        kd,
                        keplerElem{
                            argParigee,
                            eccentricity,
                            inclination,
                            raan,
                            semiMajorAxis,
                            trueAnomoly
                        },
                        kp,
                        magBias,
                        magGain,
                        maiSn,
                        majorVersion,
                        minorVersion,
                        orbitEpoch,
                        orbitEpochNext,
                        orbitPropMode,
                        procResetCntr,
                        qSat,
                        qbXDipoleGain,
                        qbXFilterGain,
                        qbXWheelSpeed,
                        rwaTrqMax,
                        rwsMotorCurrent,
                        rwsMotorTemp,
                        rwsPress,
                        rwsResetCntr,
                        rwsVolt,
                        scPosEci,
                        scPosEciEpoch,
                        scVelEci,
                        scVelEciEpoch,
                        sunMagAligned,
                        sunVecEph,
                        trueAnomolyEpoch,
                        unloadAngThresh,
                        wheelSpeedBias,
                    }
                }
            }
        }"#;

    let expected = json!({
            "telemetry": {
                "debug": {
                    "rotating": {
                    	"acsOpMode": 169,
                        "adsOpMode": 0,
                        "attDetMode": 24,
                        "bFieldIgrf": [-0.000015042761333461386, 2.054659944406012e-7, 0.0000222020080400398],
                        "cosSunMagAlignThresh": 0.9900000095367432,
                        "cssBias": [0, 0, 0, 0, 0, 0],
                        "cssGain": [1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
                        "dipoleGain": [1.0, 1.0, 1.0],
                        "kBdot": [-100000.0, -100000.0, -100000.0],
                        "kUnload": [-5000000.0, -5000000.0, -5000000.0],
                        "kd": [-12.100000381469727, -12.0, -4.0],
                        "keplerElem": {
                        	"argParigee": 0.0,
                        	"eccentricity": 0.0,
                        	"inclination": 45.0,
                        	"raan": 0.0,
                        	"semiMajorAxis": 6787.47021484375,
                        	"trueAnomoly": 0.0,
                        },
                        "kp": [-1.1100000143051148, -1.100000023841858, -0.25],
                        "magBias": [1028, 0, 0],
                        "magGain": [1.0, 1.0, 1.0],
                        "maiSn": 120,
                        "majorVersion": 2,
                        "minorVersion": 24,
                        "orbitEpoch": 511358571,
                        "orbitEpochNext": 1,
                        "orbitPropMode": 0,
                        "procResetCntr": 4,
                        "qSat": 0.30000001192092898,
                        "qbXDipoleGain": 1.0,
                        "qbXFilterGain": 1.0,
                        "qbXWheelSpeed": 0,
                        "rwaTrqMax": 0.20000000298023225,
                        "rwsMotorCurrent": [4, 4, 4004],
                        "rwsMotorTemp": 12,
                        "rwsPress": 0,
                        "rwsResetCntr": [1, 1, 2],
                        "rwsVolt": 0,
                        "scPosEci": [6720.96533203125, 669.7313842773438, 669.7247314453125],
                        "scPosEciEpoch": [6787.47021484375, 0.0, 0.0],
                        "scVelEci": [-0.20792292058467866, 5.416772842407227, 5.4167680740356449],
                        "scVelEciEpoch": [0.0, 5.418765068054199, 5.418765068054199],
                        "sunMagAligned": 0,
                        "sunVecEph": [0.1826000213623047, -0.9020766615867615, -0.39104345440864565],
                        "trueAnomolyEpoch": 0.0,
                        "unloadAngThresh": 0.05000000074505806,
                        "wheelSpeedBias": [16256, 16256, 16256],
                    }
                }
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn test_results() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"{
            testResults{
                success,
                telemetryDebug{
                    irehs{
                        dipAngleA,
                        dipAngleB,
                        solutionDegraded,
                        tempA,
                        tempB,
                        thermopileStructA{
                            dipAngle,
                            earthLimb{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                            earthRef{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                            spaceRef{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                            wideFov{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                        },
                        thermopileStructB{
                            dipAngle,
                            earthLimb{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                            earthRef{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                            spaceRef{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                            wideFov{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                        },
                        thermopilesA,
                        thermopilesB,
                    },
                    rawImu{
                        accel,
                        gyro,
                        gyroTemp
                    },
                    rotating{
                        acsOpMode,
                        adsOpMode,
                        attDetMode,
                        bFieldIgrf,
                        cosSunMagAlignThresh,
                        cssBias,
                        cssGain,
                        dipoleGain,
                        kBdot,
                        kUnload,
                        kd,
                        keplerElem{
                            argParigee,
                            eccentricity,
                            inclination,
                            raan,
                            semiMajorAxis,
                            trueAnomoly
                        },
                        kp,
                        magBias,
                        magGain,
                        maiSn,
                        majorVersion,
                        minorVersion,
                        orbitEpoch,
                        orbitEpochNext,
                        orbitPropMode,
                        procResetCntr,
                        qSat,
                        qbXDipoleGain,
                        qbXFilterGain,
                        qbXWheelSpeed,
                        rwaTrqMax,
                        rwsMotorCurrent,
                        rwsMotorTemp,
                        rwsPress,
                        rwsResetCntr,
                        rwsVolt,
                        scPosEci,
                        scPosEciEpoch,
                        scVelEci,
                        scVelEciEpoch,
                        sunMagAligned,
                        sunVecEph,
                        trueAnomolyEpoch,
                        unloadAngThresh,
                        wheelSpeedBias,
                    }
                },
                telemetryNominal{
                    acsMode,
                    angleToGo,
                    bd,
                    cmdInvalidChksumCntr,
                    cmdInvalidCntr,
                    cmdValidCntr,
                    css,
                    eclipseFlag,
                    gcRwaTorqueCmd,
                    gcTorqueCoilCmd,
                    gpsTime,
                    iBFieldMeas,
                    lastCommand,
                    nb,
                    neci
                    omegaB,
                    qError,
                    qboCmd,
                    qboHat,
                    rwaTorqueCmd,
                    rwsSpeedCmd,
                    rwsSpeedTach,
                    sunVecB,
                    timeSubsec,
                    torqueCoilCmd,
                }
            }
        }"#;

    let expected = json!({
            "testResults": {
                "success": true,
                "telemetryDebug": {
                    "irehs": {
                        "dipAngleA": 0,
                        "dipAngleB": 0,
                        "solutionDegraded": [
                            ["NO_COMM"],
                            ["NO_COMM"],
                            ["DIP_ANGLE_LIMIT"],
                            ["DIP_ANGLE_LIMIT"],
                            ["NO_COMM"],
                            ["NO_COMM"],
                            ["DIP_ANGLE_LIMIT"],
                            ["DIP_ANGLE_LIMIT"],
                        ],
                        "tempA": [0, 0, 0, 0],
                        "tempB": [0, 0, 0, 0],
                        "thermopileStructA": {
                            "dipAngle": 0,
                            "earthLimb": {
                                "adc": 0,
                                "flags": ["NO_COMM"],
                                "errors": true,
                                "temp": 0
                            },
                            "earthRef": {
                                "adc": 0,
                                "flags": ["NO_COMM"],
                                "errors": true,
                                "temp": 0
                            },
                            "spaceRef": {
                                "adc": 0,
                                "flags": ["DIP_ANGLE_LIMIT"],
                                "errors": true,
                                "temp": 0
                            },
                            "wideFov": {
                                "adc": 0,
                                "flags": ["DIP_ANGLE_LIMIT"],
                                "errors": true,
                                "temp": 0
                            },
                        },
                        "thermopileStructB": {
                            "dipAngle": 0,
                            "earthLimb": {
                                "adc": 0,
                                "flags": ["NO_COMM"],
                                "errors": true,
                                "temp": 0
                            },
                            "earthRef": {
                                "adc": 0,
                                "flags": ["NO_COMM"],
                                "errors": true,
                                "temp": 0
                            },
                            "spaceRef": {
                                "adc": 0,
                                "flags": ["DIP_ANGLE_LIMIT"],
                                "errors": true,
                                "temp": 0
                            },
                            "wideFov": {
                                "adc": 0,
                                "flags": ["DIP_ANGLE_LIMIT"],
                                "errors": true,
                                "temp": 0
                            },
                        },
                        "thermopilesA": [0, 0, 0, 0],
                        "thermopilesB": [0, 0, 0, 0],
                    },
                    "rawImu": {
                        "accel": [1, -5, 272],
                        "gyro": [38, 29, 22],
                        "gyroTemp": 19,
                    },
                    "rotating": {
                    	"acsOpMode": 169,
                        "adsOpMode": 0,
                        "attDetMode": 24,
                        "bFieldIgrf": [-0.000015042761333461386, 2.054659944406012e-7, 0.0000222020080400398],
                        "cosSunMagAlignThresh": 0.9900000095367432,
                        "cssBias": [0, 0, 0, 0, 0, 0],
                        "cssGain": [1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
                        "dipoleGain": [1.0, 1.0, 1.0],
                        "kBdot": [-100000.0, -100000.0, -100000.0],
                        "kUnload": [-5000000.0, -5000000.0, -5000000.0],
                        "kd": [-12.100000381469727, -12.0, -4.0],
                        "keplerElem": {
                        	"argParigee": 0.0,
                        	"eccentricity": 0.0,
                        	"inclination": 45.0,
                        	"raan": 0.0,
                        	"semiMajorAxis": 6787.47021484375,
                        	"trueAnomoly": 0.0,
                        },
                        "kp": [-1.1100000143051148, -1.100000023841858, -0.25],
                        "magBias": [1028, 0, 0],
                        "magGain": [1.0, 1.0, 1.0],
                        "maiSn": 120,
                        "majorVersion": 2,
                        "minorVersion": 24,
                        "orbitEpoch": 511358571,
                        "orbitEpochNext": 1,
                        "orbitPropMode": 0,
                        "procResetCntr": 4,
                        "qSat": 0.30000001192092898,
                        "qbXDipoleGain": 1.0,
                        "qbXFilterGain": 1.0,
                        "qbXWheelSpeed": 0,
                        "rwaTrqMax": 0.20000000298023225,
                        "rwsMotorCurrent": [4, 4, 4004],
                        "rwsMotorTemp": 12,
                        "rwsPress": 0,
                        "rwsResetCntr": [1, 1, 2],
                        "rwsVolt": 0,
                        "scPosEci": [6720.96533203125, 669.7313842773438, 669.7247314453125],
                        "scPosEciEpoch": [6787.47021484375, 0.0, 0.0],
                        "scVelEci": [-0.20792292058467866, 5.416772842407227, 5.4167680740356449],
                        "scVelEciEpoch": [0.0, 5.418765068054199, 5.418765068054199],
                        "sunMagAligned": 0,
                        "sunVecEph": [0.1826000213623047, -0.9020766615867615, -0.39104345440864565],
                        "trueAnomolyEpoch": 0.0,
                        "unloadAngThresh": 0.05000000074505806,
                        "wheelSpeedBias": [16256, 16256, 16256],
                    }
                },
                "telemetryNominal": {
                    "acsMode": "RATE_NULLING",
                    "angleToGo": 0.0,
                    "bd": [-0.0000017433042103220942, 1.2922178882490699e-7 ,-9.995264917961322e-7],
                    "cmdInvalidChksumCntr": 0,
                    "cmdInvalidCntr": 0,
                    "cmdValidCntr": 2,
                    "css": [0, 4, 1, 0, 0, 4],
                    "eclipseFlag": 1,
                    "gcRwaTorqueCmd": [0, 0, 0],
                    "gcTorqueCoilCmd": [127, -15, 118],
                    "gpsTime": 1198800019,
                    "iBFieldMeas": [-1369, 105, -785],
                    "lastCommand": 0x44,
                    "nb": [31769, 31769, 31769],
                    "neci": [-32754, -627, -627],
                    "omegaB": [0.0, 0.0, 0.0],
                    "qError": [0, 0, 0, 32767],
                    "qboCmd": [0, 0, 0, 32767],
                    "qboHat": [0, 0, 0, 32767],
                    "rwaTorqueCmd": [0.0, 0.0, 0.0],
                    "rwsSpeedCmd": [0, 0, 0],
                    "rwsSpeedTach": [0, 0, 0],
                    "sunVecB": [-32767, -32767, -32767],
                    "timeSubsec": 0,
                    "torqueCoilCmd": [0.1080000028014183, -0.012922178953886032, 0.09995264559984207]
                }
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn mode() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"{
            mode
        }"#;

    let expected = json!({
            "mode": "RATE_NULLING"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn orientation() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"{
            orientation
        }"#;

    let expected = json!({
            "orientation": "Not Implemented"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn spin() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"{
            spin {
                x,
                y,
                z
            }
        }"#;

    let expected = json!({
            "spin": {
                "x": -100000.0,
                "y": -100000.0,
                "z": -100000.0
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn mutation_errors_empty() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"mutation {
            errors
        }"#;

    let expected = json!({
            "errors": []
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn mutation_errors_single() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let noop = r#"mutation {
            noop {
                success
            },
            errors
        }"#;

    service.process(noop.to_owned());

    let query = r#"{
            errors
        }"#;

    let expected = json!({
            "errors": ["Noop: Unable to communicate with MAI400"]
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn mutation_errors_multiple() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let noop = r#"mutation {
            noop {
                success
            }
        }"#;

    service.process(noop.to_owned());
    service.process(noop.to_owned());

    let query = r#"{
            errors
        }"#;

    let expected = json!({
            "errors": ["Noop: Unable to communicate with MAI400", "Noop: Unable to communicate with MAI400"]
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn noop_good() {
    let mock = MockStream::default();

    let data = Arc::new(ReadData {
        std_telem: Mutex::new(STD),
        irehs_telem: Mutex::new(IREHS),
        imu: Mutex::new(IMU),
        rotating: Mutex::new(ROTATING),
    });

    let service = service_new!(mock, data);

    thread::spawn(move || loop {
        {
            let mut local = data.std_telem.lock().unwrap();
            local.tlm_counter += 1;
        }
        thread::sleep(Duration::from_millis(100));
    });

    let query = r#"mutation {
            noop {
                errors,
                success
            }
            
        }"#;

    let expected = json!({
            "noop": {
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn noop_fail() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"mutation {
            noop {
                errors,
                success
            }
            
        }"#;

    let expected = json!({
            "noop": {
                "errors": "Unable to communicate with MAI400",
                "success": false
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn control_power_good() {
    let mut mock = MockStream::default();

    mock.write.set_input(REQUEST_RESET.to_vec());
    mock.write.set_input(CONFIRM_RESET.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            controlPower(state: RESET) {
                errors,
                power,
                success
            }
        }"#;

    let expected = json!({
            "controlPower": {
                "errors": "",
                "power": "RESET",
                "success": true
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn control_power_bad() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"mutation {
            controlPower(state: OFF) {
                errors,
                power,
                success
            }
        }"#;

    let expected = json!({
            "controlPower": {
                "errors": "Invalid power state",
                "power": "OFF",
                "success": false
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn control_power_fail() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"mutation {
            controlPower(state: RESET) {
                errors,
                power,
                success
            }
        }"#;

    let expected = json!({
            "controlPower": {
                "errors": "UART Error, Generic Error",
                "power": "RESET",
                "success": false
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn configure_hardware() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"mutation {
            configureHardware
        }"#;

    let expected = json!({
            "configureHardware": "Not Implemented"
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn test_hardware_integration() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"mutation {
            testHardware(test: INTEGRATION){
            ... on IntegrationTestResults{
                errors,
                success,
                telemetryDebug{
                    irehs{
                        dipAngleA,
                        dipAngleB,
                        solutionDegraded,
                        tempA,
                        tempB,
                        thermopileStructA{
                            dipAngle,
                            earthLimb{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                            earthRef{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                            spaceRef{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                            wideFov{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                        },
                        thermopileStructB{
                            dipAngle,
                            earthLimb{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                            earthRef{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                            spaceRef{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                            wideFov{
                                adc,
                                errors,
                                flags,
                                temp
                            },
                        },
                        thermopilesA,
                        thermopilesB,
                    },
                    rawImu{
                        accel,
                        gyro,
                        gyroTemp
                    },
                    rotating{
                        acsOpMode,
                        adsOpMode,
                        attDetMode,
                        bFieldIgrf,
                        cosSunMagAlignThresh,
                        cssBias,
                        cssGain,
                        dipoleGain,
                        kBdot,
                        kUnload,
                        kd,
                        keplerElem{
                            argParigee,
                            eccentricity,
                            inclination,
                            raan,
                            semiMajorAxis,
                            trueAnomoly
                        },
                        kp,
                        magBias,
                        magGain,
                        maiSn,
                        majorVersion,
                        minorVersion,
                        orbitEpoch,
                        orbitEpochNext,
                        orbitPropMode,
                        procResetCntr,
                        qSat,
                        qbXDipoleGain,
                        qbXFilterGain,
                        qbXWheelSpeed,
                        rwaTrqMax,
                        rwsMotorCurrent,
                        rwsMotorTemp,
                        rwsPress,
                        rwsResetCntr,
                        rwsVolt,
                        scPosEci,
                        scPosEciEpoch,
                        scVelEci,
                        scVelEciEpoch,
                        sunMagAligned,
                        sunVecEph,
                        trueAnomolyEpoch,
                        unloadAngThresh,
                        wheelSpeedBias,
                    }
                },
                telemetryNominal{
                    acsMode,
                    angleToGo,
                    bd,
                    cmdInvalidChksumCntr,
                    cmdInvalidCntr,
                    cmdValidCntr,
                    css,
                    eclipseFlag,
                    gcRwaTorqueCmd,
                    gcTorqueCoilCmd,
                    gpsTime,
                    iBFieldMeas,
                    lastCommand,
                    nb,
                    neci
                    omegaB,
                    qError,
                    qboCmd,
                    qboHat,
                    rwaTorqueCmd,
                    rwsSpeedCmd,
                    rwsSpeedTach,
                    sunVecB,
                    timeSubsec,
                    torqueCoilCmd,
                }
            }}
        }"#;

    let expected = json!({
            "testHardware": {
                "errors": "",
                "success": true,
                "telemetryDebug": {
                    "irehs": {
                        "dipAngleA": 0,
                        "dipAngleB": 0,
                        "solutionDegraded": [
                            ["NO_COMM"],
                            ["NO_COMM"],
                            ["DIP_ANGLE_LIMIT"],
                            ["DIP_ANGLE_LIMIT"],
                            ["NO_COMM"],
                            ["NO_COMM"],
                            ["DIP_ANGLE_LIMIT"],
                            ["DIP_ANGLE_LIMIT"],
                        ],
                        "tempA": [0, 0, 0, 0],
                        "tempB": [0, 0, 0, 0],
                        "thermopileStructA": {
                            "dipAngle": 0,
                            "earthLimb": {
                                "adc": 0,
                                "flags": ["NO_COMM"],
                                "errors": true,
                                "temp": 0
                            },
                            "earthRef": {
                                "adc": 0,
                                "flags": ["NO_COMM"],
                                "errors": true,
                                "temp": 0
                            },
                            "spaceRef": {
                                "adc": 0,
                                "flags": ["DIP_ANGLE_LIMIT"],
                                "errors": true,
                                "temp": 0
                            },
                            "wideFov": {
                                "adc": 0,
                                "flags": ["DIP_ANGLE_LIMIT"],
                                "errors": true,
                                "temp": 0
                            },
                        },
                        "thermopileStructB": {
                            "dipAngle": 0,
                            "earthLimb": {
                                "adc": 0,
                                "flags": ["NO_COMM"],
                                "errors": true,
                                "temp": 0
                            },
                            "earthRef": {
                                "adc": 0,
                                "flags": ["NO_COMM"],
                                "errors": true,
                                "temp": 0
                            },
                            "spaceRef": {
                                "adc": 0,
                                "flags": ["DIP_ANGLE_LIMIT"],
                                "errors": true,
                                "temp": 0
                            },
                            "wideFov": {
                                "adc": 0,
                                "flags": ["DIP_ANGLE_LIMIT"],
                                "errors": true,
                                "temp": 0
                            },
                        },
                        "thermopilesA": [0, 0, 0, 0],
                        "thermopilesB": [0, 0, 0, 0],
                    },
                    "rawImu": {
                        "accel": [1, -5, 272],
                        "gyro": [38, 29, 22],
                        "gyroTemp": 19,
                    },
                    "rotating": {
                    	"acsOpMode": 169,
                        "adsOpMode": 0,
                        "attDetMode": 24,
                        "bFieldIgrf": [-0.000015042761333461386, 2.054659944406012e-7, 0.0000222020080400398],
                        "cosSunMagAlignThresh": 0.9900000095367432,
                        "cssBias": [0, 0, 0, 0, 0, 0],
                        "cssGain": [1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
                        "dipoleGain": [1.0, 1.0, 1.0],
                        "kBdot": [-100000.0, -100000.0, -100000.0],
                        "kUnload": [-5000000.0, -5000000.0, -5000000.0],
                        "kd": [-12.100000381469727, -12.0, -4.0],
                        "keplerElem": {
                        	"argParigee": 0.0,
                        	"eccentricity": 0.0,
                        	"inclination": 45.0,
                        	"raan": 0.0,
                        	"semiMajorAxis": 6787.47021484375,
                        	"trueAnomoly": 0.0,
                        },
                        "kp": [-1.1100000143051148, -1.100000023841858, -0.25],
                        "magBias": [1028, 0, 0],
                        "magGain": [1.0, 1.0, 1.0],
                        "maiSn": 120,
                        "majorVersion": 2,
                        "minorVersion": 24,
                        "orbitEpoch": 511358571,
                        "orbitEpochNext": 1,
                        "orbitPropMode": 0,
                        "procResetCntr": 4,
                        "qSat": 0.30000001192092898,
                        "qbXDipoleGain": 1.0,
                        "qbXFilterGain": 1.0,
                        "qbXWheelSpeed": 0,
                        "rwaTrqMax": 0.20000000298023225,
                        "rwsMotorCurrent": [4, 4, 4004],
                        "rwsMotorTemp": 12,
                        "rwsPress": 0,
                        "rwsResetCntr": [1, 1, 2],
                        "rwsVolt": 0,
                        "scPosEci": [6720.96533203125, 669.7313842773438, 669.7247314453125],
                        "scPosEciEpoch": [6787.47021484375, 0.0, 0.0],
                        "scVelEci": [-0.20792292058467866, 5.416772842407227, 5.4167680740356449],
                        "scVelEciEpoch": [0.0, 5.418765068054199, 5.418765068054199],
                        "sunMagAligned": 0,
                        "sunVecEph": [0.1826000213623047, -0.9020766615867615, -0.39104345440864565],
                        "trueAnomolyEpoch": 0.0,
                        "unloadAngThresh": 0.05000000074505806,
                        "wheelSpeedBias": [16256, 16256, 16256],
                    }
                },
                "telemetryNominal": {
                    "acsMode": "RATE_NULLING",
                    "angleToGo": 0.0,
                    "bd": [-0.0000017433042103220942, 1.2922178882490699e-7 ,-9.995264917961322e-7],
                    "cmdInvalidChksumCntr": 0,
                    "cmdInvalidCntr": 0,
                    "cmdValidCntr": 2,
                    "css": [0, 4, 1, 0, 0, 4],
                    "eclipseFlag": 1,
                    "gcRwaTorqueCmd": [0, 0, 0],
                    "gcTorqueCoilCmd": [127, -15, 118],
                    "gpsTime": 1198800019,
                    "iBFieldMeas": [-1369, 105, -785],
                    "lastCommand": 0x44,
                    "nb": [31769, 31769, 31769],
                    "neci": [-32754, -627, -627],
                    "omegaB": [0.0, 0.0, 0.0],
                    "qError": [0, 0, 0, 32767],
                    "qboCmd": [0, 0, 0, 32767],
                    "qboHat": [0, 0, 0, 32767],
                    "rwaTorqueCmd": [0.0, 0.0, 0.0],
                    "rwsSpeedCmd": [0, 0, 0],
                    "rwsSpeedTach": [0, 0, 0],
                    "sunVecB": [-32767, -32767, -32767],
                    "timeSubsec": 0,
                    "torqueCoilCmd": [0.1080000028014183, -0.012922178953886032, 0.09995264559984207]
                }
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn test_hardware_hardware() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"mutation {
            testHardware(test: HARDWARE) {
                ... on HardwareTestResults {
                    data,
                    errors,
                    success
                }
            }
        }"#;

    let expected = json!({
            "testHardware": {
                "data": "",
                "errors": "Not Implemented",
                "success": true
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn issue_raw_command_good() {
    let mut mock = MockStream::default();

    mock.write.set_result(Ok(()));

    let service = service_new!(mock);

    let query = r#"mutation {
            issueRawCommand(command: "90EB5A0000000000000000000000000000000000000000000000000000000000000000000000D501"){
                errors,
                success
            }
        }"#;

    let expected = json!({
            "issueRawCommand": {
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn issue_raw_command_fail() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"mutation {
            issueRawCommand(command: "90EB5A0000000000000000000000000000000000000000000000000000000000000000000000D501"){
                errors,
                success
            }
        }"#;

    let expected = json!({
            "issueRawCommand": {
                "errors": "UART Error, Generic Error",
                "success": false
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn set_mode_default() {
    let mut mock = MockStream::default();

    mock.write.set_input(SET_MODE_ACQUISITION_DEFAULT.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            setMode(mode: RATE_NULLING) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "setMode": {
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn set_mode_good() {
    let mut mock = MockStream::default();

    mock.write.set_input(SET_MODE_ACQUISITION.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            setMode(mode: RATE_NULLING, qbiCmd: [2, 3, 4, 5]) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "setMode": {
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn set_mode_fail() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"mutation {
            setMode(mode: RATE_NULLING, qbiCmd: [2, 3, 4, 5]) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "setMode": {
                "errors": "UART Error, Generic Error",
                "success": false
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn set_mode_normal_sun() {
    let mut mock = MockStream::default();

    mock.write.set_input(SET_MODE_NORMAL_SUN.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            setMode(mode: NORMAL_SUN, sunAngleEnable: true, sunRotAngle: 2.2) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "setMode": {
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn set_mode_latlong_sun() {
    let mut mock = MockStream::default();

    mock.write.set_input(SET_MODE_LATLONG_SUN.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            setMode(mode: LAT_LONG_SUN, sunAngleEnable: true, sunRotAngle: 2.2) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "setMode": {
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn set_mode_sun_default() {
    let mut mock = MockStream::default();

    mock.write.set_input(SET_MODE_SUN_DEFAULT.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            setMode(mode: LAT_LONG_SUN) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "setMode": {
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn set_mode_sun_fail() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"mutation {
            setMode(mode: LAT_LONG_SUN) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "setMode": {
                "errors": "UART Error, Generic Error",
                "success": false
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn update_gps_good() {
    let mut mock = MockStream::default();

    mock.write.set_input(SET_GPS_TIME.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            update(gpsTime: 1198800018) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "update": {
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn update_gps_fail() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"mutation {
            update(gpsTime: 1198800018) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "update": {
                "errors": "update(gpsTime): UART Error, Generic Error",
                "success": false
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn update_rv_good() {
    let mut mock = MockStream::default();

    mock.write.set_input(SET_RV.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            update(rv: {eciPos: [1.1, 2.2, 3.3], eciVel: [4.4, 5.5, 6.6], timeEpoch: 1198800018}) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "update": {
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn update_rv_fail() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"mutation {
            update(rv: {eciPos: [1.1, 2.2, 3.3], eciVel: [4.4, 5.5, 6.6], timeEpoch: 1198800018}) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "update": {
                "errors": "update(rv): UART Error, Generic Error",
                "success": false
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn update_both_both_good() {
    let mut mock = MockStream::default();

    mock.write.set_input(SET_GPS_TIME.to_vec());
    mock.write.set_input(SET_RV.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            update(gpsTime: 1198800018, rv: {eciPos: [1.1, 2.2, 3.3], eciVel: [4.4, 5.5, 6.6], timeEpoch: 1198800018}) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "update": {
                "errors": "",
                "success": true
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn update_both_both_fail() {
    let mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"mutation {
            update(gpsTime: 1198800018, rv: {eciPos: [1.1, 2.2, 3.3], eciVel: [4.4, 5.5, 6.6], timeEpoch: 1198800018}) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "update": {
                "errors": "update(gpsTime): UART Error, Generic Error, update(rv): UART Error, Generic Error",
                "success": false
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn update_both_gps_fail() {
    let mut mock = MockStream::default();

    mock.write.set_input(vec![]);
    mock.write.set_input(SET_RV.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            update(gpsTime: 1198800018, rv: {eciPos: [1.1, 2.2, 3.3], eciVel: [4.4, 5.5, 6.6], timeEpoch: 1198800018}) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "update": {
                "errors": "update(gpsTime): UART Error, Generic Error",
                "success": false
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}

#[test]
fn update_both_rv_fail() {
    let mut mock = MockStream::default();

    mock.write.set_input(SET_GPS_TIME.to_vec());

    let service = service_new!(mock);

    let query = r#"mutation {
            update(gpsTime: 1198800018, rv: {eciPos: [1.1, 2.2, 3.3], eciVel: [4.4, 5.5, 6.6], timeEpoch: 1198800018}) {
                errors,
                success
            }
        }"#;

    let expected = json!({
            "update": {
                "errors": "update(rv): UART Error, Generic Error",
                "success": false
            }
    });

    assert_eq!(service.process(query.to_owned()), wrap!(expected));
}
