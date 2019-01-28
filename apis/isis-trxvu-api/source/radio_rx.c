/*
 * Copyright (C) 2018 Kubos Corporation
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

#include <kubos-hal/i2c.h>
#include <isis-trxvu-api/trxvu.h>
#include <stdio.h>
#include <string.h>

KRadioStatus k_radio_recv(radio_rx_header * frame, uint8_t * message, uint8_t * len)
{
    if (frame == NULL || message == NULL)
    {
        return RADIO_ERROR_CONFIG;
    }

    KRadioStatus status = RADIO_OK;
    uint16_t     count  = 0;

    /*
     * We have to make sure that there is something to receive
     * before we actually try to receive. The radio has undefined
     * behavior if you attempt to receive a frame from an empty
     * RX buffer.
     */
    status = kprv_radio_rx_get_count((uint8_t *) &count);
    if (status != RADIO_OK)
    {
        fprintf(stderr, "Failed to get radio RX frame count\n");
        return status;
    }

    if (count == 0)
    {
        return RADIO_RX_EMPTY;
    }

    status = kprv_radio_rx_get_frame(frame, message, len);
    if (status != RADIO_OK)
    {
        fprintf(stderr, "Failed to receive frame from radio\n");
        return status;
    }

    status = kprv_radio_rx_remove_frame();
    if (status != RADIO_OK)
    {
        fprintf(stderr, "Failed to remove radio RX frame\n");
        return status;
    }

    return status;
}

KRadioStatus kprv_radio_rx_get_telemetry(radio_telem *  buffer,
                                         RadioTelemType type)
{
    if (buffer == NULL)
    {
        return RADIO_ERROR_CONFIG;
    }

    uint8_t cmd;
    uint8_t len;

    switch (type)
    {
        case RADIO_RX_TELEM_ALL:
            cmd = GET_RX_ALL_TELEMETRY;
            len = sizeof(trxvu_rx_telem_raw);
            break;
        case RADIO_RX_UPTIME:
            cmd = GET_UPTIME;
            len = sizeof(trxvu_uptime);
            break;
        default:
            fprintf(stderr, "Unknown radio RX telemetry type requested: %d\n",
                    type);
            return RADIO_ERROR_CONFIG;
    }

    KI2CStatus status
        = k_i2c_write(radio_bus, radio_rx.addr, (uint8_t *) &cmd, 1);

    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to request radio RX telemetry type %d: %d\n",
                type, status);
        return RADIO_ERROR;
    }

    status = k_i2c_read(radio_bus, radio_rx.addr, (char *) buffer, len);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to retrieve radio RX telemetry type %d: %d\n",
                type, status);
        return RADIO_ERROR;
    }

    return RADIO_OK;
}

KRadioStatus kprv_radio_rx_watchdog_kick(void)
{
    uint8_t cmd = WATCHDOG_RESET;

    KI2CStatus status
        = k_i2c_write(radio_bus, radio_rx.addr, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to kick radio RX watchdog: %d\n", status);
        return RADIO_ERROR;
    }

    return RADIO_OK;
}

KRadioStatus kprv_radio_rx_reset(KRadioReset type)
{
    KI2CStatus status;
    uint8_t    cmd;

    switch (type)
    {
        case RADIO_SOFT_RESET:
            cmd = SOFT_RESET;
            break;
        case RADIO_HARD_RESET:
            cmd = HARD_RESET;
            break;
        default:
            fprintf(stderr, "Unknown radio RX reset type: %d\n", type);
            return RADIO_ERROR_CONFIG;
    }

    status = k_i2c_write(radio_bus, radio_rx.addr, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to reset RX radio: %d\n", status);
        return RADIO_ERROR;
    }

    return RADIO_OK;
}

KRadioStatus kprv_radio_rx_get_count(uint8_t * count)
{
    if (count == NULL)
    {
        return RADIO_ERROR_CONFIG;
    }

    uint8_t    cmd = GET_RX_FRAME_COUNT;
    KI2CStatus status;

    status = k_i2c_write(radio_bus, radio_rx.addr, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to request radio frame count: %d\n", status);
        return RADIO_ERROR;
    }

    status = k_i2c_read(radio_bus, radio_rx.addr, count, 2);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to read radio frame count: %d\n", status);
        return RADIO_ERROR;
    }

    return RADIO_OK;
}

KRadioStatus kprv_radio_rx_remove_frame(void)
{
    uint8_t    cmd = REMOVE_RX_FRAME;
    KI2CStatus status;

    status = k_i2c_write(radio_bus, radio_rx.addr, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to remove radio frame: %d\n", status);
        return RADIO_ERROR;
    }

    return RADIO_OK;
}

KRadioStatus kprv_radio_rx_get_frame(radio_rx_header * frame, uint8_t * message, uint8_t * len)
{
    if (frame == NULL || message == NULL)
    {
        return RADIO_ERROR_CONFIG;
    }

    uint8_t cmd = GET_RX_FRAME;

    KI2CStatus status;

    status = k_i2c_write(radio_bus, radio_rx.addr, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to request radio RX frame: %d\n",
                status);
        return RADIO_ERROR;
    }

    uint8_t * buffer = malloc(sizeof(radio_rx_header) + radio_rx.max_size);

    status = k_i2c_read(radio_bus, radio_rx.addr, (char *) buffer,
            sizeof(radio_rx_header) + radio_rx.max_size);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to read radio RX frame: %d\n", status);
        free(buffer);
        return RADIO_ERROR;
    }

    radio_rx_header * temp = (radio_rx_header *) buffer;

    frame->msg_size = temp->msg_size;
    frame->doppler_offset = temp->doppler_offset;
    frame->signal_strength = temp->signal_strength;

    memcpy(message, buffer+sizeof(radio_rx_header), frame->msg_size);

    if (len != NULL)
    {
        *len = frame->msg_size;
    }

    free(buffer);
    return RADIO_OK;
}
