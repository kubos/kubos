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
 * @defgroup EPS_API
 * @addtogroup EPS_API
 * @{
 */
/**
 * This API defines a top level interface for interacting with EPS subsystems.
 * Implementations will be found in modules specific to individual EPS
 * hardware.
 */

#pragma once

#include <stdint.h>

/**
 * EPS specific error codes
 */
typedef enum {
    EPS_OK = 0,
    EPS_ERROR,
} eps_err;

/**
 * Struct for holding EPS power line status information.
 * This struct may have to change on a per-EPS basis?
 * For now the lines are represented as uint16_t but
 * this will likely change once we get more into
 * actual EPS integration.
 */
typedef struct
{
    /** Power line one */
    uint16_t line_one;
    /** Power line two */
    uint16_t line_two;
} eps_power_status;

/**
 * Enables the specified power line.
 * @param[in] line power line to enable
 * @return eps_err EPS_OK if successful, otherwise error
 */
eps_err eps_enable_power_line(uint16_t line);

/**
 * Queries the EPS for the status of all available
 * power lines.
 * @param[in] status power status structure
 * @return eps_err EPS_OK if successful, otherwise error
 */
eps_err eps_get_power_status(eps_power_status * status);

/* @} */
