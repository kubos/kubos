/*
 * KubOS Core Flight Services
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

/**
 * Unit tests for the MSP430 UARTs
 *
 * Wiring:
 *  - P3.4 -> MTK3339 Rx
 *  - P3.3 -> MTK3339 Tx
 *
 * Note:
 *  The MSP430 only has two available UART ports.  In order to reserve one for console output, these tests were
 *  created to be run against a MTK3339 GPS sensor breakout board.
 *
 */
#include "unity/unity.h"
#include "unity/k_test.h"
#include <string.h>

#include "kubos-hal-msp430f5529/uart.h"
#include "msp430f5529-hal/uart.h"

#define uart_bus K_UART1

//UART Tests

/**
 * test_uart_initGood
 *
 * Purpose:  Test the base level UART port initialization
 *
 */

static void test_uart_initGood(void)
{
    int ret;

    ret = kprv_uart_dev_init(uart_bus);

    k_uart_terminate(uart_bus);
    TEST_ASSERT_EQUAL_INT_MESSAGE(0, ret, "Failed to init UART1");
}

/**
 * test_uart_initBad
 *
 * Purpose:  Test UART port number validation during initialization
 *
 */
static void test_uart_initBad(void)
{
    int ret;

    ret = kprv_uart_dev_init(K_NUM_UARTS+1);

    TEST_ASSERT_EQUAL_INT(UART_ERROR_NULL_HANDLE, ret);
}

/**
 * test_uart_write
 *
 * Purpose:  Test writing out of the UART port
 *
 */

static void test_uart_write(void)
{
    KUARTConf conf;
    char *testString = "$PMTK000*32\r\n"; //MTK3339 test command
    int len = strlen(testString);
    int returnLen = 0;

    conf = (KUARTConf) {
        .baud_rate = 9600,
        .word_len = K_WORD_LEN_8BIT,
        .stop_bits = K_STOP_BITS_1,
        .parity = K_PARITY_NONE,
        .rx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_RXQUEUELEN,
        .tx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_TXQUEUELEN,
    };

    k_uart_init(uart_bus, &conf);
    returnLen = k_uart_write(uart_bus, testString, len);

    k_uart_terminate(uart_bus);
    TEST_ASSERT_EQUAL_INT_MESSAGE(len, returnLen, "Failed to write");
}

/**
 * test_uart_writeOverflow
 *
 * Purpose:  Test writing more bytes than the write buffer contains
 *
 */

static void test_uart_writeOverflow(void)
{
    KUARTConf conf;
    char * testString = "$PMTK000*32\r\n";
    int len = strlen(testString);
    int returnLen = 0;

    conf = (KUARTConf) {
        .baud_rate = 9600,
        .word_len = K_WORD_LEN_8BIT,
        .stop_bits = K_STOP_BITS_1,
        .parity = K_PARITY_NONE,
        .rx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_RXQUEUELEN,
        .tx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_TXQUEUELEN,
    };

    k_uart_init(uart_bus, &conf);

    returnLen = k_uart_write(uart_bus, testString, 20);

    k_uart_terminate(uart_bus);

    TEST_ASSERT_EQUAL_INT_MESSAGE(20, returnLen, "Failed to write");


}


/**
 * test_uart_writeImmediate
 *
 * Purpose:  Test the write_immediate function, which sends a single character directly out the UART port,
 *  bypassing the send queue.
 *
 */

static void test_uart_writeImmediate(void)
{
    KUARTConf conf;
    int ret = 0;

    conf = (KUARTConf) {
        .baud_rate = 9600,
        .word_len = K_WORD_LEN_8BIT,
        .stop_bits = K_STOP_BITS_1,
        .parity = K_PARITY_NONE,
        .rx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_RXQUEUELEN,
        .tx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_TXQUEUELEN,
    };

    TEST_ASSERT_FALSE(k_uart_init(uart_bus, &conf));

    ret = k_uart_write_immediate(uart_bus,'a');

    k_uart_terminate(uart_bus);
    TEST_ASSERT_EQUAL_INT(UART_OK, ret);
}

