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
#include <trxvu.h>

#define TX_SIZE 100
#define RX_SIZE 100

/* Test Data */
radio_rx_header test_header
    = {.msg_size        = 8,
       .doppler_offset  = 5,    // todo: get real value
       .signal_strength = 2,    // todo
};
char * test_message = "hi there";

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

uint8_t tx_state = RADIO_STATE_IDLE_ON | RADIO_STATE_BEACON_ACTIVE
                   | (RADIO_STATE_RATE_4800 << 2);
/* End of Test Data */

static void test_no_init_recv(void ** arg)
{
    radio_rx_header header = { 0 };
    uint8_t buffer[RX_SIZE] = { 0 };

    assert_int_equal(k_radio_recv(&header, buffer, NULL), RADIO_ERROR);
}

static void test_send(void ** arg)
{
    char         data = 'A';
    uint8_t      resp;
    KRadioStatus ret;

    expect_value(__wrap_write, cmd, SEND_FRAME);
    will_return(__wrap_read, 1);
    will_return(__wrap_read, &remaining);
    ret = k_radio_send(&data, 1, &resp);

    assert_int_equal(ret, RADIO_OK);
}

static void test_send_null(void ** arg)
{
    uint8_t      resp;
    KRadioStatus ret;

    ret = k_radio_send(NULL, 1, &resp);

    assert_int_equal(ret, RADIO_ERROR_CONFIG);
}

static void test_send_resp_null(void ** arg)
{
    char         data = 'A';
    KRadioStatus ret;

    ret = k_radio_send(&data, 1, NULL);

    assert_int_equal(ret, RADIO_ERROR_CONFIG);
}

static void test_send_override(void ** arg)
{
    char          data = 'A';
    ax25_callsign to;
    ax25_callsign from;
    uint8_t       resp;
    KRadioStatus  ret;

    expect_value(__wrap_write, cmd, SEND_AX25_OVERRIDE);
    will_return(__wrap_read, 1);
    will_return(__wrap_read, &remaining);
    ret = k_radio_send_override(to, from, &data, 1, &resp);

    assert_int_equal(ret, RADIO_OK);
}

static void test_recv(void ** arg)
{
    radio_rx_header header = { 0 };
    uint8_t buffer[RX_SIZE] = { 0 };

    KRadioStatus     ret;

    /* get_frame_count */
    expect_value(__wrap_write, cmd, GET_RX_FRAME_COUNT);
    will_return(__wrap_read, 2);
    will_return(__wrap_read, &frame_count);

    /* get_frame */
    expect_value(__wrap_write, cmd, GET_RX_FRAME);
    will_return(__wrap_read, sizeof(radio_rx_header) + RX_SIZE);
    will_return(__wrap_read, &test_header);

    /* remove_frame */
    expect_value(__wrap_write, cmd, REMOVE_RX_FRAME);

    ret = k_radio_recv(&header, buffer, NULL);

    assert_int_equal(ret, RADIO_OK);
}

static void test_recv_null(void ** arg)
{
    KRadioStatus ret;

    ret = k_radio_recv(NULL, NULL, NULL);

    assert_int_equal(ret, RADIO_ERROR_CONFIG);
}

static void test_recv_len(void ** arg)
{
    radio_rx_header header = { 0 };
    uint8_t buffer[RX_SIZE] = { 0 };

    uint16_t         len    = 0;
    KRadioStatus     ret;

    expect_value(__wrap_write, cmd, GET_RX_FRAME_COUNT);
    will_return(__wrap_read, 2);
    will_return(__wrap_read, &frame_count);

    expect_value(__wrap_write, cmd, GET_RX_FRAME);
    will_return(__wrap_read, sizeof(radio_rx_header) + RX_SIZE);
    will_return(__wrap_read, &test_header);

    expect_value(__wrap_write, cmd, REMOVE_RX_FRAME);

    ret = k_radio_recv(&header, buffer, (uint8_t *) &len);

    assert_int_equal(ret, RADIO_OK);
    assert_int_equal(len, header.msg_size);
}

