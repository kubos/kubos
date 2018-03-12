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
 */

#include <imtq-api/imtq.h>
#include <cmocka.h>

imtq_resp_header response = { 0 };

imtq_resp_header error_resp = {
        .cmd = 0,
        .status = IMTQ_ERROR_BAD_PARAM
};

imtq_config_resp config_resp = {
        .hdr = {0},
        .param = 0x2003,
        .value = {
                0,
                .uint8_val = 3
        }
};

imtq_state state = {
        .hdr = {0},
        .mode = SELFTEST,
        .error = 0,
        .config = 1,
        .uptime = 35
    };

imtq_test_result_all    test_results_all    = { 0 };
imtq_test_result_single test_results_single = { 0 };
imtq_housekeeping_raw   house_raw           = { 0 };
imtq_housekeeping_eng   house_eng           = { 0 };
imtq_detumble           detumble            = { 0 };
imtq_mtm_data           mtm                 = { 0 };
imtq_dipole             dipole              = { 0 };
imtq_coil_current       coil_current        = { 0 };
imtq_coil_temp          coil_temp           = { 0 };

/* Config Tests */

static void test_get_param_zero(void ** arg)
{
    KADCSStatus ret;

    ret = k_imtq_get_param(0, &config_resp);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_get_param_null(void ** arg)
{
    KADCSStatus ret;
    uint16_t    param = 0x2003;

    ret = k_imtq_get_param(param, NULL);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_get_param_resp(void ** arg)
{
    KADCSStatus ret;
    uint16_t    param = 0x2003;

    expect_value(__wrap_write, cmd, GET_PARAM);
    expect_value(__wrap_read, len, sizeof(config_resp));
    will_return(__wrap_read, &config_resp);
    ret = k_imtq_get_param(param, &config_resp);

    assert_int_equal(ret, ADCS_OK);
}

static void test_set_param_zero(void ** arg)
{
    KADCSStatus       ret;
    imtq_config_value config_value;
    config_value.uint8_val = 3;

    ret = k_imtq_set_param(0, &config_value, NULL);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_set_param_null_value(void ** arg)
{
    KADCSStatus ret;
    uint16_t    param = 0x2003;

    ret = k_imtq_set_param(param, NULL, NULL);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_set_param_null_resp(void ** arg)
{
    KADCSStatus       ret;
    uint16_t          param = 0x2003;
    imtq_config_value config_value;
    config_value.uint8_val = 3;

    expect_value(__wrap_write, cmd, SET_PARAM);
    expect_value(__wrap_read, len, sizeof(response));
    will_return(__wrap_read, &response);
    ret = k_imtq_set_param(param, &config_value, NULL);

    assert_int_equal(ret, ADCS_OK);
}

static void test_set_param_resp(void ** arg)
{
    KADCSStatus       ret;
    uint16_t          param = 0x2003;
    imtq_config_value config_value;
    config_value.uint8_val = 3;

    expect_value(__wrap_write, cmd, SET_PARAM);
    expect_value(__wrap_read, len, sizeof(config_resp));
    will_return(__wrap_read, &config_resp);
    ret = k_imtq_set_param(param, &config_value, &config_resp);

    assert_int_equal(ret, ADCS_OK);
}

static void test_reset_param_zero(void ** arg)
{
    KADCSStatus ret;

    ret = k_imtq_reset_param(0, &config_resp);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_reset_param_null(void ** arg)
{
    KADCSStatus ret;
    uint16_t    param = 0x2003;

    expect_value(__wrap_write, cmd, RESET_PARAM);
    expect_value(__wrap_read, len, sizeof(response));
    will_return(__wrap_read, &response);
    ret = k_imtq_reset_param(param, NULL);

    assert_int_equal(ret, ADCS_OK);
}

static void test_reset_param_resp(void ** arg)
{
    KADCSStatus ret;
    uint16_t    param = 0x2003;

    expect_value(__wrap_write, cmd, RESET_PARAM);
    expect_value(__wrap_read, len, sizeof(config_resp));
    will_return(__wrap_read, &config_resp);
    ret = k_imtq_reset_param(param, &config_resp);

    assert_int_equal(ret, ADCS_OK);
}

/* Ops Tests */

static void test_cancel(void ** arg)
{
    KADCSStatus ret;

    expect_value(__wrap_write, cmd, CANCEL_OP);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);
    ret = k_imtq_cancel_op();

    assert_int_equal(ret, ADCS_OK);
}

static void test_measure(void ** arg)
{
    KADCSStatus ret;

    expect_value(__wrap_write, cmd, START_MEASURE);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);
    ret = k_imtq_start_measurement();

    assert_int_equal(ret, ADCS_OK);
}

static void test_current(void ** arg)
{
    KADCSStatus    ret;
    imtq_axis_data data = { 0 };
    uint16_t       time = 10;

    expect_value(__wrap_write, cmd, START_CURRENT);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);
    ret = k_imtq_start_actuation_current(data, time);

    assert_int_equal(ret, ADCS_OK);
}

static void test_dipole(void ** arg)
{
    KADCSStatus    ret;
    imtq_axis_data data = { 0 };
    uint16_t       time = 10;

    expect_value(__wrap_write, cmd, START_DIPOLE);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);
    ret = k_imtq_start_actuation_dipole(data, time);

    assert_int_equal(ret, ADCS_OK);
}

static void test_PWM(void ** arg)
{
    KADCSStatus    ret;
    imtq_axis_data data = { 0 };
    uint16_t       time = 10;

    expect_value(__wrap_write, cmd, START_PWM);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);
    ret = k_imtq_start_actuation_PWM(data, time);

    assert_int_equal(ret, ADCS_OK);
}

