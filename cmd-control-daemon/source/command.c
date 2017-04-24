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

#include <command-and-control/types.h>

#include "cmd-control-daemon/daemon.h"
#include "cmd-control-daemon/logging.h"
#include "tinycbor/cbor.h"

#define CBOR_BUF_SIZE YOTTA_CFG_CSP_MTU
#define CMD_STR_LEN 150


bool cnc_daemon_parse_command_cbor(csp_packet_t * packet, char * command)
{
    CborParser parser;
    CborValue map, element;
    size_t len;

    if (packet == NULL || command == NULL)
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "%s called with a NULL pointer\n", __func__);
        return false;
    }

    CborError err = cbor_parser_init((uint8_t*) packet->data, packet->length, 0, &parser, &map);
    if (err)
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Unable to initialize Cbor parser. Error: %i\n", err);
        return false;
    }

    if(err = cbor_value_map_find_value(&map, "ARGS", &element))
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Unable to find key \"ARGS\" from buffer: %i\n", err);
    }

    if (err = cbor_value_copy_text_string(&element, command, &len, NULL))
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Copying string value key \"ARGS\" from buffer error code: %i\n", err);
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


bool cnc_daemon_run_command(CNCWrapper * wrapper, void ** handle, lib_function func)
{
    int original_stdout;
    //Redirect stdout to the response output field.
    //TODO: Redirect or figure out what to do with STDERR

    if (wrapper == NULL || handle == NULL)
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "%s called with a NULL pointer\n", __func__);
        return false;
    }

    KLOG_INFO(&log_handle, LOG_COMPONENT_NAME, "Redirecting STDOUT and running command.\n");

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

    KLOG_INFO(&log_handle, LOG_COMPONENT_NAME, "STDOUT set back after running command\n");
    //Calculate the runtime
    wrapper->response_packet->execution_time = (double)(finish_time - start_time) / (CLOCKS_PER_SEC/1000); //execution time in milliseconds
    KLOG_INFO(&log_handle, LOG_COMPONENT_NAME, "Command execution time %f\n", wrapper->response_packet->execution_time);

    //Unload the library
    dlclose(*handle);

    return true;
}


bool append_str(char * str, int * size, char * new_str)
{
    int result;

    if (str == NULL || size == NULL || new_str == NULL)
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "%s called with a NULL pointer\n", __func__);
        return false;
    }

    result = snprintf(str + *size, CMD_STR_LEN - *size, new_str);

    if (result < 0)
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "There was an error appending string: %s to buffer: %s\nerror: %i\n", new_str, str, result);
        return false;
    }

    *size += result;
    return true;
}


bool assemble_cmd_string(char * cmd_str, char * exe_path, CNCWrapper * wrapper)
{
    int i, used_size;
    int size = 0;

    if (cmd_str == NULL || exe_path == NULL || wrapper == NULL)
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "%s called with a NULL pointer\n", __func__);
        return false;
    }

    if (!append_str(cmd_str, &size, exe_path))
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "There was an error processing the command\n");
        return false;
    }

    for (i = 0; i < wrapper->command_packet->arg_count; i++)
    {
        append_str(cmd_str, &size, " ");
        append_str(cmd_str, &size, wrapper->command_packet->args[i]);
    }
}


bool cnc_daemon_load_and_run_command(CNCWrapper * wrapper)
{
    char exe_path[SO_PATH_LENGTH]   = {0};
    char cmd_str[CMD_STR_LEN]       = {0};
    char buf[RES_PACKET_STDOUT_LEN] = {0};
    clock_t start_time, end_time;
    FILE * fptr;
    int  exe_len, return_code;
    int  size = 0;

    if (wrapper == NULL)
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "%s called with a NULL pointer\n", __func__);
        return false;
    }

    // exe_len - the format specifier length (-2) + the null character (+1) leading to the -1
    exe_len = strlen(MODULE_REGISTRY_DIR) + strlen(wrapper->command_packet->cmd_name) - 1;

    if (exe_len > CMD_STR_LEN)
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "The path the executable is too long to fit into the command string\n");
        wrapper->err = true;
        snprintf(wrapper->output, sizeof(wrapper->output), "The path to the executable is too long to fit into the command string\n");
        cnc_daemon_send_result(wrapper);
        return false;
    }

    snprintf(exe_path, exe_len, MODULE_REGISTRY_DIR, wrapper->command_packet->cmd_name);

    if (!file_exists(exe_path))
    {
        KLOG_INFO(&log_handle, LOG_COMPONENT_NAME, "Requested binary %s does not exist\n", exe_path);
        wrapper->err = true;
        snprintf(wrapper->output, sizeof(wrapper->output),"The command binary %s, does not exist\n", exe_path);
        cnc_daemon_send_result(wrapper);
        return false;
    }

    if (!assemble_cmd_string(cmd_str, exe_path, wrapper))
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "There was an issue procesing the command string.\n");
        snprintf(wrapper->response_packet->output, RES_PACKET_STDOUT_LEN, "There was an issue procesing the command string.\n");
        return false;
    }

    KLOG_INFO(&log_handle, LOG_COMPONENT_NAME, "Running command: '%s'\n", cmd_str);

    start_time = clock();
    fptr = popen(cmd_str, "r");
    if (fptr == NULL)
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "There was an issue starting the command process %i\n", fptr);
        snprintf(wrapper->response_packet->output, RES_PACKET_STDOUT_LEN, "There was an issue starting the command process\n");
        return false;
    }

    while (fgets(buf, RES_PACKET_STDOUT_LEN, fptr) != NULL) {
        size += snprintf(wrapper->response_packet->output + size, RES_PACKET_STDOUT_LEN - size, "%s", buf);
    }

    wrapper->response_packet->return_code = pclose(fptr);
    end_time = clock();
    wrapper->response_packet->execution_time = (double)(end_time - start_time) / (CLOCKS_PER_SEC/1000); //execution time in milliseconds
    cnc_daemon_send_result(wrapper);
    return true;

}
