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

use juniper::FieldResult;
use kubos_service;
use model::*;
use objects::*;

type Context = kubos_service::Context<Subsystem>;

pub struct QueryRoot;

/// Base GraphQL query model
graphql_object!(QueryRoot: Context as "Query" |&self| {

    // Test query to verify service is running without attempting
    // to communicate with the underlying subsystem
    //
    // {
    //     ping: "pong"
    // }
    field ping() -> FieldResult<String>
    {
        Ok(String::from("pong"))
    }

    //----- Generic Queries -----//

    // Get the last run mutation
    //
    // {
    //     ack: AckCommand
    // }
    field ack(&executor) -> FieldResult<AckCommand>
    {
        Ok(executor.context().subsystem().last_cmd.get())
    }

    // Get all errors encountered since the last time this field was queried
    //
    // {
    //     errors: [String]
    // }
    field errors(&executor) -> FieldResult<Vec<String>>
    {
        match executor.context().subsystem().errors.try_borrow_mut() {
            Ok(mut master_vec) => {
                let current = master_vec.clone();
                master_vec.clear();
                master_vec.shrink_to_fit();
                Ok(current)
            },
            _ => Ok(vec!["Error: Failed to borrow master errors vector".to_owned()])
        }
    }

    // Get the current power state and uptime of the system
    //
    // {
    //     power {
    //         state: PowerState,
    //         uptime: Int
    //     }
    // }
    field power(&executor) -> FieldResult<GetPowerResponse>
    {
        Ok(executor.context().subsystem().get_power()?)
    }

    // Get the current configuration of the system
    //
    // {
    //     config: "Not Implemented"
    // }
    field config(&executor) -> FieldResult<String>
    {
        Ok(String::from("Not Implemented"))
    }

    // Get current telemetry information for the system
    //
    // {
    //     telemetry{
    //         nominal{
    //             gpsTime: i32,
    //             timeSubsec: i32,
    //             cmdValidCntr: i32,
    //             cmdInvalidCntr: i32,
    //             cmdInvalidChksumCntr: i32,
    //             lastCommand: i32,
    //             acsMode: i32,
    //             css: [i32; 6],
    //             eclipseFlag: i32,
    //             sunVecB: [i32; 3],
    //             iBFieldMeas: [i32; 3],
    //             bd: [f64; 3],
    //             rwsSpeedCmd: [i32; 3],
    //             rwsSpeedTach: [i32; 3],
    //             rwaTorqueCmd: [f64; 3],
    //             gcRwaTorqueCmd: [i32; 3],
    //             torqueCoilCmd: [f64; 3],
    //             gcTorqueCoilCmd: [i32; 3],
    //             qboCmd: [i32; 4],
    //             qboHat: [i32; 4],
    //             angleToGo: f64,
    //             qError: [i32; 4],
    //             omegaB: [f64; 3],
    //             nb: [i32; 3],
    //             neci: [i32; 3],
    //         },
    //         debug{
    //             irhes{
    //                 thermopilesA: [i32; 4],
    //                 thermopilesB: [i32; 4],
    //                 tempA: [i32; 4],
    //                 tempB: [i32; 4],
    //                 dipAngleA: i32,
    //                 dipAngleB: i32,
    //                 solutionDegraded: [Vec<String>; 8],
    //                 thermopileStructA{
    //                     dipAngle: i32,
    //                     earthLimb: {
    //                         adc: i32,
    //    		               temp: i32,
    //    		               errors: bool,
    //    		               flags: Vec<String>,
    //     	               },
    //                     earthRef: {...},
    //                     spaceRef: {...},
    //                     wideFov: {...},
    //                 thermopileStructB{...}
    //             },
    //             rawImu{
    //                 accel: [i32; 3],
    //                 gyro: [i32; 3],
    //                 gyroTemp: i32,
    //             },
    //             rotating{
    //                 bFieldIgrf: [f64; 3],
    //                 sunVecEph: [f64; 3],
    //                 scPosEci: [f64; 3],
    //                 scVelEci: [f64; 3],
    //                 keplerElem{
    //                     semiMajorAxis: f32,
    //                     eccentricity: f32,
    //                     inclination: f32,
    //                     raan: f32,
    //                     argParigee: f32,
    //                     trueAnomoly: f32,
    //                 },
    //                 kBdot: [f64; 3],
    //                 kp: [f64; 3],
    //                 kd: [f64; 3],
    //                 kUnload: [f64; 3],
    //                 cssBias: [i32; 6],
    //                 magBias: [i32; 3],
    //                 rwsVolt: i32,
    //                 rwsPress: i32,
    //                 attDetMode: i32,
    //                 rwsResetCntr: [i32; 3],
    //                 sunMagAligned: i32,
    //                 minorVersion: i32,
    //                 maiSn: i32,
    //                 orbitPropMode: i32,
    //                 acsOpMode: i32,
    //                 procResetCntr: i32,
    //                 majorVersion: i32,
    //                 adsOpMode: i32,
    //                 cssGain: [f64; 6],
    //                 magGain: [f64; 3],
    //                 orbitEpoch: i32,
    //                 trueAnomolyEpoch: f64,
    //                 orbitEpochNext: i32,
    //                 scPosEciEpoch: [f64; 3],
    //                 scVelEciEpoch: [f64; 3],
    //                 qbXWheelSpeed: i32,
    //                 qbXFilterGain: f64,
    //                 qbXDipoleGain: f64,
    //                 dipoleGain: [f64; 3],
    //                 wheelSpeedBias: [i32; 3],
    //                 cosSunMagAlignThresh: f64,
    //                 unloadAngThresh: f64,
    //                 qSat: f64,
    //                 rwaTrqMax: f64,
    //                 rwsMotorCurrent: [i32; 3],
    //                 rwsMotorTemp: i32,
    //             }
    //         }
    //     }
    // }
    field telemetry(&executor) -> FieldResult<Telemetry>
    {
        Ok(executor.context().subsystem().get_telemetry()?)
    }

    // Get the test results of the last run test
    //
    // Note: For this service, this actually just fetches the nominal
    // and debug telemetry of the system, since there is no actual
    // built-in test
    //
    // {
    //     testResults{
    //         success,
    //         telemetryNominal{...},
    //         telemetryDebug{...}
    //     }
    // }
    field test_results(&executor) -> FieldResult<IntegrationTestResults> {
        Ok(executor.context().subsystem().get_test_results()?)
    }

    // Get the current mode of the system
    //
    // {
    //     mode: Mode
    // }
    field mode(&executor) -> FieldResult<Mode> {
        Ok(executor.context().subsystem().get_mode()?)
    }

    // Get the last reported orientation of the system
    //
    // {
    //     orientation: "Not Implemented"
    // }
    field orientation(&executor) -> FieldResult<String> {
        Ok(String::from("Not Implemented"))
    }

    // Get the last reported spin values of the system
    // Note: The spin values are automatically updated every six seconds
    //
    // {
    //     spin{
    //         x: f64,
    //         y: f64,
    //         z: f64
    //     }
    // }
    field spin(&executor) -> FieldResult<Spin> {
        Ok(executor.context().subsystem().get_spin()?)
    }
});

