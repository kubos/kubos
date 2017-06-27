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

 /**
  * @defgroup KUBOS_CORE_AX25 Kubos Core AX.25 Interface
  * @addtogroup KUBOS_CORE_AX25
  * @{
  */

#ifndef AX25_H
#define AX25_H

#include <stdint.h>
#include "kubos-core/k_buffer.h"

#ifndef AX25_BUFFER_SIZE
#define AX25_BUFFER_SIZE 255
#endif

#define AX25_CHAR(c)         ((char) ((c) << 1))
#define AX25_ADDR_NOCALL     { .callsign = { \
        AX25_CHAR('N'), AX25_CHAR('0'), AX25_CHAR('C'), AX25_CHAR('A'), \
        AX25_CHAR('L'), AX25_CHAR('L') }, .ssid = AX25_CHAR('0') }

#define AX25_UI_CONTROL      0x03
#define AX25_UI_PROTOCOL     0xF0

typedef struct ax25_addr_s {
    char callsign[6];
    char ssid;
} ax25_addr_t;

/**
 * Turn an ASCII encoded callsign / SSID into the 7-bit encoded and space padded
 * 7 byte address that is used by AX.25. Callsign and SSID should be separated
 * by a '-' (dash)
 *
 * Examples:
 *   ax25_addr_init("MYCALL") (default SSID=0)
 *   ax25_addr_init("MYCALL-1")
 */
ax25_addr_t ax25_addr_init(char *addr);

/**
 * Prints an AX.25 address using printf with the passed in prefix
 */
void ax25_print_addr(char *prefix, ax25_addr_t addr);

k_buffer_t *ax25_pkt_build(k_buffer_t *info, ax25_addr_t *addrs,
                               uint8_t addrs_len, uint8_t ctrl,
                               uint8_t protocol);


#define ax25_ui_pkt_build(info, addrs, addrs_len) \
        ax25_pkt_build(info, addrs, addrs_len, AX25_UI_CONTROL, \
                       AX25_UI_PROTOCOL)
#endif

/* @} */
