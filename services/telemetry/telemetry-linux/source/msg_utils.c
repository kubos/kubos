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
#include <telemetry/telemetry.h>
#include <tinycbor/cbor.h>

bool telemetry_parse_packet_msg(uint8_t * buffer, int buffer_size, telemetry_packet * packet)
{
    CborParser parser;
    CborValue map, element;

    if ((buffer == NULL) || (packet == NULL))
        return false;

    CborError err = cbor_parser_init(buffer, buffer_size, 0, &parser, &map);
    if (err || !cbor_value_is_map(&map)) {
        return false;
    }

    err = cbor_value_map_find_value(&map, "TOPIC_ID", &element);
    if (err)
        return false;

    if (cbor_value_get_int(&element, &packet->source.topic_id))
        return false;

    err = cbor_value_map_find_value(&map, "SUBSYSTEM_ID", &element);
    if (err || cbor_value_get_int(&element, &packet->source.subsystem_id))
        return false;

    err = cbor_value_map_find_value(&map, "DATA_TYPE", &element);
    if (err || cbor_value_get_int(&element, &packet->source.data_type))
        return false;

    err = cbor_value_map_find_value(&map, "DATA", &element);
    if (!err)
    {
        if (packet->source.data_type == TELEMETRY_TYPE_INT)
            cbor_value_get_int(&element, &packet->data.i);
        else if (packet->source.data_type == TELEMETRY_TYPE_FLOAT)
            cbor_value_get_float(&element, &packet->data.f);
    }

    err = cbor_value_map_find_value(&map, "TIMESTAMP", &element);
    if (err || cbor_value_get_int(&element, &packet->timestamp))
        return false;

    return true;
}

bool telemetry_parse_msg_type(uint8_t * buffer, int buffer_size, telemetry_message_type * req)
{
    CborParser parser;
    CborValue map, element;

    if ((buffer == NULL) || (req == NULL))
        return false;

    CborError err = cbor_parser_init(buffer, buffer_size, 0, &parser, &map);
    if (err || !cbor_value_is_map(&map)) {
        return false;
    }

    err = cbor_value_map_find_value(&map, "MESSAGE_TYPE", &element);
    if (err)
        return false;

    if (cbor_value_get_int(&element, req))
        return false;

    return true;
}

bool telemetry_parse_subscribe_msg(uint8_t * buffer, int buffer_size, int * topic_id)
{
    CborParser parser;
    CborValue map, element;

    if ((buffer == NULL) || (topic_id == NULL))
        return false;

    CborError err = cbor_parser_init(buffer, buffer_size, 0, &parser, &map);
    if (err || !cbor_value_is_map(&map)) {
        return false;
    }

    err = cbor_value_map_find_value(&map, "TOPIC_ID", &element);
    if (err)
        return false;

    if (cbor_value_get_int(&element, topic_id))
        return false;

    return true;
}

int telemetry_encode_subscribe_msg(uint8_t * buffer, int * topic_id)
{
    CborEncoder encoder, container;
    CborError err;

    if ((buffer == NULL) || (topic_id == NULL))
        return -1;

    if (start_encode_msg(&encoder, &container, buffer, 256, 2, MESSAGE_TYPE_SUBSCRIBE))
        return -1;

    err = cbor_encode_text_stringz(&container, "TOPIC_ID");
    if (err > 0)
        return -err;
    err = cbor_encode_int(&container, *topic_id);
    if (err > 0)
        return -err;

    return end_encode_msg(buffer, &encoder, &container);
}

bool telemetry_parse_unsubscribe_msg(uint8_t * buffer, int buffer_size, int * topic_id)
{
    CborParser parser;
    CborValue map, element;

    if ((buffer == NULL) || (topic_id == NULL))
        return false;

    CborError err = cbor_parser_init(buffer, buffer_size, 0, &parser, &map);
    if (err || !cbor_value_is_map(&map)) {
        return false;
    }

    err = cbor_value_map_find_value(&map, "TOPIC_ID", &element);
    if (err)
        return false;

    if (cbor_value_get_int(&element, topic_id))
        return false;

    return true;
}

