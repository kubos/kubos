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

/* Public functions */

/* Send a message to the transmission buffer */
KRadioStatus k_radio_send(char * buffer, int len, uint8_t * response)
{
    if (buffer == NULL || len < 1 || len > radio_tx.max_size || response == NULL)
    {
        return RADIO_ERROR_CONFIG;
    }

    char * packet   = malloc(len + 1);
    packet[0]       = SEND_FRAME;

    memcpy(packet + 1, buffer, len);

    KI2CStatus status = k_i2c_write(radio_bus, radio_tx.addr, packet, len + 1);
    free(packet);

    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to send radio TX frame: %d\n", status);
        return RADIO_ERROR;
    }

    /* Read number of remaining TX buffer slots available */
    status = k_i2c_read(radio_bus, radio_tx.addr, response, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to read radio TX slots remaining: %d\n",
                status);
        return RADIO_ERROR;
    }

    return RADIO_OK;
}

/* Send a message to the transmit buffer, but use non-default AX.25 call-signs
 */
KRadioStatus k_radio_send_override(ax25_callsign to, ax25_callsign from,
                                   char * buffer, int len, uint8_t * response)
{

    if (buffer == NULL || len < 1 || len > radio_tx.max_size || response == NULL)
    {
        return RADIO_ERROR_CONFIG;
    }

    char * packet   = malloc(len + 15);
    packet[0]       = SEND_AX25_OVERRIDE;

    memcpy(packet + 1, &to, sizeof(ax25_callsign));
    memcpy(packet + 8, &from, sizeof(ax25_callsign));
    memcpy(packet + 15, buffer, len);

    KI2CStatus status = k_i2c_write(radio_bus, radio_tx.addr, packet,
                                    len + sizeof(ax25_callsign) * 2 + 1);
    free(packet);

    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to send radio TX frame (override): %d\n",
                status);
        return RADIO_ERROR;
    }

    /* Read number of remaining TX buffer slots available */
    status = k_i2c_read(radio_bus, radio_tx.addr, response, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to read radio TX slots remaining: %d\n",
                status);
        return RADIO_ERROR;
    }

    return RADIO_OK;
}

/* Set automatic beacon message + rate (override callsigns) */
KRadioStatus k_radio_set_beacon_override(ax25_callsign to, ax25_callsign from,
                                         radio_tx_beacon beacon)
{
    /* Max rate of 3000 is specified in TRXVU datasheet */
    if (beacon.interval > 3000 || beacon.msg == NULL || beacon.len < 1)
    {
        return RADIO_ERROR_CONFIG;
    }

    KI2CStatus status;
    char * packet   = malloc(beacon.len + sizeof(ax25_callsign) * 2 + 3);
    packet[0] = SET_AX25_BEACON_OVERRIDE;

    memcpy(packet + 1, (void *) &beacon.interval, sizeof(beacon.interval));
    memcpy(packet + 3, &to, sizeof(ax25_callsign));
    memcpy(packet + 10, &from, sizeof(ax25_callsign));
    memcpy(packet + 17, beacon.msg, beacon.len);

    status = k_i2c_write(radio_bus, radio_tx.addr, packet,
                         beacon.len + sizeof(ax25_callsign) * 2 + 3);

    free(packet);

    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to set radio TX beacon (override): %d\n",
                status);
        return RADIO_ERROR;
    }

    return RADIO_OK;
}

/* Stop/clear the automatic periodic beacon */
KRadioStatus k_radio_clear_beacon(void)
{
    KI2CStatus status;
    uint8_t    cmd = CLEAR_BEACON;

    status = k_i2c_write(radio_bus, radio_tx.addr, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to clear radio TX beacon: %d\n", status);
        return RADIO_ERROR;
    }

    return RADIO_OK;
}

/* Private Functions */

KRadioStatus kprv_radio_tx_get_telemetry(radio_telem *  buffer,
                                         RadioTelemType type)
{
    uint8_t cmd;
    uint8_t len;

    if (buffer == NULL)
    {
        return RADIO_ERROR_CONFIG;
    }

    switch (type)
    {
        case RADIO_TX_TELEM_ALL:
            cmd = GET_TX_ALL_TELEMETRY;
            len = sizeof(trxvu_tx_telem_raw);
            break;
        case RADIO_TX_TELEM_LAST:
            cmd = GET_LAST_TRANS_TELEM;
            len = sizeof(trxvu_tx_telem_raw);
            break;
        case RADIO_TX_UPTIME:
            cmd = GET_UPTIME;
            len = sizeof(trxvu_uptime);
            break;
        case RADIO_TX_STATE:
            cmd = GET_TX_STATE;
            len = 1;
            break;
        default:
            fprintf(stderr, "Unknown radio telemetry type requested: %d\n",
                    type);
            return RADIO_ERROR;
    }

    KI2CStatus status
        = k_i2c_write(radio_bus, radio_tx.addr, (uint8_t *) &cmd, 1);

    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to request radio TX telemetry: %d\n", status);
        return RADIO_ERROR;
    }

    status = k_i2c_read(radio_bus, radio_tx.addr, (char *) buffer, len);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to read radio TX telemetry: %d\n", status);
        return RADIO_ERROR;
    }

    return RADIO_OK;
}

