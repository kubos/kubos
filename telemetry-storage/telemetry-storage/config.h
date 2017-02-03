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
 * @defgroup Config
 * @addtogroup Config
 * @brief Configuration settings for telemetry storage
 * @{
 */

#ifndef TELEMETRY_STORAGE_CONFIG_H
#define TELEMETRY_STORAGE_CONFIG_H

/*! Maximum size/length of the filename buffer */
#ifndef YOTTA_CFG_TELEMETRY_STORAGE_FILE_NAME_BUFFER_SIZE
#define FILE_NAME_BUFFER_SIZE 128
#else
#define FILE_NAME_BUFFER_SIZE YOTTA_CFG_TELEMETRY_STORAGE_FILE_NAME_BUFFER_SIZE
#endif

/*! Maximum size/length of the storage buffer */
#ifndef YOTTA_CFG_TELEMETRY_STORAGE_DATA_BUFFER_SIZE
#define DATA_BUFFER_SIZE 64
#else
#define DATA_BUFFER_SIZE YOTTA_CFG_TELEMETRY_STORAGE_DATA_BUFFER_SIZE
#endif

/*! Telemetry log file part size */
#ifndef YOTTA_CFG_TELEMETRY_STORAGE_DATA_PART_SIZE
#define DATA_PART_SIZE 51200
#else
#define DATA_PART_SIZE YOTTA_CFG_TELEMETRY_STORAGE_DATA_PART_SIZE
#endif

/*! Telemetry log file max parts */
#ifndef YOTTA_CFG_TELEMETRY_STORAGE_DATA_MAX_PARTS
#define DATA_MAX_PARTS 10
#else
#define DATA_MAX_PARTS YOTTA_CFG_TELEMETRY_STORAGE_DATA_MAX_PARTS
#endif

/*! Output format (CSV (0), HEX (1), etc) */
#ifndef YOTTA_CFG_TELEMETRY_STORAGE_DATA_OUTPUT_FORMAT
#define DATA_OUTPUT_FORMAT FORMAT_TYPE_CSV
#else
#define DATA_OUTPUT_FORMAT YOTTA_CFG_TELEMETRY_STORAGE_DATA_OUTPUT_FORMAT
#endif

/*! The telemetry publishers for storage to subscribe to and store */
#ifndef YOTTA_CFG_TELEMETRY_STORAGE_SUBSCRIPTIONS
#define STORAGE_SUBSCRIPTIONS 0x0
#else
#define STORAGE_SUBSCRIPTIONS YOTTA_CFG_TELEMETRY_STORAGE_SUBSCRIPTIONS
#endif

/*! The interval to wait between subscribe attempts */
#ifndef YOTTA_CFG_TELEMETRY_STORAGE_SUBSCRIBE_RETRY_INTERVAL
#define STORAGE_SUBSCRIBE_RETRY_INTERVAL 50
#else
#define STORAGE_SUBSCRIBE_RETRY_INTERVAL YOTTA_CFG_TELEMETRY_STORAGE_SUBSCRIBE_RETRY_INTERVAL
#endif

/*! Telemetry storage receive task stack depth */
#ifndef YOTTA_CFG_TELEMETRY_STORAGE_STACK_DEPTH
#define STORAGE_TASK_STACK_DEPTH 1000
#else
#define STORAGE_TASK_STACK_DEPTH YOTTA_CFG_TELEMETRY_STORAGE_STACK_DEPTH
#endif

/*! Telemetry storage receive task priority */
#ifndef YOTTA_CFG_TELEMETRY_STORAGE_TASK_PRIORITY
#define STORAGE_TASK_PRIORITY 0
#else
#define STORAGE_TASK_PRIORITY YOTTA_CFG_TELEMETRY_STORAGE_TASK_PRIORITY
#endif

#endif

/* @} */