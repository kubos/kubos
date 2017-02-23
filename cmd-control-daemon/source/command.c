#include "cmd-control-daemon/daemon.h"
#include <csp/csp.h>
#include <dlfcn.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <unistd.h>

#include "tinycbor/cbor.h"
#define CBOR_BUF_SIZE 64
typedef int (*lib_func)(int, char**);

bool parse_command_cbor(csp_packet_t * packet, char * command);

bool get_command(csp_socket_t* sock, char * command) {
    csp_conn_t *conn;
    csp_packet_t *packet;


    while (1) {
        /* Process incoming packet */
        conn = csp_accept(sock, 1000);
        if (conn) {
            packet = csp_read(conn, 0);
            if (packet)
            {
                parse_command_cbor(packet, command);
            }
            printf("Received Command: %s\r\n", command);
            csp_buffer_free(packet);
            csp_close(conn);
            return true;
        }
    }
}

bool parse_command_cbor(csp_packet_t * packet, char * command) {
    CborParser parser;
    CborValue map, element;
    size_t len;

    CborError err = cbor_parser_init((uint8_t*) packet->data, packet->length, 0, &parser, &map);
    //TODO: Error checking..
    err = cbor_value_map_find_value(&map, "ARGS", &element);
    err = cbor_value_get_string_length(&element, &len);
    err = cbor_value_copy_text_string(&element, command, &len, NULL);
}


bool run_command(cnc_cmd_packet * command, cnc_res_packet * response) {
    int return_code;
    void     *handle  = NULL;
    lib_func  func    = NULL;
    char so_path[75];      //TODO: Define some constant and some macro for overflow checking.
    char * home_dir = "/home/vagrant/lib%s.so";


    int so_len = strlen(home_dir) + strlen(command->cmd_name) - 1;
    snprintf(so_path, so_len, home_dir, command->cmd_name);

    handle = dlopen(so_path, RTLD_NOW | RTLD_GLOBAL);

    if (handle == NULL)
    {
        fprintf(stderr, "Unable to open lib: %s\n", dlerror());
        return false;
    }

    switch (command->action){
        case execute:
            printf("Running Command Execute\n");
            func = dlsym(handle, "execute");
            break;
        case status:
            printf("Running Command status\n");
            func = dlsym(handle, "status");
            break;
        case version:
            printf("Running Command version\n");
            func = dlsym(handle, "version");
            break;
        case help:
            printf("Running Command help\n");
            func = dlsym(handle, "help");
            break;
        default:
            printf ("Error the requested command doesn't exist\n");
            return NULL;
    }

    if (func == NULL) {
        fprintf(stderr, "Unable to get symbol\n");
        return false;
    }
    //Redirect stdout to the response output field.
    int original_stdout, buf;
    fflush(stdout);
    original_stdout = dup(STDOUT_FILENO);
    freopen("/dev/null", "a", stdout);
    setbuf(stdout, response->output);

    clock_t start_time = clock();
    response->return_code = func(command->arg_count, command->args);
    clock_t finish_time = clock();

    //Redirect stdout back to the terminal.
    freopen("/dev/null", "a", stdout);
    dup2(original_stdout, STDOUT_FILENO); //restore the previous state of stdout
    setbuf(stdout, NULL);

    response->execution_time = (double)(finish_time - start_time)/(CLOCKS_PER_SEC/1000); //execution time in milliseconds
    printf("Return code: %i exection time %f\n", response->return_code, response->execution_time);

    //TODO: "unload" the library and other clean up
    return true;
}

