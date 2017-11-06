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
/**
 * @defgroup radio Kubos Radio Interface
 * @addtogroup radio
 * @{
 */

#pragma once

#include <stdint.h>
#include "radio-struct.h"
#include "radio-impl.h"

/* Define the radio-specific structures/enumerators */
/*
#if defined(YOTTA_CFG_RADIO_TRXVU) && !defined(HAVE_RADIO)
#define HAVE_RADIO
#include <trxvu-radio-api/radio.h>
#endif
#ifndef HAVE_RADIO
#error No radio defined!
#endif
*/

/* Define the global functions */
/**
 * Initialize the radio interface
 * @return KRadioStatus RADIO_OK if OK, error otherwise
 */
KRadioStatus k_radio_init(void);
/**
 * Terminate the radio interface
 */
void k_radio_terminate(void);
/**
 * Configure the radio
 * @note This function might not be implemented for all radios. See specific radio API documentation for configuration availability and structure
 * @param [in] config Pointer to the radio configuration structure
 * @return KRadioStatus RADIO_OK if OK, error otherwise
 */
KRadioStatus k_radio_configure(radio_config * config);
/**
 * Reset the radio
 * @note This function might not be implemented for all radios
 * @param [in] type Type of reset to perform (hard, soft, etc)
 * @return KRadioStatus RADIO_OK if OK, error otherwise
 */
KRadioStatus k_radio_reset(KRadioReset type);
/**
 * Send a message to the radio's transmit buffer
 * @param [in] buffer Pointer to the message to send
 * @param [in] len Length of the message to send
 * @param [out] response Response value from radio (if supported)
 * @return KRadioStatus RADIO_OK if OK, error otherwise
 */
KRadioStatus k_radio_send(char * buffer, int len, uint8_t * response);
/**
 * Receive a message from the radio's receive buffer
 * @param [in] buffer Pointer where the message should be copied to
 * @param [out] len Length of the received message
 * @return KRadioStatus RADIO_OK if a message was received successfully, RADIO_RX_EMPTY if there are no messages to receive, error otherwise
 */
KRadioStatus k_radio_recv(radio_rx_message * buffer, uint8_t * len);
/**
 * Read radio telemetry values
 * @note See specific radio API documentation for available telemetry types
 * @param [in] buffer Pointer to structure which data should be copied to
 * @param [in] type Telemetry packet to read
 * @return KRadioStatus RADIO_OK if OK, error otherwise
 */
KRadioStatus k_radio_get_telemetry(radio_telem * buffer, RadioTelemType type);

/* @} */
