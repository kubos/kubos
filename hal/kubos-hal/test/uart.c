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
#include "kubos-hal/uart.h"

#define TEST_UART K_UART1

static KUARTConf get_test_conf(void)
{
    KUARTConf conf = k_uart_conf_defaults();
    return conf;
}

static void test_no_init_write(void)
{
    char data = 'A';
    TEST_ASSERT_EQUAL_INT(k_uart_write(TEST_UART, &data, 1), 0);
}

static void test_no_init_read(void)
{
    char data = 'A';
    TEST_ASSERT_EQUAL_INT(k_uart_read(TEST_UART, &data, 1), 0);
}

static void test_init_write(void)
{
    KUARTConf conf = get_test_conf();
    char data = 'A';
    int ret;

    k_uart_init(TEST_UART, &conf);
    ret = k_uart_write(TEST_UART, &data, 1);
    k_uart_terminate(TEST_UART);

    TEST_ASSERT_EQUAL_INT(ret, 1);
}


static void test_init_write_null(void)
{
    KUARTConf conf = get_test_conf();
    char * data = NULL;
    int ret;

    k_uart_init(TEST_UART, &conf);
    ret = k_uart_write(TEST_UART, data, 1);
    k_uart_terminate(TEST_UART);

    TEST_ASSERT_EQUAL_INT(ret, 0);
}


static void test_init_read(void)
{
    KUARTConf conf = get_test_conf();
    char read;
    int ret;

    k_uart_init(TEST_UART, &conf);
    ret = k_uart_read(TEST_UART, &read, 1);
    k_uart_terminate(TEST_UART);

    TEST_ASSERT_EQUAL_INT(ret, 0);
}

static void test_init_read_null(void)
{
    KUARTConf conf = get_test_conf();
    char * read = NULL;
    int ret;

    k_uart_init(TEST_UART, &conf);
    ret = k_uart_read(TEST_UART, read, 1);
    k_uart_terminate(TEST_UART);

    TEST_ASSERT_EQUAL_INT(ret, 0);
}

static void test_init_write_read(void)
{
    KUARTConf conf = get_test_conf();
    char data = 'A';
    char read;
    int write_ret;
    int read_ret;

    k_uart_init(TEST_UART, &conf);
    write_ret = k_uart_write(TEST_UART, &data, 1);
    read_ret = k_uart_read(TEST_UART, &read, 1);
    k_uart_terminate(TEST_UART);

    TEST_ASSERT_EQUAL_INT(write_ret, 1);
    TEST_ASSERT_EQUAL_INT(read_ret, 1);
    TEST_ASSERT_EQUAL_INT(data, read);
}

static void test_init_write_read_str_wrong_size(void)
{
    KUARTConf conf = get_test_conf();
    char * data = "ryanp\0";
    char read[10];
    int write_ret;
    int read_ret;

    k_uart_init(TEST_UART, &conf);
    write_ret = k_uart_write(TEST_UART, data, 6);
    read_ret = k_uart_read(TEST_UART, read, 10);
    k_uart_terminate(TEST_UART);

    TEST_ASSERT_EQUAL_INT(write_ret, 6);
    TEST_ASSERT_EQUAL_INT(read_ret, 6);
    TEST_ASSERT_EQUAL_STRING(data, read);
}

static void test_init_term_write(void)
{
    KUARTConf conf = get_test_conf();
    char data = 'A';

    k_uart_init(TEST_UART, &conf);
    k_uart_terminate(TEST_UART);

    TEST_ASSERT_EQUAL_INT(k_uart_write(TEST_UART, &data, 1), 0);
}

static void test_init_term_read(void)
{
    KUARTConf conf = get_test_conf();
    char read;

    k_uart_init(TEST_UART, &conf);
    k_uart_terminate(TEST_UART);

    TEST_ASSERT_EQUAL_INT(k_uart_read(TEST_UART, &read, 1), 0);
}

static void test_init_term_write_read(void)
{
    KUARTConf conf = get_test_conf();
    char data = 'A';
    char read;

    k_uart_terminate(TEST_UART);

    TEST_ASSERT_EQUAL_INT(k_uart_write(TEST_UART, &data, 1), 0);
    // Return of -2 from k_uart_rx_queue_len indicates no queue
    // which is appropriate after running k_uart_terminate
    TEST_ASSERT_EQUAL_INT(k_uart_rx_queue_len(TEST_UART), -2);
    TEST_ASSERT_EQUAL_INT(k_uart_read(TEST_UART, &read, 1), 0);
}

static void test_init_term_init_write_read(void)
{
    KUARTConf conf = get_test_conf();
    char data = 'A';
    char read;
    int write_ret;
    int queue_len;
    int read_ret;

    k_uart_init(TEST_UART, &conf);
    k_uart_terminate(TEST_UART);
    k_uart_init(TEST_UART, &conf);
    write_ret = k_uart_write(TEST_UART, &data, 1);
    queue_len = k_uart_rx_queue_len(TEST_UART);
    read_ret = k_uart_read(TEST_UART, &read, 1);
    k_uart_terminate(TEST_UART);

    TEST_ASSERT_EQUAL_INT(write_ret, 1);
    TEST_ASSERT_EQUAL_INT(queue_len, 1);
    TEST_ASSERT_EQUAL_INT(read_ret, 1);
    TEST_ASSERT_EQUAL_INT(data, read);
}

static void test_init_push_read(void)
{
    KUARTConf conf = get_test_conf();
    char data = 'A';
    char read;
    int queue_len;
    int read_ret;

    k_uart_init(TEST_UART, &conf);
    k_uart_rx_queue_push(TEST_UART, data, NULL);
    queue_len = k_uart_rx_queue_len(TEST_UART);
    read_ret = k_uart_read(TEST_UART, &read, 1);
    k_uart_terminate(TEST_UART);

    TEST_ASSERT_EQUAL_INT(queue_len, 1);
    TEST_ASSERT_EQUAL_INT(read_ret, 1);
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
    RUN_TEST(test_init_write_read_str_wrong_size);
    RUN_TEST(test_init_push_read);
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
