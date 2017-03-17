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
#include <unistd.h>
#include <sys/reboot.h>
#include <sys/utsname.h>

#include "parser.h"
int parse_and_run(char * arg);

int execute(int argc, char **argv)
{
    char command_string[DEFAULT_COMMAND_STR_LENGTH] = {0};

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
    struct utsname uname_data;
    uname(&uname_data);
    printf("Version: %s\n", uname_data.version);
    return 0;
}

int exec_reboot()
{
    int res;
    //TODO: Log (once logging is a thing): Reboot triggered by C&C at time..
    sync(); //Sync all pending filesystem changes (Logging)
    if (res = reboot(RB_AUTOBOOT) != 0)
    {
        printf("There was an error rebooting the system. Received error code: %i\n", res);
        return res;
    }
}
