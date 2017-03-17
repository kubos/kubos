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
#include "cmd-control-daemon/daemon.h"
#include <cmd-control-daemon/logging.h>
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


/*
 * IMPORTANT: CSP FIFO example setup code. The long term intetion is to move
 * away from named pipes to the newer tcp communication mechanism. This is a
 * temporary measure that will be removed once the new system is ready.
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

    char *rx_channel_name, *tx_channel_name;

    rx_channel_name = "/home/vagrant/client-to-server";
    tx_channel_name = "/home/vagrant/server-to-client";


    /* Init CSP and CSP buffer system */
    if (csp_init(SERVER_CSP_ADDRESS) != CSP_ERR_NONE || csp_buffer_init(10, 300) != CSP_ERR_NONE)
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

bool init_logging()
{
    int res;
    log_handle.config.file_path = DAEMON_LOG_PATH;
    log_handle.config.file_path_len = strlen(DAEMON_LOG_PATH);
    log_handle.config.part_size = LOG_PART_SIZE;
    log_handle.config.max_parts = LOG_MAX_PARTS;
    log_handle.config.klog_console_level = LOG_ALL;
    log_handle.config.klog_file_level = LOG_ALL;
    log_handle.config.klog_file_logging = true;

    res = klog_init_file(&log_handle);
    if (res == 0)
    {
        printf("Loggin initialized");
        KLOG_INFO(&log_handle, "Daemon", "Logging initialized\n");
        return true;
    }
    else
    {
        fprintf(stderr, "Unable to Initialize Logging\n");
        return false;
    }
}

bool cnc_daemon_send_packet(csp_conn_t* conn, csp_packet_t* packet)
{
    if (conn == NULL || packet == NULL)
    {
        KLOG_ERR(&log_handle, "Daemon", "Received a NULL pointer while sending packet\n");
        return false;
    }

    if (!csp_send(conn, packet, 1000))
    {
        /* log packet id when we implement it */
        KLOG_ERR(&log_handle, "Daemon", "Sending csp packet failed\n");
        return false;
    }

    return true;
}


bool cnc_daemon_send_buffer(uint8_t * data, size_t data_len)
{
    csp_socket_t *sock;
    csp_conn_t *conn;
    csp_packet_t *packet;

    if (data == NULL)
    {
        KLOG_ERR(&log_handle, "Daemon", "Called with a NULL pointer while sending packet\n");
        return false;
    }

    if(packet = csp_buffer_get(BUF_SIZE))
    {
        memcpy(packet->data, data, data_len);
        packet->length = data_len;

        conn = csp_connect(CSP_PRIO_NORM, CLI_CLIENT_ADDRESS, CSP_PORT, 1000, CSP_O_NONE);
        if (!cnc_daemon_send_packet(conn, packet))
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

bool cnc_daemon_get_buffer(csp_socket_t* sock, CborDataWrapper * data_wrapper)
{
    csp_conn_t *conn;
    csp_packet_t *packet;

    if (sock == NULL || data_wrapper == NULL)
    {
        KLOG_ERR(&log_handle, "Daemon", "Called with NULL Pointer in cnc_daemon_get_buffer\n");
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
                if (!cnc_daemon_parse_buffer_from_packet(packet, data_wrapper))
                {
                    KLOG_ERR(&log_handle, "Daemon", "There was an error parsing the command packet\n");
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
    csp_socket_t *sock;
    char command_str[CMD_STR_LEN];
    CNCCommandPacket command;
    CNCResponsePacket response;
    //The wrapper keeps track of a command input, its result and
    //any pre-run processing error messages that may occur
    CNCWrapper wrapper;
    bool exit = false;
    uint8_t buffer[CMD_STR_LEN];
    //The CborDataWrapper keeps a reference to a buffer and the length of the
    //buffer tied together and simplifies passing both pieces of data between functions
    CborDataWrapper data_wrapper;
    data_wrapper.data = buffer;
    wrapper.command_packet  = &command;
    wrapper.response_packet = &response;

    init_logging();
    init();

    sock = csp_socket(CSP_SO_NONE);
    csp_bind(sock, CSP_PORT);
    csp_listen(sock, 5);

    while (!exit)
    {
        zero_vars(command_str, &command, &response, &wrapper);
        KLOG_ERR(&log_handle, "Daemon", "Getting Command\n");
        if (!cnc_daemon_get_buffer(sock, &data_wrapper))
        {
            KLOG_ERR(&log_handle, "Daemon", "There was an error getting a command\n");
            continue;
        }

        if (!cnc_daemon_parse_buffer(&wrapper, &data_wrapper))
        {
            KLOG_ERR(&log_handle, "Daemon", "There was an error decoding the received command\n");
            continue;
        }

        if(!cnc_daemon_load_and_run_command(&wrapper))
        {
            KLOG_ERR(&log_handle, "Daemon", "There was an error parsing the received command\n");
            continue;
        }

    }

    klog_cleanup(&log_handle);
    return 0;
}

