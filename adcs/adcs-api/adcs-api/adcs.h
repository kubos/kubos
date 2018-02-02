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
 * @defgroup ADCS Kubos ADCS Interface
 * @addtogroup ADCS
 * @{
 */

#pragma once

#include <stdint.h>
#include "adcs-struct.h"
#include "adcs-impl.h"

/**
 * Initialize the ADCS interface
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_init(void);
/**
 * Terminate the ADCS interface
 */
void k_adcs_terminate(void);
/**
 * Execute ADCS no-op command
 * @note This function might not be implemented for all ADCSs.
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_noop(void);
/**
 * Configure the ADCS
 * @param [in] config ADCS configuration structure
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_configure(const JsonNode * config);
/**
 * Reset the ADCS
 * @note This function might not be implemented for all ADCSs
 * @param [in] type Type of reset to perform (hard, soft, etc)
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_reset(KADCSReset type);
/**
 * Set the ADCS's operating mode
 * @note See specific ADCS API documentation for available modes
 * @param [in] mode Operating mode to change to
 * @param [in] params Pointer to optional parameters which may be needed to configure the new operational mode
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_set_mode(ADCSMode mode, const adcs_mode_param * params);
/**
 * Run an ADCS self-test
 * @note This function might not be implemented for all ADCSs.
 * See specific ADCS API documentation for available self-tests.
 * @param [in] test Type of self-test to run
 * @param [out] buffer (Pointer to) structure which the test-results should be copied to
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_run_test(ADCSTestType test, adcs_test_results buffer);
/**
 * Get the ADCS's power status
 * @param [out] data Pointer to storage for returned system uptime. If the value is zero, then the ADCS is offline.
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_get_power_status(adcs_power_status * data);
/**
 * Get the ADCS's current operating mode
 * @param [out] mode Pointer to storage which the mode value should be copied to
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_get_mode(ADCSMode * mode);
/**
 * Get the ADCS's current orientation
 * @note This function might not be implemented for all ADCSs.
 * @param [out] data Pointer to storage for returned data.
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_get_orientation(adcs_orient * data);
/**
 * Get the ADCS's current spin
 * @note This function might not be implemented for all ADCSs.
 * @param [out] data Pointer to storage for returned data.
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_get_spin(adcs_spin * data);
/**
 * Read ADCS telemetry values
 * @note See specific ADCS API documentation for available telemetry types
 * @param [in] type Telemetry packet to read
 * @param [out] buffer (Pointer to) structure which data should be copied to
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_get_telemetry(ADCSTelemType type, JsonNode * buffer);
/**
 * Pass a command packet directly through to the ADCS.
 * Useful for executing commands which have not been implemented in either the generic or specific ADCS APIs.
 * @param [in] tx Pointer to command packet to send
 * @param [in] tx_len Size of command packet
 * @param [out] rx Pointer to storage for command response
 * @param [in] rx_len Expected length of command response
 * @param [in] delay Time to wait inbetween sending the command packet and requesting a response
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_passthrough(const uint8_t * tx, int tx_len, uint8_t * rx, int rx_len, const struct timespec * delay);

/* @} */
