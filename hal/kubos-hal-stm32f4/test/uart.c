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

/*
 * Unit tests for the STM32F4 UARTs
 *
 * Wiring:
 * 	- PA2  to PA1  (UART2 TX to UART4 RX)
 * 	- PA0  to PA3  (UART4 TX to UART2 RX)
 *
 * Note:
 * 	No test case was created for stop bits because no errors are thrown if the stop bit configurations differ
 * 	between the two UART ports
 */
#include "unity/unity.h"
#include "unity/k_test.h"
#include <string.h>

#include "kubos-hal-stm32f4/uart.h"
#include "stm32cubef4/stm32f4xx_hal_uart.h"

static KUARTNum uartFrom;
static KUARTNum uartTo;

//UART Tests

/*
 * test_uart_initGood
 *
 * Purpose:  Test the base level UART port initialization
 *
 */

static void test_uart_initGood(void)
{
    int ret;

    ret = kprv_uart_dev_init(uartTo);

    kprv_uart_dev_terminate(uartTo);
    TEST_ASSERT_EQUAL_INT_MESSAGE(UART_OK, ret, "Failed to init UART1");
}

/*
 * test_uart_initBad
 *
 * Purpose:  Test UART port number validation during initialization
 *
 */
static void test_uart_initBad(void)
{
    KUARTNum num;
    int ret;

    num = K_NUM_UARTS; //Load invalid UART number

    ret = kprv_uart_dev_init(num);

    TEST_ASSERT_EQUAL_INT(UART_ERROR_NULL_HANDLE, ret);
}

/*
 * test_uart_write
 *
 * Purpose:  Test writing out of the UART port
 *
 */

static void test_uart_write(void)
{
    KUARTConf conf;
    char * testString = "test string 1";
    int len = strlen(testString);
    int returnLen = 0;

    conf = k_uart_conf_defaults();

    k_uart_init(uartTo, &conf);
    returnLen = k_uart_write(uartTo, testString, len);

    k_uart_terminate(uartTo);
    TEST_ASSERT_EQUAL_INT_MESSAGE(len, returnLen, "Failed to write");
}

/*
 * test_uart_writeOverflow
 *
 * Purpose:  Test writing more bytes than the write buffer contains
 *
 */

static void test_uart_writeOverflow(void)
{
    KUARTConf conf;
    char * testString = "test string 1";
    char buffer[100];
    int len = strlen(testString);
    int returnLen = 0;

    conf = k_uart_conf_defaults();

    k_uart_init(uartTo, &conf);
    k_uart_init(uartFrom, &conf);
    returnLen = k_uart_write(uartTo, testString, 50);

    //Clean up the receive buffer
    while(k_uart_read(uartFrom, buffer, 100) != 0);
    k_uart_terminate(uartTo);
    k_uart_terminate(uartFrom);

    TEST_ASSERT_EQUAL_INT_MESSAGE(50, returnLen, "Failed to write");


}

/*
 * test_uart_read
 *
 * Purpose:  Test reading from each UART port
 *
 */

static void test_uart_read(void)
{
    KUARTConf conf;
    char * testString = "test string 1";
    int len = strlen(testString);
    int returnLenWrite = 0;
    int returnLenRead = 0;

    conf = k_uart_conf_defaults();

    TEST_ASSERT_FALSE(k_uart_init(uartTo, &conf));
    TEST_ASSERT_FALSE(k_uart_init(uartFrom, &conf));

    returnLenWrite = k_uart_write(uartFrom, testString, len);

    vTaskDelay(50);

    returnLenRead = k_uart_read(uartTo, testString, len);

    k_uart_terminate(uartTo);
    k_uart_terminate(uartFrom);
    TEST_ASSERT_EQUAL_INT_MESSAGE(len, returnLenWrite, "Failed to write");
    TEST_ASSERT_EQUAL_INT_MESSAGE(len, returnLenRead, "Failed to read");
}

/*
 * test_uart_readOverflow
 *
 * Purpose:  Test reading more bytes than the read buffer contains
 *
 * Expectation: The read call should only read as many bytes as are available and then return
 * 	which will be the length of testString.
 *
 */

