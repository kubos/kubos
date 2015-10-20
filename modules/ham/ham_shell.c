/*
 * KubOS Core Flight Services
 * Copyright (C) 2015 Kubos Corporation
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
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <net/gnrc.h>

#include "aprs.h"
#include "ax25.h"
#include "kiss.h"
#include "ham_shell.h"

#if !KISS_DRIVER_UART
#include <errno.h>
#endif

static ax25_addr_t _ax25_addrs[8];
static int _ax25_addrs_len = 2;

void ham_cmd_init(void)
{
    _ax25_addrs[0] = ax25_addr_init("APRS");
    _ax25_addrs[1] = ax25_addr_init("N0CALL");
}

#define KISS_STACKSIZE (THREAD_STACKSIZE_DEFAULT)
#define KISS_PRIO      (THREAD_PRIORITY_MAIN - 4)

static char _kiss_stack[KISS_STACKSIZE];
static kernel_pid_t _kiss_pid = KERNEL_PID_UNDEF;

static inline int8_t hexchar_value(char c) {
    if (c >= '0' && c <= '9') {
        return c - '0';
    } else if (c >= 'A' && c <= 'F') {
        return (c - 'A') + 10;
    } else if (c >= 'a' && c <= 'f') {
        return (c - 'a') + 10;
    }

    return -1;
}

static int hexstr_value(char *hex_str, int str_len, uint8_t *dest)
{
    int b = 0;
    int8_t val;

    for (int i = 0; i < str_len; i++) {
        val = hexchar_value(hex_str[i]);
        if (val != -1) {
            dest[b/8] = ((uint8_t) val) << (b % 8);
            b += 4;
        }
    }

    return b / 8 + (b % 8 == 0 ? 0 : 1);
}

int kiss_init_cmd(int argc, char **argv)
{
#if KISS_DRIVER_UART
    if (argc < 3) {
        printf("Usage: kiss_init <uart> <baudrate>\n");
        return -1;
    }
#else
    if (argc < 2) {
        printf("Usage: kiss_init <filename>\n");
        return -1;
    }
#endif

    if (_kiss_pid > KERNEL_PID_UNDEF) {
        printf("KISS already started, pid: %d\n", _kiss_pid);
        return 0;
    }

    kiss_dev_t dev;
#if KISS_DRIVER_UART
    uart_t uart = (uart_t) atoi(argv[1]);
    uint32_t baudrate = (uint32_t) atol(argv[2]);

    _kiss_pid = kiss_init_uart(&dev, uart, baudrate, _kiss_stack, KISS_STACKSIZE,
                          KISS_PRIO);
#else
    FILE *f = fopen(argv[1], "w+");
    if (!f) {
        printf("Error opening \"%s\": %s\n", argv[1], strerror(errno));
        return -1;
    }

    _kiss_pid = kiss_init_native(&dev, fileno(f), _kiss_stack, KISS_STACKSIZE,
                                 KISS_PRIO);
#endif

    if (_kiss_pid <= KERNEL_PID_UNDEF) {
        printf("Error initializing KISS device!\n");
        return -1;
    }

    printf("KISS started, pid: %d\n", _kiss_pid);
    return 0;
}

int kiss_send_cmd(int argc, char **argv)
{
    if (argc < 2) {
        printf("Usage: kiss_send <data>\n");
        printf("    prefix data with a colon (:) to pass hex encoded binary data\n");
        return -1;
    }

    if (_kiss_pid <= KERNEL_PID_UNDEF) {
        printf("Error: KISS not initialized, use kiss_init\n");
        return -1;
    }

    void *data = argv[1];
    size_t len = strlen(argv[1]);
    if (len > 0 && argv[1][0] == ':') {
        uint8_t buf[255];
        data = (void *) buf;
        len = hexstr_value(&(argv[1][1]), len - 1, buf);
    }

    gnrc_pktsnip_t *pkt = gnrc_pktbuf_add(NULL, data, len, GNRC_NETTYPE_UNDEF);
    gnrc_netapi_send(_kiss_pid, pkt);
    return 0;
}

int ax25_getset_addr_cmd(int argc, char **argv)
{
    int i, end;

    // get
    if (argc == 1) {
        if (strcmp(argv[0], "ax25_dest") == 0) {
            i = 0; end = 1;
        } else if (strcmp(argv[0], "ax25_src") == 0) {
            i = 1; end = 2;
        } else {
            i = 2; end = _ax25_addrs_len;
        }

        if (i >= end) {
            printf("None\n");
            return 0;
        }

        for (; i < end; i++) {
            ax25_print_addr("", _ax25_addrs[i]);
        }

        return 0;
    }

    // set

    if (strcmp(argv[0], "ax25_dest") == 0) {
        _ax25_addrs[0] = ax25_addr_init(argv[1]);
        ax25_print_addr("Dest:", _ax25_addrs[0]);
    } else if (strcmp(argv[0], "ax25_src") == 0) {
        _ax25_addrs[1] = ax25_addr_init(argv[1]);
        ax25_print_addr("Source:", _ax25_addrs[1]);
    } else {
        // Digis
        for (i = 1; i < argc; i++) {
            _ax25_addrs[i + 1] = ax25_addr_init(argv[i]);
            ax25_print_addr("Digi:", _ax25_addrs[i+1]);
        }
        _ax25_addrs_len = argc + 1;
    }

    return 0;
}

int ax25ui_pkt_cmd(int argc, char **argv)
{
    char *new_argv[4];
    if (argc < 2) {
        printf("Usage: ax25ui_pkt <payload>\n");
        printf("The first addresses argument should be callsigns separated by comma\n");
        return -1;
    }

    new_argv[0] = argv[0];
    new_argv[1] = "3";   // 0x03
    new_argv[2] = "240"; // 0xF0
    new_argv[3] = argv[1];
    return ax25_pkt_cmd(4, new_argv);
}

int ax25_pkt_cmd(int argc, char **argv)
{
    uint8_t ctrl, proto;
    gnrc_pktsnip_t *payload, *pkt;

    if (argc < 4) {
        printf("Usage: ax25_pkt <ctrl> <proto> <payload>\n");
        printf("The first addresses argument should be callsigns separated by comma\n");
        return -1;
    }

    ctrl = (uint8_t) atoi(argv[1]);
    proto = (uint8_t) atoi(argv[2]);

    payload = gnrc_pktbuf_add(NULL, argv[3], strlen(argv[3]), GNRC_NETTYPE_UNDEF);
    if (!payload) {
        printf("Failed to allocate payload buffer (%d bytes)\n", strlen(argv[3]));
        return -1;
    }

    pkt = ax25_pkt_build(payload, _ax25_addrs, _ax25_addrs_len, ctrl, proto);

    printf("AX.25 packet (%d bytes):\n", gnrc_pkt_len(pkt));
    while (pkt) {
        uint8_t *data = (uint8_t *) pkt->data;
        size_t i;
        for (i = 0; i < pkt->size; i++) {
            printf("%02x", data[i]);
        }
        pkt = pkt->next;
    }
    printf("\n");
    return 0;
}


int ax25_addr_cmd(int argc, char **argv)
{
    ax25_addr_t addr;
    int i;

    if (argc < 2) {
        printf("Usage: ax25_addr <callsign>\n");
        printf("Encodes a callsign into AX.25 shifted/padded format");
        return -1;
    }

    addr = ax25_addr_init(argv[1]);

    printf("AX.25 address: ");
    for (i = 0; i < 6; i++) {
        printf("%02x ", (uint8_t) addr.callsign[i]);
    }
    printf("%02x\n", (uint8_t) addr.ssid);

    return 0;
}

int aprs_pos_cmd(int argc, char **argv)
{
    aprs_position_t pos;
    char pos_str[APRS_POSITION_LEN];

    if (argc < 9) {
        printf("Usage: aprs_pos <hour> <min> <sec> <lat> <lon> <course> <speed> <alt>\n");
        printf("lat: -90 to 90 in degrees\n");
        printf("lon: -180 to 180 in degrees\n");
        printf("course: 1 to 360 deg clockwise from due north\n");
        printf("speed: in knots\n");
        printf("alt: in feet\n");
        return -1;
    }

    pos.hour = (uint8_t) atoi(argv[1]);
    pos.minute = (uint8_t) atoi(argv[2]);
    pos.second = (uint8_t) atoi(argv[3]);
    pos.latitude = (float) atof(argv[4]);
    pos.longitude = (float) atof(argv[5]);
    pos.course = (int16_t) atoi(argv[6]);
    pos.speed = (int16_t) atoi(argv[7]);
    pos.altitude = (int32_t) atoi(argv[8]);

    aprs_position_format(pos_str, &pos);
    printf("%s\n", pos_str);

    return 0;
}

int aprs_tlm_cmd(int argc, char **argv)
{
    aprs_telemetry_t tlm;
    char tlm_str[APRS_TELEMETRY_LEN];

    if (argc < 8) {
        printf("Usage: aprs_tlm <pkt_id> <a1> <a2> <a3> <a4> <a5> <d>\n");
        printf("    pkt_id: packet id, 0-999\n");
        printf("    a1-a5:  analog values 0-255\n");
        printf("    d:      digital value broken into bits for fields 6-14\n");
        return -1;
    }

    tlm.packet_id = (uint16_t) atoi(argv[1]);
    tlm.analog[0] = (uint8_t) atoi(argv[2]);
    tlm.analog[1] = (uint8_t) atoi(argv[3]);
    tlm.analog[2] = (uint8_t) atoi(argv[4]);
    tlm.analog[3] = (uint8_t) atoi(argv[5]);
    tlm.analog[4] = (uint8_t) atoi(argv[6]);
    tlm.digital = (uint8_t) atoi(argv[7]);

    aprs_telemetry_format(tlm_str, &tlm);
    printf("%s\n", tlm_str);

    return 0;
}
