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
#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#include <argp.h>

#include <commands/parser.h>
#include <commands/commands.h>

#define BASE_HASH             5381
#define PING_HASH             6385583923
#define INFO_HASH             6385337553
#define RESET_HASH            210726503048         //reset
#define REBOOT_HASH           6953974000496        //power cycle
#define E_RESET_HASH          5359270898672569524  //emergency-reset

static char args_doc[] = "Usage: [ping, info, reboot, reset, emergency-reset]";
static char doc[] = "Core Command Library - The basic core commands of KubOS";
static int parse_opt (int key, char *arg, struct argp_state *state);

//This is required by argp and will be useful if we add options to the daemon.
//An example option could be run a command but do not send the output or something like that.
static struct argp_option options[] =
{
    {0}
};

static struct argp argp = { options, parse_opt, args_doc, doc};

//djb2 string hash function
unsigned long get_hash(char *str)
{
    int c;
    unsigned long hash = BASE_HASH;

    while (c = *str++)
    {
        hash = ((hash << 5) + hash) + c;
    }

    return hash;
}


static int parse_opt (int key, char *arg, struct argp_state *state)
{
    char * cmd_string = state->input;
    switch (key)
    {
        case ARGP_KEY_ARG:
            snprintf(cmd_string, DEFAULT_COMMAND_STR_LEN, "%s", arg);
            break;
        case ARGP_KEY_END:
            break;
    }
    return 0;
}


bool core_parse_args(int argc, char ** argv, char * cmd_string)
{
    int flags;

    if (argv == NULL)
    {
        return false;
    }

    flags = 0;

    if (argp_parse (&argp, argc, argv, flags, 0, cmd_string) != 0)
    {
        return false;
    }

    return true;
}


int get_and_run_command(char * command_name)
{
    unsigned long hash = get_hash(command_name);
    switch (hash)
    {
        case PING_HASH:
            return ping();
            break;
        case INFO_HASH:
            return build_info();
            break;
        case RESET_HASH:
            return reset();
            break;
        case REBOOT_HASH:
            return reboot();
            break;
        case E_RESET_HASH:
            return emergency_reset();
            break;
        default:
            printf("Recevied unknown command: %s\n", command_name);
            return 1;
            break;
    }
}

