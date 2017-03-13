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

#include <argp.h>
#include <csp/csp.h>
#include <stdio.h>
#include <stdarg.h>
#include <stdlib.h>
#include <string.h>

#include "command-and-control/types.h"

//Action string hash values
#define BASE_HASH    5381
#define EXEC_HASH    6385204650
#define HELP_HASH    6385292014
#define OUTPUT_HASH  6953876217206
#define STATUS_HASH  6954030894409

static int parse_opt (int key, char *arg, struct argp_state *state);

//This is required by argp and will be useful if we add options to the daemon.
//An example option could be run a command but do not send the output or something like that.
static struct argp_option options[] =
{
    {0}
};

static char args_doc[] = "Action Group-Name [following args]";
static char doc[] = "CNC - Execute commands through the Kubos command and control framework";
static struct argp argp = { options, parse_opt, args_doc, doc};


//djb2 string hash function
unsigned long get_hash(char *str)
{
    unsigned long hash = BASE_HASH;
    int c;
    while (c = *str++)
    {
        hash = ((hash << 5) + hash) + c;
    }

    return hash;
}


bool set_action(char* arg, CNCCommandPacket * command_packet)
{
    unsigned long hash;

    if (command_packet == NULL)
    {
        return false;
    }

    hash = get_hash(arg);

    switch (hash)
    {
        case EXEC_HASH:
            command_packet->action = EXECUTE;
            break;
        case HELP_HASH:
            command_packet->action = HELP;
            break;
        case OUTPUT_HASH:
            command_packet->action = OUTPUT;
            break;
        case STATUS_HASH:
            command_packet->action = STATUS;
            break;
        default:
            fprintf(stderr, "Requested action: %s, is not available\n", arg);
            return false;
    }
    return true;
}


static int parse_opt(int key, char *arg, struct argp_state *state)
{
    CNCCommandPacket * command_packet = state->input;
    int idx;
    switch (key)
    {
        case ARGP_KEY_ARG:
            switch(command_packet->arg_count++)
            {
                case 0:
                    if (!set_action(arg, command_packet))
                    {
                        state->next = state->argc; //Abort parsing the remaining args
                    }
                    break;
                case 1:
                    strcpy(command_packet->cmd_name, arg);
                    break;
                default:
                    idx = command_packet->arg_count - 3; //3 because of the increment
                    strcpy(command_packet->args[idx], arg);
            }
            break;
        case ARGP_KEY_END:
            if (strlen(command_packet->cmd_name) == 0) //TODO: Effectively validate the action
            {
                fprintf(stderr, "received incorrect command or action argument\n"); //Would be helpful to give the help message here..
            }
            command_packet->arg_count = command_packet->arg_count - 2;
            break;
    }
    return 0;
}


bool cnc_client_parse_cl_args(CNCCommandPacket * command_packet, int argc, char ** argv)
{
    int result;
    int flags = 0;
    if (command_packet == NULL || argv == NULL)
    {
        return false;
    }

    result = argp_parse (&argp, argc, argv, flags, 0, command_packet);
    if (result == 0)
    {
        return true;
    }
    else
    {
        //Do some error handling
        return false;
    }
}