static void test_uart_readOverflow(void)
{
    KUARTConf conf;
    KUART *k_uart;
    char * testString = "test string 1";
    char buffer[100] = {0};
    int len = strlen(testString);
    int returnLenWrite = 0;
    int returnLenRead = 0;

    conf = k_uart_conf_defaults();

    TEST_ASSERT_FALSE(k_uart_init(uartTo, &conf));
    TEST_ASSERT_FALSE(k_uart_init(uartFrom, &conf));

    returnLenWrite = k_uart_write(uartFrom, testString, len);

    vTaskDelay(50);

    returnLenRead = k_uart_read(uartTo, buffer, 100);

    k_uart_terminate(uartTo);
    k_uart_terminate(uartFrom);
    TEST_ASSERT_EQUAL_INT_MESSAGE(len, returnLenWrite, "Failed to write");
    TEST_ASSERT_EQUAL_INT_MESSAGE(len, returnLenRead, "Failed to read");
}

/*
 * test_uart_writeImmediate
 *
 * Purpose:  Test the write_immediate function, which sends a single character directly out the UART port,
 * 	bypassing the send queue.
 *
 */

static void test_uart_writeImmediate(void)
{
    KUARTConf conf;
    USART_TypeDef *dev2 = uart_dev(uartTo);
    USART_TypeDef *dev4 = uart_dev(uartFrom);
    char recvString[100];
    int len = 100;
    int returnLen = 0;

    conf = k_uart_conf_defaults();
    TEST_ASSERT_FALSE(k_uart_init(uartTo, &conf));
    TEST_ASSERT_FALSE(k_uart_init(uartFrom, &conf));

    k_uart_write_immediate(uartTo,'a');

    vTaskDelay(50);

    returnLen = k_uart_read(uartFrom, recvString, len);

    k_uart_terminate(uartTo);
    k_uart_terminate(uartFrom);
    TEST_ASSERT_EQUAL_MEMORY("a", recvString, 1);
    TEST_ASSERT_EQUAL_INT(1, returnLen);
}

/*
 * test_uart_wordLen
 *
 * Purpose:  Test UART communication when the word length configurations are mismatched
 *
 * Expectation: The message should be discarded and uart_read should return 0 bytes
 *
 */

static void test_uart_wordLen(void)
{
    KUARTConf conf;
    KUART *k_uart;
    char * testString = "test string 1";
    int len = strlen(testString);
    int returnLenWrite = 0;
    int returnLenRead = 0;

    conf = k_uart_conf_defaults();

    conf.word_len = K_WORD_LEN_8BIT;
    TEST_ASSERT_FALSE(k_uart_init(uartTo, &conf));

    conf.word_len = K_WORD_LEN_9BIT;
    TEST_ASSERT_FALSE(k_uart_init(uartFrom, &conf));

    returnLenWrite = k_uart_write(uartFrom, testString, len);

    vTaskDelay(50);

    returnLenRead = k_uart_read(uartTo, testString, len);

    k_uart_terminate(uartTo);
    k_uart_terminate(uartFrom);
    TEST_ASSERT_EQUAL_INT_MESSAGE(len, returnLenWrite, "Failed to write");
    TEST_ASSERT_EQUAL_INT_MESSAGE(0, returnLenRead, "Should have received 0 bytes");
}

/*
 * test_uart_parity
 *
 * Purpose:  Test UART communication when the parity bit configurations are mismatched
 *
 * Expectation: The message should be discarded and uart_read should return 0 bytes
 *
 * Note:  Don't use K_PARITY_NONE for either of them.  If you do, then you'll successfully write/read the string
 */
