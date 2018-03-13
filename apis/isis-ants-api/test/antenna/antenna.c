/*
 * Kubos API for ISIS Antenna Systems
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

#include <isis-ants-api/ants-api.h>
#include <cmocka.h>

/* Test Data */
ants_telemetry system_telem
    = {.raw_temp = 574,
       .deploy_status
       = SYS_BURN_ACTIVE | ANT_2_STOPPED_TIME | ANT_2_NOT_DEPLOYED,
       .uptime = 9876 };

uint32_t uptime           = 432;
uint16_t deploy_status    = SYS_ARMED | ANT_1_ACTIVE | ANT_4_NOT_DEPLOYED;
uint16_t activation_time  = 44;
uint8_t  activation_count = 3;
/* End of Test Data */

static void test_no_init_arm(void ** arg)
{

    assert_int_equal(k_ants_arm(), ANTS_ERROR);
}

static void test_reset(void ** arg)
{
    KANTSStatus ret;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, SYSTEM_RESET);
    expect_value(__wrap_ioctl, addr, ANTS_SECONDARY);
    expect_value(__wrap_write, cmd, SYSTEM_RESET);
    ret = k_ants_reset();

    assert_int_equal(ret, ANTS_OK);
}

static void test_watchdog_kick(void ** arg)
{
    KANTSStatus ret;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, WATCHDOG_RESET);
    expect_value(__wrap_ioctl, addr, ANTS_SECONDARY);
    expect_value(__wrap_write, cmd, WATCHDOG_RESET);
    ret = k_ants_watchdog_kick();

    assert_int_equal(ret, ANTS_OK);
}

static void test_watchdog_thread(void ** arg)
{
    KANTSStatus start_ret;
    KANTSStatus stop_ret;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, WATCHDOG_RESET);
    expect_value(__wrap_ioctl, addr, ANTS_SECONDARY);
    expect_value(__wrap_write, cmd, WATCHDOG_RESET);

    start_ret = k_ants_watchdog_start();

    const struct timespec delay = {.tv_sec = 0, .tv_nsec = 2000001 };

    nanosleep(&delay, NULL);

    stop_ret = k_ants_watchdog_stop();

    assert_int_equal(start_ret, ANTS_OK);
    assert_int_equal(stop_ret, ANTS_OK);
}

static void test_watchdog_thread_twice(void ** arg)
{
    KANTSStatus start_ret;
    KANTSStatus stop_ret;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, WATCHDOG_RESET);
    expect_value(__wrap_ioctl, addr, ANTS_SECONDARY);
    expect_value(__wrap_write, cmd, WATCHDOG_RESET);

    start_ret = k_ants_watchdog_start();

    const struct timespec delay = {.tv_sec = 0, .tv_nsec = 2000001 };

    nanosleep(&delay, NULL);

    stop_ret = k_ants_watchdog_stop();

    assert_int_equal(start_ret, ANTS_OK);
    assert_int_equal(stop_ret, ANTS_OK);

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, WATCHDOG_RESET);
    expect_value(__wrap_ioctl, addr, ANTS_SECONDARY);
    expect_value(__wrap_write, cmd, WATCHDOG_RESET);

    start_ret = k_ants_watchdog_start();

    nanosleep(&delay, NULL);

    stop_ret = k_ants_watchdog_stop();

    assert_int_equal(start_ret, ANTS_OK);
    assert_int_equal(stop_ret, ANTS_OK);
}

static void test_watchdog_stop_no_start(void ** arg)
{
    KANTSStatus ret;

    ret = k_ants_watchdog_stop();

    assert_int_equal(ret, ANTS_ERROR);
}

static void test_arm(void ** arg)
{
    KANTSStatus ret;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, ARM_ANTS);
    ret = k_ants_arm();

    assert_int_equal(ret, ANTS_OK);
}

static void test_disarm(void ** arg)
{
    KANTSStatus ret;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, DISARM_ANTS);
    ret = k_ants_disarm();

    assert_int_equal(ret, ANTS_OK);
}

