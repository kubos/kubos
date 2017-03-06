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

#define PING_HASH   6385583923
#define NUM_ARGS    5   //In the future these values will be inherited from the command
#define ARG_LEN     20  //and control framework configuration. With this build config that's not simple to do.

int parse_and_run(char * arg);

int execute(int argc, char argv[NUM_ARGS][ARG_LEN])
{
    switch (argc)
    {
        case 1:
            return parse_and_run(argv[0]);
        default:
            printf("Error: incorrect number of arguments.\n");
            return 1;
    }
    return 0;
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


//djb2 string hash function
unsigned long get_hash(char *str)
{
    unsigned long hash = 5381;
    int c;
    while (c = *str++)
    {
        hash = ((hash << 5) + hash) + c;
    }

    return hash;
}


int parse_and_run(char* arg)
{
    unsigned long hash = get_hash(arg);
    switch (hash)
    {
        case PING_HASH:
            printf("PONG!\n");
            return 0;
            break;
        default:
            printf("Received unknown command: %s\n", arg);
            return 1;
    }
}

