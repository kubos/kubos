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

//Where the magic happens - Bascially ignore everything above this line

bool send_packet(csp_conn_t* conn, csp_packet_t* packet) {
    printf("Sending: %s\r\n", packet->data);
    if (!conn || !csp_send(conn, packet, 1000))
        return false;
    return true;
}

bool encode_packet(csp_packet_t * packet, cnc_response_packet * result) {
    CborEncoder encoder, container;
    CborError err;
    uint8_t data[BUF_SIZE];

    cbor_encoder_init(&encoder, data, BUF_SIZE, 0);
    err = cbor_encoder_create_map(&encoder, &container, 3);
    if (err)
        return false;

    err = cbor_encode_text_stringz(&container, "RETURN_CODE");
    if (err || cbor_encode_simple_value(&container, result->return_code))
        return false;

    err = cbor_encode_text_stringz(&container, "EXEC_TIME");
    if (err || cbor_encode_double(&container, result->execution_time))
        return false;

    err = cbor_encode_text_stringz(&container, "OUTPUT");
    if (err || cbor_encode_text_stringz(&container, result->output))
        return false;

    cbor_encoder_close_container(&encoder, &container);
    memcpy(packet->data, data, BUF_SIZE); //TODO: Make more efficient
    packet->length = BUF_SIZE;
    return true;
}


void send_response(cnc_response_packet* response) {
    int my_address = 1, client_address = 2;
    char *rx_channel_name, *tx_channel_name;
    uint8_t buffer[BUF_SIZE];

    csp_socket_t *sock;
    csp_conn_t *conn;
    csp_packet_t *packet;

    while (1) {
        packet = csp_buffer_get(BUF_SIZE);
        if (packet) {
            if (!encode_packet(packet, response)) {
                fprintf(stderr, "There was an issue encoding the run output into the response packet.\n");
                return;
            }
            conn = csp_connect(CSP_PRIO_NORM, client_address, PORT, 1000, CSP_O_NONE);
            send_packet(conn, packet);
            csp_buffer_free(packet);
            csp_close(conn);
            return;
        }
    }
}

void zero_vars(char * command_str, cnc_command_packet * command, cnc_response_packet * response)
{
    memset(command_str, 0, sizeof(command_str) * sizeof(char));
    memset(command, 0, sizeof(cnc_command_packet));
    memset(response, 0, sizeof(cnc_response_packet));
}


int main(int argc, char **argv) {
    int my_address = 1;
    csp_socket_t *sock;
    char command_str[75];
    cnc_command_packet command;
    cnc_response_packet response;

    csp_init_things(my_address);
    sock = csp_socket(CSP_SO_NONE);
    csp_bind(sock, PORT);
    csp_listen(sock, 5);

    while (1) {
        zero_vars(command_str, &command, &response);
        get_command(sock, command_str);
        parse(command_str, &command);
        run_command(&command, &response);
        send_response(&response);
    }

    close(rx_channel);
    close(tx_channel);

    return 0;
}

