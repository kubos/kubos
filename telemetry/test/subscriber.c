/*
 * KubOS HAL
 * Copyright (C) 2016 Kubos Corporation
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

static void test_subscriber(void)
{
    pubsub_conn conn;
    pubsub_conn conn2;
    telemetry_packet in_packet;
    telemetry_packet in_packet2;
    telemetry_packet out_packet;

    out_packet.data.i = 10;
    
    telemetry_init();
    
    assert_true(telemetry_subscribe(&conn2, 0));

    bool subscribed = telemetry_subscribe(&conn, 0);

    int num_subs = telemetry_num_subscribers();

    bool published = telemetry_publish(out_packet);
    
    bool read = telemetry_read(conn, &in_packet);

    bool read2 = (telemetry_read(conn2, &in_packet2));

    int num_subs_pre_clean = telemetry_num_subscribers();

    telemetry_cleanup();

    csp_close(conn.conn_handle);

    int num_subs_post_clean = telemetry_num_subscribers();

    assert_true(subscribed);
    assert_true(published);
    assert_true(read);
    assert_true(out_packet.data.i == in_packet.data.i);
    assert_true(out_packet.data.i == in_packet2.data.i);
    assert_true(num_subs_pre_clean == 2);
    assert_true(num_subs_post_clean == 0);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_subscriber)
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}