/**
 * test_uart_writeImmediateStr
 *
 * Purpose:  Test the write_immediate_str function, which sends a string directly out the UART port,
 *  bypassing the send queue.
 *
 */

static void test_uart_writeImmediateStr(void)
{
    KUARTConf conf;
    char *testString = "$PMTK000*32\r\n"; //Empty test command
    int ret = 0;

    conf = (KUARTConf) {
        .baud_rate = 9600,
        .word_len = K_WORD_LEN_8BIT,
        .stop_bits = K_STOP_BITS_1,
        .parity = K_PARITY_NONE,
        .rx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_RXQUEUELEN,
        .tx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_TXQUEUELEN,
    };

    TEST_ASSERT_FALSE(k_uart_init(uart_bus, &conf));

    ret = k_uart_write_immediate_str(uart_bus, testString, strlen(testString));

    k_uart_terminate(uart_bus);
    TEST_ASSERT_EQUAL_INT(UART_OK, ret);
}

/**
 * test_uart_read
 *
 * Purpose:  Test reading from the UART port
 *
 * Note:  Ideally we'd check the characters that were read to make sure they're what we were
 *   expecting.  However, the MTK3339 is constantly sending data, so we can't be sure what data
 *   exactly we're reading in.
 *
 */

static void test_uart_read(void)
{
    KUARTConf conf;
    KUART *k_uart;
    char recvString[10] = {0};
    int returnLenRead = 0;

    conf = (KUARTConf) {
        .baud_rate = 9600,
        .word_len = K_WORD_LEN_8BIT,
        .stop_bits = K_STOP_BITS_1,
        .parity = K_PARITY_NONE,
        .rx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_RXQUEUELEN,
        .tx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_TXQUEUELEN,
    };

    TEST_ASSERT_FALSE(k_uart_init(uart_bus, &conf));

    vTaskDelay(1100); //wait for there to be things in the buffer

    returnLenRead = k_uart_read(uart_bus, recvString, sizeof recvString);

    k_uart_terminate(uart_bus);

    TEST_ASSERT_EQUAL_INT_MESSAGE(sizeof recvString, returnLenRead, "Failed to read");
}

/**
 * test_uart_readOverflow
 *
 * Purpose:  Test reading more bytes than the read buffer contains
 *
 * Expectation: The read call should only read as many bytes as are available and then return
 *  which will be the length of testString.
 *
 * TODO:  The MTK3339 GPS sensor continually sends data, so the receive buffer will always be full.
 *   Start using this test case if there's another device that can be used to test with that doesn't
 *   always send data.
 *
 */

static void test_uart_readOverflow(void)
{
    KUARTConf conf;
    KUART *k_uart;
    char * testString = "test string 1";
    char buffer[100] = {0};
    int len = strlen(testString);
    int returnLenRead = 0;

    conf = (KUARTConf) {
        .baud_rate = 9600,
        .word_len = K_WORD_LEN_8BIT,
        .stop_bits = K_STOP_BITS_1,
        .parity = K_PARITY_NONE,
        .rx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_RXQUEUELEN,
        .tx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_TXQUEUELEN,
    };

    TEST_ASSERT_FALSE(k_uart_init(uart_bus, &conf));

    vTaskDelay(50);

    returnLenRead = k_uart_read(uart_bus, buffer, 100);

    k_uart_terminate(uart_bus);

    TEST_ASSERT_EQUAL_INT_MESSAGE(len, returnLenRead, "Failed to read");
}


/**
 * test_uart_wordLen7
 *
 * Purpose:  Test UART communication when the word length configurations are mismatched
 *
 * Expectation: The message should be discarded and uart_read should return 0 bytes
 *
 */

