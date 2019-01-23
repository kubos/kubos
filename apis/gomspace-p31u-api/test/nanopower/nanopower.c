/*
 * Kubos API for GOMspace NanoPower P31u
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

#include <gomspace-p31u-api/gomspace-p31u-api.h>
#include <cmocka.h>

/* Test Data */
eps_resp_header response = { 0 };

eps_resp_header error_resp = {
        .cmd = 0,
        .status = -1
};

eps_system_config_t sys_config_le = {
        .ppt_mode = 1,
        .battheater_mode = 0,
        .battheater_low = -110,
        .battheater_high = 1,
        .output_normal_value = {1, 0, 1, 0, 1, 0, 1, 0},
        .output_safe_value = {0, 1, 0, 1, 0, 1, 0, 1},
        .output_initial_on_delay = {1,2,3,4,5,6,7,8},
        .output_initial_off_delay = {21,22,23,24,25,26,27,28},
        .vboost = {3600, 3600, 3600}
};

eps_system_config_t sys_config_be = {
        .ppt_mode = 1,
        .battheater_mode = 0,
        .battheater_low = -110,
        .battheater_high = 1,
        .output_normal_value = {1, 0, 1, 0, 1, 0, 1, 0},
        .output_safe_value = {0, 1, 0, 1, 0, 1, 0, 1},
        .output_initial_on_delay = {256,512,768,1024,1280,1536,1792,2048},
        .output_initial_off_delay = {5376,5632,5888,6144,6400,6656,6912,7168},
        .vboost = {4110, 4110, 4110}
};

eps_battery_config_t batt_config_le = {
        .batt_maxvoltage = 8200,
        .batt_safevoltage = 7100,
        .batt_criticalvoltage = 6400,
        .batt_normalvoltage = 7300,
};

eps_battery_config_t batt_config_be = {
        .batt_maxvoltage = 2080,
        .batt_safevoltage = 48155,
        .batt_criticalvoltage = 25,
        .batt_normalvoltage = 33820,
};

eps_hk_t hk_le = {
        .vboost = {387, 378, 386},
        .vbatt = 7200,
        .curin = {77, 24, 23},
        .cursun = 30,
        .cursys = 47,
        .curout = {1,2,3,4,5,6},
        .output = {0, 1, 0, 1, 0, 1, 0, 1},
        .output_on_delta = {1,2,3,4,5,6,7,8},
        .output_off_delta = {21,22,23,24,25,26,27,28},
        .latchup = {1,2,3,4,5,6},
        .wdt_i2c_time_left = 9600,
        .wdt_gnd_time_left = 4321,
        .wdt_csp_pings_left = {4, 5},
        .counter_wdt_i2c = 3210,
        .counter_wdt_gnd = 123456789,
        .counter_wdt_csp = {6543, 76543210},
        .counter_boot = 9,
        .temp = {23, 24, 25, 26, 27, 28},
        .boot_cause = 2,
        .batt_mode = 3,
        .ppt_mode = 1,
};

eps_hk_t hk_be = {
        .vboost = {33537, 31233, 33281},
        .vbatt = 8220,
        .curin = {19712, 6144, 5888},
        .cursun = 7680,
        .cursys = 12032,
        .curout = {256,512,768,1024,1280,1536},
        .output = {0, 1, 0, 1, 0, 1, 0, 1},
        .output_on_delta = {256,512,768,1024,1280,1536,1792,2048},
        .output_off_delta = {5376,5632,5888,6144,6400,6656,6912,7168},
        .latchup = {256,512,768,1024,1280,1536},
        .wdt_i2c_time_left = 2149908480,
        .wdt_gnd_time_left = 3775922176,
        .wdt_csp_pings_left = {4, 5},
        .counter_wdt_i2c = 2316042240,
        .counter_wdt_gnd = 365779719,
        .counter_wdt_csp = {2400780288, 3941895940},
        .counter_boot = 150994944,
        .temp = {5888,6144,6400,6656,6912,7168},
        .boot_cause = 2,
        .batt_mode = 3,
        .ppt_mode = 1,
};

typedef struct __attribute__((packed))
{
    eps_resp_header hdr;
    uint8_t bp4;
    uint8_t onboard;
} heater_struct;

heater_struct heater_data = {
    .bp4 = 1,
    .onboard = 1
};

