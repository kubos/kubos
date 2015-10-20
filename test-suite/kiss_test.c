/*
 * KubOS Core Flight Services
 * Copyright (C) 2015 Kubos Corporation
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
#include <stdio.h>

#include <embUnit.h>
#include <net/gnrc.h>

#include "kiss.h"

#define KISS_STACKSIZE (THREAD_STACKSIZE_DEFAULT)
#define KISS_PRIO      (THREAD_PRIORITY_MAIN - 4)

static kiss_dev_t _kiss_dev;
static FILE *_kiss_file;
static kernel_pid_t _kiss_pid;
static char _kiss_stack[KISS_STACKSIZE];

static void setUp(void)
{
    _kiss_file = fopen("/tmp/_kiss_test", "w+");
    TEST_ASSERT(_kiss_file);

    _kiss_pid = kiss_init_native(&_kiss_dev, fileno(_kiss_file),
                                 _kiss_stack, KISS_STACKSIZE, KISS_PRIO);
    TEST_ASSERT(_kiss_pid > KERNEL_PID_UNDEF);
}

static void tearDown(void)
{
    if (_kiss_file) {
        fclose(_kiss_file);
    }
}

static void _send(void *data, size_t len, char *dest, size_t *dest_len)
{
    gnrc_pktsnip_t *payload = gnrc_pktbuf_add(NULL, data, len, GNRC_NETTYPE_UNDEF);
    gnrc_netapi_send(_kiss_pid, payload);

    // force an ack using gnrc_netapi_get
    gnrc_netapi_get(_kiss_pid, 0, 0, NULL, 0);

    // read back from the file
    *dest_len = (size_t) ftell(_kiss_file);
    fseek(_kiss_file, 0, SEEK_SET);
    fread(dest, 1, *dest_len, _kiss_file);
}

static void kiss_basic_framing(void)
{
    char *data = "A";
    char buffer[16];
    size_t size;

    _send(data, 1, buffer, &size);

    TEST_ASSERT_EQUAL_INT(size, 4);
    TEST_ASSERT_EQUAL_INT(buffer[0], KISS_FEND);
    TEST_ASSERT_EQUAL_INT(buffer[1], 0);
    TEST_ASSERT_EQUAL_INT(buffer[2], 'A');
    TEST_ASSERT_EQUAL_INT(buffer[3], KISS_FEND);
}

static void kiss_escapes(void)
{
    char data[] = { 'A', 'B', KISS_FEND, KISS_FESC, 'C' };
    char buffer[16];
    size_t size;

    _send(data, 5, buffer, &size);

    TEST_ASSERT_EQUAL_INT(size, 10);
    TEST_ASSERT_EQUAL_INT(buffer[0], KISS_FEND);
    TEST_ASSERT_EQUAL_INT(buffer[1], 0);
    TEST_ASSERT_EQUAL_INT(buffer[2], 'A');
    TEST_ASSERT_EQUAL_INT(buffer[3], 'B');
    TEST_ASSERT_EQUAL_INT(buffer[4], KISS_FESC);
    TEST_ASSERT_EQUAL_INT(buffer[5], KISS_TFEND);
    TEST_ASSERT_EQUAL_INT(buffer[6], KISS_FESC);
    TEST_ASSERT_EQUAL_INT(buffer[7], KISS_TFESC);
    TEST_ASSERT_EQUAL_INT(buffer[8], 'C');
    TEST_ASSERT_EQUAL_INT(buffer[9], KISS_FEND);
}

TestRef kiss_suite(void)
{
    EMB_UNIT_TESTFIXTURES(fixtures) {
        new_TestFixture(kiss_basic_framing),
        new_TestFixture(kiss_escapes),
    };

    EMB_UNIT_TESTCALLER(kiss_tests, setUp, tearDown, fixtures);
    return (TestRef) &kiss_tests;
}
