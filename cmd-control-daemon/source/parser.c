#include <argp.h>
#include <csp/csp.h>
#include <stdio.h>
#include <stdarg.h>
#include <stdlib.h>
#include <string.h>

#include "command-and-control/types.h"
#include "cmd-control-daemon/daemon.h"

static struct argp_option options[] =
{
    {"action",   'f', "act", 0,
        "Specify the action to execute. Allows values are [execute, status, version, help]"},
    {0}
};


static int parse_opt (int key, char *arg, struct argp_state *state)
{
    cnc_command_packet *arguments = state->input;
    int idx;
    switch (key)
    {
        case ARGP_KEY_ARG:
            {
                printf("ARG: %s\n", arg);
                switch(arguments->arg_count++)
                {
                    case 0:
                        arguments->action = execute;
                        break;
                    case 1:
                        strcpy(arguments->cmd_name, arg);
                        break;
                    default:
                        idx = arguments->arg_count - 3; //3 because of the increment
                        strcpy(arguments->args[idx], arg);
                }
            }
            break;
        case ARGP_KEY_END:
            {
                //Do some validation to make sure we have a minimum number of arguments
                arguments->arg_count = arguments->arg_count - 2;
                printf("End of parsing\n");
            }
            break;
    }
    return 0;
}


int get_num_args(char* string){
    int count = 0, i = 0;
    while (string[++i])
    {
        if (string[i] == ' ') count++;
    }
    return count + 1;
}


//TODO: Make all the parsing help more helpful and more accurate
static char args_doc[] = "action group-name [following args]";
static char doc[] = "Command Doc";

static struct argp argp = { options, parse_opt, "WORD[WORD]"};

bool parse (char * args, cnc_command_packet * my_arguments)
{
    int res, argsc;
    char * sub_str;
    char * tok = " ";
    int idx = 0;

    my_arguments->arg_count = 0;

    int my_argc = get_num_args(args);
    //TODO: statically allocate and make it play nicely with argp
    char ** result = malloc(sizeof(char*) * my_argc);

    //Splitting string to gernerate an "argc, **argv" to pass into the argument parser.
    sub_str = strtok (args, tok);
    while (sub_str != NULL)
    {
        result[idx++] = sub_str;
        sub_str = strtok (NULL, tok);
    }

    int flags = ARGP_PARSE_ARGV0 | ARGP_NO_ERRS;
    printf("Parsing args\n");

    argp_parse (&argp, my_argc, result, flags, 0, my_arguments);
    //All the allocated memory is freed at the end of main()
    return true;
}

