#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <pthread.h>
#include <stdint.h>
#include <stdlib.h>
#include <csp/csp.h>
#include <csp/csp_interface.h>

#include "command-and-control/types.h"

#define PORT        10
#define BUF_SIZE    250

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
    printf("Sending Command Packet\r\n");
    if (!conn || !csp_send(conn, packet, 1000))
        return -1;
    return 0;
}


void csp_init_everything(){

    int my_address = 2;
    char *message = "Testing CSP", *rx_channel_name, *tx_channel_name;

        tx_channel_name = "/home/vagrant/client_to_server";
        rx_channel_name = "/home/vagrant/server_to_client";


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
}


void send_msg(char* message) {
    int server_address = 1;
    csp_conn_t *conn;
    csp_packet_t *packet;

    int size = strlen(message) + 1;
    while(packet = csp_buffer_get(size)) {
        if (packet) {
            memcpy(packet->data, message, size);
            packet->length = size;

            conn = csp_connect(CSP_PRIO_NORM, server_address, PORT, 1000, CSP_O_NONE);
            send_packet(conn, packet);
            csp_buffer_free(packet);
            csp_close(conn);
            return;
        }
    }
}


void get_response() {
    csp_socket_t *sock;
    csp_conn_t *conn;
    csp_packet_t *packet;

    sock = csp_socket(CSP_SO_NONE);
    csp_bind(sock, PORT);
    csp_listen(sock, 5);

    while (conn = csp_accept(sock, 1000)) {
        if (conn) {
            packet = csp_read(conn, 0);
            if (packet)
                printf("Received Response: %s\r\n", packet->data);
            csp_buffer_free(packet);
            csp_close(conn);
            return;
        }
    }
}


int main(int argc, char **argv) {
    //terrible name for starting all the tedious csp stuff
    csp_init_everything();

    char* args = "exec foo";

    /*cnc_cmd_packet packet;*/
    /*packet.action = execute;*/

    /*packet.args = malloc(sizeof(args));*/
    /*memcpy(packet.args, args, 4);*/

    send_msg(args);
    get_response();

    /*free(packet.args);*/
    close(rx_channel);
    close(tx_channel);

    return 0;
}

