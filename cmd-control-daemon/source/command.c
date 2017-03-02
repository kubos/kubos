#include "cmd-control-daemon/daemon.h"
#include <csp/csp.h>
#include <dlfcn.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <unistd.h>
#include "tinycbor/cbor.h"

#define CBOR_BUF_SIZE YOTTA_CFG_CSP_MTU
#define SO_PATH_LENGTH 75


bool get_command(csp_socket_t* sock, char * command)
{
    csp_conn_t *conn;
    csp_packet_t *packet;

    while (1)
    {
        conn = csp_accept(sock, 1000);
        if (conn)
        {
            packet = csp_read(conn, 0);
            if (packet)
            {
                if (!parse_command_cbor(packet, command))
                {
                    fprintf(stderr, "There was an error parsing the command packet\n");
                    return false;
                }
                csp_buffer_free(packet);
            }
            csp_close(conn);
            return true;
        }
    }
}


bool parse_command_cbor(csp_packet_t * packet, char * command)
{
    CborParser parser;
    CborValue map, element;
    size_t len;

    CborError err = cbor_parser_init((uint8_t*) packet->data, packet->length, 0, &parser, &map);
    if (err)
        return false;

    err = cbor_value_map_find_value(&map, "ARGS", &element);
    if (err || cbor_value_copy_text_string(&element, command, &len, NULL))
        return false;
    return true;
}

bool file_exists(char * path) //Should this live in a higher level module utility?
{
    if ( access(path, F_OK) != -1)
        return true;
    return false;
}


bool load_command(cnc_command_wrapper * wrapper, void ** handle, lib_function * func)
{
    int return_code;
    char so_path[SO_PATH_LENGTH];
    char * home_dir = "/home/vagrant/lib%s.so"; //Just a dev path for now.

    // so_len - the format specifier length (-2) + the null character (+1) leading to the -1
    int so_len = strlen(home_dir) + strlen(wrapper->command_packet->cmd_name) - 1;
    snprintf(so_path, so_len, home_dir, wrapper->command_packet->cmd_name);

    if (!file_exists(so_path)){
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
        case execute:
            *func = dlsym(*handle, "execute");
            break;
        case status:
            *func = dlsym(*handle, "status");
            break;
        case output:
            *func = dlsym(*handle, "output");
            break;
        case help:
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


bool run_command(cnc_command_wrapper * wrapper, void ** handle, lib_function func)
{
    //Redirect stdout to the response output field.
    //TODO: Redirect or figure out what to do with STDERR

    int original_stdout;
    fflush(stdout);
    original_stdout = dup(STDOUT_FILENO);
    freopen("/dev/null", "a", stdout);
    setbuf(stdout, wrapper->response_packet->output);

    //Measure the clock before and after the function has run
    clock_t start_time = clock();
    wrapper->response_packet->return_code = func(wrapper->command_packet->arg_count, wrapper->command_packet->args);
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


bool process_and_run_command(cnc_command_wrapper * wrapper)
{
    lib_function func = NULL;
    void * handle;

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


