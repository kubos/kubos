#include <argp.h>
#include <csp/csp.h>
#include <stdio.h>
#include <stdarg.h>
#include <stdlib.h>
#include <string.h>

#include "command-and-control/types.h"
#include "cmd-control-daemon/daemon.h"

//Action string hash values
#define EXEC_HASH    6385204650
#define HELP_HASH    6385292014
#define OUTPUT_HASH  6953876217206
#define STATUS_HASH  6954030894409

//djb2 string hash function
unsigned long get_hash(char *str)
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
        case EXEC_HASH:
            wrapper->command_packet->action = execute;
            break;
        case HELP_HASH:
            wrapper->command_packet->action = help;
            break;
        case OUTPUT_HASH:
            wrapper->command_packet->action = output;
            break;
        case STATUS_HASH:
            wrapper->command_packet->action = status;
            break;
        default:
            snprintf(wrapper->output, sizeof(wrapper->output) - 1, "Requested action: %s, is not available\n", arg);
            wrapper->err = true;
            return false;
    }
    return true;
}

//This is required by argp and will be useful if we add options to the daemon.
//An example option could be run a command but do not send the output or something like that.
static struct argp_option options[] =
{
    {0}
};


static int parse_opt (int key, char *arg, struct argp_state *state)
{
    cnc_command_wrapper *arguments = state->input;
    int idx;
    switch (key)
    {
        case ARGP_KEY_ARG:
            switch(arguments->command_packet->arg_count++)
            {
                case 0:
                    if (!set_action(arg, arguments)) {
                        state->next = state->argc; //Abort parsing the remaining args
                        send_result(arguments);
                    }
                    break;
                case 1:
                    strcpy(arguments->command_packet->cmd_name, arg);
                    break;
                default:
                    idx = arguments->command_packet->arg_count - 3; //3 because of the increment
                    strcpy(arguments->command_packet->args[idx], arg);
            }
            break;
        case ARGP_KEY_END:
            if (strlen(arguments->command_packet->cmd_name) == 0) //TODO: Effectively validate the action
            {
                arguments->err = true;
                snprintf(arguments->output, sizeof(arguments->output) - 1, "received incorrect command or action argument\n"); //Would be helpful to give the help message here..
                send_result(arguments);
            }
            arguments->command_packet->arg_count = arguments->command_packet->arg_count - 2;
    }
    return 0;
}


int get_num_args(char* string){
    int count = 0, i = 1;
    while (string[i++])
    {
        if (string[i] == ' ') count++;
    }
    return count + 1;
}


static char args_doc[] = "Action Group-Name [following args]";
static char doc[] = "CNC Daemon - Execute commands through the Kubos command and control framework";

static struct argp argp = { options, parse_opt, args_doc, doc};

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

    argp_parse (&argp, my_argc, result, flags, 0, wrapper);
    free(result);
    if (wrapper->err)
        return false;
    return true;
}

