/*
 * Kubos TRXVU API
 * Copyright (C) 2018 Kubos Corporation
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
#include <ants-api/ants.h>

/* Test Data */
ants_rx_message test_packet
    = {.msg_size        = 8,
       .doppler_offset  = 5,    // todo: get real value
       .signal_strength = 2,    // todo
       .message         = "hi there" };

uint16_t frame_count = 5;
uint8_t  remaining   = 39;

trxvu_tx_telem_raw tx_telem = {.inst_RF_reflected = 63,
                               .inst_RF_forward   = 72,
                               .supply_voltage    = 1634,
                               .supply_current    = 290,
                               .temp_power_amp    = 2228,
                               .temp_oscillator   = 2224 };

trxvu_rx_telem_raw rx_telem = {.inst_doppler_offset  = 2069,
                               .supply_current       = 288,
                               .supply_voltage       = 1634,
                               .temp_oscillator      = 2240,
                               .temp_power_amp       = 2245,
                               .inst_signal_strength = 1153 };

trxvu_uptime uptime = 3456;

uint8_t tx_state = ants_STATE_IDLE_ON | ants_STATE_BEACON_ACTIVE
                   | (ants_STATE_RATE_4800 << 2);
/* End of Test Data */

static void test_no_init_send(void ** arg)
{
    char    data = 'A';
    uint8_t resp;

    assert_int_equal(k_ants_send(&data, 1, &resp), ants_ERROR);
}

static void test_no_init_recv(void ** arg)
{
    ants_rx_message buffer = { 0 };

    assert_int_equal(k_ants_recv(&buffer, NULL), ants_ERROR);
}

static void test_send(void ** arg)
{
    char         data = 'A';
    uint8_t      resp;
    KANTSStatus ret;

    expect_value(__wrap_write, cmd, SEND_FRAME);
    will_return(__wrap_read, 1);
    will_return(__wrap_read, &remaining);
    ret = k_ants_send(&data, 1, &resp);

    assert_int_equal(ret, ants_OK);
}

static void test_send_null(void ** arg)
{
    uint8_t      resp;
    KANTSStatus ret;

    ret = k_ants_send(NULL, 1, &resp);

    assert_int_equal(ret, ants_ERROR_CONFIG);
}

static int init(void ** state)
{
    will_return(__wrap_open, 1);
    k_ants_init();

    return 0;
}

static int term(void ** state)
{
    will_return(__wrap_close, 0);
    k_ants_terminate();

    return 0;
}

int main(void)
{
    const struct CMUnitTest init_tests[] = {
        cmocka_unit_test(test_no_init_send),
        cmocka_unit_test(test_no_init_recv)
    };

    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_send),
        cmocka_unit_test(test_send_null),
        cmocka_unit_test(test_send_resp_null),
        cmocka_unit_test(test_send_override),
        cmocka_unit_test(test_recv),
        cmocka_unit_test(test_recv_null),
        cmocka_unit_test(test_recv_len),
        cmocka_unit_test(test_config_null),
        cmocka_unit_test(test_set_beacon),
        cmocka_unit_test(test_set_beacon_override),
        cmocka_unit_test(test_clear_beacon),
        cmocka_unit_test(test_set_to),
        cmocka_unit_test(test_set_from),
        cmocka_unit_test(test_set_idle),
        cmocka_unit_test(test_set_rate),
        cmocka_unit_test(test_hard_reset),
        cmocka_unit_test(test_soft_reset),
        cmocka_unit_test(test_watchdog),
        cmocka_unit_test(test_watchdog_no_start_stop),
        cmocka_unit_test(test_telem_null),
        cmocka_unit_test(test_telem_tx_all),
        cmocka_unit_test(test_telem_tx_last),
        cmocka_unit_test(test_telem_tx_state),
        cmocka_unit_test(test_telem_tx_uptime),
        cmocka_unit_test(test_telem_rx_all),
        cmocka_unit_test(test_telem_rx_uptime),
    };

    if(cmocka_run_group_tests(init_tests, NULL, NULL) != 0)
    {
        return -1;
    }

    return 0;
    //return cmocka_run_group_tests(tests, init, term);
}