int telemetry_encode_disconnect_msg(uint8_t * buffer)
{
    CborEncoder encoder, container;
    CborError err;

    if (buffer == NULL)
        return -1;

    if (start_encode_msg(&encoder, &container, buffer, 256, 1, MESSAGE_TYPE_DISCONNECT))
        return -1;

    return end_encode_msg(buffer, &encoder, &container);
}

int telemetry_encode_unsubscribe_msg(uint8_t * buffer, int * topic_id)
{
    CborEncoder encoder, container;
    CborError err;

    if ((buffer == NULL) || (topic_id == NULL))
        return -1;

    if (start_encode_msg(&encoder, &container, buffer, 256, 2, MESSAGE_TYPE_UNSUBSCRIBE))
        return -1;

    err = cbor_encode_text_stringz(&container, "TOPIC_ID");
    if (err > 0)
        return -err;
    err = cbor_encode_int(&container, *topic_id);
    if (err > 0)
        return -err;

    return end_encode_msg(buffer, &encoder, &container);
}

int start_encode_msg(CborEncoder * encoder, CborEncoder * container, uint8_t * buffer, int buffer_size, int num_elements, int message_type)
{
    CborError err;

    if (buffer == NULL)
        return -1;
    cbor_encoder_init(encoder, buffer, buffer_size, 0);
    err = cbor_encoder_create_map(encoder, container, num_elements);
    if (err > 0)
        return - err;
    err = cbor_encode_text_stringz(container, "MESSAGE_TYPE");
    if (err > 0)
        return -err;
    err = cbor_encode_int(container, message_type);
    if (err > 0)
        return -err;
    return 0;
}

int end_encode_msg(uint8_t * buffer, CborEncoder * encoder, CborEncoder * container)
{
    CborError err = cbor_encoder_close_container_checked(encoder, container);
    if (err > 0)
        return -err;

    return cbor_encoder_get_buffer_size(encoder, buffer);
}

int telemetry_encode_packet_msg(uint8_t * buffer, telemetry_packet * pkt)
{
    CborEncoder encoder, container;
    CborError err;

    if ((buffer == NULL) || (pkt == NULL))
        return -1;

    if (start_encode_msg(&encoder, &container, buffer, 256, 6, MESSAGE_TYPE_PACKET))
        return -1;

    err = cbor_encode_text_stringz(&container, "TOPIC_ID");
    if (err > 0)
        return -err;
    err = cbor_encode_int(&container, pkt->source.topic_id);
    if (err > 0)
        return -err;
    err = cbor_encode_text_stringz(&container, "SUBSYSTEM_ID");
    if (err > 0)
        return -err;
    err = cbor_encode_int(&container, pkt->source.subsystem_id);
    if (err > 0)
        return -err;
    err = cbor_encode_text_stringz(&container, "DATA_TYPE");
    if (err > 0)
        return -err;
    err = cbor_encode_int(&container, pkt->source.data_type);
    if (err > 0)
        return -err;
    err = cbor_encode_text_stringz(&container, "DATA");
    if (err > 0)
        return -err;
    if (pkt->source.data_type == TELEMETRY_TYPE_INT)
    {   
        err = cbor_encode_int(&container, pkt->data.i);
    } 
    else if (pkt->source.data_type == TELEMETRY_TYPE_FLOAT)
    {
        err = cbor_encode_float(&container, pkt->data.f);
    }
    else
    {
        err = cbor_encode_int(&container, 0);
    }
    if (err > 0)
        return -err;
    err = cbor_encode_text_stringz(&container, "TIMESTAMP");
    if (err > 0)
        return -err;
    err = cbor_encode_int(&container, pkt->timestamp);
    if (err > 0)
        return -err;
    
    return end_encode_msg(buffer, &encoder, &container);
}