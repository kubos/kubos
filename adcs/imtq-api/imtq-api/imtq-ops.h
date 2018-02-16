/*
 * Copyright (C) 2017 Kubos Corporation
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
/**
 * @addtogroup IMTQ_API
 * @{
 */

#pragma once

#include <json/json.h>
#include "imtq-core.h"

/** \cond WE DO NOT WANT TO HAVE THESE IN OUR GENERATED DOCS */
/* Operational Commands */
#define RESET_MTQ       0xAAA5 /* Reset has a two-byte command code */
#define NOOP            0x02
#define CANCEL_OP       0x03
#define START_MEASURE   0x04
#define START_CURRENT   0x05
#define START_DIPOLE    0x06
#define START_PWM       0x07
#define START_TEST      0x08
#define START_BDOT      0x09
/** \endcond */

/**
 * Available iMTQ system modes
 */
typedef enum {
    IDLE,           /**< Idle mode */
    SELFTEST,       /**< Self-test mode */
    DETUMBLE        /**< Detumble mode */
} ADCSMode;

/**
 * Available iMTQ system reset types
 */
typedef enum {
    SOFT_RESET      /**< Software reset */
} KADCSReset;

/**
 * Self-Test Axis Options
 */
typedef enum {
    TEST_ALL,        /**< Test all axes */
    TEST_X_POS,      /**< Test positive x-axis */
    TEST_X_NEG,      /**< Test negative x-axis */
    TEST_Y_POS,      /**< Test positive y-axis */
    TEST_Y_NEG,      /**< Test negative y-axis */
    TEST_Z_POS,      /**< Test positive z-axis */
    TEST_Z_NEG       /**< Test negative z-axis */
} ADCSTestType;

/**
 * Parameter for `k_adcs_set_mode`
 *
 * For the iMTQ, exclusively used to specify the duration when entering ::DETUMBLE mode
 */
typedef uint16_t   adcs_mode_param;
/**
 * Pointer to self-test results JSON structure created by `k_adcs_run_test`
 */
typedef JsonNode * adcs_test_results;

/* Operational Commands */
/**
 * Switch to idle mode and cancel any ongoing actuation
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_cancel_op(void);
/**
 * Start a 3-axis measurement of the magnetic field using the MTM
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_start_measurement(void);
/**
 * Turn on the coils by current
 * @param current Axes current values in [10<sup>-4</sup> Am<sup>2</sup>]
 * @param time Amount of time coils should remain at the specified levels, in
 * milliseconds.
 * If time is zero, the coils will stay on until another command capable of
 * changing the coils' states/levels is run.
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_start_actuation_current(imtq_axis_data current,
                                           uint16_t       time);
/**
 * Turn on the coils by desired dipole
 * @note If the specified values exceed the coils' capabilities, the dipole
 * will be scaled.
 * The resulting values can be retrieved with ::k_imtq_get_dipole.
 * @param dipole Axes dipole values in [10<sup>-4</sup> Am<sup>2</sup>]
 * @param time Amount of time coils should remain at the specified levels, in
 * milliseconds.
 * If time is zero, the coils will stay on until another command capable of
 * changing the coils' states/levels is run.
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_start_actuation_dipole(imtq_axis_data dipole, uint16_t time);
/**
 * Turn on the coils by PWM duty cycle
 * @param pwm Axes PWM duty cycles in [0.1%]. <em>Max single-axis value: 1000
 * (100%)</em>
 * @param time Amount of time coils should remain at the specified levels, in
 * milliseconds.
 * If time is zero, the coils will stay on until another command capable of
 * changing the coils' states/levels is run.
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_start_actuation_PWM(imtq_axis_data pwm, uint16_t time);
/**
 * Start axis self-test
 *
 * Use ::k_imtq_get_test_results_single or ::k_imtq_get_test_results_all to
 * check the output
 * @param axis Axis to test. Should be a ::ADCSTestType value (use ::TEST_ALL
 * to text each axis sequentially)
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_start_test(ADCSTestType axis);
/**
 * Switch to detumble mode
 * @param time Length of time to spend in detumble mode (seconds)
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_start_detumble(uint16_t time);

/* @} */