static void test_config_null(void ** arg)
{
    assert_int_equal(k_radio_configure(NULL), RADIO_ERROR_CONFIG);
}

static void test_set_beacon(void ** arg)
{
    KRadioStatus ret;

    radio_config config       = { 0 };
    char         beacon_msg[] = "Radio Beacon Message";
    config.beacon.interval    = 5;
    config.beacon.msg         = beacon_msg;
    config.beacon.len         = sizeof(beacon_msg);

    expect_value(__wrap_write, cmd, SET_BEACON);
    ret = k_radio_configure(&config);

    assert_int_equal(ret, RADIO_OK);
}

static void test_set_beacon_override(void ** arg)
{
    ax25_callsign   to;
    ax25_callsign   from;
    KRadioStatus    ret;
    radio_tx_beacon beacon = { 0 };

    char beacon_msg[] = "Radio Beacon Message";
    beacon.interval   = 5;
    beacon.msg        = beacon_msg;
    beacon.len        = sizeof(beacon_msg);

    expect_value(__wrap_write, cmd, SET_AX25_BEACON_OVERRIDE);
    ret = k_radio_set_beacon_override(to, from, beacon);

    assert_int_equal(ret, RADIO_OK);
}

static void test_clear_beacon(void ** arg)
{
    ax25_callsign to;
    ax25_callsign from;
    KRadioStatus  ret;

    expect_value(__wrap_write, cmd, CLEAR_BEACON);
    ret = k_radio_clear_beacon();

    assert_int_equal(ret, RADIO_OK);
}

static void test_set_to(void ** arg)
{
    KRadioStatus ret;

    radio_config config = { 0 };
    strncpy(config.to.ascii, "HMTLN1", sizeof(config.to.ascii));

    expect_value(__wrap_write, cmd, SET_DEFAULT_AX25_TO);
    ret = k_radio_configure(&config);

    assert_int_equal(ret, RADIO_OK);
}

static void test_set_from(void ** arg)
{
    KRadioStatus ret;

    radio_config config = { 0 };
    strncpy(config.from.ascii, "HMLTN1", sizeof(config.from.ascii));

    expect_value(__wrap_write, cmd, SET_DEFAULT_AX25_FROM);
    ret = k_radio_configure(&config);

    assert_int_equal(ret, RADIO_OK);
}

static void test_set_idle(void ** arg)
{
    KRadioStatus ret;

    radio_config config = { 0 };
    config.idle         = RADIO_IDLE_ON;

    expect_value(__wrap_write, cmd, SET_IDLE_STATE);
    ret = k_radio_configure(&config);

    assert_int_equal(ret, RADIO_OK);
}

static void test_set_rate(void ** arg)
{
    KRadioStatus ret;

    radio_config config = { 0 };
    config.data_rate    = RADIO_TX_RATE_9600;

    expect_value(__wrap_write, cmd, SET_TX_RATE);
    ret = k_radio_configure(&config);

    assert_int_equal(ret, RADIO_OK);
}

static void test_hard_reset(void ** arg)
{
    KRadioStatus ret;

    /* Two reset calls, one for TX and one for RX */
    expect_value(__wrap_write, cmd, HARD_RESET);
    expect_value(__wrap_write, cmd, HARD_RESET);
    ret = k_radio_reset(RADIO_HARD_RESET);

    assert_int_equal(ret, RADIO_OK);
}

static void test_soft_reset(void ** arg)
{
    KRadioStatus ret;

    /* Two reset calls, one for TX and one for RX */
    expect_value(__wrap_write, cmd, SOFT_RESET);
    expect_value(__wrap_write, cmd, SOFT_RESET);
    ret = k_radio_reset(RADIO_SOFT_RESET);

    assert_int_equal(ret, RADIO_OK);
}

