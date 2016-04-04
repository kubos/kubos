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
#ifndef KLOG_H
#define KLOG_H

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>

//#define MODULE_LOG
#include "kubos-core/k_log.h"

#ifdef __cplusplus
extern "C" {
#endif

#ifndef KLOG_MAX_LINE
#define KLOG_MAX_LINE 255
#endif

#define KLOG(level, logger, ...)  klog_write(level, logger, __VA_ARGS__)
#define KLOG_ERR(logger, ...)     KLOG(LOG_ERROR, logger, __VA_ARGS__)
#define KLOG_WARN(logger, ...)    KLOG(LOG_WARNING, logger, __VA_ARGS__)
#define KLOG_INFO(logger, ...)    KLOG(LOG_INFO, logger, __VA_ARGS__)
#define KLOG_DEBUG(logger, ...)   KLOG(LOG_DEBUG, logger, __VA_ARGS__)

#define KLOG_SUFFIX_LEN 4
#define KLOG_PATH_LEN   255
#define KLOG_MAX_PATH   (KLOG_PATH_LEN - KLOG_SUFFIX_LEN - 1)

#define KLOG_PART_SIZE_DEFAULT (1024 * 512)
#define KLOG_MAX_PARTS_DEFAULT 4

extern uint8_t klog_console_level;
extern uint8_t klog_file_level;
extern bool klog_file_logging;

int klog_init_file(char *file_path, uint8_t file_path_len,
                   uint32_t part_size, uint8_t max_parts);

void klog_console(unsigned level, const char *logger, const char *format, ...);
void klog_file(unsigned level, const char *logger, const char *format, ...);
void klog_cleanup(void);

#define klog_write(level, logger, ...) do { \
    if (level <= klog_console_level) { \
        klog_console(level, logger, __VA_ARGS__); \
    } \
    if (level <= klog_file_level && klog_file_logging) { \
        klog_file(level, logger, __VA_ARGS__); \
    } \
} while (0)

#ifdef __cplusplus
}
#endif

#endif