static void test_configure_primary(void ** arg)
{
    KANTSStatus ret;

    ret = k_ants_configure(PRIMARY);

    /*
     * Two-part verification:
     * 1. Configuration should complete successfully
     */
    assert_int_equal(ret, ANTS_OK);

    /*
     * 2. Commands now run against other address
     */
    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, ARM_ANTS);
    ret = k_ants_arm();

    assert_int_equal(ret, ANTS_OK);
}

static void test_configure_secondary(void ** arg)
{
    KANTSStatus ret;

    ret = k_ants_configure(SECONDARY);

    /*
     * Two-part verification:
     * 1. Configuration should complete successfully
     */
    assert_int_equal(ret, ANTS_OK);

    /*
     * 2. Commands now run against other address
     */
    expect_value(__wrap_ioctl, addr, ANTS_SECONDARY);
    expect_value(__wrap_write, cmd, ARM_ANTS);
    ret = k_ants_arm();

    assert_int_equal(ret, ANTS_OK);
}

static void test_configure_fake(void ** arg)
{
    KANTSStatus ret;

    ret = k_ants_configure(7);

    assert_int_equal(ret, ANTS_ERROR_CONFIG);
}

static void test_deploy_1_normal(void ** arg)
{
    KANTSStatus ret;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, DEPLOY_1);
    ret = k_ants_deploy(ANT_1, false, 5);

    assert_int_equal(ret, ANTS_OK);
}

static void test_deploy_1_override(void ** arg)
{
    KANTSStatus ret;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, DEPLOY_1_OVERRIDE);
    ret = k_ants_deploy(ANT_1, true, 10);

    assert_int_equal(ret, ANTS_OK);
}

static void test_deploy_2_normal(void ** arg)
{
    KANTSStatus ret;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, DEPLOY_2);
    ret = k_ants_deploy(ANT_2, false, 5);

    assert_int_equal(ret, ANTS_OK);
}

static void test_deploy_2_override(void ** arg)
{
    KANTSStatus ret;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, DEPLOY_2_OVERRIDE);
    ret = k_ants_deploy(ANT_2, true, 10);

    assert_int_equal(ret, ANTS_OK);
}

static void test_deploy_3_normal(void ** arg)
{
    KANTSStatus ret;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, DEPLOY_3);
    ret = k_ants_deploy(ANT_3, false, 5);

    assert_int_equal(ret, ANTS_OK);
}

static void test_deploy_3_override(void ** arg)
{
    KANTSStatus ret;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, DEPLOY_3_OVERRIDE);
    ret = k_ants_deploy(ANT_3, true, 10);

    assert_int_equal(ret, ANTS_OK);
}

static void test_deploy_4_normal(void ** arg)
{
    KANTSStatus ret;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, DEPLOY_4);
    ret = k_ants_deploy(ANT_4, false, 2);

    assert_int_equal(ret, ANTS_OK);
}

static void test_deploy_4_override(void ** arg)
{
    KANTSStatus ret;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, DEPLOY_4_OVERRIDE);
    ret = k_ants_deploy(ANT_4, true, 0);

    assert_int_equal(ret, ANTS_OK);
}

static void test_deploy_fake(void ** arg)
{
    KANTSStatus ret;

    ret = k_ants_deploy(6, true, 10);

    assert_int_equal(ret, ANTS_ERROR_CONFIG);
}

static void test_deploy_auto(void ** arg)
{
    KANTSStatus ret;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, AUTO_DEPLOY);
    ret = k_ants_auto_deploy(30);

    assert_int_equal(ret, ANTS_OK);
}

static void test_deploy_cancel(void ** arg)
{
    KANTSStatus ret;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, CANCEL_DEPLOY);
    ret = k_ants_cancel_deploy();

    assert_int_equal(ret, ANTS_OK);
}

static void test_get_deploy_status(void ** arg)
{
    KANTSStatus ret;
    uint16_t    resp;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, GET_STATUS);

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    will_return(__wrap_read, sizeof(deploy_status));
    will_return(__wrap_read, &deploy_status);

    ret = k_ants_get_deploy_status(&resp);

    assert_int_equal(ret, ANTS_OK);
}

