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

static int parse_opt (int key, char *arg, struct argp_state *state);

//This is required by argp and will be useful if we add options to the daemon.
//An example option could be run a command but do not send the output or something like that.
static struct argp_option options[] =
{
    {0}
};

static char args_doc[] = "Executable-Name [following args]";
static char doc[] = "CNC - Execute commands through the Kubos command and control framework";
static struct argp argp = { options, parse_opt, args_doc, doc};

static int parse_opt(int key, char *arg, struct argp_state *state)
{
    CNCCommandPacket * command_packet;
    int idx;

    //Delay NULL checking arg. This function is run a large number of times, some of which
    //arg is NULL and it should be (ie. when the parser is initializing, finishing, etc).

    if (state == NULL || state->input == NULL)
    {
        return 1;
    }

    command_packet = state->input;

    switch (key)
    {
        case ARGP_KEY_ARG:
            if (arg == NULL)
            {
                return ARGP_ERR_UNKNOWN;
            }

            switch(command_packet->arg_count++)
            {
                case 0:
                    strcpy(command_packet->cmd_name, arg);
                    break;
                default:
                    //The array index is arg_count-2 because of the first cmd_name value and the increment
                    idx = command_packet->arg_count - 2;
                    strcpy(command_packet->args[idx], arg);
            }
            break;
        case ARGP_KEY_END:
            if (strlen(command_packet->cmd_name) == 0)
            {
                fprintf(stderr, "received incorrect command or action argument\n");
            }
            command_packet->arg_count = command_packet->arg_count - 1;
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