pub struct MutationRoot;

/// Base GraphQL mutation model
graphql_object!(MutationRoot: Context as "Mutation" |&self| {

    // Get all errors encountered while processing this GraphQL request
    //
    // Note: This will only return errors thrown by fields which have
    // already been processed, so it is recommended that this field be specified last.
    //
    // mutation {
    //     errors: [String]
    // }
    field errors(&executor) -> FieldResult<Vec<String>>
    {
        match executor.context().subsystem().errors.try_borrow() {
            Ok(master_vec) => Ok(master_vec.clone()),
            _ => Ok(vec!["Error: Failed to borrow master errors vector".to_owned()])
        }
    }

    // Execute a trivial command against the system
    //
    // mutation {
    //     noop {
    //         errors: String,
    //         success: Boolean
    //    }
    // }
    field noop(&executor) -> FieldResult<GenericResponse>
    {
        executor.context().subsystem().last_cmd.set(AckCommand::Noop);
        Ok(executor.context().subsystem().noop()?)
    }

    // Control the power state of the system
    //
    // state: Power state the system should be changed to
    //   Note: The only valid input for this service is RESET
    //
    // mutation {
    //     controlPower(state: PowerState) {
    //         errors: String,
    //         success: Boolean,
    //         power: PowerState
    //     }
    // }
    field control_power(&executor, state: PowerState) -> FieldResult<ControlPowerResponse>
    {
        executor.context().subsystem().last_cmd.set(AckCommand::ControlPower);
        Ok(executor.context().subsystem().control_power(state)?)
    }

    // Configure the system
    //
    // mutation {
    //     configureHardware: "Not Implemented"
    // }
    field configure_hardware(&executor) -> FieldResult<String>
    {
        executor.context().subsystem().last_cmd.set(AckCommand::ConfigureHardware);
        Ok(String::from("Not Implemented"))
    }

    // Run a system self-test
    //
    // test: Type of self-test to perform
    //
    // mutation {
    //     testHardware(test: TestType) {
    //         ... on IntegrationTestRsults {
    //             errors: String,
    //             success: Boolean,
    //             telemetryNominal{...},
    //             telemetryDebug{...}
    //         }
    //         ... on HardwareTestResults {
    //             errors: "Not Implemented",
    //             success: true,
    //             data: Empty
    //         }
    //    }
    // }
    field test_hardware(&executor, test: TestType) -> FieldResult<TestResults>
    {
        executor.context().subsystem().last_cmd.set(AckCommand::TestHardware);
        match test {
            TestType::Integration => Ok(TestResults::Integration(executor.context().subsystem()
                    .get_test_results().unwrap())),
            TestType::Hardware => Ok(TestResults::Hardware(HardwareTestResults {
                        errors: "Not Implemented".to_owned(), success: true, data: "".to_owned()}))
        }
    }

    // Pass a custom command through to the system
    //
    // command: String containing the hex values to be sent (ex. "C3")
    //          It will be converted to a byte array before transfer.
    //
    // mutation {
    //     issueRawCommand(command: String) {
    //         errors: String,
    //         success: Boolean,
    //         response: String
    //     }
    // }
    field issue_raw_command(&executor, command: String) -> FieldResult<GenericResponse>
    {
        executor.context().subsystem().last_cmd.set(AckCommand::IssueRawCommand);
        Ok(executor.context().subsystem().passthrough(command)?)
    }

    // Set the attitude control mode
    //
    // mode: Control mode to change to
    // qbiCmd: Optional array of four values needed for Qinertial and Qtable mode
    // sunAngleEnable: Optional. Specifies whether the sun rotating angle should be updated when
    //                 using Normal-Sun or LatLong-Sun mode
    // sunRotAngle: Optional. The sun rotating angle for use in Normal-Sun and LatLong-Sun mode
    //
    // mutation {
    //     setMode(mode: Mode, qbiCmd: Vec<i32>, sunAngleEnable: bool, sunRotAngle: f64) {
    //         errors: String,
    //         success: Boolean,
    //         response: String
    //     }
    // }
    field set_mode(
        &executor,
        mode: Mode,
        qbi_cmd = {vec![0,0,0,0]}: Vec<i32>,
        sun_angle_enable = false: bool,
        sun_rot_angle = 0.0: f64)
    -> FieldResult<GenericResponse> {
        executor.context().subsystem().last_cmd.set(AckCommand::SetMode);
        match mode {
            Mode::NormalSun | Mode::LatLongSun => Ok(executor.context().subsystem().set_mode_sun(
                    mode as u8, sun_angle_enable as i16, sun_rot_angle as f32)?),
            _ => Ok(executor.context().subsystem().set_mode(mode as u8, qbi_cmd)?),
        }
    }

    // Update system values
    //
    // gpsTime: Optional. If specified, updates the system's ADACS clock
    // rv: Optional. If specified, updates the orbital position and velocity at epoch.
    //     The argument has the following sub-fields:
    //         - eciPos: Vector containing the new X, Y, and Z ECI position values
    //         - eciVel: Vector containing the new X, Y, and Z ECI velocity values
    //         - timeEpoch: GPS time at which the eciPos and eciVel values will go into effect
    //
    // mutation {
    //     update(gps_time: Option<i32>,
    //         rv: Option<{eciPos: [f64; 3], eciVel: [f64; 3], timeEpoch: i32}>) {
    //         errors: String,
    //         success: Boolean,
    //     }
    // }
    field update(&executor, gps_time: Option<i32>, rv: Option<RVInput>)
    -> FieldResult<GenericResponse> {
        executor.context().subsystem().last_cmd.set(AckCommand::Update);
        Ok(executor.context().subsystem().update(gps_time, rv)?)
    }

});