typedef struct __attribute__((packed))
{
    uint8_t cmd;
    uint16_t in1_voltage;
    uint16_t in2_voltage;
    uint16_t in3_voltage;
}  input_values_packet;

input_values_packet input_vals_be = {
    .in1_voltage = 47115, /* 3000 */
    .in2_voltage = 47371, /* 3001 */
    .in3_voltage = 47627  /* 3002 */
};

typedef struct __attribute__((packed))
{
    uint8_t cmd;
    uint8_t channel;
    uint8_t value;
    int16_t delay;
}  single_output_packet;

single_output_packet single_output_be = {
    .cmd = SET_SINGLE_OUTPUT,
    .channel = 1, /* Converted value (orig. 6) */
    .value = 1,
    .delay = 1280 /* 5 */
};
/* End of Test Data */

static void test_init_no_bus(void ** arg)
{
    KEPSConf config = {
            .addr = 0x02
    };

    assert_int_equal(k_eps_init(config), EPS_ERROR_CONFIG);
}


static void test_init_no_addr(void ** arg)
{
    KEPSConf config = {
            .bus = "/dev/i2c-0"
    };

    assert_int_equal(k_eps_init(config), EPS_ERROR_CONFIG);
}

static void test_no_init_reset(void ** arg)
{
    assert_int_equal(k_eps_reset(), EPS_ERROR);
}

static void test_ping(void ** arg)
{
    KEPSStatus ret;
    uint8_t    resp = PING;

    expect_value(__wrap_write, cmd, PING);
    expect_value(__wrap_read, len, 1);
    will_return(__wrap_read, &resp);
    ret = k_eps_ping();

    assert_int_equal(ret, EPS_OK);
}

static void test_reset(void ** arg)
{
    KEPSStatus ret;

    expect_value(__wrap_write, cmd, HARD_RESET);
    ret = k_eps_reset();

    assert_int_equal(ret, EPS_OK);
}

static void test_reboot(void ** arg)
{
    KEPSStatus ret;

    uint8_t    test_packet[] = { REBOOT, 0x80, 0x07, 0x80, 0x07 };

    expect_value(__wrap_write, cmd, REBOOT);
    expect_memory(__wrap_write, buf, test_packet, sizeof(test_packet));
    ret = k_eps_reboot();

    assert_int_equal(ret, EPS_OK);
}

static void test_configure_system_null(void ** arg)
{
    assert_int_equal(k_eps_configure_system(NULL), EPS_ERROR_CONFIG);
}

static void test_configure_system(void ** arg)
{
    KEPSStatus ret;

    uint8_t test_packet[sizeof(eps_system_config_t) + 1] = { 0 };

    test_packet[0] = SET_CONFIG1;
    memcpy(test_packet + 1, &sys_config_be, sizeof(eps_system_config_t));

    expect_value(__wrap_write, cmd, SET_CONFIG1);
    expect_memory(__wrap_write, buf, test_packet, sizeof(test_packet));
    expect_value(__wrap_read, len, sizeof(eps_resp_header));
    will_return(__wrap_read, &response);

    ret = k_eps_configure_system(&sys_config_le);

    assert_int_equal(ret, EPS_OK);
}

static void test_configure_battery_null(void ** arg)
{
    assert_int_equal(k_eps_configure_battery(NULL), EPS_ERROR_CONFIG);
}

static void test_configure_battery(void ** arg)
{
    KEPSStatus ret;

    uint8_t test_packet[sizeof(eps_battery_config_t) + 1] = { 0 };

    test_packet[0] = SET_CONFIG2;
    memcpy(test_packet + 1, &batt_config_be, sizeof(eps_battery_config_t));

    expect_value(__wrap_write, cmd, SET_CONFIG2);
    expect_memory(__wrap_write, buf, test_packet, sizeof(test_packet));
    expect_value(__wrap_read, len, sizeof(eps_resp_header));
    will_return(__wrap_read, &response);

    ret = k_eps_configure_battery(&batt_config_le);

    assert_int_equal(ret, EPS_OK);
}

static void test_save_battery_config(void ** arg)
{
    KEPSStatus ret;

    expect_value(__wrap_write, cmd, CMD_CONFIG2);
    expect_value(__wrap_read, len, sizeof(eps_resp_header));
    will_return(__wrap_read, &response);

    ret = k_eps_save_battery_config();

    assert_int_equal(ret, EPS_OK);
}

