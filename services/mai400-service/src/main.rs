//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#![deny(missing_docs)]
#![deny(warnings)]

//! Kubos Service for interacting with an [Adcole Maryland Aerospace MAI-400](https://www.adcolemai.com/adacs)
//!
//! # Configuration
//!
//! The service can be configured in the `/home/system/etc/config.toml` with the following fields:
//! ```
//! [mai400-service]
//! ip = "127.0.0.1"
//! port = 8082
//! ```
//!
//! Where `ip` specifies the service's IP address, and `port` specifies the port which UDP requests should be sent to.
//!
//! # Starting the Service
//!
//! The service should be started automatically by its init script, but may also be started manually:
//!
//! ```
//! $ mai400-service
//! Kubos MAI-400 service started
//! Listening on: 10.63.1.20:8082
//! ```
//!
//! # Available Fields
//!
//! ```json
//! query {
//! 	ping,
//! 	ack,
//! 	errors,
//! 	power{
//! 		state,
//! 		uptime
//! 	},
//! 	config,
//! 	mode,
//! 	orientation,
//! 	spin,
//!     telemetry{
//!         nominal{
//! 			acsMode,
//! 			angleToGo,
//! 			bd,
//! 			cmdInvalidChksumCntr,
//! 			cmdInvalidCntr,
//! 			cmdValidCntr,
//! 			css,
//! 			eclipseFlag,
//! 			gcRwaTorqueCmd,
//! 			gcTorqueCoilCmd,
//! 			gpsTime,
//! 			iBFieldMeas,
//! 			lastCommand,
//! 			nb,
//! 			neci
//! 			omegaB,
//! 			qError,
//! 			qboCmd,
//! 			qboHat,
//! 			rwaTorqueCmd,
//! 			rwsSpeedCmd,
//! 			rwsSpeedTach,
//! 			sunVecB,
//! 			timeSubsec,
//! 			torqueCoilCmd,
//! 		},
//!         debug{
//!             irehs{
//!                 dipAngleA,
//! 				dipAngleB,
//! 				solutionDegraded,
//! 				tempA,
//! 				tempB,
//! 				thermopileStructA{
//! 					dipAngle,
//! 					earthLimb{
//! 						adc,
//! 						errors,
//! 						flags,
//! 						temp
//! 					},
//! 					earthRef{
//! 						adc,
//! 						errors,
//! 						flags,
//! 						temp
//! 					},
//! 					spaceRef{
//! 						adc,
//! 						errors,
//! 						flags,
//! 						temp
//! 					},
//! 					wideFov{
//! 						adc,
//! 						errors,
//! 						flags,
//! 						temp
//! 					},
//! 				},
//! 				thermopileStructB{
//! 					dipAngle,
//! 					earthLimb{
//! 						adc,
//! 						errors,
//! 						flags,
//! 						temp
//! 					},
//! 					earthRef{
//! 						adc,
//! 						errors,
//! 						flags,
//! 						temp
//! 					},
//! 					spaceRef{
//! 						adc,
//! 						errors,
//! 						flags,
//! 						temp
//! 					},
//! 					wideFov{
//! 						adc,
//! 						errors,
//! 						flags,
//! 						temp
//! 					},
//! 				},
//! 				thermopilesA,
//! 				thermopilesB,
//!             },
//!             rawImu{
//!                 accel,
//!                 gyro,
//!                 gyroTemp
//!             },
//!             rotating{
//!                 acsOpMode,
//! 				adsOpMode,
//! 				attDetMode,
//! 				bFieldIgrf,
//! 				cosSunMagAlignThresh,
//! 				cssBias,
//! 				cssGain,
//! 				dipoleGain,
//! 				kBdot,
//! 				kUnload,
//! 				kd,
//! 				keplerElem{
//! 					argParigee,
//! 					eccentricity,
//! 					inclination,
//! 					raan,
//! 					semiMajorAxis,
//! 					trueAnomoly
//! 				},
//! 				kp,
//! 				magBias,
//! 				magGain,
//! 				maiSn,
//! 				majorVersion,
//! 				minorVersion,
//! 				orbitEpoch,
//! 				orbitEpochNext,
//! 				orbitPropMode,
//! 				procResetCntr,
//! 				qSat,
//! 				qbXDipoleGain,
//! 				qbXFilterGain,
//! 				qbXWheelSpeed,
//! 				rwaTrqMax,
//! 				rwsMotorCurrent,
//! 				rwsMotorTemp,
//! 				rwsPress,
//! 				rwsResetCntr,
//! 				rwsVolt,
//! 				scPosEci,
//! 				scPosEciEpoch,
//! 				scVelEci,
//! 				scVelEciEpoch,
//! 				sunMagAligned,
//! 				sunVecEph,
//! 				trueAnomolyEpoch,
//! 				unloadAngThresh,
//! 				wheelSpeedBias,
//!             }
//!         }
//!     },
//! 	testResults {
//! 		success,
//! 		telemetryNominal {...},
//! 		telemetryDebug {...}
//! 	},
//! }
//!
//! mutation {
//! 	errors,
//! 	noop {
//! 		errors,
//! 		success
//! 	},
//! 	controlPower(state: RESET) {
//! 		errors,
//! 		state,
//! 		success
//! 	},
//! 	configureHardware,
//! 	testHardware(test: TestType) {
//! 		... on IntegrationTestResults {
//! 			errors,
//! 			success,
//! 			telemetryDebug {...},
//! 			telemetryNominal {...},
//! 		},
//! 		... on HardwareTestResults {
//! 			data,
//! 			errors,
//! 			success
//! 		}
//! 	},
//! 	issueRawCommand(command: String) {
//! 		errors,
//! 		success
//! 	},
//! 	setMode(mode: Mode, qbi_cmd = {vec![0,0,0,0]}: Vec<i32>, sun_angle_enable = false: bool, sun_rot_angle = 0.0: f64) {
//! 		errors,
//! 		success
//! 	},
//! 	update(gps_time: Option<i32>, rv: Option<{eciPos: [f64; 3], eciVel: [f64; 3], timeEpoch: i32}>) {
//! 		errors,
//! 		success
//! 	}
//! }
//! ```
//!

#![recursion_limit = "256"]

extern crate failure;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate kubos_service;
extern crate mai400_api;
#[cfg(test)]
#[macro_use]
extern crate serde_json;

mod model;
mod objects;
mod schema;
#[cfg(test)]
mod tests;

use kubos_service::{Config, Service};
use mai400_api::MAIResult;
use model::{ReadData, Subsystem};
use schema::{MutationRoot, QueryRoot};
use std::sync::Arc;

fn main() -> MAIResult<()> {
    Service::new(
        Config::new("mai400-service"),
        Subsystem::new("/dev/ttyS5", Arc::new(ReadData::new()))?,
        QueryRoot,
        MutationRoot,
    ).start();

    Ok(())
}
