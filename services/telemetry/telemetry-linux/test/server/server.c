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


static void test_server_add_subscription(void ** arg)
{
    subscriber_list_item sub = {
        .topics = NULL
    };

    assert_true(kprv_add_topic(&sub, 5));

    assert_true(kprv_has_topic(&sub, 5));
}

static void test_server_remove_subscription(void ** arg)
{
    subscriber_list_item sub = {
        .topics = NULL
    };

    kprv_add_topic(&sub, 5);

    assert_true(kprv_remove_topic(&sub, 5));

    assert_false(kprv_has_topic(&sub, 5));
}

static void test_server_create_subscriber(void ** arg)
{
    pubsub_conn conn;
    subscriber_list_item * sub = NULL;

    sub = create_subscriber(conn);

    assert_non_null(sub);
}

static void test_server_publish_packet(void ** arg)
{
    telemetry_packet in_packet = {
        .source.topic_id = 5,
        .source.data_type = TELEMETRY_TYPE_INT,
        .data.i = 12
    };
    pubsub_conn conn;
    telemetry_packet out_packet;
    subscriber_list_item * sub = NULL;

    sub = create_subscriber(conn);

    assert_true(telemetry_publish_packet(sub, in_packet));
    assert_int_equal(telemetry_get_num_packets(sub), 1);
    assert_true(telemetry_get_packet(sub, &out_packet));

    assert_int_equal(in_packet.data.i, out_packet.data.i);

    assert_int_equal(telemetry_get_num_packets(sub), 0);
}

static void test_server_publish_multiple_packets(void ** arg)
{
    telemetry_packet in_packet = {
        .source.topic_id = 5,
        .source.data_type = TELEMETRY_TYPE_INT
    };
    int i = 0;
    pubsub_conn conn;
    telemetry_packet out_packet;
    subscriber_list_item * sub = NULL;

    sub = create_subscriber(conn);

    for (i = 0; i < 5; i++)
    {
        in_packet.data.i = i;
        telemetry_publish_packet(sub, in_packet);
    }

    assert_int_equal(telemetry_get_num_packets(sub), 5);

    for (i = 0; i < 5; i++)
    {
        assert_true(telemetry_get_packet(sub, &out_packet));
        assert_int_equal(out_packet.data.i, i);
    }
    
    assert_int_equal(telemetry_get_num_packets(sub), 0);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_server_add_subscription),
        cmocka_unit_test(test_server_remove_subscription),
        cmocka_unit_test(test_server_create_subscriber),
        cmocka_unit_test(test_server_publish_packet),
        cmocka_unit_test(test_server_publish_multiple_packets),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}