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
#include <stdio.h>
#include <stdint.h>
#include <string.h>

#include <net/gnrc.h>

#include "ax25.h"

#define MAX_CALLSIGN_LEN 6
#define MAX_ADDR_LEN     8

#define ax25_to_char(c) ((char) ((uint8_t) (c) >> 1))

void ax25_print_addr(char *prefix, ax25_addr_t addr)
{
    printf("%s callsign=[%c%c%c%c%c%c], ssid=[%c]\n", prefix,
           ax25_to_char(addr.callsign[0]),
           ax25_to_char(addr.callsign[1]),
           ax25_to_char(addr.callsign[2]),
           ax25_to_char(addr.callsign[3]),
           ax25_to_char(addr.callsign[4]),
           ax25_to_char(addr.callsign[5]),
           ax25_to_char(addr.ssid));
}

ax25_addr_t ax25_addr_init(char *addr)
{
    ax25_addr_t result = AX25_ADDR_NOCALL;
    int i = 0, callsign_len = 0;
    char *p = addr;
    int addr_len;

    if (!addr) {
        return result;
    }

    addr_len = strlen(addr);
    addr_len = addr_len > MAX_ADDR_LEN ? MAX_ADDR_LEN : addr_len;

    for (; i < addr_len; i++, p++, callsign_len++) {
        if (*p == '-') {
            if (i + 1 < addr_len) {
                result.ssid = *(p + 1) << 1;
            }
            break;
        }

        if (i < MAX_CALLSIGN_LEN) {
            result.callsign[i] = *p << 1;
        }
    }

    // add space padding
    for (i = callsign_len; i < MAX_CALLSIGN_LEN; i++) {
        result.callsign[i] = ' ' << 1;
    }

    return result;
}

unsigned short ax25_calc_fcs(char *buf, int size)
{
    unsigned short fcs = 0xFFFF;
    uint8_t e = 0, f = 0;
    int i;
    for (i = 0; i < size; i++) {
        e = fcs ^ buf[i];
        f = e ^ (e << 4);
        fcs = (fcs >> 8) ^ (f << 8) ^ (f << 3) ^ (f >> 4);
    }

    return fcs;
}

unsigned short ax25_calc_fcs_pkt(gnrc_pktsnip_t *pkt)
{
    unsigned short fcs = 0xFFFF;
    uint8_t e = 0, f = 0;

    while (pkt) {
        char *data = (char *) pkt->data;
        size_t i, size = pkt->size;
        if (!pkt->next) {
            size -= 2;
        }

        for (i = 0; i < size; i++) {
            e = fcs ^ data[i];
            f = e ^ (e << 4);
            fcs = (fcs >> 8) ^ (f << 8) ^ (f << 3) ^ (f >> 4);
        }
        pkt = pkt->next;
    }

    return fcs;
}

gnrc_pktsnip_t *ax25_pkt_build(gnrc_pktsnip_t *info, ax25_addr_t *addrs,
                               uint8_t addrs_len, uint8_t ctrl, uint8_t protocol)
{
    gnrc_pktsnip_t *pkt;
    unsigned short fcs = 0xFFFF;
    int i, size = (7 * addrs_len) + 2;
    char *fcs_data, *pkt_data;

    if (!addrs || addrs_len < 1) {
        return NULL;
    }

    // First add an additional 2 bytes for the FCS marker

    if (!info) {
        info = gnrc_pktbuf_add(NULL, &fcs, 2, GNRC_NETTYPE_UNDEF);
    } else {
        if (gnrc_pktbuf_realloc_data(info, info->size + 2) != 0) {
            return NULL;
        }
    }

    fcs_data = ((char *) info->data) + (info->size - 2);

    pkt = gnrc_pktbuf_add(info, NULL, size, GNRC_NETTYPE_UNDEF);
    pkt_data = (char *) pkt->data;

    for (i = 0; i < addrs_len; i++) {
        memcpy(pkt_data, &addrs[i], 7);
        pkt_data += 7;
    }

    // End of addresses
    *(pkt_data - 1) |= 0x01;

    *pkt_data = (char) ctrl;
    ++pkt_data;

    *pkt_data = (char) protocol;
    ++pkt_data;

    fcs = ax25_calc_fcs_pkt(pkt);
    fcs_data[0] = ~(fcs & 0xFF);
    fcs_data[1] = ~((fcs >> 8) & 0xFF);
    return pkt;
}