KRadioStatus kprv_radio_tx_watchdog_kick(void)
{
    KI2CStatus status;
    uint8_t    cmd = WATCHDOG_RESET;

    status = k_i2c_write(radio_bus, radio_tx.addr, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to kick radio TX watchdog: %d\n", status);
        return RADIO_ERROR;
    }

    return RADIO_OK;
}

KRadioStatus kprv_radio_tx_reset(KRadioReset type)
{
    KRadioStatus status = RADIO_OK;
    uint8_t      cmd;

    switch (type)
    {
        case RADIO_SOFT_RESET:
            cmd = SOFT_RESET;
            break;
        case RADIO_HARD_RESET:
            cmd = HARD_RESET;
            break;
        default:
            fprintf(stderr, "Unknown radio TX reset type: %d\n", type);
            return RADIO_ERROR_CONFIG;
    }

    status = k_i2c_write(radio_bus, radio_tx.addr, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to reset TX radio: %d\n", status);
        return RADIO_ERROR;
    }

    return RADIO_OK;
}

KRadioStatus kprv_radio_tx_set_beacon(uint16_t rate, char * buffer, int len)
{
    /* Max rate of 3000 is specified in TRXVU datasheet */
    if (rate > 3000 || buffer == NULL || len < 1)
    {
        return RADIO_ERROR_CONFIG;
    }

    char * packet   = malloc(len + 3);
    packet[0]       = SET_BEACON;

    memcpy(packet + 1, (void *) &rate, 2);
    memcpy(packet + 3, buffer, len);

    KI2CStatus status = k_i2c_write(radio_bus, radio_tx.addr, packet, len + 3);

    free(packet);

    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to set radio TX beacon: %d\n", status);
        return RADIO_ERROR;
    }

    return RADIO_OK;
}

KRadioStatus kprv_radio_tx_set_default_to(ax25_callsign to)
{
    char packet[8] = { 0 };
    packet[0]      = SET_DEFAULT_AX25_TO;

    memcpy(packet + 1, &to, sizeof(ax25_callsign));

    KI2CStatus status
        = k_i2c_write(radio_bus, radio_tx.addr, packet, sizeof(packet));
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to set radio TX destination callsign: %d\n",
                status);
        return RADIO_ERROR;
    }

    return RADIO_OK;
}

KRadioStatus kprv_radio_tx_set_default_from(ax25_callsign from)
{
    char packet[8] = { 0 };
    packet[0]      = SET_DEFAULT_AX25_FROM;

    memcpy(packet + 1, &from, sizeof(ax25_callsign));

    KI2CStatus status
        = k_i2c_write(radio_bus, radio_tx.addr, packet, sizeof(packet));
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to set radio TX sender callsign: %d\n", status);
        return RADIO_ERROR;
    }

    return RADIO_OK;
}

KRadioStatus kprv_radio_tx_set_idle(RadioIdleState state)
{
    char packet[2] = { 0 };
    packet[0]      = SET_IDLE_STATE;

    if (state == RADIO_IDLE_OFF)
    {
        packet[1] = 0;
    }
    else if (state == RADIO_IDLE_ON)
    {
        packet[1] = 1;
    }
    else
    {
        return RADIO_ERROR_CONFIG;
    }

    KI2CStatus status
        = k_i2c_write(radio_bus, radio_tx.addr, packet, sizeof(packet));
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to set radio TX idle state: %d\n", status);
        return RADIO_ERROR;
    }

    return RADIO_OK;
}

/* Set the transmission data rate */
KRadioStatus kprv_radio_tx_set_rate(RadioTXRate rate)
{
    char packet[2] = { 0 };
    packet[0]      = SET_TX_RATE;
    packet[1]      = (uint8_t) rate;

    KI2CStatus status
        = k_i2c_write(radio_bus, radio_tx.addr, packet, sizeof(packet));
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to set radio TX data rate: %d\n", status);
        return RADIO_ERROR;
    }

    return RADIO_OK;
}