static void test_reset_system_config(void ** arg)
{
    KEPSStatus ret;

    expect_value(__wrap_write, cmd, CMD_CONFIG1);
    expect_value(__wrap_read, len, sizeof(eps_resp_header));
    will_return(__wrap_read, &response);

    ret = k_eps_reset_system_config();

    assert_int_equal(ret, EPS_OK);
}

static void test_reset_battery_config(void ** arg)
{
    KEPSStatus ret;

    expect_value(__wrap_write, cmd, CMD_CONFIG2);
    expect_value(__wrap_read, len, sizeof(eps_resp_header));
    will_return(__wrap_read, &response);

    ret = k_eps_reset_battery_config();

    assert_int_equal(ret, EPS_OK);
}

static void test_set_output(void ** arg)
{
    KEPSStatus ret;

    expect_value(__wrap_write, cmd, SET_OUTPUT);
    expect_value(__wrap_read, len, sizeof(eps_resp_header));
    will_return(__wrap_read, &response);

    ret = k_eps_set_output(0x64);

    assert_int_equal(ret, EPS_OK);
}

static void test_set_single_output_bad_channel(void ** arg)
{
    assert_int_equal(k_eps_set_single_output(100, 1, 5), EPS_ERROR_CONFIG);
}

static void test_set_single_output_bad_value(void ** arg)
{
    assert_int_equal(k_eps_set_single_output(6, 2, 5), EPS_ERROR_CONFIG);
}

static void test_set_single_output(void ** arg)
{
    KEPSStatus ret;

    uint8_t test_packet[sizeof(single_output_be)] = { 0 };

    memcpy(test_packet, &single_output_be, sizeof(single_output_be));

    expect_value(__wrap_write, cmd, SET_SINGLE_OUTPUT);
    expect_memory(__wrap_write, buf, test_packet, sizeof(test_packet));
    expect_value(__wrap_read, len, sizeof(eps_resp_header));
    will_return(__wrap_read, &response);

    ret = k_eps_set_single_output(6, 1, 5);

    assert_int_equal(ret, EPS_OK);
}

static void test_set_input_value(void ** arg)
{
    KEPSStatus ret;

    uint8_t test_packet[sizeof(input_vals_be)] = { 0 };

    memcpy(test_packet, &input_vals_be, sizeof(input_vals_be));
    test_packet[0] = SET_PV_VOLT;

    expect_value(__wrap_write, cmd, SET_PV_VOLT);
    expect_memory(__wrap_write, buf, test_packet, sizeof(test_packet));
    expect_value(__wrap_read, len, sizeof(eps_resp_header));
    will_return(__wrap_read, &response);

    ret = k_eps_set_input_value(3000, 3001, 3002);

    assert_int_equal(ret, EPS_OK);
}

static void test_set_input_mode_bad(void ** arg)
{
    assert_int_equal(k_eps_set_input_mode(3), EPS_ERROR_CONFIG);
}

static void test_set_input_mode(void ** arg)
{
    KEPSStatus ret;

    expect_value(__wrap_write, cmd, SET_PV_AUTO);
    expect_value(__wrap_read, len, sizeof(eps_resp_header));
    will_return(__wrap_read, &response);

    ret = k_eps_set_input_mode(2);

    assert_int_equal(ret, EPS_OK);
}

static void test_set_heater_bad_cmd(void ** arg)
{
    assert_int_equal(k_eps_set_heater(1, 1, 1), EPS_ERROR_CONFIG);
}

static void test_set_heater_bad_heater(void ** arg)
{
    assert_int_equal(k_eps_set_heater(1, 2, 1), EPS_ERROR_CONFIG);
}

static void test_set_heater_bad_mode(void ** arg)
{
    assert_int_equal(k_eps_set_heater(1, 1, 2), EPS_ERROR_CONFIG);
}

static void test_set_heater(void ** arg)
{
    KEPSStatus ret;

    expect_value(__wrap_write, cmd, SET_HEATER);
    expect_value(__wrap_read, len, sizeof(eps_resp_header));
    will_return(__wrap_read, &response);

    ret = k_eps_set_heater(0, 1, 1);

    assert_int_equal(ret, EPS_OK);
}

