/*
 * Kubos iMTQ API
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
 *
 * Unit tests for the generic ADCS API functions
 */

#include <isis-imtq-api/imtq.h>
#include <cmocka.h>

static KI2CNum bus = K_I2C1;
static uint16_t addr = 0x40;
static int timeout = 60;

#define NUM_CONFIG_PARAMS 83

imtq_resp_header response = { 0 };

imtq_state state = {
        .hdr = {0},
        .mode = SELFTEST,
        .error = 0,
        .config = 1,
        .uptime = 35
    };

/* Self-test structs */
imtq_test_result_all    test_results_all    = { 0 };
imtq_test_result_single test_results_single = { 0 };

/* Nominal telemetry structs */
imtq_housekeeping_raw house_raw = { 0 };
imtq_housekeeping_eng house_eng = { 0 };
imtq_detumble         detumble  = { 0 };
imtq_mtm_data         mtm       = { 0 };
imtq_dipole           dipole    = { 0 };

/* Debug telemetry structs */
imtq_config_resp config_resp = { 0 };

static void test_init(void ** arg)
{
    KADCSStatus ret;

    will_return(__wrap_open, 1);
    expect_value(__wrap_write, cmd, NOOP);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);
    ret = k_adcs_init(bus, addr, timeout);

    will_return(__wrap_close, 0);
    k_adcs_terminate();

    assert_int_equal(ret, ADCS_OK);
}

static void test_no_init_noop(void ** arg)
{
    assert_int_equal(k_adcs_noop(), ADCS_ERROR_MUTEX);
}

static void test_noop(void ** arg)
{
    KADCSStatus ret;

    expect_value(__wrap_write, cmd, NOOP);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);
    ret = k_adcs_noop();

    assert_int_equal(ret, ADCS_OK);
}

static void test_configure(void ** arg)
{
    KADCSStatus ret;

    JsonNode * config = json_decode("{\"0x2003\": 1,   \"0x2004\": 2}");

    expect_value(__wrap_write, cmd, SET_PARAM);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);
    expect_value(__wrap_write, cmd, SET_PARAM);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);

    ret = k_adcs_configure(config);

    json_delete(config);

    assert_int_equal(ret, ADCS_OK);
}

static void test_reset(void ** arg)
{
    KADCSStatus ret;

    expect_value(__wrap_write, cmd, RESET_MTQ >> 8);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);
    ret = k_adcs_reset(SOFT_RESET);

    assert_int_equal(ret, ADCS_OK);
}

static void test_set_mode_detumble(void ** arg)
{
    KADCSStatus     ret;
    adcs_mode_param param = 10;

    expect_value(__wrap_write, cmd, START_BDOT);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);
    ret = k_adcs_set_mode(DETUMBLE, &param);

    assert_int_equal(ret, ADCS_OK);
}

static void test_set_mode_detumble_null(void ** arg)
{
    KADCSStatus ret;

    ret = k_adcs_set_mode(DETUMBLE, NULL);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_set_mode_idle(void ** arg)
{
    KADCSStatus ret;

    expect_value(__wrap_write, cmd, CANCEL_OP);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);
    ret = k_adcs_set_mode(IDLE, NULL);

    assert_int_equal(ret, ADCS_OK);
}

static void test_run_test_all(void ** arg)
{
    KADCSStatus  ret;
    ADCSTestType axis = TEST_ALL;

    adcs_test_results results = json_mkobject();

    expect_value(__wrap_write, cmd, START_TEST);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);
    expect_value(__wrap_write, cmd, GET_TEST);
    expect_value(__wrap_read, len, sizeof(test_results_all));
    will_return(__wrap_read, &test_results_all);
    ret = k_adcs_run_test(axis, results);

    int json_ret = json_check(results, NULL);
    json_delete(results);

    assert_int_equal(ret, ADCS_OK);
    assert_true(json_ret);
}

static void test_run_test_single(void ** arg)
{
    KADCSStatus  ret;
    ADCSTestType axis = TEST_X_POS;

    adcs_test_results results = json_mkobject();

    expect_value(__wrap_write, cmd, START_TEST);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);

    expect_value(__wrap_write, cmd, GET_TEST);
    expect_value(__wrap_read, len, sizeof(test_results_single));
    will_return(__wrap_read, &test_results_single);

    ret = k_adcs_run_test(axis, results);

    int json_ret = json_check(results, NULL);
    json_delete(results);

    assert_int_equal(ret, ADCS_OK);
    assert_true(json_ret);
}

static void test_get_power_status(void ** arg)
{
    KADCSStatus       ret;
    adcs_power_status uptime = { 0 };

    expect_value(__wrap_write, cmd, GET_STATE);
    expect_value(__wrap_read, len, sizeof(imtq_state));
    will_return(__wrap_read, &state);
    ret = k_adcs_get_power_status(&uptime);

    assert_int_equal(ret, ADCS_OK);
    assert_int_equal(uptime, 35);
}

static void test_get_mode(void ** arg)
{
    KADCSStatus ret;
    ADCSMode    mode = 0;

    expect_value(__wrap_write, cmd, GET_STATE);
    expect_value(__wrap_read, len, sizeof(imtq_state));
    will_return(__wrap_read, &state);
    ret = k_adcs_get_mode(&mode);

    assert_int_equal(ret, ADCS_OK);
    assert_int_equal(mode, SELFTEST);
}

static void test_get_orientation(void ** arg)
{
    KADCSStatus ret;

    ret = k_adcs_get_orientation(NULL);

    assert_int_equal(ret, ADCS_ERROR_NOT_IMPLEMENTED);
}

