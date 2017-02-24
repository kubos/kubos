#include <argp.h>
#include <csp/csp.h>
#include <stdio.h>
#include <stdarg.h>
#include <stdlib.h>
#include <string.h>

#include "command-and-control/types.h"
#include "cmd-control-daemon/daemon.h"

//Action string hash values
#define EXEC    6385204650
#define HELP    6385292014
#define OUTPUT  6953876217206
#define STATUS  6954030894409

unsigned long get_hash(char *str) //Should this function be in some higher level kubos utility package?
{
    unsigned long hash = 5381;
    int c;
    while (c = *str++)
        hash = ((hash << 5) + hash) + c;

    return hash;
}


bool set_action(char* arg, cnc_command_wrapper * wrapper)
{
    unsigned long hash = get_hash(arg);
    switch (hash)
    {
        case EXEC:
            wrapper->command_packet->action = execute;
            break;
        case HELP:
            wrapper->command_packet->action = help;
            break;
        case OUTPUT:
            wrapper->command_packet->action = output;
            break;
        case STATUS:
            wrapper->command_packet->action = status;
            break;
        default:
            fprintf(wrapper->output, "Requested action: %s, with hash %lu is not available\n", arg, hash);
            wrapper->err = true;
            return false;
    }
    return true;
}


static struct argp_option options[] =
{
    {"action",   'f', "act", 0,
        "Specify the action to execute. Allows values are [execute, status, version, help]"},
    {0}
};


static int parse_opt (int key, char *arg, struct argp_state *state)
{
    cnc_command_wrapper *arguments = state->input;
    int idx;
    switch (key)
    {
        case ARGP_KEY_ARG:
            {
                printf("ARG: %s\n", arg);
                switch(arguments->command_packet->arg_count++)
                {
                    case 0:
                        if (!set_action(arg, arguments)) {
                            state->next = state->argc; //Abort parsing the remaining args

                            return;
                        }
                        break;
                    case 1:
                        strcpy(arguments->command_packet->cmd_name, arg);
                        break;
                    default:
                        idx = arguments->command_packet->arg_count - 3; //3 because of the increment
                        strcpy(arguments->command_packet->args[idx], arg);
                }
            }
            break;
        case ARGP_KEY_END:
            {
                //Do some validation to make sure we have a minimum number of arguments
                if (strlen(arguments->command_packet->cmd_name) == 0 || !arguments->command_packet->action)
                {
                    send_usage_error(arguments);
                    fprintf(stderr, "received incorrect command or action argument\n");
                }
                arguments->command_packet->arg_count = arguments->command_packet->arg_count - 2;
                printf("End of parsing\n");
                return -1;
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

bool parse (char * args, cnc_command_wrapper * wrapper)
{
    int res, argsc;
    char * sub_str;
    char * tok = " ";
    int idx = 0;

    wrapper->command_packet->arg_count = 0;

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

    argp_parse (&argp, my_argc, result, flags, 0, wrapper);
    free(result);
    if (wrapper->err)
        return false;
    return true;
}