static void test_uart_parity(void)
{
    KUARTConf conf;
    KUART *k_uart;
    char * testString = "test string 123456789012345678";
    int len = strlen(testString);
    int returnLenWrite = 0;
    int returnLenRead = 0;

    conf = k_uart_conf_defaults();

    conf.parity = K_PARITY_ODD;
    TEST_ASSERT_FALSE(k_uart_init(uartTo, &conf));

    conf.parity = K_PARITY_EVEN;
    TEST_ASSERT_FALSE(k_uart_init(uartFrom, &conf));

    returnLenWrite = k_uart_write(uartFrom, testString, len);

    vTaskDelay(50);

    returnLenRead = k_uart_read(uartTo, testString, len);

    k_uart_terminate(uartTo);
    k_uart_terminate(uartFrom);
    TEST_ASSERT_EQUAL_INT_MESSAGE(len, returnLenWrite, "Failed to write");
    TEST_ASSERT_EQUAL_INT_MESSAGE(0, returnLenRead, "Should have received 0 bytes");
}

/*
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
    int returnLenWrite = 0;
    int returnLenRead = 0;

    conf = k_uart_conf_defaults();

    conf.baud_rate = 115200;
    TEST_ASSERT_FALSE(k_uart_init(uartTo, &conf));

    conf.baud_rate = 9600;
    TEST_ASSERT_FALSE(k_uart_init(uartFrom, &conf));

    returnLenWrite = k_uart_write(uartFrom, testString, len);

    vTaskDelay(50);

    returnLenRead = k_uart_read(uartTo, testString, len);

    k_uart_terminate(uartTo);
    k_uart_terminate(uartFrom);
    TEST_ASSERT_EQUAL_INT_MESSAGE(len, returnLenWrite, "Failed to write");
    TEST_ASSERT_EQUAL_INT_MESSAGE(0, returnLenRead, "Should have received 0 bytes");
}

/*
 * test_uart_overrun
 *
 * Purpose:  Test UART receive processing when a character is received before the previous character has
 * 	been processed.
 *
 * Expectation: The overrun characters should be discarded and uart_read should return 0 bytes
 *
 */

static void test_uart_overrun(void)
{
    USART_TypeDef *dev = uart_dev(uartTo);
    UART_HandleTypeDef u = { .Instance = uart_dev(uartTo) };

    KUARTConf conf;
    KUART *k_uart;
    char * testString = "test string 1";
    int len = strlen(testString);
    int returnLenWrite = 0;
    int returnLenRead = 0;

    conf = k_uart_conf_defaults();

    TEST_ASSERT_FALSE(k_uart_init(uartTo, &conf));

    TEST_ASSERT_FALSE(k_uart_init(uartFrom, &conf));

    HAL_NVIC_DisableIRQ(uart_irqn(uartTo));
    returnLenWrite = k_uart_write(uartFrom, testString, len);

    vTaskDelay(50);

    HAL_NVIC_EnableIRQ(uart_irqn(uartTo));

    vTaskDelay(50);

    returnLenRead = k_uart_read(uartTo, testString, len);

    k_uart_terminate(uartTo);
    k_uart_terminate(uartFrom);
    TEST_ASSERT_EQUAL_INT_MESSAGE(len, returnLenWrite, "Failed to write");
    TEST_ASSERT_EQUAL_INT_MESSAGE(0, returnLenRead, "Should have received 0 bytes");
}


K_TEST_MAIN() {
    UNITY_BEGIN();

    printf("\r\n---------------------------------\r\n");
    printf("STM32F4 Kubos-HAL Uart Tests:\r\n");
    printf("---------------------------------\r\n");

    uartTo = K_UART2;
    uartFrom = K_UART4;

    RUN_TEST(test_uart_initGood);
    RUN_TEST(test_uart_initBad);
    RUN_TEST(test_uart_write);
    RUN_TEST(test_uart_writeOverflow);
    RUN_TEST(test_uart_read);
    RUN_TEST(test_uart_readOverflow);
    RUN_TEST(test_uart_writeImmediate);
    RUN_TEST(test_uart_wordLen);
    RUN_TEST(test_uart_parity);
    RUN_TEST(test_uart_baudRate);
    RUN_TEST(test_uart_overrun);

    return UNITY_END();
}

int main(void) {

    K_TEST_RUN_MAIN();

}
