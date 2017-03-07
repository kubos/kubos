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
#include <csp/drivers/socket.h>
#include <csp/interfaces/csp_if_socket.h>

#include "command-and-control/types.h"
#include "cmd-control-daemon/daemon.h"
#include <ipc/csp.h>
#include "tinycbor/cbor.h"

#define PORT        10
#define BUF_SIZE    MTU

#define SERVER_CSP_ADDRESS 1
#define CSP_PORT           11
#define CLI_CLIENT_ADDRESS 2
#define SOCKET_PORT        8189

csp_iface_t csp_socket_if;
csp_socket_handle_t socket_driver;

bool init()
{
    if(!kubos_csp_init(SERVER_CSP_ADDRESS))
    {
        return false;
    }

    csp_route_set(CLI_CLIENT_ADDRESS, &csp_socket_if, CSP_NODE_MAC);
    csp_socket_init(&csp_socket_if, &socket_driver);
}


bool send_packet(csp_conn_t* conn, csp_packet_t* packet)
{
    if (conn == NULL || packet == NULL)
    {
        return false;
    }

    if (!csp_send(conn, packet, 1000))
    {
        return false;
    }

    return true;
}


bool send_buffer(uint8_t * data, size_t data_len)
{
    csp_socket_t *sock;
    csp_conn_t *conn;
    csp_packet_t *packet;

    if (data == NULL)
    {
        return false;
    }

    if(packet = csp_buffer_get(BUF_SIZE))
    {
        memcpy(packet->data, data, data_len);
        packet->length = data_len;

        conn = csp_connect(CSP_PRIO_NORM, CLI_CLIENT_ADDRESS, CSP_PORT, 1000, CSP_O_NONE);
        if (!send_packet(conn, packet))
        {
            csp_buffer_free(packet);
            csp_close(conn);
            return false;
        }
        csp_buffer_free(packet);
        csp_close(conn);
        return true;
    }
    return false;
}


void zero_vars(char * command_str, CNCCommandPacket * command, CNCResponsePacket * response, CNCWrapper * wrapper)
{
    memset(command_str, 0, sizeof(command_str) * sizeof(char));
    memset(command, 0, sizeof(CNCCommandPacket));
    memset(response, 0, sizeof(CNCResponsePacket));
    memset(wrapper->output, 0, sizeof(wrapper->output));
    wrapper->err = false;
}

bool get_command(csp_socket_t* sock, char * command)
{
    csp_conn_t *conn;
    csp_packet_t *packet;

    if (sock == NULL || command == NULL)
    {
        return false;
    }

    if (socket_init(&socket_driver, CSP_SOCKET_SERVER, SOCKET_PORT) != CSP_ERR_NONE)
    {
        return false;
    }

    if (csp_socket_init(&csp_socket_if, &socket_driver) != CSP_ERR_NONE)
    {
        return false;
    }

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
                    csp_buffer_free(packet);
                    csp_close(conn);
                    return false;
                }
                csp_buffer_free(packet);
            }
            csp_close(conn);
            return true;
        }
    }
}



int main(int argc, char **argv)
{
    int my_address = 1;
    csp_socket_t *sock;
    char command_str[CMD_STR_LEN];
    CNCCommandPacket command;
    CNCResponsePacket response;
    //The wrapper keeps track of a command input, its result and
    //any pre-run processing error messages that may occur
    CNCWrapper wrapper;
    bool exit = false;

    wrapper.command_packet  = &command;
    wrapper.response_packet = &response;

    init(my_address);
    sock = csp_socket(CSP_SO_NONE);
    csp_bind(sock, CSP_PORT);
    csp_listen(sock, 5);

    while (!exit)
    {
        zero_vars(command_str, &command, &response, &wrapper);
        if (!get_command(sock, command_str))
        {
            //Do some error handling
            continue;
        }

        if (!parse(command_str, &wrapper))
        {
            //Do some error handling
            continue;
        }

        if(!load_and_run_command(&wrapper))
        {
            //Do some error handling
            continue;
        }
    }

    return 0;
}