static void test_get_spin(void ** arg)
{
    KADCSStatus ret;

    ret = k_adcs_get_spin(NULL);

    assert_int_equal(ret, ADCS_ERROR_NOT_IMPLEMENTED);
}

static void test_get_telemetry_nominal(void ** arg)
{
    KADCSStatus ret;

    JsonNode * results = json_mkobject();

    /* System State */
    expect_value(__wrap_write, cmd, GET_STATE);
    expect_value(__wrap_read, len, sizeof(imtq_state));
    will_return(__wrap_read, &state);

    /* Nominal Telemetry: */
    /* Raw Housekeeping */
    expect_value(__wrap_write, cmd, GET_HOUSE_RAW);
    expect_value(__wrap_read, len, sizeof(house_raw));
    will_return(__wrap_read, &house_raw);
    /* Engineering Housekeeping */
    expect_value(__wrap_write, cmd, GET_HOUSE_ENG);
    expect_value(__wrap_read, len, sizeof(house_eng));
    will_return(__wrap_read, &house_eng);
    /* Last Detumble Data */
    expect_value(__wrap_write, cmd, GET_DETUMBLE);
    expect_value(__wrap_read, len, sizeof(detumble));
    will_return(__wrap_read, &detumble);
    /* (Prep for measurement requests) */
    expect_value(__wrap_write, cmd, START_MEASURE);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);
    /* Current Raw MTM Measurement */
    expect_value(__wrap_write, cmd, GET_MTM_RAW);
    expect_value(__wrap_read, len, sizeof(mtm));
    will_return(__wrap_read, &mtm);
    /* Current Calibrated MTM Measurement */
    expect_value(__wrap_write, cmd, GET_MTM_CALIB);
    expect_value(__wrap_read, len, sizeof(mtm));
    will_return(__wrap_read, &mtm);
    /* Last Dipole Data */
    expect_value(__wrap_write, cmd, GET_DIPOLE);
    expect_value(__wrap_read, len, sizeof(dipole));
    will_return(__wrap_read, &dipole);

    ret = k_adcs_get_telemetry(NOMINAL, results);

    int json_ret = json_check(results, NULL);
    json_delete(results);

    assert_int_equal(ret, ADCS_OK);
    assert_true(json_ret);
}

static void test_get_telemetry_debug(void ** arg)
{
    KADCSStatus ret;

    JsonNode * results = json_mkobject();

    /* System State */
    expect_value(__wrap_write, cmd, GET_STATE);
    expect_value(__wrap_read, len, sizeof(imtq_state));
    will_return(__wrap_read, &state);

    /* Debug Telemetry: */
    /* Current Configuration */
    expect_value_count(__wrap_write, cmd, GET_PARAM, NUM_CONFIG_PARAMS);
    expect_value_count(__wrap_read, len, sizeof(config_resp),
                       NUM_CONFIG_PARAMS);
    will_return_count(__wrap_read, &config_resp, NUM_CONFIG_PARAMS);

    /* Last Test Results */
    expect_value(__wrap_write, cmd, GET_TEST);
    expect_value(__wrap_read, len, sizeof(test_results_all));
    will_return(__wrap_read, &test_results_all);

    ret = k_adcs_get_telemetry(DEBUG, results);

    int json_ret = json_check(results, NULL);
    json_delete(results);

    assert_int_equal(ret, ADCS_OK);
    assert_true(json_ret);
}

static void test_passthrough(void ** arg)
{
    KADCSStatus ret;

    uint8_t          packet[] = { 0x11, 0x22, 0x33, 0x44 };
    imtq_resp_header resp     = { 0 };

    expect_value(__wrap_write, cmd, packet[0]);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);
    ret = k_adcs_passthrough(packet, sizeof(packet), (uint8_t *) &resp,
                             sizeof(resp), NULL);

    assert_int_equal(ret, ADCS_OK);
}

static int init(void ** state)
{
    will_return(__wrap_open, 1);
    expect_value(__wrap_write, cmd, NOOP);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);
    k_adcs_init(bus, addr, timeout);

    return 0;
}

static int term(void ** state)
{
    will_return(__wrap_close, 0);
    k_adcs_terminate();

    return 0;
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_init),
        cmocka_unit_test(test_no_init_noop),
        cmocka_unit_test_setup_teardown(test_noop, init, term),
        cmocka_unit_test_setup_teardown(test_configure, init, term),
        cmocka_unit_test_setup_teardown(test_reset, init, term),
        cmocka_unit_test_setup_teardown(test_set_mode_detumble, init, term),
        cmocka_unit_test_setup_teardown(test_set_mode_detumble_null, init, term),
        cmocka_unit_test_setup_teardown(test_set_mode_idle, init, term),
        cmocka_unit_test_setup_teardown(test_run_test_all, init, term),
        cmocka_unit_test_setup_teardown(test_run_test_single, init, term),
        cmocka_unit_test_setup_teardown(test_get_power_status, init, term),
        cmocka_unit_test_setup_teardown(test_get_mode, init, term),
        cmocka_unit_test_setup_teardown(test_get_orientation, init, term),
        cmocka_unit_test_setup_teardown(test_get_spin, init, term),
        cmocka_unit_test_setup_teardown(test_get_telemetry_nominal, init, term),
        cmocka_unit_test_setup_teardown(test_get_telemetry_debug, init, term),
        cmocka_unit_test_setup_teardown(test_passthrough, init, term),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}
