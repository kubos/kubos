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
#include <errno.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/unistd.h>
#include <sys/types.h>
#include <unistd.h>

#include "kubos-core/arch/k_timer.h"
#include "kubos-core/modules/klog.h"

uint8_t klog_console_level = LOG_INFO;
uint8_t klog_file_level = LOG_DEBUG;
bool klog_file_logging = false;

static FILE *_log_file = NULL;
static char *_file_path = NULL;
static uint8_t _file_path_len = 0;
static uint8_t _current_part = 0;
static uint32_t _current_part_size = 0;
static uint32_t _part_size;
static uint8_t _max_parts;

static void _next_log_file(void)
{
    char buf[KLOG_PATH_LEN];
    char *tail;
    uint32_t pos = 0;
    struct stat st;

    klog_cleanup();
    if (!_file_path) {
        return;
    }

    _current_part = -1;
    _current_part_size = 0;

    strncpy(buf, _file_path, _file_path_len);
    tail = buf + _file_path_len;

    for (uint8_t i = 0; i < _max_parts; i++) {
        sprintf(tail, ".%03d", i);

        if (stat(buf, &st) == -1) {
            if (errno == ENOENT) {
                klog_console(LOG_DEBUG, "klog", "creating %s", buf);
                _log_file = fopen(buf, "w+");
                _current_part = i;
                _current_part_size = 0;
                break;
            }
            continue;
        }

        if (st.st_size < (off_t) _part_size) {
            _log_file = fopen(buf, "r+");
            _current_part = i;
            pos = st.st_size - 1;
            _current_part_size = st.st_size;
            break;
        }
    }

    if (!_log_file) {
        // no empty or partial log file found, rotate
        _current_part++;
        if (_current_part > _max_parts) {
            _current_part = 0;
        }

        sprintf(tail, ".%03d", _current_part);
        remove(buf);

        klog_console(LOG_DEBUG, "klog", "rotating to %s", buf);
        _log_file = fopen(buf, "w+");
    }

    if (_log_file) {
        klog_console(LOG_INFO, "klog", "logging to %s", buf);
        klog_file_logging = true;
        if (pos > 0) {
            fseek(_log_file, 0, SEEK_END);
        }
    }
}

int klog_init_file(char *file_path, uint8_t file_path_len,
                   uint32_t part_size, uint8_t max_parts)
{
    if (!file_path || file_path_len > KLOG_MAX_PATH) {
        return -EINVAL;
    }

    _file_path = file_path;
    _file_path_len = file_path_len;
    _part_size = part_size;
    _max_parts = max_parts;

    _next_log_file();
    return klog_file_logging ? 0 : -1;
}

static inline char *_level_str(unsigned level)
{
    switch (level) {
        case LOG_ERROR: return "E";
        case LOG_WARNING: return "W";
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
    struct timeval time;
    int written = 0;
    k_timer_now_time(&time);

    written += fprintf(f, "%010d.%03d %s:%s ", (int) time.tv_sec,
                       (int) time.tv_usec / 1000, logger,
                       _level_str(level));
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

void klog_file(unsigned level, const char *logger, const char *format, ...)
{
    va_list args;

    va_start(args, format);
    if (!_log_file) {
        _next_log_file();
        if (!_log_file) {
            va_end(args);
            return;
        }
    }

    _current_part_size += _klog(_log_file, level, logger, format, args);

    va_end(args);
    fsync(fileno(_log_file));

    if (_current_part_size >= _part_size) {
        _next_log_file();
    }
}

void klog_cleanup(void)
{
    if (_log_file) {
        fsync(fileno(_log_file));
        fclose(_log_file);
    }

    _log_file = NULL;
}

#ifndef HAVE_FSYNC
int fsync(int fd)
{
    return 0;
}
#endif
