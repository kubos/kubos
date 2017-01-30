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

#ifndef TARGET_LIKE_MSP430


#include <errno.h>
#include <stdarg.h>
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/unistd.h>
#include <sys/types.h>
#include <unistd.h>

#include <csp/arch/csp_time.h>
#include "kubos-core/modules/klog.h"

static void _next_log_file(klog_handle *handle, klog_config config)
{
    char buf[KLOG_PATH_LEN];
    char *tail;
    uint32_t pos = 0;
    struct stat st;

    klog_cleanup(handle);
    if (!config.file_path) {
        return;
    }

    strncpy(buf, config.file_path, config.file_path_len);
    tail = buf + config.file_path_len;

    for (uint8_t i = 0; i < config.max_parts; i++) {
        sprintf(tail, ".%03d", i);

        if (stat(buf, &st) == -1) {
            if (errno == ENOENT) {
                klog_console(LOG_DEBUG, "klog", "creating %s", buf);
                handle->log_file = fopen(buf, "w+");
                handle->current_part = i;
                handle->current_part_size = 0;
                break;
            }
            continue;
        }

        if (st.st_size < (off_t) config.part_size) {
            handle->log_file = fopen(buf, "r+");
            handle->current_part = i;
            pos = st.st_size - 1;
            handle->current_part_size = st.st_size;
            break;
        }
    }

    if (!handle->log_file) {
        // no empty or partial log file found, rotate
        handle->current_part++;
        if (handle->current_part > config.max_parts) {
            handle->current_part = 0;
        }

        sprintf(tail, ".%03d", handle->current_part);
        remove(buf);

        klog_console(LOG_DEBUG, "klog", "rotating to %s", buf);
        handle->log_file = fopen(buf, "w+");
    }

    if (handle->log_file) {
        klog_console(LOG_INFO, "klog", "logging to %s", buf);
        if (pos > 0) {
            fseek(handle->log_file, 0, SEEK_END);
        }
    }
}

klog_handle klog_init_file(klog_config config)
{
    klog_handle ret_handle = { .log_file = NULL, .current_part = -1, \
                               .current_part_size = 0 };
    
    if (!config.file_path || config.file_path_len > KLOG_MAX_PATH) {
        errno = -EINVAL;
        return ret_handle;
    }
    _next_log_file(&ret_handle, config);
    return ret_handle;
}

static inline char *_level_str(unsigned level)
{
    switch (level) {
        case LOG_ERROR: return "E";
        case LOG_WARNING: return "W";
        case LOG_TELEMETRY: return "T";
        case LOG_INFO: return "I";
        case LOG_DEBUG: return "D";
        case LOG_NONE:
        default:
            return "N";
    }
}

static int _klog(FILE *f, unsigned level, const char *logger,
                 const char *format, va_list args)
{
    int written = 0;
    uint32_t millis = csp_get_ms();

    written += fprintf(f, "%010ld.%03ld %s:%s ", millis / 1000, millis % 1000,
                       logger, _level_str(level));
    written += vfprintf(f, format, args);
    written += fprintf(f, "\n");
    return written;
}

void klog_console(unsigned level, const char *logger, const char *format, ...)
{
    va_list args;
    va_start(args, format);

    _klog(level == LOG_ERROR ? stderr : stdout, level, logger, format, args);

    va_end(args);
}

void klog_file(klog_handle *handle, klog_config config, unsigned level, const char *logger, const char *format, ...)
{
    va_list args;

    va_start(args, format);
    if (!handle->log_file) {
        _next_log_file(handle, config);
        if (!handle->log_file) {
            va_end(args);
            return;
        }
    }
    handle->current_part_size += _klog(handle->log_file, level, logger, format, args);

    va_end(args);
    fsync(fileno(handle->log_file));

    if (handle->current_part_size >= config.part_size) {
        _next_log_file(handle, config);
    }
}

void klog_cleanup(klog_handle *handle)
{
    if (handle->log_file) {
        fsync(fileno(handle->log_file));
        fclose(handle->log_file);
    }

    handle->log_file = NULL;
}

#ifndef HAVE_FSYNC
int fsync(int fd)
{
    (void)fd;
    return 0;
}
#endif

#endif
