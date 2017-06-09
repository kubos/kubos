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

#include <commands/errors.h>
#include <commands/commands.h>

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


int status(int argc, char **argv)
{
    printf("Status is not implemented for the core commands\n");
    return 0;
}


int output(int argc, char **argv)
{
    printf("Output is not implemented for the core commands\n");
    return 0;
}


int help(int argc, char **argv)
{
    printf("Core C&C Commands.\nUsage '<action> <subcommand name>'\n");
    return 0;
}


int ping()
{
    printf("Pong!\n");
    return 0;
}


int build_info()
{
    bool result = true;
    supervisor_version_t version = {0};

    result = supervisor_get_version(&version);
    if (!result)
    {
        printf("There was an error getting the supervisor version information. Error: %i\n", result);
        return result;
    }
    printf("iOBC Supervisor Version: %u.%u.%u\n", version.fields.major_version, version.fields.minor_version, version.fields.patch_version);
    return result;
}

//TODO: I don't like having 3 nearly identical functions. I'm working on implementing the reboot type as an option
//Theres an issue in the client that is preventing that implementation at the moment.
int reboot()
{
    bool result = true;

    result = supervisor_powercycle();
    if (!result)
    {
        printf("There was an error requesting the iOBC power cycle.\n");
        return GENERIC_ERR;
    }

    return NO_ERR;
}


int reset()
{
    bool result = true;

    result = supervisor_reset();
    if (!result)
    {
        printf("There was an error requesting the iOBC power cycle.\n");
        return GENERIC_ERR;
    }

    return NO_ERR;
}


int emergency_reset()
{
    bool result = true;

    result = supervisor_emergency_reset();
    if (!result)
    {
        printf("There was an error requesting the iOBC power cycle.\n");
        return GENERIC_ERR;
    }

    return NO_ERR;
}
