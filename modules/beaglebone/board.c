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

#include <dirent.h>
#include <stdio.h>
#include <string.h>
#include <sys/errno.h>

#include "beaglebone.h"
#include "board_internal.h"

#define ENABLE_DEBUG (1)
#include "debug.h"

void _native_init_uart(int uart, char *path)
{
    (void) path;

    char slots_path[64], uart_str[16];
    size_t result;

    // Export the UART through the BeagleBone's cape manager
    DIR *devices = opendir("/sys/devices");
    struct dirent *entry;

    while ((entry = readdir(devices)) != NULL) {
        if (strncmp("bone_capemgr.", entry->d_name, 13) == 0) {
            break;
        }
    }

    if (entry == NULL) {
        DEBUG("Couldn't find bone_capemgr.*\n");
        return;
    }

    snprintf(slots_path, 64, "/sys/devices/%s/slots", entry->d_name);

    DEBUG("slots path: %s\n", slots_path);

    FILE *slots = fopen(slots_path, "w+");
    if (!slots) {
        DEBUG("Error opening %s: %s\n", slots_path, strerror(errno));
        return;
    }

    result = (size_t) snprintf(uart_str, 16, "BB-UART%d", uart + 1);
    DEBUG("Enable %s in %s\n", uart_str, slots_path);

    result = fwrite(uart_str, (int) result + 1, 1, slots);
    fclose(slots);
}