static void test_reset_counters(void ** arg)
{
    KEPSStatus ret;

    expect_value(__wrap_write, cmd, RESET_COUNTERS);
    expect_value(__wrap_read, len, sizeof(eps_resp_header));
    will_return(__wrap_read, &response);

    ret = k_eps_reset_counters();

    assert_int_equal(ret, EPS_OK);
}

static void test_get_housekeeping_null(void ** arg)
{
    assert_int_equal(k_eps_get_housekeeping(NULL), EPS_ERROR_CONFIG);
}

static void test_get_housekeeping(void ** arg)
{
    KEPSStatus ret;

    eps_hk_t hk = { 0 };
    uint8_t test_response[sizeof(eps_hk_t) + sizeof(eps_resp_header)] = { 0 };

    memcpy(test_response + sizeof(eps_resp_header), &hk_be, sizeof(eps_hk_t));

    expect_value(__wrap_write, cmd, GET_HOUSEKEEPING);
    expect_value(__wrap_read, len, sizeof(test_response));
    will_return(__wrap_read, test_response);

    ret = k_eps_get_housekeeping(&hk);

    assert_int_equal(ret, EPS_OK);
    assert_memory_equal(&hk, &hk_le, sizeof(eps_hk_t));
}

static void test_get_system_config_null(void ** arg)
{
    assert_int_equal(k_eps_get_system_config(NULL), EPS_ERROR_CONFIG);
}

static void test_get_system_config(void ** arg)
{
    KEPSStatus ret;

    eps_system_config_t config = { 0 };
    uint8_t test_response[sizeof(eps_system_config_t) + sizeof(eps_resp_header)] = { 0 };

    memcpy(test_response + sizeof(eps_resp_header), &sys_config_be, sizeof(eps_system_config_t));

    expect_value(__wrap_write, cmd, GET_CONFIG1);
    expect_value(__wrap_read, len, sizeof(test_response));
    will_return(__wrap_read, test_response);

    ret = k_eps_get_system_config(&config);

    assert_int_equal(ret, EPS_OK);
    assert_memory_equal(&config, &sys_config_le, sizeof(eps_system_config_t));
}

static void test_get_battery_config_null(void ** arg)
{
    assert_int_equal(k_eps_get_battery_config(NULL), EPS_ERROR_CONFIG);
}

static void test_get_battery_config(void ** arg)
{
    KEPSStatus ret;

    eps_battery_config_t config = { 0 };
    uint8_t test_response[sizeof(eps_battery_config_t) + sizeof(eps_resp_header)] = { 0 };

    memcpy(test_response + sizeof(eps_resp_header), &batt_config_be, sizeof(eps_battery_config_t));

    expect_value(__wrap_write, cmd, GET_CONFIG2);
    expect_value(__wrap_read, len, sizeof(test_response));
    will_return(__wrap_read, test_response);

    ret = k_eps_get_battery_config(&config);

    assert_int_equal(ret, EPS_OK);
    assert_memory_equal(&config, &batt_config_le, sizeof(eps_battery_config_t));
}

static void test_get_heater_null_null(void ** arg)
{
    assert_int_equal(k_eps_get_heater(NULL, NULL), EPS_ERROR_CONFIG);
}

static void test_get_heater_null_bp4_good_onboard(void ** arg)
{
    KEPSStatus ret;

    uint8_t onboard = 0;

    expect_value(__wrap_write, cmd, SET_HEATER);
    expect_value(__wrap_read, len, sizeof(heater_data));
    will_return(__wrap_read, &heater_data);

    ret = k_eps_get_heater(NULL, &onboard);

    assert_int_equal(ret, EPS_OK);
    assert_int_not_equal(onboard, 0);
}

static void test_get_heater_good_bp4_null_onboard(void ** arg)
{
    KEPSStatus ret;

    uint8_t bp4 = 0;

    expect_value(__wrap_write, cmd, SET_HEATER);
    expect_value(__wrap_read, len, sizeof(heater_data));
    will_return(__wrap_read, &heater_data);

    ret = k_eps_get_heater(&bp4, NULL);

    assert_int_equal(ret, EPS_OK);
    assert_int_not_equal(bp4, 0);
}

