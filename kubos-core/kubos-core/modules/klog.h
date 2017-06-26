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

#define KLOG(handle, level, logger, ...)    klog_write(handle, level, logger, __VA_ARGS__)
#define KLOG_ERR(handle, logger, ...)       KLOG(handle, LOG_ERROR, logger, __VA_ARGS__)
#define KLOG_WARN(handle, logger, ...)      KLOG(handle, LOG_WARNING, logger, __VA_ARGS__)
#define KLOG_TELEMETRY(handle, logger, ...) KLOG(handle, LOG_TELEMETRY, logger, __VA_ARGS__)
#define KLOG_INFO(handle, logger, ...)      KLOG(handle, LOG_INFO, logger, __VA_ARGS__)
#define KLOG_DEBUG(handle, logger, ...)     KLOG(handle, LOG_DEBUG, logger, __VA_ARGS__)


#define KLOG_SUFFIX_LEN 4
#define KLOG_PATH_LEN   255
#define KLOG_MAX_PATH   (KLOG_PATH_LEN - KLOG_SUFFIX_LEN - 1)

#define KLOG_PART_SIZE_DEFAULT (1024 * 512)
#define KLOG_MAX_PARTS_DEFAULT 4

typedef struct
{
    char *file_path;
    uint8_t file_path_len;
    uint32_t part_size;
    uint8_t max_parts;
    uint8_t klog_console_level;
    uint8_t klog_file_level;
    bool klog_file_logging;
} klog_config;

typedef struct
{
    FILE *log_file;
    uint8_t current_part;
    uint32_t current_part_size;
    klog_config config;
} klog_handle;

int klog_init_file(klog_handle *handle);
void klog_console(unsigned level, const char *logger, const char *format, ...);
void klog_file(klog_handle *handle, unsigned level, const char *logger, const char *format, ...);
void klog_cleanup(klog_handle *handle);

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
