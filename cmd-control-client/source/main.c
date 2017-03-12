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
#include <csp/csp_debug.h>
#include <csp/drivers/socket.h>

#include "command-and-control/types.h"
#include "cmd-control-client/client.h"
#include <ipc/csp.h>
#include "tinycbor/cbor.h"

#define BUF_SIZE           MTU

#define CLI_CLIENT_ADDRESS 2
#define CSP_PORT           11
#define SERVER_CSP_ADDRESS 1
#define SOCKET_PORT        8189

bool parse_processing_error(CborParser * parser, CborValue * map);
bool parse_command_result( CborParser * parser, CborValue * map);
bool parse (CNCCommandPacket * command_packet, int argc, char ** argv);

csp_iface_t csp_socket_if;
csp_socket_handle_t socket_driver;


/*
 * IMPORTANT: For review ignore everything before line #91
 * We won't be using named pipes and everything before that is all csp code to
 * set up a named pipe connection.
 */

pthread_t rx_thread, my_thread;
int rx_channel, tx_channel;

int csp_fifo_tx(csp_iface_t *ifc, csp_packet_t *packet, uint32_t timeout);

csp_iface_t csp_if_fifo =
{
    .name = "fifo",
    .nexthop = csp_fifo_tx,
    .mtu = MTU,
};

int csp_fifo_tx(csp_iface_t *ifc, csp_packet_t *packet, uint32_t timeout)
{
    /* Write packet to fifo */
    if (write(tx_channel, &packet->length, packet->length + sizeof(uint32_t) + sizeof(uint16_t)) < 0)
    {
        printf("Failed to write frame\r\n");
    }
    csp_buffer_free(packet);
    return CSP_ERR_NONE;
}

void * fifo_rx(void * parameters)
{
    csp_packet_t *buf = csp_buffer_get(BUF_SIZE);
    /* Wait for packet on fifo */
    while (read(rx_channel, &buf->length, BUF_SIZE) > 0)
    {
        csp_new_packet(buf, &csp_if_fifo, NULL);
        buf = csp_buffer_get(BUF_SIZE);
    }

    return NULL;
}

bool init()
{

    int my_address = 2;
    char *rx_channel_name, *tx_channel_name;

    tx_channel_name = "/home/vagrant/client-to-server";
    rx_channel_name = "/home/vagrant/server-to-client";


    /* Init CSP and CSP buffer system */
    if (csp_init(CLI_CLIENT_ADDRESS) != CSP_ERR_NONE || csp_buffer_init(10, 300) != CSP_ERR_NONE)
    {
        printf("Failed to init CSP\r\n");
        return false;
    }

    tx_channel = open(tx_channel_name, O_RDWR);
    if (tx_channel < 0)
    {
        printf("Failed to open TX channel\r\n");
        return false;
    }

    rx_channel = open(rx_channel_name, O_RDWR);
    if (rx_channel < 0)
    {
        printf("Failed to open RX channel\r\n");
        return false;
    }

    /* Start fifo RX task */
    pthread_create(&rx_thread, NULL, fifo_rx, NULL);

    /* Set default route and start router */
    csp_route_set(CSP_DEFAULT_ROUTE, &csp_if_fifo, CSP_NODE_MAC);
    csp_route_start_task(256, 1);
    return true;
}

//Where the magic happens - End of the CSP FIFO setup code

bool send_packet(csp_packet_t* packet)
{
    csp_conn_t *conn;

    if (packet) {

        conn = csp_connect(CSP_PRIO_NORM, SERVER_CSP_ADDRESS, CSP_PORT, 1000, CSP_O_NONE);

        if (!conn)
        {
            csp_buffer_free(packet);
            return false;
        }
        printf("Sending Packet:%s\n", packet);
        if (!csp_send(conn, packet, 1000))
        {
            csp_buffer_free(packet);
            return false;
        }
        csp_close(conn);
    }
    return true;
}


bool send_msg(CborDataWrapper * data_wrapper)
{
    csp_conn_t *conn;
    csp_packet_t *packet;
    if (data_wrapper == NULL)
    {
        return false;
    }

    if (packet = csp_buffer_get(data_wrapper->length))
    {
        memcpy(packet->data, data_wrapper->data, data_wrapper->length);
        packet->length = data_wrapper->length;

        if (!send_packet(packet))
        {
            return false;
        }
        return true;
    }
    else
    {
        return false;
    }
}