static void test_get_deploy_status_null(void ** arg)
{
    KANTSStatus ret;

    ret = k_ants_get_deploy_status(NULL);

    assert_int_equal(ret, ANTS_ERROR_CONFIG);
}

static void test_get_uptime(void ** arg)
{
    KANTSStatus ret;
    uint32_t    resp;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, GET_UPTIME_SYS);

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    will_return(__wrap_read, sizeof(uptime));
    will_return(__wrap_read, &uptime);

    ret = k_ants_get_uptime(&resp);

    assert_int_equal(ret, ANTS_OK);
}

static void test_get_uptime_null(void ** arg)
{
    KANTSStatus ret;

    ret = k_ants_get_uptime(NULL);

    assert_int_equal(ret, ANTS_ERROR_CONFIG);
}

static void test_get_system_telemetry(void ** arg)
{
    KANTSStatus    ret;
    ants_telemetry resp;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, GET_TELEMETRY);

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    will_return(__wrap_read, sizeof(system_telem));
    will_return(__wrap_read, &system_telem);

    ret = k_ants_get_system_telemetry(&resp);

    assert_int_equal(ret, ANTS_OK);
}

static void test_get_system_telemetry_null(void ** arg)
{
    KANTSStatus ret;

    ret = k_ants_get_system_telemetry(NULL);

    assert_int_equal(ret, ANTS_ERROR_CONFIG);
}

static void test_get_activation_count_1(void ** arg)
{
    KANTSStatus ret;
    uint8_t     resp;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, GET_COUNT_1);

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    will_return(__wrap_read, sizeof(activation_count));
    will_return(__wrap_read, &activation_count);

    ret = k_ants_get_activation_count(ANT_1, &resp);

    assert_int_equal(ret, ANTS_OK);
}

static void test_get_activation_count_2(void ** arg)
{
    KANTSStatus ret;
    uint8_t     resp;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, GET_COUNT_2);

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    will_return(__wrap_read, sizeof(activation_count));
    will_return(__wrap_read, &activation_count);

    ret = k_ants_get_activation_count(ANT_2, &resp);

    assert_int_equal(ret, ANTS_OK);
}

static void test_get_activation_count_3(void ** arg)
{
    KANTSStatus ret;
    uint8_t     resp;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, GET_COUNT_3);

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    will_return(__wrap_read, sizeof(activation_count));
    will_return(__wrap_read, &activation_count);

    ret = k_ants_get_activation_count(ANT_3, &resp);

    assert_int_equal(ret, ANTS_OK);
}

static void test_get_activation_count_4(void ** arg)
{
    KANTSStatus ret;
    uint8_t     resp;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, GET_COUNT_4);

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    will_return(__wrap_read, sizeof(activation_count));
    will_return(__wrap_read, &activation_count);

    ret = k_ants_get_activation_count(ANT_4, &resp);

    assert_int_equal(ret, ANTS_OK);
}

static void test_get_activation_count_null(void ** arg)
{
    KANTSStatus ret;

    ret = k_ants_get_activation_count(ANT_1, NULL);

    assert_int_equal(ret, ANTS_ERROR_CONFIG);
}

static void test_get_activation_count_fake(void ** arg)
{
    KANTSStatus ret;
    uint8_t     resp;

    ret = k_ants_get_activation_count(6, &resp);

    assert_int_equal(ret, ANTS_ERROR_CONFIG);
}

static void test_get_activation_time_1(void ** arg)
{
    KANTSStatus ret;
    uint8_t     resp;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, GET_UPTIME_1);

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    will_return(__wrap_read, sizeof(activation_time));
    will_return(__wrap_read, &activation_time);

    ret = k_ants_get_activation_time(ANT_1, &resp);

    assert_int_equal(ret, ANTS_OK);
}

