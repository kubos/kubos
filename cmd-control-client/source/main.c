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
#include "tinycbor/cbor.h"

#define PORT        10
#define BUF_SIZE    MTU

bool parse_processing_error(CborParser * parser, CborValue * map);
bool parse_command_result( CborParser * parser, CborValue * map);

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

    tx_channel_name = "/home/vagrant/client_to_server";
    rx_channel_name = "/home/vagrant/server_to_client";


    /* Init CSP and CSP buffer system */
    if (csp_init(my_address) != CSP_ERR_NONE || csp_buffer_init(10, 300) != CSP_ERR_NONE)
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
    csp_route_start_task(0, 0);
    return true;
}

//Where the magic happens - Bascially ignore everything above this line - The initialization is going to change a lot.

bool send_packet(csp_conn_t* conn, csp_packet_t* packet)
{
    if (!conn || !csp_send(conn, packet, 1000))
    {
        return false;
    }
    return true;
}


bool send_msg(uint8_t* data, size_t length)
{
    int server_address = 1;
    csp_conn_t *conn;
    csp_packet_t *packet;

    if (packet = csp_buffer_get(length)) 
    {
        memcpy(packet->data, data, length);
        packet->length = length;

        conn = csp_connect(CSP_PRIO_NORM, server_address, PORT, 1000, CSP_O_NONE);
        if (!send_packet(conn, packet))
        {
            return false;
        }
        csp_buffer_free(packet);
        csp_close(conn);
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
    size_t len;
    uint8_t return_code;
    double execution_time;
    char output[BUF_SIZE];

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
    size_t len;
    char error_message[BUF_SIZE];
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
    csp_bind(sock, PORT);
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

    CborEncoder encoder, container;
    CborError err;

    if (!init())
    {
        fprintf(stderr, "There was an error initializing the csp configuration\n");
        return 1;
    }

    for (i = 1; i < argc; i++)
    {
        strcat(args, argv[i]);
        if (i != argc-1) //Skip the final separator
        {
            strcat(args, separator);
        }
    }

    cbor_encoder_init(&encoder, data, BUF_SIZE, 0);
    err = cbor_encoder_create_map(&encoder, &container, 1);
    if (err)
    {
        fprintf(stderr, "There was an error creating the map. CBOR Error code: %i\n", err);
        return 0;
    }
    err = cbor_encode_text_stringz(&container, "ARGS");
    if (err || cbor_encode_text_stringz(&container, args))
    {
        fprintf(stderr, "There was an error encoding the commands into the CBOR message\n");
    }

    send_msg(data, BUF_SIZE);
    get_response();

    close(rx_channel);
    close(tx_channel);

    return 0;
}