bool parse_response(csp_packet_t * packet)
{
    CborParser parser;
    CborValue map, element;
    int message_type;

    if (packet == NULL)
    {
        return false;
    }

    CborError err = cbor_parser_init((uint8_t*) packet->data, packet->length, 0, &parser, &map);
    if (err)
    {
        return false;
    }

    err = cbor_value_map_find_value(&map, "MSG_TYPE", &element);
    if (err || cbor_value_get_int(&element, &message_type))
    {
        return false;
    }

    switch (message_type)
    {
        case RESPONSE_TYPE_COMMAND_RESULT:
            return parse_command_result(&parser, &map);
            break;
        case RESPONSE_TYPE_PROCESSING_ERROR:
            return parse_processing_error(&parser, &map);
            break;
        default:
            fprintf(stderr, "Received unknown message type: %i\n", message_type);
            return false;
    }
}

bool parse_command_result( CborParser * parser, CborValue * map)
{
    uint8_t return_code;
    double execution_time;
    char output[BUF_SIZE];
    size_t len = BUF_SIZE;

    CborValue element;
    CborError err;

    err = cbor_value_map_find_value(map, "RETURN_CODE", &element);
    if (err || cbor_value_get_simple_type(&element, &return_code))
    {
        return false;
    }
    printf("Return Code: %i\n", return_code);

    err = cbor_value_map_find_value(map, "EXEC_TIME", &element);
    if (err || cbor_value_get_double(&element, &execution_time))
    {
        return false;
    }
    printf("Exectuion Time %f\n", execution_time);

    err = cbor_value_map_find_value(map, "OUTPUT", &element);
    if (err || cbor_value_copy_text_string(&element, output, &len, NULL))
    {
        return false;
    }
    printf("Output: %s\n", output);
    return true;

}


bool parse_processing_error(CborParser * parser, CborValue * map)
{
    size_t len = BUF_SIZE;
    char error_message[BUF_SIZE] = {0};
    CborValue element;
    CborError err;

    err = cbor_value_map_find_value(map, "ERROR_MSG", &element);
    if (err || cbor_value_copy_text_string(&element, error_message, &len, NULL))
    {
        return false;
    }

    printf("Error Message: %s\n", error_message);
    return true;
}


void get_response()
{
    csp_socket_t *sock;
    csp_conn_t *conn;
    csp_packet_t *packet;

    sock = csp_socket(CSP_SO_NONE);
    csp_bind(sock, CSP_PORT);
    csp_listen(sock, 5);

    while (conn = csp_accept(sock, 1000))
    {
        if (conn)
        {
            packet = csp_read(conn, 0);
            if (packet)
            {
                parse_response(packet);
            }
            csp_buffer_free(packet);
            csp_close_socket(sock);
            csp_close(conn);
            return;
        }
    }
}


int main(int argc, char **argv)
{
    int i;
    char* separator = " ";
    char args[BUF_SIZE] = {0};
    uint8_t data[BUF_SIZE] = {0};

    CborDataWrapper data_wrapper;
    data_wrapper.length = 0;
    data_wrapper.data = data;

    CNCCommandPacket cmd_packet;
    CborEncoder encoder, container;
    CborError err;


    cmd_packet.arg_count = 0;
    cmd_packet.action = 0;
    memset(&cmd_packet.cmd_name, sizeof(cmd_packet.cmd_name), 0);
    for (i = 0; i < CMD_PACKET_NUM_ARGS; i++)
    {
        memset(&cmd_packet.args[i], CMD_PACKET_ARG_LEN, 0);
    }

    parse(&cmd_packet, argc, argv);

    if (!init())
    {
        fprintf(stderr, "There was an error initializing the csp configuration\n");
        return 1;
    }

    if (!encode_packet(&data_wrapper, &cmd_packet))
    {
        fprintf(stderr, "Error encoding command packet\n");
        return 1;
    }

    send_msg(&data_wrapper);
    get_response();

    return 0;
}

