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
#ifdef YOTTA_CFG_SENSORS_GPS
#include "kubos-core/modules/gps.h"
#include "kubos-core/modules/nmea.h"

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


void gps_rx_cb (void * arg, uint8_t *buf, int len, void *pxTaskWoken)
{
    gps_cfg_t *gps_cfg = (gps_cfg_t *) arg;
    DEBUG("GPS_RX: %c\n", *buf);

    if (*buf == '\n' || *buf == '\r') {
        gps_fix_t *fix = gps_incr_fix();
        int len = gps_buf_cur + 1;
        int result = nmea_parse(gps_buf, len, fix);
        memset(gps_buf, '\0', len);
        gps_buf_cur = 0;

        if (result != NMEA_OK) {
            DEBUG("error parsing NMEA: %d\n", result);
            return;
        }

        // Send location fix
        if (NULL != gps_cfg->conn) {
            DEBUG("Parsed NMEA sentence, sending message\n");
            DEBUG("NMEA %02d:%02d:%02d.%03d\n", fix->hour, fix->minute,
                    fix->seconds, fix->milliseconds);

            csp_packet_t * msg;
            msg = csp_buffer_get(sizeof(gps_fix_t));
            memcpy(msg->data, fix, sizeof(gps_fix_t));
            //*((gps_fix_t*)msg->data) = *fix;
            msg->length = sizeof(gps_fix_t);

            if (!csp_send(gps_cfg->conn, msg, 1000)) {
                DEBUG("csp_send failed\n");
                csp_buffer_free(msg);
            }
        }
        return;
    }

    gps_buf[gps_buf_cur++] = *buf;
    DEBUG("GPS_BUF: %.*s\n", gps_buf_cur, gps_buf);

    if (gps_buf_cur > GPS_BUFSIZE - 1) {
        DEBUG("Too much data for GPS buffer, discarding");
        gps_buf_cur = 0;
    }
}

/* Establish a gps over serial. Function blocks until there's a connection.
*/
void gps_connect(gps_cfg_t *gps_cfg)
{
    uart_init(gps_cfg->uart_conf);
    uart_set_callback((void*)gps_cfg, gps_rx_cb);
    DEBUG("Connected to UART%s\n", gps_cfg->uart_conf->device);
}
#endif
