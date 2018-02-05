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
/**
 * @defgroup Antenna Kubos Antenna Interface
 * @addtogroup Antenna
 * @{
 */

#pragma once

#include <stdint.h>
#include "antenna-struct.h"
#include "antenna-impl.h"
/**
 * Initialize the antenna interface
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_init(void);
/**
 * Terminate the antenna interface
 */
void k_ants_terminate(void);
/**
 * Run the antenna's no-op command
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_noop(void);
/**
 * Configure the antenna
 * @param [in] config Pointer to JSON structure containing configuration parameters
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_configure(const JsonNode * config);
/**
 * Change the antenna's power state
 * @param [in] state Desired power state (on, off, reset, etc)
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_power(KANTSPower state);
/**
 * Arm the antenna
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_arm(void);
/**
 * Disarm the antenna
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_disarm(void);
/**
 * Trigger an antenna deployment action
 * @param [in] type Type of deployment action to trigger
 * @param [in] param Pointer to JSON structure containing deployment parameters
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_deploy(KANTSDeployType type, uint8_t param);
/**
 * Perform a test of the antenna system
 * @param [in] type Type of test to run
 * @param [out] results Pointer to JSON structure where test results should be stored
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_test(KANTSTestType type, JsonNode * results);
/**
 * Get antenna power status
 * @param [out] buffer Pointer to JSON structure where results should be stored
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_get_power_status(JsonNode * buffer);
/**
 * Get antenna system status.
 * Indicates whether the system is arm, deployed, and whether any errors have occurred.
 * @param [out] buffer Pointer to JSON structure where results should be stored
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_get_system_status(JsonNode * buffer);
/**
 * Get antenna telemetry values
 * @note See specific antenna API documentation for available telemetry types
 * @param [in] type Telemetry category to fetch
 * @param [out] buffer Pointer to JSON structure which data should be copied to
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_get_telemetry(KANTSTelemType type, JsonNode * buffer);
/**
 * Pass a command packet directly through to the antenna.
 * Useful for executing commands which have not been implemented in either the generic or specific antenna APIs.
 * @param [in] tx Pointer to command packet to send
 * @param [in] tx_len Size of command packet
 * @param [out] rx Pointer to storage for command response
 * @param [in] rx_len Expected length of command response
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_passthrough(const uint8_t * tx, int tx_len, uint8_t * rx, int rx_len);

/* @} */