static void test_uart_wordLen7(void)
{
    KUARTConf conf;
    KUART *k_uart;
    char * testString = "test string 1";
    int len = strlen(testString);
    int returnLenRead = 0;

    conf = (KUARTConf) {
        .baud_rate = 9600,
        .word_len = K_WORD_LEN_7BIT,
        .stop_bits = K_STOP_BITS_1,
        .parity = K_PARITY_NONE,
        .rx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_RXQUEUELEN,
        .tx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_TXQUEUELEN,
    };

    TEST_ASSERT_FALSE(k_uart_init(uart_bus, &conf));

    vTaskDelay(1100);

    returnLenRead = k_uart_read(uart_bus, testString, len);

    k_uart_terminate(uart_bus);
    TEST_ASSERT_EQUAL_INT_MESSAGE(0, returnLenRead, "Should have received 0 bytes");
}

/**
 * test_uart_wordLen9
 *
 * Purpose:  Test UART communication with an unsupported word length
 *
 * Expectation: The initialization should fail
 *
 *TODO: FIX ME
 */

static void test_uart_wordLen9(void)
{
    int ret = 0;
    KUARTConf conf;

    conf = (KUARTConf) {
        .baud_rate = 9600,
        .word_len = K_WORD_LEN_9BIT,
        .stop_bits = K_STOP_BITS_1,
        .parity = K_PARITY_NONE,
        .rx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_RXQUEUELEN,
        .tx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_TXQUEUELEN,
    };

    ret = k_uart_init(uart_bus, &conf);
    TEST_ASSERT_EQUAL_INT(UART_ERROR_CONFIG, ret);
}

/**
 * test_uart_parity
 *
 * Purpose:  Test UART communication when the parity bit configurations are mismatched
 *
 * Expectation: The message should be discarded and uart_read should return 0 bytes
 *
 * Note:  The MTK3339 is configured with parity none.  As a result, we'll occasionally
 *   receive messages that look like they have the correct parity.  For this test, we're
 *   looking to reject at least some of the bytes we're trying to read.  If this test is
 *   ever updated to have an endpoint with a parity other than none, the pass condition
 *   can be updated to be exactly zero bytes read.
 *
 */
static void test_uart_parity(void)
{
    KUARTConf conf;
    KUART *k_uart;
    char * testString = "test string 123456789012345678";
    int len = strlen(testString);
    int returnLenRead = 0;

    conf = (KUARTConf) {
        .baud_rate = 9600,
        .word_len = K_WORD_LEN_8BIT,
        .stop_bits = K_STOP_BITS_1,
        .parity = K_PARITY_EVEN,
        .rx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_RXQUEUELEN,
        .tx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_TXQUEUELEN,
    };

    TEST_ASSERT_FALSE(k_uart_init(uart_bus, &conf));

    vTaskDelay(1100);

    returnLenRead = k_uart_read(uart_bus, testString, len);

    k_uart_terminate(uart_bus);
    TEST_ASSERT_INT_WITHIN_MESSAGE((len-1),0, returnLenRead, "Should have received < 30 bytes");
}

/**
 * test_uart_stopbits
 *
 * Purpose:  Test UART communication when the parity bit configurations are mismatched
 *
 * Expectation: The message should be discarded and uart_read should return 0 bytes
 *
 * Note:  Detecting stop bits isn't perfect.  Sometimes the next bit just happens to look correct.
 *   As a result, we're looking to just receive fewer bytes than we requested.
 *
 */
static void test_uart_stopbits(void)
{
    KUARTConf conf;
    KUART *k_uart;
    char * testString = "test string 123456789012345678";
    int len = strlen(testString);
    int returnLenRead = 0;

    conf = (KUARTConf) {
        .baud_rate = 9600,
        .word_len = K_WORD_LEN_8BIT,
        .stop_bits = K_STOP_BITS_2,
        .parity = K_PARITY_NONE,
        .rx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_RXQUEUELEN,
        .tx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_TXQUEUELEN,
    };

    TEST_ASSERT_FALSE(k_uart_init(uart_bus, &conf));

    vTaskDelay(1100);

    returnLenRead = k_uart_read(uart_bus, testString, len);

    k_uart_terminate(uart_bus);
    TEST_ASSERT_INT_WITHIN_MESSAGE((len-1),0, returnLenRead, "Should have received < 30 bytes");
}

