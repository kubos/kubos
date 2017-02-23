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
#include <csp/csp_interface.h>
#include <csp/csp_error.h>
#include <csp/interfaces/csp_if_socket.h>
#include <csp/drivers/socket.h>
#include <csp/arch/csp_thread.h>

#include <inttypes.h>
#include <stdint.h>

#include <sys/socket.h>
#include <arpa/inet.h>

#include <tinycbor/cbor.h>

#define BUF_SIZE 256

/**
 * Callback function used by CSP to transmit packets
 * @param ifc csp socket interface
 * @param packet packet to send
 * @param timeout currently not used
 * @return int currently always returns CSP_ERR_NONE
 */
static int csp_socket_tx(csp_iface_t *ifc, csp_packet_t *packet, uint32_t timeout);

static int cbor_encode_csp_packet(csp_packet_t * packet, uint8_t * buffer)
{
    CborEncoder encoder, container;
    CborError err;

    if (buffer == NULL)
        return -1;
    cbor_encoder_init(&encoder, buffer, BUF_SIZE, 0);
    err = cbor_encoder_create_map(&encoder, &container, 3);
    if (err > 0)
        return - err;

    err = cbor_encode_text_stringz(&container, "LENGTH");
    if (err > 0)
        return -err;
    err = cbor_encode_half_float(&container, (void*)&(packet->length));
    if (err > 0)
        return -err;

    err = cbor_encode_text_stringz(&container, "ID");
    if (err > 0)
        return -err;
    err = cbor_encode_half_float(&container, (void*)&(packet->id.ext));
    if (err > 0)
        return -err;

    err = cbor_encode_text_stringz(&container, "DATA");
    if (err > 0)
        return -err;
    err = cbor_encode_byte_string(&container, packet->data, packet->length);
    if (err > 0)
        return -err;

    err = cbor_encoder_close_container_checked(&encoder, &container);
    if (err > 0)
        return -err;

    return cbor_encoder_get_buffer_size(&encoder, buffer);
}

static bool cbor_parse_csp_packet(csp_packet_t * packet, void * buffer, int buffer_size)
{
    CborParser parser;
    CborValue map, element;
    int length = 0;
    uint8_t buf[256];

    if ((buffer == NULL) || (packet == NULL))
        return false;

    CborError err = cbor_parser_init(buffer, buffer_size, 0, &parser, &map);
    if (err || !cbor_value_is_map(&map)) {
        return false;
    }

    err = cbor_value_map_find_value(&map, "LENGTH", &element);
    if (err)
        return false;

    if (cbor_value_get_half_float(&element, &(packet->length)))
        return false;

    err = cbor_value_map_find_value(&map, "ID", &element);
    if (err)
        return false;

    if (cbor_value_get_half_float(&element, &(packet->id.ext)))
        return false;

    err = cbor_value_map_find_value(&map, "DATA", &element);
    if (err)
        return false;

    if(cbor_value_is_byte_string(&element)) {
        csp_log_info("found byte string\r\n");
    } else {
        csp_log_error("no byte string\r\n");
    }

    size_t byte_size;
    if (cbor_value_dup_byte_string(&element, (uint8_t**)&buf, &byte_size, &element)) {
        csp_log_error("Error parsing byte string\r\n");
    } else {
        csp_log_info("Got dat byte string %d %d\r\n", byte_size, packet->length);
        memcpy(packet->data, buf, packet->length);
    }
    // free(buf);
    // if (cbor_value_copy_byte_string(&element, buf, packet->length, &element))
    //     return false;

    return true;
}

static void print_buf(uint8_t * buff, int buff_size)
{
    int i = 0;
    for (i = 0; i < buff_size; i++)
        printf("%02X", buff[i]);
    printf("\r\n");
}

/**
 * Task spawned for each new csp_if_socket for handling receiving data
 */
CSP_DEFINE_TASK(csp_socket_rx);

static int csp_socket_tx(csp_iface_t *ifc, csp_packet_t *packet, uint32_t timeout) {
    if ((ifc == NULL) || (ifc->driver == NULL)) {
        csp_log_error("Null pointer for interface or driver\r\n");
        return CSP_ERR_DRIVER;
    }

    csp_log_info("csp_socket_tx go\r\n");

    csp_socket_handle_t * socket_driver = ifc->driver;

    /* Write packet to socket */
    int result = write(socket_driver->socket_handle, &packet->length, packet->length + sizeof(uint32_t) + sizeof(uint16_t)); 
    if ( result < 0) {
        csp_log_error("Socket write error: %u %s\r\n", result, strerror(result));
    }
    csp_buffer_free(packet);

/**
    // uint8_t * write_buffer = malloc(BUF_SIZE);
    uint8_t write_buffer[BUF_SIZE];

    if (write_buffer != NULL)
    {
        int write_size = cbor_encode_csp_packet(packet, write_buffer);
        if (write_size > 0) {
            csp_log_info("about to write csp packet %d\r\n", packet->length);
            int result = write(socket_driver->socket_handle, write_buffer, write_size); 
            csp_log_info("csp_socket_tx write %d\r\n", result);
            print_buf(write_buffer, write_size);
            if ( result < 0) {
                csp_log_error("Socket write error: %u %s\r\n", result, strerror(result));
            }
            csp_buffer_free(packet);
        } else { csp_log_error("encode csp packet failed\r\n"); }
    } else { csp_log_error("write_buffer malloc failed\r\n"); }
**/
 
    return CSP_ERR_NONE;
}