static void test_get_heater(void ** arg)
{
    KEPSStatus ret;

    uint8_t bp4 = 0;
    uint8_t onboard = 0;

    expect_value(__wrap_write, cmd, SET_HEATER);
    expect_value(__wrap_read, len, sizeof(heater_data));
    will_return(__wrap_read, &heater_data);

    ret = k_eps_get_heater(&bp4, &onboard);

    assert_int_equal(ret, EPS_OK);
    assert_int_not_equal(bp4, 0);
    assert_int_not_equal(onboard, 0);
}

static void test_watchdog_kick(void ** arg)
{
    KEPSStatus ret;

    expect_value(__wrap_write, cmd, RESET_WDT);
    expect_value(__wrap_read, len, sizeof(eps_resp_header));
    will_return(__wrap_read, &response);
    ret = k_eps_watchdog_kick();

    assert_int_equal(ret, EPS_OK);
}

static void test_watchdog_thread(void ** arg)
{
    KEPSStatus start_ret;
    KEPSStatus stop_ret;

    expect_value(__wrap_write, cmd, RESET_WDT);
    expect_value(__wrap_read, len, sizeof(eps_resp_header));
    will_return(__wrap_read, &response);

    start_ret = k_eps_watchdog_start(1);

    const struct timespec delay = {.tv_sec = 0, .tv_nsec = 2000001 };

    nanosleep(&delay, NULL);

    stop_ret = k_eps_watchdog_stop();

    assert_int_equal(start_ret, EPS_OK);
    assert_int_equal(stop_ret, EPS_OK);
}

static void test_watchdog_thread_twice(void ** arg)
{
    KEPSStatus start_ret;
    KEPSStatus stop_ret;

    expect_value(__wrap_write, cmd, RESET_WDT);
    expect_value(__wrap_read, len, sizeof(eps_resp_header));
    will_return(__wrap_read, &response);

    start_ret = k_eps_watchdog_start(1);

    const struct timespec delay = {.tv_sec = 0, .tv_nsec = 2000001 };

    nanosleep(&delay, NULL);

    stop_ret = k_eps_watchdog_stop();

    assert_int_equal(start_ret, EPS_OK);
    assert_int_equal(stop_ret, EPS_OK);

    expect_value(__wrap_write, cmd, RESET_WDT);
    expect_value(__wrap_read, len, sizeof(eps_resp_header));
    will_return(__wrap_read, &response);

    start_ret = k_eps_watchdog_start(1);

    nanosleep(&delay, NULL);

    stop_ret = k_eps_watchdog_stop();

    assert_int_equal(start_ret, EPS_OK);
    assert_int_equal(stop_ret, EPS_OK);
}

static void test_watchdog_stop_no_start(void ** arg)
{
    KEPSStatus ret;

    ret = k_eps_watchdog_stop();

    assert_int_equal(ret, EPS_ERROR);
}

static void test_passthrough_null_tx(void ** arg)
{
    KEPSStatus ret;
    uint8_t     tx[1] = { 0 };
    uint8_t     rx[1] = { 0 };

    ret = k_eps_passthrough(NULL, sizeof(tx), rx, sizeof(rx));
    assert_int_equal(ret, EPS_ERROR_CONFIG);
}

static void test_passthrough_zero_tx_len(void ** arg)
{
    KEPSStatus ret;
    uint8_t     tx[1] = { 0 };
    uint8_t     rx[1] = { 0 };

    ret = k_eps_passthrough(tx, 0, rx, sizeof(rx));
    assert_int_equal(ret, EPS_ERROR_CONFIG);
}

static void test_passthrough_null_rx_nonzero_rx_len(void ** arg)
{
    KEPSStatus ret;
    uint8_t     tx[1] = { 0 };
    uint8_t     rx[1] = { 0 };

    ret = k_eps_passthrough(tx, sizeof(tx), NULL, sizeof(rx));
    assert_int_equal(ret, EPS_ERROR_CONFIG);
}

static void test_passthrough_nonnull_rx_zero_rx_len(void ** arg)
{
    KEPSStatus ret;
    uint8_t     tx[1] = { 0 };
    uint8_t     rx[1] = { 0 };

    ret = k_eps_passthrough(tx, sizeof(tx), rx, 0);
    assert_int_equal(ret, EPS_ERROR_CONFIG);
}

