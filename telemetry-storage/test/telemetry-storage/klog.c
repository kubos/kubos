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

int __wrap_klog_init_file(char *file_path, uint8_t file_path_len, uint32_t part_size, uint8_t max_parts)
{
    check_expected(file_path);
    check_expected(file_path_len);
    check_expected(part_size);
    check_expected(max_parts);
    return mock_type(int);
}

void __wrap_KLOG_TELEMETRY(unsigned level, const char *logger, const char *format, ...)
{
    /* Do nothing */
}

void __wrap_klog_cleanup(void)
{
    /* Do nothing */
}