static void test_get_activation_time_2(void ** arg)
{
    KANTSStatus ret;
    uint8_t     resp;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, GET_UPTIME_2);

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    will_return(__wrap_read, sizeof(activation_time));
    will_return(__wrap_read, &activation_time);

    ret = k_ants_get_activation_time(ANT_2, &resp);

    assert_int_equal(ret, ANTS_OK);
}

static void test_get_activation_time_3(void ** arg)
{
    KANTSStatus ret;
    uint8_t     resp;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, GET_UPTIME_3);

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    will_return(__wrap_read, sizeof(activation_time));
    will_return(__wrap_read, &activation_time);

    ret = k_ants_get_activation_time(ANT_3, &resp);

    assert_int_equal(ret, ANTS_OK);
}

static void test_get_activation_time_4(void ** arg)
{
    KANTSStatus ret;
    uint8_t     resp;

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, GET_UPTIME_4);

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    will_return(__wrap_read, sizeof(activation_time));
    will_return(__wrap_read, &activation_time);

    ret = k_ants_get_activation_time(ANT_4, &resp);

    assert_int_equal(ret, ANTS_OK);
}

static void test_get_activation_time_null(void ** arg)
{
    KANTSStatus ret;

    ret = k_ants_get_activation_time(ANT_1, NULL);

    assert_int_equal(ret, ANTS_ERROR_CONFIG);
}

static void test_get_activation_time_fake(void ** arg)
{
    KANTSStatus ret;
    uint8_t     resp;

    ret = k_ants_get_activation_time(6, &resp);

    assert_int_equal(ret, ANTS_ERROR_CONFIG);
}

static void test_passthrough_null_tx(void ** arg)
{
    KANTSStatus ret;
    uint8_t     tx[1] = { 0 };
    uint8_t     rx[1] = { 0 };

    ret = k_ants_passthrough(NULL, sizeof(tx), rx, sizeof(rx));
    assert_int_equal(ret, ANTS_ERROR_CONFIG);
}

static void test_passthrough_zero_tx_len(void ** arg)
{
    KANTSStatus ret;
    uint8_t     tx[1] = { 0 };
    uint8_t     rx[1] = { 0 };

    ret = k_ants_passthrough(tx, 0, rx, sizeof(rx));
    assert_int_equal(ret, ANTS_ERROR_CONFIG);
}

static void test_passthrough_null_rx_nonzero_rx_len(void ** arg)
{
    KANTSStatus ret;
    uint8_t     tx[1] = { 0 };
    uint8_t     rx[1] = { 0 };

    ret = k_ants_passthrough(tx, sizeof(tx), NULL, sizeof(rx));
    assert_int_equal(ret, ANTS_ERROR_CONFIG);
}

static void test_passthrough_nonnull_rx_zero_rx_len(void ** arg)
{
    KANTSStatus ret;
    uint8_t     tx[1] = { 0 };
    uint8_t     rx[1] = { 0 };

    ret = k_ants_passthrough(tx, sizeof(tx), rx, 0);
    assert_int_equal(ret, ANTS_ERROR_CONFIG);
}

static void test_passthrough_null_rx_zero_rx_len(void ** arg)
{
    KANTSStatus ret;
    uint8_t     tx[1] = { 0x77 };
    uint8_t     rx[1] = { 0 };

    /*
     * Valid test case. If rx==null and rx_len==0,
     * we're only writing to the I2C device
     */
    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, tx[0]);

    ret = k_ants_passthrough(tx, sizeof(tx), NULL, 0);
    assert_int_equal(ret, ANTS_OK);
}

static void test_passthrough(void ** arg)
{
    KANTSStatus ret;
    uint8_t     tx[1] = { 0x99 };
    uint8_t     rx[1] = { 0 };

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    expect_value(__wrap_write, cmd, tx[0]);

    expect_value(__wrap_ioctl, addr, ANTS_PRIMARY);
    will_return(__wrap_read, sizeof(rx));
    will_return(__wrap_read, "K");

    ret = k_ants_passthrough(tx, sizeof(tx), rx, sizeof(rx));
    assert_int_equal(ret, ANTS_OK);
}

