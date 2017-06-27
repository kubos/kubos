/*
 * KubOS Core Flight Services
 * Copyright (C) 2015 Kubos Corporation
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
  * @defgroup  KUBOS_CORE_KLOG Kubos Core KLog Interface
  * @addtogroup  KUBOS_CORE_KLOG
  * @{
  */

//TODO: add note. this is not the same thing as klogd (linux's kernel logging daemon)
//TODO: these descriptions

#ifndef KLOG_H
#define KLOG_H

#include <stdint.h>
#include <stdio.h>
#include <csp/csp_types.h>

//#define MODULE_LOG
#include "kubos-core/k_log.h"

#ifdef __cplusplus
extern "C" {
#endif

#ifndef KLOG_MAX_LINE
#define KLOG_MAX_LINE 255
#endif

/**
 * @brief Log message if level <= LOG_LEVEL
 */
#define KLOG(handle, level, logger, ...)    klog_write(handle, level, logger, __VA_ARGS__)
#define KLOG_ERR(handle, logger, ...)       KLOG(handle, LOG_ERROR, logger, __VA_ARGS__) /**< Error logging define for convenience */
#define KLOG_WARN(handle, logger, ...)      KLOG(handle, LOG_WARNING, logger, __VA_ARGS__) /**< Warning logging define for convenience */
#define KLOG_TELEMETRY(handle, logger, ...) KLOG(handle, LOG_TELEMETRY, logger, __VA_ARGS__) /**< Telemetry logging define for convenience */
#define KLOG_INFO(handle, logger, ...)      KLOG(handle, LOG_INFO, logger, __VA_ARGS__) /**< Info logging define for convenience */
#define KLOG_DEBUG(handle, logger, ...)     KLOG(handle, LOG_DEBUG, logger, __VA_ARGS__) /**< Debug logging define for convenience */


#define KLOG_SUFFIX_LEN 4 /**< Length of suffix added to KLog files  */
#define KLOG_PATH_LEN   255 /**< Maximum KLog file path length  */
#define KLOG_MAX_PATH   (KLOG_PATH_LEN - KLOG_SUFFIX_LEN - 1) /**< Maximum KLog file path length available to users */

#define KLOG_PART_SIZE_DEFAULT (1024 * 512) /**< Default file partition size  */
#define KLOG_MAX_PARTS_DEFAULT 4 /**< Default partition count limit  */

/**
 * KLog configuration structure
 */
typedef struct
{
    char *file_path; /**< Path to logging file */
    uint8_t file_path_len; /**< Character length of logging file path */
    uint32_t part_size; /**< Partition size */
    uint8_t max_parts; /**< Partition count limit */
    uint8_t klog_console_level; /**< Console logging level */
    uint8_t klog_file_level; /**< File logging level */
    bool klog_file_logging; /**< Specifies whether logging-to-file is enabled */
} klog_config;

/**
 * KLog handle
 */
typedef struct
{
    FILE *log_file; /**< Pointer to logging file */
    uint8_t current_part; /**< Current file partition in use */
    uint32_t current_part_size; /**< Size of current partition */
    klog_config config; /**< Pointer to KLog configuration */
} klog_handle;

/**
 * @brief Initialize KLog logging file
 *
 * This function will create a new log file and save the file
 * pointer into handle->log_file.
 *
 * @param[in] handle Pointer to logging handle
 *
 * @return int 0 on success, -1 on error
 */
int klog_init_file(klog_handle *handle);
/**
 * @brief Add message to the console
 *
 * This function will add a message to the console
 * if the passed message level is high enough
 *
 * @param[in] level Severity level of message
 * @param[in] logger Tag for message
 * @param[in] format Message to log
 */
void klog_console(unsigned level, const char *logger, const char *format, ...);
/**
 * @brief Add message to file
 *
 * This function will add a message to the specified logging
 * file if the passed message level is high enough
 *
 * @param[in] handle Pointer to logging handle
 * @param[in] level  Severity level of message
 * @param[in] logger Tag for message
 * @param[in] format Message to log
 */
void klog_file(klog_handle *handle, unsigned level, const char *logger, const char *format, ...);
/**
 * @brief Sync and close logging file
 *
 * @param[in] handle Pointer to logging handle
 */
void klog_cleanup(klog_handle *handle);

/**
 * @brief KLog write macro
 *
 * If the specified level is greater than or equal to the current configured minimum logging level,
 * calls logging function. Otherwise, ignores the input
 *
 * @param[in] handle Pointer to logging handle
 * @param[in] level  Severity level of message
 * @param[in] logger Tag for message
 */
#define klog_write(handle, level, logger, ...) do { \
    if (level <= ((handle)->config.klog_console_level)) { \
        klog_console(level, logger, __VA_ARGS__); \
    } \
    if (level <= ((handle)->config.klog_file_level) && ((handle)->config.klog_file_logging)) { \
        klog_file(handle, level, logger, __VA_ARGS__); \
    } \
} while (0)

#ifdef __cplusplus
}
#endif

#endif

/* @} */
