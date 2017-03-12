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

#include <csp/csp.h>
#include <dlfcn.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <unistd.h>

#include "cmd-control-daemon/daemon.h"
#include "tinycbor/cbor.h"

#define CBOR_BUF_SIZE YOTTA_CFG_CSP_MTU


bool parse_command_cbor(csp_packet_t * packet, char * command)
{
    CborParser parser;
    CborValue map, element;
    size_t len;

    CborError err = cbor_parser_init((uint8_t*) packet->data, packet->length, 0, &parser, &map);
    if (err)
    {
        return false;
    }

    err = cbor_value_map_find_value(&map, "ARGS", &element);
    if (err || cbor_value_copy_text_string(&element, command, &len, NULL))
    {
        return false;
    }
    return true;
}


bool file_exists(char * path) //Should this live in a higher level module utility?
{
    if ( access(path, F_OK) != -1)
    {
        return true;
    }
    return false;
}


bool load_command(CNCWrapper * wrapper, void ** handle, lib_function * func)
{
    int return_code;
    char so_path[SO_PATH_LENGTH];

    if (wrapper == NULL || handle == NULL || func == NULL)
    {
        return false;
    }

    // so_len - the format specifier length (-2) + the null character (+1) leading to the -1
    int so_len = strlen(MODULE_REGISTRY_DIR) + strlen(wrapper->command_packet->cmd_name) - 1;
    snprintf(so_path, so_len, MODULE_REGISTRY_DIR, wrapper->command_packet->cmd_name);

    if (!file_exists(so_path))
    {
        wrapper->err = true;
        snprintf(wrapper->output, sizeof(wrapper->output) - 1,"The command library %s, does not exist\n", so_path);
        return false;
    }

    *handle = dlopen(so_path, RTLD_NOW | RTLD_GLOBAL);

    if (*handle == NULL)
    {
        wrapper->err = true;
        snprintf(wrapper->output, sizeof(wrapper->output) - 1, "Unable to open lib: %s\n", dlerror());
        return false;
    }

    switch (wrapper->command_packet->action)
    {
        case EXECUTE:
            *func = dlsym(*handle, "execute");
            break;
        case STATUS:
            *func = dlsym(*handle, "status");
            break;
        case OUTPUT:
            *func = dlsym(*handle, "output");
            break;
        case HELP:
            *func = dlsym(*handle, "help");
            break;
        default:
            snprintf(wrapper->output, sizeof(wrapper->output) - 1, "Unable to open lib: %s\n", dlerror());
            return false;
    }

    if (func == NULL)
    {
        snprintf(wrapper->output, sizeof(wrapper->output) - 1, "The requested symbol doesn't exist\n");
        return false;
    }
    return true;
}


bool run_command(CNCWrapper * wrapper, void ** handle, lib_function func)
{
    int original_stdout;
    //Redirect stdout to the response output field.
    //TODO: Redirect or figure out what to do with STDERR

    if (wrapper == NULL || handle == NULL)
    {
        return false;
    }

    fflush(stdout);
    original_stdout = dup(STDOUT_FILENO);
    freopen("/dev/null", "a", stdout);
    setbuf(stdout, wrapper->response_packet->output);

    //This block is not good. This is generating a Char** from a Char[][]
    //After discussion today it seems all parsing should move to the client
    //side. Doing so would eliminate this and all parsing from the service
    //side of the C&C framework. In the next release all parsing will be moved
    //into the client.
    char *argv[CMD_PACKET_NUM_ARGS];
    int i = 0;
    for (; i < wrapper->command_packet->arg_count; i++)
    {
        argv[i] = wrapper->command_packet->args[i];
    }

    //Measure the clock before and after the function has run
    clock_t start_time = clock();
    wrapper->response_packet->return_code = func(wrapper->command_packet->arg_count, (char**)argv);
    clock_t finish_time = clock();

    //Redirect stdout back to the terminal.
    freopen("/dev/null", "a", stdout);
    dup2(original_stdout, STDOUT_FILENO); //restore the previous state of stdout
    setbuf(stdout, NULL);

    //Calculate the runtime
    wrapper->response_packet->execution_time = (double)(finish_time - start_time) / (CLOCKS_PER_SEC/1000); //execution time in milliseconds

    //Unload the library
    dlclose(*handle);

    return true;
}


bool load_and_run_command(CNCWrapper * wrapper)
{
    void * handle;

    if (wrapper == NULL)
    {
        return false;
    }

    lib_function func = NULL;

    if (!load_command(wrapper, &handle, &func))
    {
        printf("Failed to load command\n");
        wrapper->err = true;
        send_result(wrapper);
        return false;
    }
    if (!run_command(wrapper, &handle, func))
    {
        printf("Failed to run command\n");
        wrapper->err = true;
        send_result(wrapper);
        return false;
    }

    //Running the command succeeded
    printf("Command succeeded - Sending Result\n");
    return send_result(wrapper);
}