/**
 * test_uart_baudRate
 *
 * Purpose:  Test UART communication when the baud rate configurations are mismatched
 *
 * Expectation: The message should be discarded and uart_read should return 0 bytes
 *
 */

static void test_uart_baudRate(void)
{
    KUARTConf conf;
    KUART *k_uart;
    char * testString = "test string 123456789012345678";
    int len = strlen(testString);
    int returnLenRead = 0;

    conf = (KUARTConf) {
        .baud_rate = 115200,
        .word_len = K_WORD_LEN_8BIT,
        .stop_bits = K_STOP_BITS_1,
        .parity = K_PARITY_NONE,
        .rx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_RXQUEUELEN,
        .tx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_TXQUEUELEN,
    };

    TEST_ASSERT_FALSE(k_uart_init(uart_bus, &conf));

    vTaskDelay(1000);

    returnLenRead = k_uart_read(uart_bus, testString, len);

    k_uart_terminate(uart_bus);
    TEST_ASSERT_EQUAL_INT_MESSAGE(0, returnLenRead, "Should have received 0 bytes");
}

/**
 * test_uart_overrun
 *
 * Purpose:  Test UART receive processing when a character is received before the previous character has
 *  been processed.
 *
 * Expectation: The overrun characters should be discarded and uart_read should return 0 bytes
 *
 */

static void test_uart_overrun(void)
{
    hal_uart_handle *handle = uart_handle(uart_bus);

    KUARTConf conf;
    KUART *k_uart;
    char * testString = "test string 1";
    int len = strlen(testString);
    int returnLenRead = 0;

    conf = (KUARTConf) {
        .baud_rate = 9600,
        .word_len = K_WORD_LEN_8BIT,
        .stop_bits = K_STOP_BITS_1,
        .parity = K_PARITY_NONE,
        .rx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_RXQUEUELEN,
        .tx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_TXQUEUELEN,
    };

    TEST_ASSERT_FALSE(k_uart_init(uart_bus, &conf));

    handle->reg->interruptEnable &= ~UCRXIE; //Disable receive interrupt

    vTaskDelay(1000); //Make sure we're trying to receive something

    handle->reg->interruptEnable |= UCRXIE; //Enable receive interrupt

    returnLenRead = k_uart_read(uart_bus, testString, len);

    k_uart_terminate(uart_bus);
    TEST_ASSERT_EQUAL_INT_MESSAGE(0, returnLenRead, "Should have received 0 bytes");
}

/*
 * Note:  Added vTaskDelay between each test because without them the MSP430 will sometimes lock up between
 * passing one test and starting the next.
 */
K_TEST_MAIN() {
    UNITY_BEGIN();

    printf("\r\n---------------------------------\r\n");
    printf("MSP430 Kubos-HAL Uart Tests:\r\n");
    printf("---------------------------------\r\n");

    RUN_TEST(test_uart_initGood);
    vTaskDelay(10);
    RUN_TEST(test_uart_initBad);
    vTaskDelay(10);
    RUN_TEST(test_uart_write);
    vTaskDelay(10);
    RUN_TEST(test_uart_writeOverflow);
    vTaskDelay(10);
    RUN_TEST(test_uart_writeImmediate);
    vTaskDelay(10);
    RUN_TEST(test_uart_writeImmediateStr);
    vTaskDelay(10);
    RUN_TEST(test_uart_read);
    vTaskDelay(10);
    RUN_TEST(test_uart_wordLen7);
    vTaskDelay(10);
    RUN_TEST(test_uart_wordLen9);
    vTaskDelay(10);
    RUN_TEST(test_uart_parity);
    vTaskDelay(10);
    RUN_TEST(test_uart_stopbits);
    vTaskDelay(10);
    RUN_TEST(test_uart_baudRate);
    vTaskDelay(10);
    RUN_TEST(test_uart_overrun);

    return UNITY_END();
}

int main(void) {

    /* Stop the watchdog. */
    WDTCTL = WDTPW + WDTHOLD;

    __enable_interrupt();

    K_TEST_RUN_MAIN();

}