static void test_watchdog(void ** arg)
{
    KRadioStatus start_ret;
    KRadioStatus stop_ret;

    /* Two calls, one for TX and one for RX */
    expect_value(__wrap_write, cmd, WATCHDOG_RESET);
    expect_value(__wrap_write, cmd, WATCHDOG_RESET);
    start_ret = k_radio_watchdog_start();

    stop_ret = k_radio_watchdog_stop();

    assert_int_equal(start_ret, RADIO_OK);
    assert_int_equal(stop_ret, RADIO_OK);
}

static void test_watchdog_no_start_stop(void ** arg)
{
    assert_int_equal(k_radio_watchdog_stop(), RADIO_ERROR);
}

static void test_telem_null(void ** arg)
{
    assert_int_equal(k_radio_get_telemetry(NULL, RADIO_TX_TELEM_ALL),
                     RADIO_ERROR_CONFIG);
}

static void test_telem_tx_all(void ** arg)
{
    KRadioStatus ret;
    radio_telem  telem;

    expect_value(__wrap_write, cmd, GET_TX_ALL_TELEMETRY);
    will_return(__wrap_read, sizeof(trxvu_tx_telem_raw));
    will_return(__wrap_read, &tx_telem);
    ret = k_radio_get_telemetry(&telem, RADIO_TX_TELEM_ALL);

    assert_int_equal(ret, RADIO_OK);

    assert_int_equal(get_rf_power_dbm(telem.tx_telem.inst_RF_reflected), -6);
    assert_int_equal(get_rf_power_mw(telem.tx_telem.inst_RF_forward), 0);
    assert_int_equal(get_voltage(telem.tx_telem.supply_voltage), 7);
    assert_int_equal(get_current(telem.tx_telem.supply_current), 48);
    assert_int_equal(get_temperature(telem.tx_telem.temp_power_amp), 24);
    assert_int_equal(get_temperature(telem.tx_telem.temp_oscillator), 25);
}

static void test_telem_tx_last(void ** arg)
{
    KRadioStatus ret;
    radio_telem  telem;

    expect_value(__wrap_write, cmd, GET_LAST_TRANS_TELEM);
    will_return(__wrap_read, sizeof(trxvu_tx_telem_raw));
    will_return(__wrap_read, &tx_telem);
    ret = k_radio_get_telemetry(&telem, RADIO_TX_TELEM_LAST);

    assert_int_equal(ret, RADIO_OK);

    assert_int_equal(get_rf_power_dbm(telem.tx_telem.inst_RF_reflected), -6);
    assert_int_equal(get_rf_power_mw(telem.tx_telem.inst_RF_forward), 0);
    assert_int_equal(get_voltage(telem.tx_telem.supply_voltage), 7);
    assert_int_equal(get_current(telem.tx_telem.supply_current), 48);
    assert_int_equal(get_temperature(telem.tx_telem.temp_power_amp), 24);
    assert_int_equal(get_temperature(telem.tx_telem.temp_oscillator), 25);
}

static void test_telem_tx_state(void ** arg)
{
    KRadioStatus ret;
    radio_telem  telem;

    expect_value(__wrap_write, cmd, GET_TX_STATE);
    will_return(__wrap_read, sizeof(tx_state));
    will_return(__wrap_read, &tx_state);
    ret = k_radio_get_telemetry(&telem, RADIO_TX_STATE);

    assert_int_equal(ret, RADIO_OK);
    assert_int_equal(tx_state, telem.tx_state);
}

static void test_telem_tx_uptime(void ** arg)
{
    KRadioStatus ret;
    radio_telem  telem;

    expect_value(__wrap_write, cmd, GET_UPTIME);
    will_return(__wrap_read, sizeof(uptime));
    will_return(__wrap_read, &uptime);
    ret = k_radio_get_telemetry(&telem, RADIO_TX_UPTIME);

    assert_int_equal(ret, RADIO_OK);
    assert_int_equal(uptime, telem.uptime);
}

