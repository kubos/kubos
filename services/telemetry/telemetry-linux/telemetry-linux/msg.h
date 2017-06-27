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
 * @defgroup Telemetry-Message
 * @addtogroup Telemetry-Message
 * @brief Telemetry Message Parsing/Encoding Interface
 * @{
 */

#include <stdbool.h>
#include <stdint.h>
#include <telemetry/types.h>
#include <tinycbor/CBOR.h>

/**
 * Parses out the message type from an encoded message
 * @param[in] buffer buffer with encoded message
 * @param[in] buffer_size size of buffer
 * @param[out] msg_type parsed out type of message
 * @return bool true if successful, otherwise false
 */
bool telemetry_parse_msg_type(const uint8_t * buffer, uint32_t buffer_size, telemetry_message_type * msg_type);

/**
 * Attempts to encode a telemetry_packet
 * @param[out] buffer buffer to store encoded packet in
 * @param[in] pkt telemetry_packet to encode
 * @return int 0 if successful, otherwise negative error code
 */
int telemetry_encode_packet_msg(uint8_t * buffer, const telemetry_packet * pkt);

/**
 * Attempt to parse telemetry_packet from buffer
 * @param[in] buffer buffer storing packet data
 * @param[in] buffer_size size of buffer
 * @param[out] packet telemetry_packet to store data in
 * @return bool true if successful, otherwise false
 */
bool telemetry_parse_packet_msg(const uint8_t * buffer, uint32_t buffer_size, telemetry_packet * packet);

/**
 * Attempts to encode a subscribe message
 * @param[out] buffer buffer to store encoded packet in
 * @param[in] topic_id topic ID for subscribing
 * @return int 0 if successful, otherwise negative error code
 */
int telemetry_encode_subscribe_msg(uint8_t * buffer, const uint16_t * topic_id);

/**
 * Attempt to parse a subscribe message
 * @param[in] buffer buffer storing packet data
 * @param[in] buffer_size size of buffer
 * @param[out] topic_id topic ID read from message
 * @return bool true if successful, otherwise false
 */
bool telemetry_parse_subscribe_msg(const uint8_t * buffer, uint32_t buffer_size, uint16_t * topic_id);

/**
 * Attempts to encode an unsubscribe message
 * @param[out] buffer buffer to store encoded packet in
 * @param[in] topic_id topic ID to encode in message
 * @return int 0 if successful, otherwise negative error code
 */
int telemetry_encode_unsubscribe_msg(uint8_t * buffer, const uint16_t * topic_id);

/**
 * Attempt to parse an unsubscribe message
 * @param[in] buffer buffer storing packet data
 * @param[in] buffer_size size of buffer
 * @param[out] topic_id topic ID read from message
 * @return bool true if successful, otherwise false
 */
bool telemetry_parse_unsubscribe_msg(const uint8_t * buffer, uint32_t buffer_size, uint16_t * topic_id);

/**
 * Attempts to encode disconnect message
 * @param[out] buffer buffer to store encoded packet in
 * @return int 0 if successful, otherwise negative error code
 */
int telemetry_encode_disconnect_msg(uint8_t * buffer);

/**
 * Sets up the structures for encoding a message
 * @param[out] encoder Master CBOR encoder
 * @param[out] container CBOR container for map
 * @param[out] buffer buffer to store end data in
 * @param[in] buffer_size size of buffer
 * @param[in] num_elements number of elements in the message map
 * @param[in] message_type message type to be encoded
 * @return int 0 if successful, otherwise negative error
 */
int start_encode_msg(CborEncoder * encoder, CborEncoder * container, uint8_t * buffer, uint32_t buffer_size, uint8_t num_elements, telemetry_message_type message_type);

/**
 * Finishing the CBOR encoding process
 * @param[in,out] buffer buffer for storing serialized data
 * @param[in,out] encoder CBOR encoder
 * @param[in,out] container CBOR map container
 * @return int 0 if successful, otherwise negative error code
 */
int end_encode_msg(uint8_t * buffer, CborEncoder * encoder, CborEncoder * container);

/* @} */