static void test_PWM_exceed_x(void ** arg)
{
    KADCSStatus    ret;
    imtq_axis_data data = {
            .x = 2000,
            .y = 0,
            .z = 0
    };
    uint16_t time = 10;

    ret = k_imtq_start_actuation_PWM(data, time);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_PWM_exceed_y(void ** arg)
{
    KADCSStatus    ret;
    imtq_axis_data data = {
            .x = 0,
            .y = 2000,
            .z = 0
    };
    uint16_t time = 10;

    ret = k_imtq_start_actuation_PWM(data, time);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_PWM_exceed_z(void ** arg)
{
    KADCSStatus    ret;
    imtq_axis_data data = {
            .x = 0,
            .y = 0,
            .z = 2000
    };
    uint16_t time = 10;

    ret = k_imtq_start_actuation_PWM(data, time);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_selftest(void ** arg)
{
    KADCSStatus  ret;
    ADCSTestType axis = TEST_ALL;

    expect_value(__wrap_write, cmd, START_TEST);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);
    ret = k_imtq_start_test(axis);

    assert_int_equal(ret, ADCS_OK);
}

static void test_detumble(void ** arg)
{
    KADCSStatus ret;
    uint16_t    time = 20;

    expect_value(__wrap_write, cmd, START_BDOT);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);
    ret = k_imtq_start_detumble(time);

    assert_int_equal(ret, ADCS_OK);
}

/* Data Tests */

static void test_get_system_state(void ** arg)
{
    KADCSStatus ret;
    imtq_state  data = { 0 };

    expect_value(__wrap_write, cmd, GET_STATE);
    expect_value(__wrap_read, len, sizeof(imtq_state));
    will_return(__wrap_read, &state);

    ret = k_imtq_get_system_state(&data);

    assert_int_equal(ret, ADCS_OK);
}

static void test_get_system_state_null(void ** arg)
{
    KADCSStatus ret;

    ret = k_imtq_get_system_state(NULL);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_get_raw_mtm(void ** arg)
{
    KADCSStatus  ret;
    imtq_mtm_msg data = { 0 };

    expect_value(__wrap_write, cmd, GET_MTM_RAW);
    expect_value(__wrap_read, len, sizeof(mtm));
    will_return(__wrap_read, &mtm);

    ret = k_imtq_get_raw_mtm(&data);

    assert_int_equal(ret, ADCS_OK);
}

static void test_get_raw_mtm_null(void ** arg)
{
    KADCSStatus ret;

    ret = k_imtq_get_raw_mtm(NULL);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_get_calib_mtm(void ** arg)
{
    KADCSStatus  ret;
    imtq_mtm_msg data = { 0 };

    expect_value(__wrap_write, cmd, GET_MTM_CALIB);
    expect_value(__wrap_read, len, sizeof(mtm));
    will_return(__wrap_read, &mtm);

    ret = k_imtq_get_calib_mtm(&data);

    assert_int_equal(ret, ADCS_OK);
}

static void test_get_calib_mtm_null(void ** arg)
{
    KADCSStatus ret;

    ret = k_imtq_get_calib_mtm(NULL);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_get_coil_current(void ** arg)
{
    KADCSStatus       ret;
    imtq_coil_current data = { 0 };

    expect_value(__wrap_write, cmd, GET_CURRENT);
    expect_value(__wrap_read, len, sizeof(coil_current));
    will_return(__wrap_read, &coil_current);

    ret = k_imtq_get_coil_current(&data);

    assert_int_equal(ret, ADCS_OK);
}

static void test_get_coil_current_null(void ** arg)
{
    KADCSStatus ret;

    ret = k_imtq_get_coil_current(NULL);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_get_coil_temps(void ** arg)
{
    KADCSStatus    ret;
    imtq_coil_temp data = { 0 };

    expect_value(__wrap_write, cmd, GET_TEMPS);
    expect_value(__wrap_read, len, sizeof(coil_temp));
    will_return(__wrap_read, &coil_temp);

    ret = k_imtq_get_coil_temps(&data);

    assert_int_equal(ret, ADCS_OK);
}

static void test_get_coil_temps_null(void ** arg)
{
    KADCSStatus ret;

    ret = k_imtq_get_coil_temps(NULL);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_get_dipole(void ** arg)
{
    KADCSStatus ret;
    imtq_dipole data = { 0 };

    expect_value(__wrap_write, cmd, GET_DIPOLE);
    expect_value(__wrap_read, len, sizeof(dipole));
    will_return(__wrap_read, &dipole);

    ret = k_imtq_get_dipole(&data);

    assert_int_equal(ret, ADCS_OK);
}

static void test_get_dipole_null(void ** arg)
{
    KADCSStatus ret;

    ret = k_imtq_get_dipole(NULL);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_get_test_results_single(void ** arg)
{
    KADCSStatus             ret;
    imtq_test_result_single data = { 0 };

    expect_value(__wrap_write, cmd, GET_TEST);
    expect_value(__wrap_read, len, sizeof(test_results_single));
    will_return(__wrap_read, &test_results_single);
    ret = k_imtq_get_test_results_single(&data);

    assert_int_equal(ret, ADCS_OK);
}

static void test_get_test_results_single_null(void ** arg)
{
    KADCSStatus ret;

    ret = k_imtq_get_test_results_single(NULL);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_get_test_results_all(void ** arg)
{
    KADCSStatus          ret;
    imtq_test_result_all data = { 0 };

    expect_value(__wrap_write, cmd, GET_TEST);
    expect_value(__wrap_read, len, sizeof(test_results_all));
    will_return(__wrap_read, &test_results_all);
    ret = k_imtq_get_test_results_all(&data);

    assert_int_equal(ret, ADCS_OK);
}

static void test_get_test_results_all_null(void ** arg)
{
    KADCSStatus ret;

    ret = k_imtq_get_test_results_all(NULL);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_get_detumble(void ** arg)
{
    KADCSStatus   ret;
    imtq_detumble data = { 0 };

    expect_value(__wrap_write, cmd, GET_DETUMBLE);
    expect_value(__wrap_read, len, sizeof(detumble));
    will_return(__wrap_read, &detumble);
    ret = k_imtq_get_detumble(&data);

    assert_int_equal(ret, ADCS_OK);
}

static void test_get_detumble_null(void ** arg)
{
    KADCSStatus ret;

    ret = k_imtq_get_detumble(NULL);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_get_raw_housekeeping(void ** arg)
{
    KADCSStatus           ret;
    imtq_housekeeping_raw data = { 0 };

    expect_value(__wrap_write, cmd, GET_HOUSE_RAW);
    expect_value(__wrap_read, len, sizeof(house_raw));
    will_return(__wrap_read, &house_raw);
    ret = k_imtq_get_raw_housekeeping(&data);

    assert_int_equal(ret, ADCS_OK);
}

static void test_get_raw_housekeeping_null(void ** arg)
{
    KADCSStatus ret;

    ret = k_imtq_get_raw_housekeeping(NULL);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_get_eng_housekeeping(void ** arg)
{
    KADCSStatus           ret;
    imtq_housekeeping_eng data = { 0 };

    expect_value(__wrap_write, cmd, GET_HOUSE_ENG);
    expect_value(__wrap_read, len, sizeof(house_eng));
    will_return(__wrap_read, &house_eng);
    ret = k_imtq_get_eng_housekeeping(&data);

    assert_int_equal(ret, ADCS_OK);
}

static void test_get_eng_housekeeping_null(void ** arg)
{
    KADCSStatus ret;

    ret = k_imtq_get_eng_housekeeping(NULL);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_get_status_telemetry_null(void ** arg)
{
    KADCSStatus ret;

    ret = kprv_adcs_get_status_telemetry(NULL);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_get_nominal_telemetry_null(void ** arg)
{
    KADCSStatus ret;

    ret = kprv_adcs_get_nominal_telemetry(NULL);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_get_debug_telemetry_null(void ** arg)
{
    KADCSStatus ret;

    ret = kprv_adcs_get_debug_telemetry(NULL);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

/* Core Tests */

static void test_watchdog(void ** arg)
{
    KADCSStatus start_ret;
    KADCSStatus stop_ret;

    /* Stubs for underlying watchdog kick */
    expect_value(__wrap_write, cmd, NOOP);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);

    start_ret = k_imtq_watchdog_start();

    const struct timespec delay = {.tv_sec = 0, .tv_nsec = 2000001 };

    nanosleep(&delay, NULL);

    stop_ret = k_imtq_watchdog_stop();

    assert_int_equal(start_ret, ADCS_OK);
    assert_int_equal(stop_ret, ADCS_OK);
}

static void test_watchdog_twice(void ** arg)
{
    KADCSStatus start_ret;
    KADCSStatus stop_ret;

    /* Stubs for underlying watchdog kick */
    expect_value(__wrap_write, cmd, NOOP);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);

    start_ret = k_imtq_watchdog_start();

    const struct timespec delay = {.tv_sec = 0, .tv_nsec = 2000001 };

    nanosleep(&delay, NULL);

    stop_ret = k_imtq_watchdog_stop();

    assert_int_equal(start_ret, ADCS_OK);
    assert_int_equal(stop_ret, ADCS_OK);

    expect_value(__wrap_write, cmd, NOOP);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);

    start_ret = k_imtq_watchdog_start();

    nanosleep(&delay, NULL);

    stop_ret = k_imtq_watchdog_stop();

    assert_int_equal(start_ret, ADCS_OK);
    assert_int_equal(stop_ret, ADCS_OK);
}

static void test_watchdog_stop_no_start(void ** arg)
{
    KADCSStatus ret;

    ret = k_imtq_watchdog_stop();

    assert_int_equal(ret, ADCS_ERROR);
}

static void test_transfer_null_tx(void ** arg)
{
    KADCSStatus           ret;
    uint8_t               tx[1] = { 0 };
    uint8_t               rx[1] = { 0 };
    const struct timespec delay = {.tv_sec = 0, .tv_nsec = 1 };

    ret = kprv_imtq_transfer(NULL, sizeof(tx), rx, sizeof(rx), &delay);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_transfer_zero_tx_len(void ** arg)
{
    KADCSStatus           ret;
    uint8_t               tx[1] = { 0 };
    uint8_t               rx[1] = { 0 };
    const struct timespec delay = {.tv_sec = 0, .tv_nsec = 1 };

    ret = kprv_imtq_transfer(tx, 0, rx, sizeof(rx), &delay);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_transfer_null_rx(void ** arg)
{
    KADCSStatus           ret;
    uint8_t               tx[1] = { 0 };
    uint8_t               rx[1] = { 0 };
    const struct timespec delay = {.tv_sec = 0, .tv_nsec = 1 };

    ret = kprv_imtq_transfer(tx, sizeof(tx), NULL, sizeof(rx), &delay);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_transfer_zero_rx_len(void ** arg)
{
    KADCSStatus           ret;
    uint8_t               tx[1] = { 0 };
    uint8_t               rx[1] = { 0 };
    const struct timespec delay = {.tv_sec = 0, .tv_nsec = 1 };

    ret = kprv_imtq_transfer(tx, sizeof(tx), rx, 0, &delay);

    assert_int_equal(ret, ADCS_ERROR_CONFIG);
}

static void test_transfer_null_delay(void ** arg)
{
    KADCSStatus      ret;
    uint8_t          packet[] = { 0x11, 0x22, 0x33, 0x44 };
    imtq_resp_header resp     = { 0 };

    expect_value(__wrap_write, cmd, packet[0]);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);

    ret = kprv_imtq_transfer(packet, sizeof(packet), (uint8_t *) &resp,
                             sizeof(resp), NULL);

    assert_int_equal(ret, ADCS_OK);
}

static void test_transfer_cmd_mismatch(void ** arg)
{
    KADCSStatus ret;
    /* Dummy command value */
    uint8_t          packet[] = { 0x55 };
    imtq_resp_header resp     = { 0 };

    expect_value(__wrap_write, cmd, packet[0]);
    expect_value(__wrap_read, len, sizeof(error_resp));
    will_return(__wrap_read, &error_resp);

    ret = kprv_imtq_transfer(packet, sizeof(packet), (uint8_t *) &resp,
                             sizeof(resp), NULL);

    assert_int_equal(ret, ADCS_ERROR);
}

static void test_transfer_no_resp(void ** arg)
{
    KADCSStatus ret;
    /* 
     * Faking the empty response, since our stubs are set up to echo the
     * requested command 
     */
    uint8_t          packet[] = { 0xFF };
    imtq_resp_header resp     = { 0 };

    expect_value(__wrap_write, cmd, packet[0]);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);

    ret = kprv_imtq_transfer(packet, sizeof(packet), (uint8_t *) &resp,
                             sizeof(resp), NULL);

    assert_int_equal(ret, ADCS_ERROR_NO_RESPONSE);
}

static void test_transfer_error(void ** arg)
{
    KADCSStatus      ret;
    uint8_t          packet[] = { 0x11, 0x22, 0x33, 0x44 };
    imtq_resp_header resp     = { 0 };

    expect_value(__wrap_write, cmd, packet[0]);
    expect_value(__wrap_read, len, sizeof(error_resp));
    will_return(__wrap_read, &error_resp);

    ret = kprv_imtq_transfer(packet, sizeof(packet), (uint8_t *) &resp,
                             sizeof(resp), NULL);

    assert_int_equal(ret, ADCS_ERROR_INTERNAL);
}

/* End of Test Declarations */

static int init(void ** state)
{
    will_return(__wrap_open, 1);
    expect_value(__wrap_write, cmd, NOOP);
    expect_value(__wrap_read, len, sizeof(imtq_resp_header));
    will_return(__wrap_read, &response);
    k_adcs_init();

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
        /* Config tests */
        cmocka_unit_test(test_get_param_zero),
        cmocka_unit_test(test_get_param_null),
        cmocka_unit_test(test_get_param_resp),

        cmocka_unit_test(test_set_param_zero),
        cmocka_unit_test(test_set_param_null_value),
        cmocka_unit_test(test_set_param_null_resp),
        cmocka_unit_test(test_set_param_resp),

        cmocka_unit_test(test_reset_param_zero),
        cmocka_unit_test(test_reset_param_null),
        cmocka_unit_test(test_reset_param_resp),

        /* Ops tests */
        cmocka_unit_test(test_cancel),
        cmocka_unit_test(test_measure),
        cmocka_unit_test(test_current),
        cmocka_unit_test(test_dipole),
        cmocka_unit_test(test_PWM),
        cmocka_unit_test(test_PWM_exceed_x),
        cmocka_unit_test(test_PWM_exceed_y),
        cmocka_unit_test(test_PWM_exceed_z),
        cmocka_unit_test(test_selftest),
        cmocka_unit_test(test_detumble),

        /* Data Tests */
        cmocka_unit_test(test_get_system_state),
        cmocka_unit_test(test_get_system_state_null),
        cmocka_unit_test(test_get_raw_mtm),
        cmocka_unit_test(test_get_raw_mtm_null),
        cmocka_unit_test(test_get_calib_mtm),
        cmocka_unit_test(test_get_calib_mtm_null),
        cmocka_unit_test(test_get_coil_current),
        cmocka_unit_test(test_get_coil_current_null),
        cmocka_unit_test(test_get_coil_temps),
        cmocka_unit_test(test_get_coil_temps_null),
        cmocka_unit_test(test_get_dipole),
        cmocka_unit_test(test_get_dipole_null),
        cmocka_unit_test(test_get_test_results_single),
        cmocka_unit_test(test_get_test_results_single_null),
        cmocka_unit_test(test_get_test_results_all),
        cmocka_unit_test(test_get_test_results_all_null),
        cmocka_unit_test(test_get_detumble),
        cmocka_unit_test(test_get_detumble_null),
        cmocka_unit_test(test_get_raw_housekeeping),
        cmocka_unit_test(test_get_raw_housekeeping_null),
        cmocka_unit_test(test_get_eng_housekeeping),
        cmocka_unit_test(test_get_eng_housekeeping_null),
        cmocka_unit_test(test_get_status_telemetry_null),
        cmocka_unit_test(test_get_nominal_telemetry_null),
        cmocka_unit_test(test_get_debug_telemetry_null),

        /* Core Tests */
        cmocka_unit_test(test_watchdog),
        cmocka_unit_test(test_watchdog_twice),
        cmocka_unit_test(test_watchdog_stop_no_start),
        cmocka_unit_test(test_transfer_null_tx),
        cmocka_unit_test(test_transfer_zero_tx_len),
        cmocka_unit_test(test_transfer_null_rx),
        cmocka_unit_test(test_transfer_zero_rx_len),
        cmocka_unit_test(test_transfer_null_delay),
        cmocka_unit_test(test_transfer_cmd_mismatch),
        cmocka_unit_test(test_transfer_no_resp),
        cmocka_unit_test(test_transfer_error),
    };

    return cmocka_run_group_tests(tests, init, term);
}