static void test_telem_rx_all(void ** arg)
{
    KRadioStatus ret;
    radio_telem  telem;

    expect_value(__wrap_write, cmd, GET_RX_ALL_TELEMETRY);
    will_return(__wrap_read, sizeof(trxvu_rx_telem_raw));
    will_return(__wrap_read, &rx_telem);
    ret = k_radio_get_telemetry(&telem, RADIO_RX_TELEM_ALL);

    assert_int_equal(ret, RADIO_OK);
    assert_int_equal(get_doppler_offset(telem.rx_telem.inst_doppler_offset),
                     5325);
    assert_int_equal(get_signal_strength(telem.rx_telem.inst_signal_strength),
                     -117);
    assert_int_equal(get_voltage(telem.rx_telem.supply_voltage), 7);
    assert_int_equal(get_current(telem.rx_telem.supply_current), 47);
    assert_int_equal(get_temperature(telem.rx_telem.temp_power_amp), 23);
    assert_int_equal(get_temperature(telem.rx_telem.temp_oscillator), 23);
}

static void test_telem_rx_uptime(void ** arg)
{
    KRadioStatus ret;

    radio_telem telem;

    expect_value(__wrap_write, cmd, GET_UPTIME);
    will_return(__wrap_read, sizeof(uptime));
    will_return(__wrap_read, &uptime);
    ret = k_radio_get_telemetry(&telem, RADIO_RX_UPTIME);

    assert_int_equal(ret, RADIO_OK);
    assert_int_equal(uptime, telem.uptime);
}

static int init(void ** state)
{
    trx_prop tx = {
            .addr = 0x60,
            .max_size = TX_SIZE,
            .max_frames = 40,
    };
    trx_prop rx = {
                .addr = 0x61,
                .max_size = RX_SIZE,
                .max_frames = 40,
    };

    will_return(__wrap_open, 1);
    k_radio_init("/dev/i2c-0", tx, rx, 10);

    return 0;
}

static int term(void ** state)
{
    will_return(__wrap_close, 0);
    k_radio_terminate();

    return 0;
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_no_init_recv),
        cmocka_unit_test_setup_teardown(test_send, init, term),
        cmocka_unit_test_setup_teardown(test_send_null, init, term),
        cmocka_unit_test_setup_teardown(test_send_resp_null, init, term),
        cmocka_unit_test_setup_teardown(test_send_override, init, term),
        cmocka_unit_test_setup_teardown(test_recv, init, term),
        cmocka_unit_test_setup_teardown(test_recv_null, init, term),
        cmocka_unit_test_setup_teardown(test_recv_len, init, term),
        cmocka_unit_test_setup_teardown(test_config_null, init, term),
        cmocka_unit_test_setup_teardown(test_set_beacon, init, term),
        cmocka_unit_test_setup_teardown(test_set_beacon_override, init, term),
        cmocka_unit_test_setup_teardown(test_clear_beacon, init, term),
        cmocka_unit_test_setup_teardown(test_set_to, init, term),
        cmocka_unit_test_setup_teardown(test_set_from, init, term),
        cmocka_unit_test_setup_teardown(test_set_idle, init, term),
        cmocka_unit_test_setup_teardown(test_set_rate, init, term),
        cmocka_unit_test_setup_teardown(test_hard_reset, init, term),
        cmocka_unit_test_setup_teardown(test_soft_reset, init, term),
        cmocka_unit_test_setup_teardown(test_watchdog, init, term),
        cmocka_unit_test_setup_teardown(test_watchdog_no_start_stop, init, term),
        cmocka_unit_test_setup_teardown(test_telem_null, init, term),
        cmocka_unit_test_setup_teardown(test_telem_tx_all, init, term),
        cmocka_unit_test_setup_teardown(test_telem_tx_last, init, term),
        cmocka_unit_test_setup_teardown(test_telem_tx_state, init, term),
        cmocka_unit_test_setup_teardown(test_telem_tx_uptime, init, term),
        cmocka_unit_test_setup_teardown(test_telem_rx_all, init, term),
        cmocka_unit_test_setup_teardown(test_telem_rx_uptime, init, term),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}
