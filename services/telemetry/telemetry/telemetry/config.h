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
 * @defgroup Config Telemetry Configuration
 * @addtogroup Config
 * @brief Configuration settings for Telemetry
 * @{
 */

#ifndef TELEMETRY_CONFIG_H
#define TELEMETRY_CONFIG_H

#include <csp/csp_autoconfig.h>

/*! Address used for the current CSP instance */
#ifndef YOTTA_CFG_TELEMETRY_CSP_ADDRESS
#define TELEMETRY_CSP_ADDRESS 1
#else
#define TELEMETRY_CSP_ADDRESS YOTTA_CFG_TELEMETRY_CSP_ADDRESS
#endif

/*! Address used for the current CSP instance */
#ifndef YOTTA_CFG_TELEMETRY_CSP_CLIENT_ADDRESS
#define TELEMETRY_CSP_CLIENT_ADDRESS 2
#else
#define TELEMETRY_CSP_CLIENT_ADDRESS YOTTA_CFG_TELEMETRY_CSP_ADDRESS
#endif

/*! Size of incoming telemetry message queue */
#ifndef YOTTA_CFG_TELEMETRY_MESSAGE_QUEUE_SIZE
#define MESSAGE_QUEUE_SIZE 10
#else
#define MESSAGE_QUEUE_SIZE YOTTA_CFG_TELEMETRY_MESSAGE_QUEUE_SIZE
#endif

/*! Port number used for the telemetry server's internal connections */
#ifndef YOTTA_CFG_TELEMETRY_INTERNAL_PORT
#define TELEMETRY_INTERNAL_PORT 20
#else
#define TELEMETRY_INTERNAL_PORT YOTTA_CFG_TELEMETRY_INTERNAL_PORT
#endif

/*! Port number used for telemetry's external socket connections */
#ifndef YOTTA_CFG_TELEMETRY_EXTERNAL_PORT
#define TELEMETRY_EXTERNAL_PORT 10
#else
#define TELEMETRY_EXTERNAL_PORT YOTTA_CFG_TELEMETRY_EXTERNAL_PORT
#endif

/*! TCP socket port number used by telemetry server under Linux */
#define TELEMETRY_SOCKET_PORT 8199

/*! Max number of subscribers supported */
#ifndef YOTTA_CFG_TELEMETRY_SUBSCRIBERS_MAX_NUM
#define TELEMETRY_SUBSCRIBERS_MAX_NUM ((CSP_CONN_MAX / 2) - 1)
#else
#define TELEMETRY_SUBSCRIBERS_MAX_NUM YOTTA_CFG_TELEMETRY_SUBSCRIBERS_MAX_NUM
#endif

/* Check to ensure the configured number of subscribers is even possible
   based on how many connections CSP has allocated. This should eventually get
   migrated into a higher level CSP manager.
*/
#if ((TELEMETRY_SUBSCRIBERS_MAX_NUM * 2) + 1) > CSP_CONN_MAX
#warning "Number of telemetry subscribers exceeds number of available CSP connections"
#endif

/*! Number of subscriber read attempts */
#ifndef YOTTA_CFG_TELEMETERY_SUBSCRIBERS_READ_ATTEPMTS
#define TELEMETRY_SUBSCRIBER_READ_ATTEMPTS 10
#else
#define TELEMETRY_SUBSCRIBER_READ_ATTEMPTS YOTTA_CFG_TELEMETRY_SUBSCRIBERS_READ_ATTEMPTS
#endif

/*! Stack size of thread for receiving incoming messages */
#ifndef YOTTA_CFG_TELEMETRY_RX_THREAD_STACK_SIZE
#define TELEMETRY_RX_THREAD_STACK_SIZE 1000
#else
#define TELEMETRY_RX_THREAD_STACK_SIZE YOTTA_CFG_TELEMETRY_RX_THREAD_STACK_SIZE
#endif

/*! Priority of thread for receiving incoming messages */
#ifndef YOTTA_CFG_TELEMETRY_RX_THREAD_PRIORITY
#define TELEMETRY_RX_THREAD_PRIORITY 2
#else
#define TELEMETRY_RX_THREAD_PRIORITY YOTTA_CFG_TELEMETRY_RX_THREAD_PRIORITY
#endif

/*! Standard telemetry buffer size */
#ifndef YOTTA_CFG_TELEMETRY_BUFFER_SIZE
#define TELEMETRY_BUFFER_SIZE 256
#else
#define TELEMETRY_BUFFER_SIZE YOTTA_CFG_TELEMETRY_BUFFER_SIZE
#endif

#endif

/* @} */