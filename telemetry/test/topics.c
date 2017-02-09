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
    pubsub_conn * connection;
    telemetry_packet incoming_packet;
    int i = 0;
    uint16_t topic_id = 18;
    telemetry_packet outgoing_packet = {
        .data.i = 16,
        .source.topic_id = topic_id,
        .source.data_type = TELEMETRY_TYPE_INT,
        .source.subsystem_id = 1
    };

    telemetry_init();
    
    connection = telemetry_connect();
    assert_non_null(connection);
    
    assert_true(telemetry_subscribe(connection, topic_id));

    assert_int_equal(telemetry_num_subscribers(), 1);

    assert_true(telemetry_publish(outgoing_packet));

    assert_true(telemetry_read(connection, &incoming_packet));

    assert_true(telemetry_unsubscribe(connection, topic_id));

    assert_true(telemetry_publish(outgoing_packet));

    assert_false(telemetry_read(connection, &incoming_packet));

    assert_true(telemetry_disconnect(connection));

    telemetry_cleanup();

    assert_int_equal(telemetry_num_subscribers(), 0);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_subscriber)
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}