#include <argp.h>
#include <csp/csp.h>
#include <csp/csp_interface.h>
#include <fcntl.h>
#include <pthread.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "command-and-control/types.h"
#include "cmd-control-daemon/daemon.h"
#include "tinycbor/cbor.h"

#define PORT        10
#define BUF_SIZE    MTU

/*
 * IMPORTANT: For review ignore everything before line #91
 * We won't be using named pipes and everything before that is all csp code to
 * set up a named pipe connection.
 */

pthread_t rx_thread, my_thread;
int rx_channel, tx_channel;

int csp_fifo_tx(csp_iface_t *ifc, csp_packet_t *packet, uint32_t timeout);

csp_iface_t csp_if_fifo = {
    .name = "fifo",
    .nexthop = csp_fifo_tx,
    .mtu = MTU,
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


int init(int my_address)
{
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

//Where the magic happens - Bascially ignore everything above this line - The initialization is going to change a lot.

bool send_packet(csp_conn_t* conn, csp_packet_t* packet)
{
    if (!conn || !csp_send(conn, packet, 1000))
        return false;
    return true;
}


bool send_buffer(uint8_t * data, size_t data_len)
{
    int my_address = 1, client_address = 2;
    char *rx_channel_name, *tx_channel_name;

    csp_socket_t *sock;
    csp_conn_t *conn;
    csp_packet_t *packet;

    while (1) {
        packet = csp_buffer_get(BUF_SIZE);
        if (packet) {
            memcpy(packet->data, data, data_len);
            packet->length = data_len;

            conn = csp_connect(CSP_PRIO_NORM, client_address, PORT, 1000, CSP_O_NONE);
            send_packet(conn, packet);
            csp_buffer_free(packet);
            csp_close(conn);
            return true;
        }
    }
}


void zero_vars(char * command_str, cnc_command_packet * command, cnc_response_packet * response, cnc_command_wrapper * wrapper)
{
    memset(command_str, 0, sizeof(command_str) * sizeof(char));
    memset(command, 0, sizeof(cnc_command_packet));
    memset(response, 0, sizeof(cnc_response_packet));
    memset(wrapper->output, 0, sizeof(wrapper->output));
    wrapper->err = false;
}


int main(int argc, char **argv)
{
    int my_address = 1;
    csp_socket_t *sock;
    char command_str[75];
    cnc_command_packet command;
    cnc_response_packet response;
    //The wrapper keeps track of a command input, it's result and
    //any pre-run processing error messages that may occur
    cnc_command_wrapper wrapper;
    bool exit = false;

    wrapper.command_packet  = &command;
    wrapper.response_packet = &response;

    init(my_address);
    sock = csp_socket(CSP_SO_NONE);
    csp_bind(sock, PORT);
    csp_listen(sock, 5);

    while (!exit)
    {
        zero_vars(command_str, &command, &response, &wrapper);
        get_command(sock, command_str);

        if (!parse(command_str, &wrapper))
        {
            //Do some error handling
            continue;
        }

        if(!process_and_run_command(&wrapper))
        {
            //Do some error handling
            continue;
        }
    }

    close(rx_channel);
    close(tx_channel);

    return 0;

}

