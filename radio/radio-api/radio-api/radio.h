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
    RADIO_ERROR
} KRadioStatus;

KRadioStatus radio_tx_init(void);
KRadioStatus radio_tx_configure(uint8_t * conf);

KRadioStatus radio_tx_watchdog_kick(void);
KRadioStatus radio_tx_reset(int hard);

uint8_t radio_tx_send(char * buffer, int len);

KRadioStatus radio_tx_get_telemetry(uint8_t * buffer, uint8_t type);
