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
#include "kubos-hal/spi.h"

#define TEST_SPI K_SPI1

static KSPIConf get_conf(void)
{
    return (KSPIConf) k_spi_conf_defaults();
}

static void test_no_init_write(void)
{
    char data = 'A';
    TEST_ASSERT_EQUAL_INT(k_spi_write(TEST_SPI, &data, 1), SPI_ERROR);
}

static void test_no_init_read(void)
{
    char data;
    TEST_ASSERT_EQUAL_INT(k_spi_read(TEST_SPI, &data, 1), SPI_ERROR);
}

static void test_init_write(void)
{
    char data = 'A';
    KSPIConf conf = get_conf();
    int ret;

    k_spi_init(TEST_SPI, &conf);
    ret = k_spi_write(TEST_SPI, &data, 1);
    k_spi_terminate(TEST_SPI);

    TEST_ASSERT_EQUAL_INT(ret, SPI_OK);
}

static void test_init_write_null(void)
{
    char * data = NULL;
    KSPIConf conf = get_conf();
    int ret;

    k_spi_init(TEST_SPI, &conf);
    ret = k_spi_write(TEST_SPI, data, 100);
    k_spi_terminate(TEST_SPI);

    TEST_ASSERT_EQUAL_INT(ret, SPI_ERROR);
}

static void test_init_read(void)
{
    char data;
    KSPIConf conf = get_conf();
    int ret;

    k_spi_init(TEST_SPI, &conf);
    ret = k_spi_read(TEST_SPI, &data, 1);
    k_spi_terminate(TEST_SPI);

    TEST_ASSERT_EQUAL_INT(ret, SPI_ERROR);
}

static void test_init_read_null(void)
{
    char * data = NULL;
    KSPIConf conf = get_conf();
    int ret;

    k_spi_init(TEST_SPI, &conf);
    ret = k_spi_read(TEST_SPI, data, 1);
    k_spi_terminate(TEST_SPI);

    TEST_ASSERT_EQUAL_INT(ret, SPI_ERROR);
}

static void test_init_write_read(void)
{
    char data = 'A';
    char read;
    KSPIConf conf = get_conf();
    int write_ret;
    int read_ret;

    k_spi_init(TEST_SPI, &conf);
    write_ret = k_spi_write(TEST_SPI, &data, 1);
    read_ret = k_spi_read(TEST_SPI, &read, 1);
    k_spi_terminate(TEST_SPI);

    TEST_ASSERT_EQUAL_INT(write_ret, SPI_OK);
    TEST_ASSERT_EQUAL_INT(read_ret, SPI_OK);
    TEST_ASSERT_EQUAL_INT(data, read);
}

static void test_init_term_write(void)
{
    KSPIConf conf = get_conf();
    char data = 'A';

    k_spi_init(TEST_SPI, &conf);
    k_spi_terminate(TEST_SPI);

    TEST_ASSERT_EQUAL_INT(k_spi_write(TEST_SPI, &data, 1), SPI_ERROR);
}

static void test_init_term_read(void)
{
    KSPIConf conf = get_conf();
    char read;

    k_spi_init(TEST_SPI, &conf);
    k_spi_terminate(TEST_SPI);

    TEST_ASSERT_EQUAL_INT(k_spi_read(TEST_SPI, &read, 1), SPI_ERROR);
}

static void test_init_term_write_read(void)
{
    KSPIConf conf = get_conf();
    char data = 'A';
    char read;

    k_spi_init(TEST_SPI, &conf);
    k_spi_terminate(TEST_SPI);

    TEST_ASSERT_EQUAL_INT(k_spi_write(TEST_SPI, &data, 1), SPI_ERROR);
    TEST_ASSERT_EQUAL_INT(k_spi_read(TEST_SPI, &read, 1), SPI_ERROR);
}

static void test_init_term_init_write_read(void)
{
    KSPIConf conf = get_conf();
    char data = 'A';
    char read;
    int write_ret;
    int read_ret;

    k_spi_init(TEST_SPI, &conf);
    k_spi_terminate(TEST_SPI);
    k_spi_init(TEST_SPI, &conf);
    write_ret = k_spi_write(TEST_SPI, &data, 1);
    read_ret = k_spi_read(TEST_SPI, &read, 1);
    k_spi_terminate(TEST_SPI);

    TEST_ASSERT_EQUAL_INT(write_ret, SPI_OK);
    TEST_ASSERT_EQUAL_INT(read_ret, SPI_OK);
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
