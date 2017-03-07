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
#include <csp/drivers/socket.h>
#include <csp/interfaces/csp_if_socket.h>
#include <fcntl.h>
#include <pthread.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "command-and-control/types.h"
#include <ipc/csp.h>
#include "tinycbor/cbor.h"

#define BUF_SIZE           MTU

#define CLI_CLIENT_ADDRESS 2
#define CSP_PORT           11
#define SERVER_CSP_ADDRESS 1
#define SOCKET_PORT        8189

bool parse_processing_error(CborParser * parser, CborValue * map);
bool parse_command_result( CborParser * parser, CborValue * map);

csp_iface_t csp_socket_if;
csp_socket_handle_t socket_driver;

bool init()
{
    if (!kubos_csp_init(CLI_CLIENT_ADDRESS))
    {
        return false;
    }

    csp_route_set(SERVER_CSP_ADDRESS, &csp_socket_if, CSP_NODE_MAC);

    return true;
}


bool send_packet(csp_packet_t* packet)
{
    csp_conn_t *conn;

    if (packet) {
        socket_init(&socket_driver, CSP_SOCKET_CLIENT, SOCKET_PORT);
        csp_socket_init(&csp_socket_if, &socket_driver);

        conn = csp_connect(CSP_PRIO_NORM, SERVER_CSP_ADDRESS, CSP_PORT, 1000, CSP_O_NONE);

        if (!conn) {
            csp_buffer_free(packet);
            return false;
        }

        if (!csp_send(conn, packet, 1000))
        {
            csp_buffer_free(packet);
            return false;
        }
        csp_close(conn);
    }
    return true;
}


bool send_msg(uint8_t* data, size_t length)
{
    csp_conn_t *conn;
    csp_packet_t *packet;

    if (packet = csp_buffer_get(length))
    {
        memcpy(packet->data, data, length);
        packet->length = length;

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

    return 0;
}

