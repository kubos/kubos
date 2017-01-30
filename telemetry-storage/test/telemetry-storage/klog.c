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
#include <kubos-core/modules/klog.h>
#include <cmocka.h>

klog_handle __wrap_klog_init_file(klog_config config)
{
    klog_handle ret_handle = { .log_file = NULL, .current_part = 1, \
                               .current_part_size = 0 };
                               
    check_expected(config.file_path);
    check_expected(config.file_path_len);
    check_expected(config.part_size);
    check_expected(config.max_parts);
    check_expected(config.klog_console_level);
    check_expected(config.klog_file_level);
    check_expected(config.klog_file_logging);
    return ret_handle;
}

void __wrap_KLOG_TELEMETRY(klog_handle *handle, klog_config config, const char *logger, const char *format)
{
    /* Do nothing */
}

void __wrap_klog_cleanup(klog_handle *handle,)
{
    /* Do nothing */
}