/* Watchdog tests? */

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
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_no_init_arm),
        cmocka_unit_test_setup_teardown(test_reset, init, term),
        cmocka_unit_test_setup_teardown(test_watchdog_kick, init, term),
        cmocka_unit_test_setup_teardown(test_watchdog_thread, init, term),
        cmocka_unit_test_setup_teardown(test_watchdog_thread_twice, init, term),
        cmocka_unit_test_setup_teardown(test_watchdog_stop_no_start, init, term),
        cmocka_unit_test_setup_teardown(test_arm, init, term),
        cmocka_unit_test_setup_teardown(test_disarm, init, term),
        cmocka_unit_test_setup_teardown(test_configure_primary, init, term),
        cmocka_unit_test_setup_teardown(test_configure_secondary, init, term),
        cmocka_unit_test_setup_teardown(test_configure_fake, init, term),
        cmocka_unit_test_setup_teardown(test_deploy_1_normal, init, term),
        cmocka_unit_test_setup_teardown(test_deploy_1_override, init, term),
        cmocka_unit_test_setup_teardown(test_deploy_2_normal, init, term),
        cmocka_unit_test_setup_teardown(test_deploy_2_override, init, term),
        cmocka_unit_test_setup_teardown(test_deploy_3_normal, init, term),
        cmocka_unit_test_setup_teardown(test_deploy_3_override, init, term),
        cmocka_unit_test_setup_teardown(test_deploy_4_normal, init, term),
        cmocka_unit_test_setup_teardown(test_deploy_4_override, init, term),
        cmocka_unit_test_setup_teardown(test_deploy_fake, init, term),
        cmocka_unit_test_setup_teardown(test_deploy_auto, init, term),
        cmocka_unit_test_setup_teardown(test_deploy_cancel, init, term),
        cmocka_unit_test_setup_teardown(test_get_deploy_status, init, term),
        cmocka_unit_test_setup_teardown(test_get_deploy_status_null, init, term),
        cmocka_unit_test_setup_teardown(test_get_uptime, init, term),
        cmocka_unit_test_setup_teardown(test_get_uptime_null, init, term),
        cmocka_unit_test_setup_teardown(test_get_system_telemetry, init, term),
        cmocka_unit_test_setup_teardown(test_get_system_telemetry_null, init,
                                        term),
        cmocka_unit_test_setup_teardown(test_get_activation_count_null, init,
                                        term),
        cmocka_unit_test_setup_teardown(test_get_activation_count_1, init, term),
        cmocka_unit_test_setup_teardown(test_get_activation_count_2, init, term),
        cmocka_unit_test_setup_teardown(test_get_activation_count_3, init, term),
        cmocka_unit_test_setup_teardown(test_get_activation_count_4, init, term),
        cmocka_unit_test_setup_teardown(test_get_activation_count_fake, init,
                                        term),
        cmocka_unit_test_setup_teardown(test_get_activation_time_null, init,
                                        term),
        cmocka_unit_test_setup_teardown(test_get_activation_time_1, init, term),
        cmocka_unit_test_setup_teardown(test_get_activation_time_2, init, term),
        cmocka_unit_test_setup_teardown(test_get_activation_time_3, init, term),
        cmocka_unit_test_setup_teardown(test_get_activation_time_4, init, term),
        cmocka_unit_test_setup_teardown(test_get_activation_time_fake, init,
                                        term),
        cmocka_unit_test_setup_teardown(test_passthrough_null_tx, init, term),
        cmocka_unit_test_setup_teardown(test_passthrough_zero_tx_len, init,
                                        term),
        cmocka_unit_test_setup_teardown(
            test_passthrough_null_rx_nonzero_rx_len, init, term),
        cmocka_unit_test_setup_teardown(
            test_passthrough_nonnull_rx_zero_rx_len, init, term),
        cmocka_unit_test_setup_teardown(test_passthrough_null_rx_zero_rx_len,
                                        init, term),
        cmocka_unit_test_setup_teardown(test_passthrough, init, term),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}
