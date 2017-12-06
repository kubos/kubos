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
 *
 * Global Radio Structures/Enumerators
 */
/**
 * @addtogroup radio
 * @{
 */

#pragma once

/**
 * Radio function return values
 */
typedef enum {
    /** Function call completed successfully */
    RADIO_OK = 0,
    /** Radio receive buffer is empty */
    RADIO_RX_EMPTY,
    /** Generic radio error */
    RADIO_ERROR,
    /** Function input parameter is invalid */
    RADIO_ERROR_CONFIG
} KRadioStatus;

/**
 * Radio reset types
 */
typedef enum {
    /** Perform hardware-level radio reset */
    RADIO_HARD_RESET,
    /** Perform software radio reset */
    RADIO_SOFT_RESET
} KRadioReset;

/**
 * AX.25 call-sign structure
 */
typedef struct
{
    /**
     * Six character station call-sign
     */
    uint8_t ascii[6];
    /**
     * One byte station SSID value
     */
    uint8_t ssid;
} ax25_callsign;
/* @} */
