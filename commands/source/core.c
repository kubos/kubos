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

#include <stdio.h>
#include <stdbool.h>
#include <unistd.h>
#include <stdint.h>

#include <kubos-hal-iobc/supervisor.h>

#include "commands/errors.h"
#include "commands/commands.h"

int parse_and_run(char * arg);

int main(int argc, char **argv)
{
    char command_string[DEFAULT_COMMAND_STR_LEN] = {0};

    if (!core_parse_args(argc, argv, command_string))
    {
        printf("An error occurred parsing arguments\n");
        return 1;
    }

    return get_and_run_command(command_string);
}


int ping()
{
    printf("Pong!\n");
    return 0;
}


int build_info()
{
    int retries;
    bool result = false;
    supervisor_version_t version = {0};

    for (retries = 0; retries < SUPERVISOR_MAX_REQUEST_RETRIES; retries++)
    {
        result = supervisor_get_version(&version);
        if (result)
        {
            printf("iOBC Supervisor Version: %c.%c.%c\n", version.fields.major_version, version.fields.minor_version, version.fields.patch_version);
            return NO_ERR;
        }
    }

    printf("Error: Exceeded the maximum number of supervisor retries. Aborting the info command.\n");
    return GENERIC_ERR;
}


int reboot()
{
    int retries;
    bool result = false;

    for (retries = 0; retries < SUPERVISOR_MAX_REQUEST_RETRIES; retries++)
    {
        result = supervisor_powercycle();
        if (result)
        {
            return NO_ERR;
        }
    }

    printf("Error: Exceeded the maximum number of supervisor retries. Aborting the power cycle.\n");
    return GENERIC_ERR;
}


int reset()
{
    int retries;
    bool result = false;

    for (retries = 0; retries < SUPERVISOR_MAX_REQUEST_RETRIES; retries++)
    {
        result = supervisor_reset();
        if (result)
        {
            return NO_ERR;
        }
    }

    printf("Error: Exceeded the maximum number of supervisor retries. Aborting the reset\n");
    return GENERIC_ERR;
}


int emergency_reset()
{
    int retries;
    bool result = false;

    for (retries = 0; retries < SUPERVISOR_MAX_REQUEST_RETRIES; retries++)
    {
        result = supervisor_emergency_reset();
        if (result)
        {
            return NO_ERR;
        }
    }

    printf("Error: Exceeded the maximum number of supervisor retries. Aborting the emergency reset\n");
    return GENERIC_ERR;
}

