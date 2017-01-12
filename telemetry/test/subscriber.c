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
    telemetry_packet in_packet;
    telemetry_packet out_packet;

    out_packet.data.i = 10;
    
    telemetry_init();

    csp_sleep_ms(100);
    
    assert_true(telemetry_subscribe(&conn, 0));

    csp_sleep_ms(100);
    
    // assert_true(telemetry_num_subscribers() == 1);

    assert_true(telemetry_publish(out_packet));
    
    assert_true(telemetry_read(conn, &in_packet));

    assert_true(in_packet.data.i == out_packet.data.i);

    assert_true(telemetry_num_subscribers() == 1);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_subscriber)
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}