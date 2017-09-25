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
#include "telemetry/telemetry.h"

#define NUM_SUBS TELEMETRY_SUBSCRIBERS_MAX_NUM

static void test_subscriber(void ** arg)
{
    pubsub_conn * connections[NUM_SUBS];
    telemetry_packet incoming_packets[NUM_SUBS];
    
    bool subscribe_status[NUM_SUBS] = {false};
    bool read_status[NUM_SUBS] = {false};
    int i = 0;
    uint16_t topic_id = 18;
    telemetry_packet outgoing_packet = {
        .data.i = 16,
        .source.topic_id = topic_id,
        .source.data_type = TELEMETRY_TYPE_INT,
        .source.subsystem_id = 1
    };

    telemetry_init();
    
    for (i = 0; i < NUM_SUBS; i++)
    {
        connections[i] = telemetry_connect();
        assert_non_null(connections[i]);
    }

    for (i = 0; i < NUM_SUBS; i++)
    {
        subscribe_status[i] = telemetry_subscribe(connections[i], topic_id);
    }

    int total_subs = telemetry_num_subscribers();

    bool disconnect_status = telemetry_disconnect(connections[0]);

    int total_subs_minus_one = telemetry_num_subscribers();

    bool packet_published = telemetry_publish(outgoing_packet);

    for (i = 0; i < NUM_SUBS; i++)
    {
        read_status[i] = telemetry_read((connections[i]), &incoming_packets[i]);
    }

    for (i = 0; i < NUM_SUBS; i++)
    {
        telemetry_disconnect(connections[i]);
    }

    telemetry_cleanup();

    int end_total_subs = telemetry_num_subscribers();
    
    assert_int_equal(total_subs, NUM_SUBS);

    for (i = 0; i < NUM_SUBS; i++)
        assert_true(subscribe_status[i]);

    assert_true(packet_published);

    for (i = 1; i < NUM_SUBS; i++)
        assert_true(read_status[i]);

    assert_false(read_status[0]);

    for (i = 1; i < NUM_SUBS; i++)
        assert_int_equal(outgoing_packet.data.i, incoming_packets[i].data.i);

    assert_int_not_equal(outgoing_packet.data.i, incoming_packets[0].data.i);

    assert_true(disconnect_status);
    
    assert_int_equal(total_subs_minus_one, (NUM_SUBS - 1));

    assert_int_equal(end_total_subs, 0);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_subscriber)
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}