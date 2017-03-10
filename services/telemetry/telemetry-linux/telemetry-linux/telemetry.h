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
 * @defgroup Telemetry
 * @addtogroup Telemetry
 * @brief Telemetry Public telemetry interface
 * @{
 */

#ifndef TELEMETRY_LINUX_H
#define TELEMETRY_LINUX_H

#include <ipc/pubsub_socket.h>
#include <telemetry/telemetry.h>

CSP_DEFINE_TASK(client_handler);

bool telemetry_parse_packet_msg(uint8_t * buffer, int buffer_size, telemetry_packet * packet);
int telemetry_encode_packet_msg(uint8_t * buffer, telemetry_packet * pkt);

int telemetry_encode_subscribe_msg(uint8_t * buffer, int * topic_id);
bool telemetry_parse_subscribe_msg(uint8_t * buffer, int buffer_size, int * topic_id);

int telemetry_encode_unsubscribe_msg(uint8_t * buffer, int * topic_id);
bool telemetry_parse_unsubscribe_msg(uint8_t * buffer, int buffer_size, int * topic_id);

int telemetry_encode_disconnect_msg(uint8_t * buffer);

int start_encode_msg(CborEncoder * encoder, CborEncoder * container, uint8_t * buffer, int buffer_size, int num_elements, int message_type);
int end_encode_msg(uint8_t * buffer, CborEncoder * encoder, CborEncoder * container);
bool telemetry_parse_msg_type(uint8_t * buffer, int buffer_size, telemetry_message_type * req);

bool add_subscriber(subscriber_list_item * sub);
subscriber_list_item * create_subscriber(socket_conn conn);
void destroy_subscriber(subscriber_list_item ** sub);
bool telemetry_get_packet(subscriber_list_item * sub, telemetry_packet * packet);
int telemetry_get_num_packets(subscriber_list_item * sub);
bool telemetry_publish_packet(subscriber_list_item * sub, telemetry_packet packet);
bool telemetry_process_message(subscriber_list_item * sub, void * buffer, int buffer_size);
bool kprv_has_topic(const subscriber_list_item * sub, uint16_t topic_id);
bool kprv_remove_topic(subscriber_list_item * sub, int topic_id);
bool kprv_add_topic(subscriber_list_item * sub, int topic_id);

void kprv_delete_topics(subscriber_list_item * sub);

void telemetry_server_init(void);
void telemetry_server_cleanup(void);
bool client_rx_work(subscriber_list_item * sub);

void kprv_delete_subscribers();

bool kprv_publish_packet(telemetry_packet packet);

bool kprv_cbor_read(const socket_conn * conn, void * buffer, int max_buffer_size, uint8_t port, uint16_t * size_received);

#endif

/* @} */