CSP_DEFINE_TASK(csp_socket_rx) {
    csp_iface_t socket_interface;
    csp_socket_handle_t * socket_driver;
    csp_packet_t * packet = csp_buffer_get(BUF_SIZE);

    csp_log_info("Starting socket rx thread\r\n");

    if (param == NULL) {
        csp_log_error("No socket param found\r\n");
        return CSP_TASK_RETURN;
    }
    socket_interface = *((csp_iface_t*)param);

    if (socket_interface.driver == NULL) {
        csp_log_error("No socket driver found\r\n");
        return CSP_TASK_RETURN;
    }

    socket_driver = socket_interface.driver;

    while(recv(socket_driver->socket_handle, &packet->length, BUF_SIZE, 0) > 0) {
        csp_new_packet(packet, &socket_interface, NULL);

        packet = csp_buffer_get(BUF_SIZE);
        if (packet == NULL) {
            break;
        }
    }

/**
    int recved_size = 0;
    uint8_t buffer[BUF_SIZE];
    while (1)
    {
        // csp_log_info("csp_socket_rx attempt to recv data\r\n");
        memset(buffer, '\0', BUF_SIZE);

        int recv_size = recv(socket_driver->socket_handle, buffer, BUF_SIZE, 0);
        if (recv_size > 0)
        {
            csp_log_info("csp_socket_rx recv'd packet %d\r\n", recv_size);
            print_buf(buffer, recv_size);
            csp_log_info("csp_socket_rx new packet length %d\r\n", packet->length);
            if (cbor_parse_csp_packet(packet, (void*)buffer, recv_size))
            {
                csp_log_info("Got valid csp packet\r\n");
                csp_log_info("csp_socket_rx got packet length %d\r\n", packet->length);
                csp_new_packet(packet, &socket_interface, NULL);
                packet = csp_buffer_get(BUF_SIZE);
                if (packet == NULL) {
                    csp_log_error("Out of packet buffers\r\n");
                    break;
                }
            }
        }


        // // recved_size = recv(socket_driver->socket_handle, &(packet->length), BUF_SIZE, 0);
        // recved_size = recv(socket_driver->socket_handle, packet, BUF_SIZE, 0);

        // // csp_log_info("csp_socket_rx recvd %d\r\n", recved_size);
        // if (recved_size > 0) {
        //     csp_log_info("csp_socket_rx recv'd packet %d\r\n", recved_size);
        //     print_buf(packet, recved_size + sizeof(uint32_t) + sizeof(uint16_t));
        //     // packet->length = recved_size;
        //     packet->length = recved_size - (sizeof(uint32_t) + sizeof(uint16_t));
        //     csp_new_packet(packet, &socket_interface, NULL);

        //     packet = csp_buffer_get(BUF_SIZE);
        //     if (packet == NULL) {
        //         csp_log_error("Out of packet buffers\r\n");
        //         break;
        //     }
        // }
        // csp_sleep_ms(10);
    }
**/
    csp_log_info("Socket rx thread done\r\n");

    return CSP_TASK_RETURN;
}

int csp_socket_init(csp_iface_t * socket_iface, csp_socket_handle_t * socket_driver) {
    if ((socket_iface == NULL) || (socket_driver == NULL))
        return CSP_ERR_DRIVER;

    socket_iface->driver = socket_driver;
    socket_iface->nexthop = csp_socket_tx;
    socket_iface->name = "socket";
    socket_iface->mtu = BUF_SIZE;

    /* Start RX thread */
	int ret = csp_thread_create(csp_socket_rx, "SOCKET_RX", 1000, socket_iface, 0, &(socket_driver->rx_thread_handle));

    /* Register interface */
    csp_iflist_add(socket_iface);

    return CSP_ERR_NONE;
}

int csp_socket_close(csp_iface_t * socket_iface, csp_socket_handle_t * socket_driver) {
    if ((socket_iface == NULL) || (socket_driver == NULL))
        return CSP_ERR_DRIVER;

    socket_close(socket_driver);

    printf("Destroying csp rx thread\r\n");
    csp_thread_kill(&socket_driver->rx_thread_handle);
    
    return CSP_ERR_NONE;
}