static void test_passthrough_null_rx_zero_rx_len(void ** arg)
{
    KEPSStatus ret;
    uint8_t     tx[1] = { 0x77 };
    uint8_t     rx[1] = { 0 };

    /*
     * Valid test case. If rx==null and rx_len==0,
     * we'll genarate the required response header under the covers,
     * but we won't return anything back to the user
     */
    expect_value(__wrap_write, cmd, tx[0]);
    expect_value(__wrap_read, len, sizeof(eps_resp_header));
    will_return(__wrap_read, &response);

    ret = k_eps_passthrough(tx, sizeof(tx), NULL, 0);
    assert_int_equal(ret, EPS_OK);
}

static void test_passthrough(void ** arg)
{
    KEPSStatus ret;

    uint8_t          packet[] = { 0x11, 0x22, 0x33, 0x44 };
    eps_resp_header resp     = { 0 };

    expect_value(__wrap_write, cmd, packet[0]);
    expect_value(__wrap_read, len, sizeof(eps_resp_header));
    will_return(__wrap_read, &response);
    ret = k_eps_passthrough(packet, sizeof(packet), (uint8_t *) &resp,
                             sizeof(resp));

    assert_int_equal(ret, EPS_OK);
}


static void test_transfer_null_tx(void ** arg)
{
    KEPSStatus           ret;
    uint8_t               tx[1] = { 0 };
    uint8_t               rx[1] = { 0 };

    ret = kprv_eps_transfer(NULL, sizeof(tx), rx, sizeof(rx));

    assert_int_equal(ret, EPS_ERROR_CONFIG);
}

static void test_transfer_zero_tx_len(void ** arg)
{
    KEPSStatus           ret;
    uint8_t               tx[1] = { 0 };
    uint8_t               rx[1] = { 0 };

    ret = kprv_eps_transfer(tx, 0, rx, sizeof(rx));

    assert_int_equal(ret, EPS_ERROR_CONFIG);
}

static void test_transfer_null_rx(void ** arg)
{
    KEPSStatus           ret;
    uint8_t               tx[1] = { 0 };
    uint8_t               rx[1] = { 0 };

    ret = kprv_eps_transfer(tx, sizeof(tx), NULL, sizeof(rx));

    assert_int_equal(ret, EPS_ERROR_CONFIG);
}

static void test_transfer_zero_rx_len(void ** arg)
{
    KEPSStatus           ret;
    uint8_t               tx[1] = { 0 };
    uint8_t               rx[1] = { 0 };

    ret = kprv_eps_transfer(tx, sizeof(tx), rx, 0);

    assert_int_equal(ret, EPS_ERROR_CONFIG);
}

static void test_transfer_cmd_mismatch(void ** arg)
{
    KEPSStatus ret;
    /* Dummy command value */
    uint8_t          packet[] = { 0x55 };
    eps_resp_header resp     = { 0 };

    expect_value(__wrap_write, cmd, packet[0]);
    expect_value(__wrap_read, len, sizeof(response));
    will_return(__wrap_read, &response);

    ret = kprv_eps_transfer(packet, sizeof(packet), (uint8_t *) &resp,
                             sizeof(resp));

    assert_int_equal(ret, EPS_ERROR);
}

static void test_transfer_error(void ** arg)
{
    KEPSStatus      ret;
    uint8_t          packet[] = { 0x11, 0x22, 0x33, 0x44 };
    eps_resp_header resp     = { 0 };

    expect_value(__wrap_write, cmd, packet[0]);
    expect_value(__wrap_read, len, sizeof(error_resp));
    will_return(__wrap_read, &error_resp);

    ret = kprv_eps_transfer(packet, sizeof(packet), (uint8_t *) &resp,
                             sizeof(resp));

    assert_int_equal(ret, EPS_ERROR_INTERNAL);
}

static int init(void ** state)
{
    KEPSConf config = {
            .bus = "/dev/i2c-0",
            .addr = 0x02
    };

    will_return(__wrap_open, 1);
    k_eps_init(config);

    return 0;
}

