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

#define CSP_DEBUG
#define CSP_VERBOSE

#include <stdbool.h>
#include "kubos-core/modules/gps.h"
#include "kubos-core/modules/nmea.h"

#include "kubos-core/arch/k_timer.h"

#define ENABLE_DEBUG 0
#include "kubos-core/k_debug.h"

/* time to sleep between attempts to connect 3 seconds*/
#define CONNECT_RETRY_INTERVAL (3000000U)

/* set polling interval to .5 second */
#define POLLING_INTERVAL (500000U)

/* Circular buffer of gps fixes to pass as message content */
static int next_buffer_slot = 0;
static gps_fix_t buffer[GPS_MSG_Q_SIZE];

static gps_fix_t* gps_incr_fix(void) {
    gps_fix_t* ptr = &buffer[next_buffer_slot];
    next_buffer_slot++;
    if (next_buffer_slot >= GPS_MSG_Q_SIZE) {
        next_buffer_slot = 0;
    }
    return ptr;
}

gps_fix_t *gps_last_fix(void)
{
    return &buffer[next_buffer_slot];
}

static char gps_buf[GPS_BUFSIZE];
static uint8_t gps_buf_cur = 0;

void simple_rx(uint8_t * buf, int len, void * px)
{
    //gps_cfg_t *gps_cfg; // = (gps_cfg_t *) arg;
    printf("GPS_RX: %c\n", *buf);

    if (*buf == '\n') {
        gps_fix_t *fix = gps_incr_fix();
        int len = gps_buf_cur + 1;
        int result = nmea_parse(gps_buf, len, fix);
        gps_buf_cur = 0;

        if (result != NMEA_OK) {
            printf("error parsing NMEA: %d\n", result);
            printf("error reading...");
            return;
        }

        // Send location fix
        // if (NULL != gps_cfg->conn) {
        //     DEBUG("Parsed NMEA sentence, sending message\n");
        //     printf("NMEA %s\n", (char*)fix);
        //     csp_packet_t * msg;
        //     msg = csp_buffer_get(sizeof(csp_packet_t));
        //     memcpy(msg->data, fix, sizeof(gps_fix_t*));
        //     msg->length = sizeof(gps_fix_t*);
        //
        //     csp_send(gps_cfg->conn, msg, 1000);
        // }
        return;
    }

    gps_buf[gps_buf_cur++] = *buf;
    printf("GPS_BUF: %.*s\n", gps_buf_cur, gps_buf);

    if (gps_buf_cur > GPS_BUFSIZE - 1) {
        printf("Too much data for GPS buffer, discarding");
        gps_buf_cur = 0;
    }
}

void gps_rx_cb (void * arg, uint8_t *buf, int len, void *pxTaskWoken) //(void *arg, uint8_t data)
{
    gps_cfg_t *gps_cfg = (gps_cfg_t *) arg;
    printf("GPS_RX: %c\n", *buf);

    if (*buf == '\n' || *buf == '\r') {
        gps_fix_t *fix = gps_incr_fix();
        int len = gps_buf_cur + 1;
        int result = nmea_parse(gps_buf, len, fix);
        memset(gps_buf, '\0', len);
        gps_buf_cur = 0;

        if (result != NMEA_OK) {
            printf("error parsing NMEA: %d\n", result);
            printf("error reading...");
            return;
        }

        // Send location fix
        if (NULL != gps_cfg->conn) {
            printf("Parsed NMEA sentence, sending message\n");
            printf("NMEA %02d:%02d:%02d.%03d\n", fix->hour, fix->minute,
                    fix->seconds, fix->milliseconds);

            csp_packet_t * msg;
            msg = csp_buffer_get(sizeof(gps_fix_t));
            *((gps_fix_t*)msg->data) = *fix;
            msg->length = sizeof(gps_fix_t);

            // char * test = "hello";
            // csp_packet_t * msg;
            // msg = csp_buffer_get(100);
            // strcpy((char*)msg->data, test);
            // msg->length = strlen(test) + 1;
            if (!csp_send(gps_cfg->conn, msg, 1000)) {
                printf("csp_send failed\n");
                csp_buffer_free(msg);
            }
        }
        return;
    }

    gps_buf[gps_buf_cur++] = *buf;
    printf("GPS_BUF: %.*s\n", gps_buf_cur, gps_buf);

    if (gps_buf_cur > GPS_BUFSIZE - 1) {
        printf("Too much data for GPS buffer, discarding");
        gps_buf_cur = 0;
    }
}

/* Establish a gps over serial. Function blocks until there's a connection.
*/
void gps_connect(gps_cfg_t *gps_cfg)
{
    bool connected = false;
    uint32_t last_wakeup = k_timer_now();

    // while (!connected) {
    //     if (uart_init(gps_cfg->uart_conf, gps_rx_cb) == 0) {
    //         DEBUG("Connected to UART%s\n", gps_cfg->uart_conf->device);
    //         connected = true;
    //     } else {
    //         // Sleep for a while before trying again
    //         k_timer_usleep_until(&last_wakeup, CONNECT_RETRY_INTERVAL);
    //     }
    // }

    // void setup_uart_rx(uint8_t * buf, int len, void * pxTaskWoken) {
    //     gps_rx_cb(gps_cfg, buf, len, pxTaskWoken);
    // }

    uart_init(gps_cfg->uart_conf);
    uart_set_callback((void*)gps_cfg, gps_rx_cb);
    DEBUG("Connected to UART%s\n", gps_cfg->uart_conf->device);
    connected = true;
}
