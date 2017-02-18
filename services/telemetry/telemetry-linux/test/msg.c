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

#include <cmocka.h>
#include <tinycbor/cbor.h>
#include "telemetry/telemetry.h"

static void test_packet_msg(void ** arg)
{
    telemetry_packet in = {
        .source.topic_id = 1,
        .source.subsystem_id = 2,
        .source.data_type = TELEMETRY_TYPE_INT,
        .data.i = 11,
        .timestamp = 1010101
    };
    telemetry_message_type msg_type;
    telemetry_packet out;
    char buffer[100];

    int msg_size = telemetry_encode_packet_msg(buffer, &in);
    bool parsed_type = telemetry_parse_msg_type(buffer, msg_size, &msg_type);
    bool parsed = telemetry_parse_packet_msg(buffer, msg_size, &out);

    assert_true(msg_size > 0);
    assert_true(parsed_type);
    assert_true(parsed);

    assert_int_equal(msg_type, MESSAGE_TYPE_PACKET);
    assert_int_equal(in.source.topic_id, out.source.topic_id);
    assert_int_equal(in.source.subsystem_id, out.source.subsystem_id);
    assert_int_equal(in.source.data_type, out.source.data_type);
    assert_int_equal(in.data.i, out.data.i);
    assert_int_equal(in.timestamp, out.timestamp);
}

static void test_subscribe_msg(void ** arg)
{
    char buffer[100];
    telemetry_message_type msg_type;
    int topic_in, topic_out;

    int msg_size = telemetry_encode_subscribe_msg(buffer, &topic_in);
    bool parsed_type = telemetry_parse_msg_type(buffer, msg_size, &msg_type);
    bool parsed = telemetry_parse_subscribe_msg(buffer, msg_size, &topic_out);

    assert_true(msg_size > 0);
    assert_true(parsed_type);
    assert_true(parsed);

    assert_int_equal(msg_type, MESSAGE_TYPE_SUBSCRIBE);
    assert_int_equal(topic_in, topic_out);
}

static void test_unsubscribe_msg(void ** arg)
{
    char buffer[100];
    telemetry_message_type msg_type;
    int topic_in, topic_out;

    int msg_size = telemetry_encode_unsubscribe_msg(buffer, &topic_in);
    bool parsed_type = telemetry_parse_msg_type(buffer, msg_size, &msg_type);
    bool parsed = telemetry_parse_unsubscribe_msg(buffer, msg_size, &topic_out);

    assert_true(msg_size > 0);
    assert_true(parsed_type);
    assert_true(parsed);

    assert_int_equal(msg_type, MESSAGE_TYPE_UNSUBSCRIBE);
    assert_int_equal(topic_in, topic_out);
}

static void test_disconnect_msg(void ** arg)
{
    char buffer[100];
    telemetry_message_type msg_type;

    int msg_size = telemetry_encode_disconnect_msg(buffer);
    bool parsed_type = telemetry_parse_msg_type(buffer, msg_size, &msg_type);

    assert_true(msg_size > 0);
    assert_true(parsed_type);

    assert_int_equal(msg_type, MESSAGE_TYPE_DISCONNECT);
}

static void test_start_encode_msg(void ** arg)
{
    char buffer[100];
    CborEncoder encoder, container;

    assert_int_equal(start_encode_msg(&encoder, &container, buffer, 100, 1, MESSAGE_TYPE_PACKET), 0);
}

static void test_end_encode_msg(void ** arg)
{
    char buffer[100];
    CborEncoder encoder, container;

    start_encode_msg(&encoder, &container, buffer, 100, 1, MESSAGE_TYPE_PACKET);

    assert_true(end_encode_msg(buffer, &encoder, &container) > 0);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_packet_msg),
        cmocka_unit_test(test_subscribe_msg),
        cmocka_unit_test(test_unsubscribe_msg),
        cmocka_unit_test(test_disconnect_msg),
        cmocka_unit_test(test_start_encode_msg),
        cmocka_unit_test(test_end_encode_msg),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}