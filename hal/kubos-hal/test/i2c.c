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

#include "unity/unity.h"
#include "unity/k_test.h"
#include "kubos-hal/i2c.h"

#define TEST_I2C K_I2C1
#define TEST_ADDR 0x50


static KI2CConf get_conf(void)
{
    return (KI2CConf) k_i2c_conf_defaults();
}

static void test_no_init_write(void)
{
    char data = 'A';
    TEST_ASSERT_EQUAL_INT(k_i2c_write(TEST_I2C, TEST_ADDR, &data, 1), I2C_ERROR);
}

static void test_no_init_read(void)
{
    char data;
    TEST_ASSERT_EQUAL_INT(k_i2c_read(TEST_I2C, TEST_ADDR, &data, 1), I2C_ERROR);
}

static void test_init_write(void)
{
    char data = 'A';
    KI2CConf conf = get_conf();
    int ret;

    k_i2c_init(TEST_I2C, &conf);
    ret = k_i2c_write(TEST_I2C, TEST_ADDR, &data, 1);
    k_i2c_terminate(TEST_I2C);

    TEST_ASSERT_EQUAL_INT(ret, I2C_OK);
}

static void test_init_write_null(void)
{
    char * data = NULL;
    KI2CConf conf = get_conf();
    int ret;

    k_i2c_init(TEST_I2C, &conf);
    ret = k_i2c_write(TEST_I2C, TEST_ADDR, data, 1);
    k_i2c_terminate(TEST_I2C);

    TEST_ASSERT_EQUAL_INT(ret, I2C_ERROR);
}

static void test_init_read(void)
{
    char data;
    KI2CConf conf = get_conf();
    int ret;

    k_i2c_init(TEST_I2C, &conf);
    ret = k_i2c_read(TEST_I2C, TEST_ADDR, &data, 1);
    k_i2c_terminate(TEST_I2C);

    TEST_ASSERT_EQUAL_INT(ret, I2C_ERROR);
}

static void test_init_read_null(void)
{
    char * data = NULL;
    KI2CConf conf = get_conf();
    int ret;

    k_i2c_init(TEST_I2C, &conf);
    ret = k_i2c_read(TEST_I2C, TEST_ADDR, data, 1);
    k_i2c_terminate(TEST_I2C);

    TEST_ASSERT_EQUAL_INT(ret, I2C_ERROR);
}

static void test_init_write_read(void)
{
    char data = 'A';
    char read;
    KI2CConf conf = get_conf();
    int write_ret;
    int read_ret;

    k_i2c_init(TEST_I2C, &conf);
    write_ret = k_i2c_write(TEST_I2C, TEST_ADDR, &data, 1);
    read_ret = k_i2c_read(TEST_I2C, TEST_ADDR, &read, 1);
    k_i2c_terminate(TEST_I2C);

    TEST_ASSERT_EQUAL_INT(write_ret, I2C_OK);
    TEST_ASSERT_EQUAL_INT(read_ret, I2C_OK);
    TEST_ASSERT_EQUAL_INT(data, read);
}

static void test_init_term_write(void)
{
    KI2CConf conf = get_conf();
    char data = 'A';

    k_i2c_init(TEST_I2C, &conf);
    k_i2c_terminate(TEST_I2C);

    TEST_ASSERT_EQUAL_INT(k_i2c_write(TEST_I2C, TEST_ADDR, &data, 1), I2C_ERROR);
}

static void test_init_term_read(void)
{
    KI2CConf conf = get_conf();
    char read;

    k_i2c_init(TEST_I2C, &conf);
    k_i2c_terminate(TEST_I2C);

    TEST_ASSERT_EQUAL_INT(k_i2c_read(TEST_I2C, TEST_ADDR, &read, 1), I2C_ERROR);
}

static void test_init_term_write_read(void)
{
    KI2CConf conf = get_conf();
    char data = 'A';
    char read;

    k_i2c_init(TEST_I2C, &conf);
    k_i2c_terminate(TEST_I2C);

    TEST_ASSERT_EQUAL_INT(k_i2c_write(TEST_I2C, TEST_ADDR, &data, 1), I2C_ERROR);
    TEST_ASSERT_EQUAL_INT(k_i2c_read(TEST_I2C, TEST_ADDR, &read, 1), I2C_ERROR);
}

static void test_init_term_init_write_read(void)
{
    KI2CConf conf = get_conf();
    char data = 'A';
    char read;
    int write_ret;
    int read_ret;

    k_i2c_init(TEST_I2C, &conf);
    k_i2c_terminate(TEST_I2C);
    k_i2c_init(TEST_I2C, &conf);
    write_ret = k_i2c_write(TEST_I2C, TEST_ADDR, &data, 1);
    read_ret = k_i2c_read(TEST_I2C, TEST_ADDR, &read, 1);
    k_i2c_terminate(TEST_I2C);
    
    TEST_ASSERT_EQUAL_INT(write_ret, I2C_OK);
    TEST_ASSERT_EQUAL_INT(read_ret, I2C_OK);
    TEST_ASSERT_EQUAL_INT(data, read);
}

K_TEST_MAIN()
{
    UNITY_BEGIN();
    RUN_TEST(test_no_init_write);
    RUN_TEST(test_no_init_read);
    RUN_TEST(test_init_write);
    RUN_TEST(test_init_write_null);
    RUN_TEST(test_init_read);
    RUN_TEST(test_init_read_null);
    RUN_TEST(test_init_write_read);
    RUN_TEST(test_init_term_write);
    RUN_TEST(test_init_term_read);
    RUN_TEST(test_init_term_write_read);
    RUN_TEST(test_init_term_init_write_read);
    return UNITY_END();
}

int main(void)
{
    K_TEST_RUN_MAIN();
}
