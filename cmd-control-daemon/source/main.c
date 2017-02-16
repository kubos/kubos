#include <argp.h>
#include <csp/csp.h>
#include <csp/csp_interface.h>
#include <dlfcn.h>
#include <fcntl.h>
#include <pthread.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "command-and-control/types.h"

#define PORT        10
#define BUF_SIZE    250

typedef void (*lib_func)();

pthread_t rx_thread, my_thread;
int rx_channel, tx_channel;

int csp_fifo_tx(csp_iface_t *ifc, csp_packet_t *packet, uint32_t timeout);

csp_iface_t csp_if_fifo = {
    .name = "fifo",
    .nexthop = csp_fifo_tx,
    .mtu = BUF_SIZE,
};

int csp_fifo_tx(csp_iface_t *ifc, csp_packet_t *packet, uint32_t timeout) {
    /* Write packet to fifo */
    if (write(tx_channel, &packet->length, packet->length + sizeof(uint32_t) + sizeof(uint16_t)) < 0)
        printf("Failed to write frame\r\n");
    csp_buffer_free(packet);
    return CSP_ERR_NONE;
}

void * fifo_rx(void * parameters) {
    csp_packet_t *buf = csp_buffer_get(BUF_SIZE);
    /* Wait for packet on fifo */
    while (read(rx_channel, &buf->length, BUF_SIZE) > 0) {
        csp_new_packet(buf, &csp_if_fifo, NULL);
        buf = csp_buffer_get(BUF_SIZE);
    }

    return NULL;
}



int send_packet(csp_conn_t* conn, csp_packet_t* packet) {
    printf("Sending: %s\r\n", packet->data);
    if (!conn || !csp_send(conn, packet, 1000))
        return -1;
    return 0;
}


int csp_init_things(int my_address){
    char *rx_channel_name, *tx_channel_name;
    /* Set type */
    tx_channel_name = "/home/vagrant/server_to_client";
    rx_channel_name = "/home/vagrant/client_to_server";

    /* Init CSP and CSP buffer system */
    if (csp_init(my_address) != CSP_ERR_NONE || csp_buffer_init(10, 300) != CSP_ERR_NONE) {
        printf("Failed to init CSP\r\n");
        return -1;
    }

    tx_channel = open(tx_channel_name, O_RDWR);
    if (tx_channel < 0) {
        printf("Failed to open TX channel\r\n");
        return -1;
    }

    rx_channel = open(rx_channel_name, O_RDWR);
    if (rx_channel < 0) {
        printf("Failed to open RX channel\r\n");
        return -1;
    }

    /* Start fifo RX task */
    pthread_create(&rx_thread, NULL, fifo_rx, NULL);

    /* Set default route and start router */
    csp_route_set(CSP_DEFAULT_ROUTE, &csp_if_fifo, CSP_NODE_MAC);
    csp_route_start_task(0, 0);
    return 0;
}

//Where the magic happens

void send_response() {
    int my_address = 1, client_address = 2;
    char *message = "Return MSG", *rx_channel_name, *tx_channel_name;
    csp_socket_t *sock;
    csp_conn_t *conn;
    csp_packet_t *packet;

    while (1) {
        packet = csp_buffer_get(strlen(message));
        if (packet) {
            strcpy((char *) packet->data, message);
            packet->length = strlen(message);

            conn = csp_connect(CSP_PRIO_NORM, client_address, PORT, 1000, CSP_O_NONE);
            send_packet(conn, packet);
            csp_buffer_free(packet);
            csp_close(conn);
            return;
        }
    }
}


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

cnc_action parse_command(char * command) {
    cnc_action action;
    action = execute;
    return action;
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

    if (handle == NULL)
    {
        fprintf(stderr, "Unable to open lib: %s\n", dlerror());
        return -1;
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
    printf("Calling fucntion with argc: %i, argv: %s\n", command->arg_count, command->args);
    func(command->arg_count, command->args);
    free(so_path);
    return res_ptr;
}

int main(int argc, char **argv) {
    int my_address = 1;
    char *message = "Testing CSP";
    csp_socket_t *sock;
    cnc_action action;
    cnc_cmd_packet* cmd_ptr = NULL;
    cnc_res_packet* res_ptr = NULL;

    csp_init_things(my_address);
    sock = csp_socket(CSP_SO_NONE);
    csp_bind(sock, PORT);
    csp_listen(sock, 5);

    char * command = NULL;

    while (1) {
        command = get_command(sock);
        cmd_ptr = parse(command);
        res_ptr = run_command(cmd_ptr);
        send_response();
        free(command);
    }

    close(rx_channel);
    close(tx_channel);

    //Free all the allocated things.
    int i;
    for (i = 0; i < my_arguments.arg_count-3; i++){
        free(my_arguments.args[i]);
    }
    free(my_arguments.cmd_name);
    free(result);


    return 0;
}