static int term(void ** state)
{
    will_return(__wrap_close, 0);
    k_eps_terminate();

    return 0;
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_init_no_bus),
        cmocka_unit_test(test_init_no_addr),
        cmocka_unit_test(test_no_init_reset),
        cmocka_unit_test_setup_teardown(test_ping, init, term),
        cmocka_unit_test_setup_teardown(test_reset, init, term),
        cmocka_unit_test_setup_teardown(test_reboot, init, term),
        cmocka_unit_test_setup_teardown(test_configure_system_null, init, term),
        cmocka_unit_test_setup_teardown(test_configure_system, init, term),
        cmocka_unit_test_setup_teardown(test_configure_battery_null, init, term),
        cmocka_unit_test_setup_teardown(test_configure_battery, init, term),
        cmocka_unit_test_setup_teardown(test_save_battery_config, init, term),
        cmocka_unit_test_setup_teardown(test_reset_system_config, init, term),
        cmocka_unit_test_setup_teardown(test_reset_battery_config, init, term),
        cmocka_unit_test_setup_teardown(test_set_output, init, term),
        cmocka_unit_test_setup_teardown(test_set_single_output_bad_channel, init, term),
        cmocka_unit_test_setup_teardown(test_set_single_output_bad_value, init, term),
        cmocka_unit_test_setup_teardown(test_set_single_output, init, term),
        cmocka_unit_test_setup_teardown(test_set_single_output, init, term),
        cmocka_unit_test_setup_teardown(test_set_input_value, init, term),
        cmocka_unit_test_setup_teardown(test_set_input_mode, init, term),
        cmocka_unit_test_setup_teardown(test_set_input_mode_bad, init, term),
        cmocka_unit_test_setup_teardown(test_set_heater_bad_cmd, init, term),
        cmocka_unit_test_setup_teardown(test_set_heater_bad_heater, init, term),
        cmocka_unit_test_setup_teardown(test_set_heater_bad_mode, init, term),
        cmocka_unit_test_setup_teardown(test_set_heater, init, term),
        cmocka_unit_test_setup_teardown(test_reset_counters, init, term),
        cmocka_unit_test_setup_teardown(test_get_housekeeping_null, init, term),
        cmocka_unit_test_setup_teardown(test_get_housekeeping, init, term),
        cmocka_unit_test_setup_teardown(test_get_system_config_null, init, term),
        cmocka_unit_test_setup_teardown(test_get_system_config, init, term),
        cmocka_unit_test_setup_teardown(test_get_battery_config_null, init, term),
        cmocka_unit_test_setup_teardown(test_get_battery_config, init, term),
        cmocka_unit_test_setup_teardown(test_get_heater_null_null, init, term),
        cmocka_unit_test_setup_teardown(test_get_heater_null_bp4_good_onboard, init, term),
        cmocka_unit_test_setup_teardown(test_get_heater_good_bp4_null_onboard, init, term),
        cmocka_unit_test_setup_teardown(test_get_heater, init, term),
        cmocka_unit_test_setup_teardown(test_watchdog_kick, init, term),
        cmocka_unit_test_setup_teardown(test_watchdog_thread, init, term),
        cmocka_unit_test_setup_teardown(test_watchdog_thread_twice, init, term),
        cmocka_unit_test_setup_teardown(test_watchdog_stop_no_start, init, term),
        cmocka_unit_test_setup_teardown(test_passthrough_null_tx, init, term),
        cmocka_unit_test_setup_teardown(test_passthrough_zero_tx_len, init, term),
        cmocka_unit_test_setup_teardown(test_passthrough_null_rx_nonzero_rx_len, init, term),
        cmocka_unit_test_setup_teardown(test_passthrough_nonnull_rx_zero_rx_len, init, term),
        cmocka_unit_test_setup_teardown(test_passthrough_null_rx_zero_rx_len, init, term),
        cmocka_unit_test_setup_teardown(test_passthrough, init, term),
        cmocka_unit_test_setup_teardown(test_transfer_null_tx, init, term),
        cmocka_unit_test_setup_teardown(test_transfer_zero_tx_len, init, term),
        cmocka_unit_test_setup_teardown(test_transfer_null_rx, init, term),
        cmocka_unit_test_setup_teardown(test_transfer_zero_rx_len, init, term),
        cmocka_unit_test_setup_teardown(test_transfer_cmd_mismatch, init, term),
        cmocka_unit_test_setup_teardown(test_transfer_error, init, term),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}
