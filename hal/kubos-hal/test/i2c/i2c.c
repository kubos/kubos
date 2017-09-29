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
#include "kubos-hal/i2c.h"

#define TEST_I2C K_I2C1
#define TEST_ADDR 0x50


static KI2CConf get_conf(void)
{
    return (KI2CConf) k_i2c_conf_defaults();
}

static void test_no_init_write(void ** arg)
{
    char data = 'A';
    assert_int_equal(k_i2c_write(TEST_I2C, TEST_ADDR, &data, 1), I2C_ERROR);
}

static void test_no_init_read(void ** arg)
{
    char data;
    assert_int_equal(k_i2c_read(TEST_I2C, TEST_ADDR, &data, 1), I2C_ERROR);
}

static void test_init_write(void ** arg)
{
    char data = 'A';
    KI2CConf conf = get_conf();
    int ret;

    will_return(__wrap_open, 1);
    k_i2c_init(TEST_I2C, &conf);

    will_return(__wrap_ioctl, 0);
    will_return(__wrap_write, 1);
    ret = k_i2c_write(TEST_I2C, TEST_ADDR, &data, 1);

    will_return(__wrap_close, 0);
    k_i2c_terminate(TEST_I2C);

    assert_int_equal(ret, I2C_OK);
}

static void test_init_write_null(void ** arg)
{
    char * data = NULL;
    KI2CConf conf = get_conf();
    int ret;

    will_return(__wrap_open, 1);
    k_i2c_init(TEST_I2C, &conf);

    ret = k_i2c_write(TEST_I2C, TEST_ADDR, data, 1);

    will_return(__wrap_close, 0);
    k_i2c_terminate(TEST_I2C);

    assert_int_equal(ret, I2C_ERROR);
}

/*
 * The result of the underlying read call has varied behavior,
 * depending on the I2C device being used. It has been stubbed
 * to always fail.
 */
static void test_init_read(void ** arg)
{
    char data;
    KI2CConf conf = get_conf();
    int ret;

    will_return(__wrap_open, 1);
    k_i2c_init(TEST_I2C, &conf);

    will_return(__wrap_ioctl, 0);
    will_return(__wrap_read, 0);
    ret = k_i2c_read(TEST_I2C, TEST_ADDR, &data, 1);

    will_return(__wrap_close, 0);
    k_i2c_terminate(TEST_I2C);

    assert_int_equal(ret, I2C_ERROR);
}

static void test_init_read_null(void ** arg)
{
    char * data = NULL;
    KI2CConf conf = get_conf();
    int ret;

    will_return(__wrap_open, 1);
    k_i2c_init(TEST_I2C, &conf);

    ret = k_i2c_read(TEST_I2C, TEST_ADDR, data, 1);

    will_return(__wrap_close, 0);
    k_i2c_terminate(TEST_I2C);

    assert_int_equal(ret, I2C_ERROR);
}

static void test_init_write_read(void ** arg)
{
    char data = 'A';
    char read;
    KI2CConf conf = get_conf();
    int write_ret;
    int read_ret;

    will_return(__wrap_open, 1);
    k_i2c_init(TEST_I2C, &conf);

    will_return(__wrap_ioctl, 0);
    will_return(__wrap_write, 1);
    write_ret = k_i2c_write(TEST_I2C, TEST_ADDR, &data, 1);

    will_return(__wrap_ioctl, 0);
    will_return(__wrap_read, 1);
    read_ret = k_i2c_read(TEST_I2C, TEST_ADDR, &read, 1);

    will_return(__wrap_close, 0);
    k_i2c_terminate(TEST_I2C);

    assert_int_equal(write_ret, I2C_OK);
    assert_int_equal(read_ret, I2C_OK);
    assert_int_equal(data, read);
}

static void test_init_term_write(void ** arg)
{
    KI2CConf conf = get_conf();
    char data = 'A';

    will_return(__wrap_open, 1);
    k_i2c_init(TEST_I2C, &conf);

    will_return(__wrap_close, 0);
    k_i2c_terminate(TEST_I2C);

    assert_int_equal(k_i2c_write(TEST_I2C, TEST_ADDR, &data, 1), I2C_ERROR);
}

static void test_init_term_read(void ** arg)
{
    KI2CConf conf = get_conf();
    char read;

    will_return(__wrap_open, 1);
    k_i2c_init(TEST_I2C, &conf);

    will_return(__wrap_close, 0);
    k_i2c_terminate(TEST_I2C);

    assert_int_equal(k_i2c_read(TEST_I2C, TEST_ADDR, &read, 1), I2C_ERROR);
}

static void test_init_term_write_read(void ** arg)
{
    KI2CConf conf = get_conf();
    char data = 'A';
    char read;

    will_return(__wrap_open, 1);
    k_i2c_init(TEST_I2C, &conf);

    will_return(__wrap_close, 0);
    k_i2c_terminate(TEST_I2C);

    assert_int_equal(k_i2c_write(TEST_I2C, TEST_ADDR, &data, 1), I2C_ERROR);
    assert_int_equal(k_i2c_read(TEST_I2C, TEST_ADDR, &read, 1), I2C_ERROR);
}

static void test_init_term_init_write_read(void ** arg)
{
    KI2CConf conf = get_conf();
    char data = 'A';
    char read;
    int write_ret;
    int read_ret;

    will_return(__wrap_open, 1);
    k_i2c_init(TEST_I2C, &conf);

    will_return(__wrap_close, 0);
    k_i2c_terminate(TEST_I2C);

    will_return(__wrap_open, 2);
    k_i2c_init(TEST_I2C, &conf);

    will_return(__wrap_ioctl, 0);
    will_return(__wrap_write, 1);
    write_ret = k_i2c_write(TEST_I2C, TEST_ADDR, &data, 1);

    will_return(__wrap_ioctl, 0);
    will_return(__wrap_read, 1);
    read_ret = k_i2c_read(TEST_I2C, TEST_ADDR, &read, 1);

    will_return(__wrap_close, 0);
    k_i2c_terminate(TEST_I2C);
    
    assert_int_equal(write_ret, I2C_OK);
    assert_int_equal(read_ret, I2C_OK);
    assert_int_equal(data, read);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
            cmocka_unit_test(test_no_init_write),
            cmocka_unit_test(test_no_init_read),
            cmocka_unit_test(test_init_write),
            cmocka_unit_test(test_init_write_null),
            cmocka_unit_test(test_init_read),
            cmocka_unit_test(test_init_read_null),
            cmocka_unit_test(test_init_write_read),
            cmocka_unit_test(test_init_term_write),
            cmocka_unit_test(test_init_term_read),
            cmocka_unit_test(test_init_term_write_read),
            cmocka_unit_test(test_init_term_init_write_read),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}
