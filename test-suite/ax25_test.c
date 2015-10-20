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
#include <embUnit.h>
#include <string.h>

#include <net/gnrc.h>

#include "ax25.h"
#include "aprs.h"

#include "tests.h"

#define assert_ax25_chars(s1, s2) do { \
    char *s1_ = s1; \
    char *s2_ = s2; \
    TEST_ASSERT_EQUAL_INT(s1_[0], AX25_CHAR(s2_[0])); \
    TEST_ASSERT_EQUAL_INT(s1_[1], AX25_CHAR(s2_[1])); \
    TEST_ASSERT_EQUAL_INT(s1_[2], AX25_CHAR(s2_[2])); \
    TEST_ASSERT_EQUAL_INT(s1_[3], AX25_CHAR(s2_[3])); \
    TEST_ASSERT_EQUAL_INT(s1_[4], AX25_CHAR(s2_[4])); \
    TEST_ASSERT_EQUAL_INT(s1_[5], AX25_CHAR(s2_[5])); \
    TEST_ASSERT_EQUAL_INT(s1_[6], AX25_CHAR(s2_[6])); \
} while(0)

#define assert_ax25_addr(addr_, str) do { \
    ax25_addr_t addr = addr_; \
    char *c = str; \
    char addr_str[7]; \
    memcpy(addr_str, &addr, 7); \
    assert_ax25_chars(addr_str, c); \
} while (0)

static void ax25_short_callsign(void)
{
    assert_ax25_addr(ax25_addr_init("CALL"), "CALL  0");
    assert_ax25_addr(ax25_addr_init("CALL-3"), "CALL  3");
}

static void ax25_long_callsign(void)
{
    assert_ax25_addr(ax25_addr_init("ABCDEF"), "ABCDEF0");
    assert_ax25_addr(ax25_addr_init("ABCDEF-3"), "ABCDEF3");
}

static void ax25_trimmed_callsign(void)
{
    assert_ax25_addr(ax25_addr_init("ABCDEFGH"), "ABCDEF0");
}

static void ax25_null_addr_nocall(void)
{
    assert_ax25_addr(ax25_addr_init(NULL), "N0CALL0");
}

static void ax25_pkt_build_fail_cases(void)
{
    ax25_addr_t addrs[] = { AX25_ADDR_NOCALL };

    // NULL addrs
    TEST_ASSERT_NULL(ax25_pkt_build(NULL, NULL, 1, 0, 0));

    // 0 addrs
    TEST_ASSERT_NULL(ax25_pkt_build(NULL, addrs, 0, 0, 0));
}

static void ax25_pkt_build_ui_packet(void)
{
    ax25_addr_t addrs[] = {
        AX25_ADDR_NOCALL,
        AX25_ADDR_NOCALL
    };

    char *info = "ABC", *data, *payload_data;
    int i = 0, j = 0;

    gnrc_pktsnip_t *payload = gnrc_pktbuf_add(NULL, info, 3, GNRC_NETTYPE_UNDEF);
    gnrc_pktsnip_t *pkt = ax25_ui_pkt_build(payload, addrs, 2);

    TEST_ASSERT_NOT_NULL(pkt);
    TEST_ASSERT_EQUAL_INT(gnrc_pkt_len(pkt), 21);

    data = (char *) pkt->data;
    assert_ax25_chars(data, "N0CALL0");
    i = 7;

    TEST_ASSERT_EQUAL_INT(data[i++], AX25_CHAR('N'));
    TEST_ASSERT_EQUAL_INT(data[i++], AX25_CHAR('0'));
    TEST_ASSERT_EQUAL_INT(data[i++], AX25_CHAR('C'));
    TEST_ASSERT_EQUAL_INT(data[i++], AX25_CHAR('A'));
    TEST_ASSERT_EQUAL_INT(data[i++], AX25_CHAR('L'));
    TEST_ASSERT_EQUAL_INT(data[i++], AX25_CHAR('L'));
    // The last address has an extra bit at the end
    TEST_ASSERT_EQUAL_INT(data[i++], (AX25_CHAR('0') | 0x01));

    // control / protocol
    TEST_ASSERT_EQUAL_INT(data[i++], (char) AX25_UI_CONTROL);
    TEST_ASSERT_EQUAL_INT(data[i++], (char) AX25_UI_PROTOCOL);

    // payload data should lives in the payload buffer unchanged
    payload_data = (char *) payload->data;
    for (; j < 3; j++) {
        TEST_ASSERT_EQUAL_INT(payload_data[j], info[j]);
    }

    // ensure extra bytes were added to the end of the payload for the fcs
    TEST_ASSERT_EQUAL_INT(payload->size, 5);

    // fcs
    TEST_ASSERT_EQUAL_INT(payload_data[3], (char) 0xB8);
    TEST_ASSERT_EQUAL_INT(payload_data[4], (char) 0xE1);
}

TestRef ax25_suite(void)
{
    EMB_UNIT_TESTFIXTURES(fixtures) {
        new_TestFixture(ax25_short_callsign),
        new_TestFixture(ax25_long_callsign),
        new_TestFixture(ax25_trimmed_callsign),
        new_TestFixture(ax25_null_addr_nocall),
        new_TestFixture(ax25_pkt_build_ui_packet),
        new_TestFixture(ax25_pkt_build_fail_cases),
    };

    EMB_UNIT_TESTCALLER(ax25_tests, NULL, NULL, fixtures);
    return (TestRef) &ax25_tests;
}
