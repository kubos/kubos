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
#include "telemetry-storage/disk.h"
#include <cmocka.h>

uint16_t __wrap_disk_save_string(const char *file_path, char *data_buffer, uint16_t data_len)
{
    check_expected(file_path);
    check_expected(data_buffer);
    check_expected(data_len);
    return mock_type(uint16_t);
}


