#include "cmd-control-daemon/daemon.h"
#include <csp/csp.h>
#include <dlfcn.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

typedef int (*lib_func)(int, char**);

char* get_command(csp_socket_t* sock) {
    csp_conn_t *conn;
    csp_packet_t *packet;
    char* command = NULL;

    while (1) {
        /* Process incoming packet */
        conn = csp_accept(sock, 1000);
        if (conn) {
            packet = csp_read(conn, 0);
            if (packet)
                command = malloc(packet->length);
                memcpy(command, packet->data, packet->length);
                printf("Received Command: %s\r\n", command);
            csp_buffer_free(packet);
            csp_close(conn);
            return command;
       }
    }
}


cnc_res_packet * run_command(cnc_cmd_packet * command){

    int return_code;
    cnc_res_packet* res_ptr = malloc(sizeof(cnc_res_packet));
    void     *handle  = NULL;
    lib_func  func    = NULL;
    char * home_dir = "/home/vagrant/lib%s.so";

    int len = strlen(home_dir) + strlen(command->cmd_name) - 1;
    char * so_path = malloc(len);
    snprintf(so_path, len, home_dir, command->cmd_name);

    handle = dlopen(so_path, RTLD_NOW | RTLD_GLOBAL);

    free(so_path);

    if (handle == NULL)
    {
        fprintf(stderr, "Unable to open lib: %s\n", dlerror());
        res_ptr->return_code = -1;
        return res_ptr;
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
    }

    if (func == NULL) {
        fprintf(stderr, "Unable to get symbol\n");
    }

    printf("Calling fucntion with argc: %i, argv: %s\n", command->arg_count, *command->args);

    clock_t start_time = clock();
    res_ptr->return_code = func(command->arg_count, command->args);
    clock_t finish_time = clock();

    res_ptr->execution_time = (double)(finish_time - start_time)/(CLOCKS_PER_SEC/1000); //execution time in milliseconds
    printf("Return code: %i exection time %f\n", res_ptr->return_code, res_ptr->execution_time);

    //TODO: "unload" the library and other clean up
    return res_ptr;
}


