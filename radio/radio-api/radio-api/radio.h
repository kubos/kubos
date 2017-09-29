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

#pragma once

#include <stdint.h>

/**
 * Radio function status
 */
typedef enum {
    RADIO_OK = 0,
    RADIO_RX_EMPTY,
    RADIO_ERROR,
    RADIO_ERROR_CONFIG
} KRadioStatus;

typedef enum {
    RADIO_HARD_RESET,
    RADIO_SOFT_RESET
} radio_reset;

typedef struct
{
    uint8_t ascii[6];
    uint8_t ssid;
} ax25_callsign;

KRadioStatus k_radio_tx_init(void);
void k_radio_tx_terminate(void);
KRadioStatus k_radio_tx_configure(uint8_t * radio_config);

KRadioStatus k_radio_tx_watchdog_kick(void);
KRadioStatus k_radio_tx_reset(uint8_t type);

uint8_t k_radio_send(char * buffer, int len);
KRadioStatus k_radio_recv(char * buffer, int len);

KRadioStatus k_radio_tx_get_telemetry(uint8_t * buffer, uint8_t type);
