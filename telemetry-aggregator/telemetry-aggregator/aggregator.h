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
 * @defgroup Aggregator
 * @addtogroup Aggregator
 * @brief Easy aggregator interface for telemetry
 * @{
 */

#ifndef AGGREGATOR_H
#define AGGREGATOR_H

#include <csp/arch/csp_thread.h>
#include <telemetry/telemetry.h>

/**
 * Thread for aggregating telemetry data. Currently this just calls the
 * user-defined function user_aggregator in a loop.
 */
CSP_DEFINE_TASK(aggregator);

/**
 * Macro for creating the aggregator thread
 */
#define INIT_AGGREGATOR_THREAD                                                      \
{                                                                                   \
    csp_thread_handle_t agg_handle;                                                 \
    csp_thread_create(aggregator, "AGGREGATOR", 2048, NULL, 0, &agg_handle);        \
}

/**
 * Function stub for user-defined telemetry aggregator. This function
 * will be called repeatedly in a loop by the aggregator thread.
 *
 */
void user_aggregator(void);

/**
 * Convenience wrapper function for telemetry submission
 */ 
void aggregator_submit(telemetry_source, uint16_t data);

#endif

/* @} */