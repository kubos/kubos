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
 * Unit tests for the MSP430 SPI bus
 *
 * Wiring:
 *  - P3.0 to SDI
 *  - P3.1 to SDO
 *  - P3.2 to SCK
 *  - CS (P2.7) to CS
 *
 * Note:
 *  Kubos-HAL doesn't currently support SPI slave mode, so all of these tests were created to be
 *  run with the SPI bus connected to a BME280 sensor.  Once slave mode is implemented, these tests
 *  should be updated to keep the setup entirely contained within the MSP430 board.
 */
#include "unity/unity.h"
#include "unity/k_test.h"
#include <string.h>

#include "kubos-hal/spi.h"
#include "kubos-hal/gpio.h"

#define CS P27
#define SPI_BUS K_SPI1

/*
 * soft_reset
 *
 * This function resets the power of the BME280 sensor.  This is important particularly for tests
 * where the configuration changes.  The sensor doesn't like it when we just change the clock rate,
 * for instance.
 */
void soft_reset(void)
{
    int ret = 0;
    uint8_t resetReg = 0xE0 & ~0x80; //Reset register, high bit low for write request
    uint8_t resetValue = 0xB6;       //Soft reset

    vTaskDelay(5);

    k_gpio_write(CS, 0);
    ret = k_spi_write(SPI_BUS, &resetReg, 1);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write reset register to SPI_BUS");
    ret = k_spi_write(SPI_BUS, &resetValue, 1);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write reset value to SPI_BUS");

    k_gpio_write(CS, 1);

    vTaskDelay(5);
}

void test_spi_setup(void)
{
    int ret = 0;

    //Initialize the chip select pin
    k_gpio_init(CS, K_GPIO_OUTPUT, K_GPIO_PULL_UP);
    k_gpio_write(CS, 1);

    //Set up the configuration parameters
    KSPI * k_spi = kprv_spi_get(SPI_BUS);
    k_spi->config.role = K_SPI_MASTER;
    k_spi->config.direction = K_SPI_DIRECTION_2LINES;
    k_spi->config.data_size = K_SPI_DATASIZE_8BIT;
    k_spi->config.speed = 10000;
    k_spi->config.clock_phase = K_SPI_CPHA_1EDGE;
    k_spi->config.clock_polarity = K_SPI_CPOL_LOW;
    k_spi->config.first_bit = K_SPI_FIRSTBIT_MSB;

    k_spi->bus_num = SPI_BUS;
    k_spi->spi_lock = xSemaphoreCreateMutex();

    //Initialize the bus
    ret = kprv_spi_dev_init(SPI_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to init SPI_BUS");

    //Soft-reset the BME280 sensor
    soft_reset();

}

/*
 * test_spi_initGood
 *
 * Purpose:  Test the base level SPI port initialization
 *
 */

static void test_spi_initGood(void)
{
    int ret;

    KSPI * k_spi = kprv_spi_get(SPI_BUS);
    k_spi->config.role = K_SPI_MASTER;
    k_spi->config.direction = K_SPI_DIRECTION_2LINES;
    k_spi->config.data_size = K_SPI_DATASIZE_8BIT;
    k_spi->config.speed = 10000;
    k_spi->config.clock_phase = K_SPI_CPHA_1EDGE;
    k_spi->config.clock_polarity = K_SPI_CPOL_LOW;
    k_spi->config.first_bit = K_SPI_FIRSTBIT_MSB;

    k_spi->bus_num = SPI_BUS;
    k_spi->spi_lock = xSemaphoreCreateMutex();

    ret = kprv_spi_dev_init(SPI_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to init SPI_BUS");

    kprv_spi_dev_terminate(SPI_BUS);

}

/*
 * test_spi_initBad
 *
 * Purpose:  Try initializing a non-existent SPI port
 *
 */

static void test_spi_initBad(void)
{
    int ret;

    ret = kprv_spi_dev_init(K_NUM_SPI+1);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_ERROR_NULL_HANDLE, ret, "Successfully initialized fake SPI port?");
}

/*
 * test_spi_termInit
 *
 * Purpose:  Test terminating a properly initialized SPI port
 *
 */

static void test_spi_termInit(void)
{
    int ret;

    test_spi_setup();

    ret = kprv_spi_dev_terminate(SPI_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to terminate SPI_BUS");
}

/*
 * test_spi_termNoninit
 *
 * Purpose:  Test terminating a SPI port that wasn't initialized
 *
 * Expectation: Should return a successful value
 *
 */

static void test_spi_termNoninit(void)
{
    int ret;

    ret = kprv_spi_dev_terminate(SPI_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to terminate SPI_BUS");
}

/*
 * test_spi_termBad
 *
 * Purpose:  Test terminating a SPI port that doesn't exist
 *
 */

static void test_spi_termBad(void)
{
    int ret;

    ret = kprv_spi_dev_terminate(K_NUM_SPI+1);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_ERROR_NULL_HANDLE, ret, "Successfully terminated fake SPI port?");
}

/*
 * test_spi_writeMaster
 *
 * Purpose:  Test writing from a properly initialized SPI port
 *
 */

static void test_spi_writeMaster(void)
{
    int ret;
    uint8_t chipReg = 0xD0; //Chip ID register
    chipReg |= 0x80; //Turn on high bit for read request

    test_spi_setup();

    k_gpio_write(CS, 0);

    ret = kprv_spi_write(SPI_BUS, &chipReg, sizeof chipReg);

    k_gpio_write(CS, 1);

    kprv_spi_dev_terminate(SPI_BUS);

    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");
}

/*
 * test_spi_writeMasterNoCS
 *
 * Purpose:  Test writing from a properly initialized SPI port without driving chip select
 *  low to select a slave device
 *
 * Expectation: Should complete successfully
 *
 */

static void test_spi_writeMasterNoCS(void)
{
    int ret;
    uint8_t chipReg = 0xD0;
    chipReg |= 0x80;

    test_spi_setup();

    ret = kprv_spi_write(SPI_BUS, &chipReg, sizeof chipReg);

    kprv_spi_dev_terminate(SPI_BUS);

    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");
}

/*
 * test_spi_writeMasterNoninit
 *
 * Purpose:  Test writing from a SPI port without initializing it first
 *
 */

static void test_spi_writeMasterNoninit(void)
{
    int ret;
    uint8_t chipReg = 0xD0;
    chipReg |= 0x80;

    k_gpio_init(CS, K_GPIO_OUTPUT, K_GPIO_PULL_UP);
    k_gpio_write(CS, 1);

    vTaskDelay(20);

    k_gpio_write(CS, 0);

    ret = kprv_spi_write(SPI_BUS, &chipReg, sizeof chipReg);

    k_gpio_write(CS, 1);

    kprv_spi_dev_terminate(SPI_BUS);

    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_ERROR_BUSY, ret, "Unexpected value returned from write");
}

/*
 * test_spi_writeMasterOverflow
 *
 * Purpose:  Test writing more bytes than the write buffer contains
 *
 */

static void test_spi_writeMasterOverflow(void)
{
    int ret;
    uint8_t chipReg = 0xD0; //Chip ID register
    chipReg |= 0x80; //Turn on high bit for read request

    test_spi_setup();

    k_gpio_write(CS, 0);

    ret = kprv_spi_write(SPI_BUS, &chipReg, 100);

    k_gpio_write(CS, 1);

    kprv_spi_dev_terminate(SPI_BUS);

    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");
}

/*
 * test_spi_readMaster
 *
 * Purpose:  Test reading from a properly initialized SPI port
 *
 * Expectation:  This test currently requests the chip ID number from the BME280 sensor. The returned
 *  value should be 0x60.
 *
 */

static void test_spi_readMaster(void)
{
    int ret;
    uint8_t id;
    uint8_t chipReg = 0xD0;
    chipReg |= 0x80;

    test_spi_setup();

    k_gpio_write(CS, 0);

    ret = kprv_spi_write(SPI_BUS, &chipReg, sizeof chipReg);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");

    ret = kprv_spi_read(SPI_BUS, &id, sizeof id);

    k_gpio_write(CS, 1);

    vTaskDelay(50);

    kprv_spi_dev_terminate(SPI_BUS);

    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to read from SPI_BUS");
    TEST_ASSERT_EQUAL_INT_MESSAGE(0x60, id, "ID incorrect");
}

/*
 * test_spi_readMasterNoCS
 *
 * Purpose:  Test reading from a properly initialized SPI port without driving chip select
 *  low to select a slave device
 *
 * Expectation: TODO
 *
 */

static void test_spi_readMasterNoCS(void)
{
    int ret;
    uint8_t id;
    uint8_t chipReg = 0xD0;
    chipReg |= 0x80;

    test_spi_setup();

    ret = kprv_spi_write(SPI_BUS, &chipReg, sizeof chipReg);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");

    ret = kprv_spi_read(SPI_BUS, &id, sizeof id);

    kprv_spi_dev_terminate(SPI_BUS);

    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to read from SPI_BUS");
    TEST_ASSERT_EQUAL_INT_MESSAGE(0xFF, id, "Read request returned unexpected value");
}

/*
 * test_spi_readMasterNoWrite
 *
 * Purpose:  Test reading from a properly initialized SPI port without writing which register to
 *  read from first
 *
 * Expectation: The read call should complete, but the slave should only give a value of 0xFF
 *  since it won't know what was supposed to be sent
 *
 */

static void test_spi_readMasterNoWrite(void)
{
    int ret;
    uint8_t id = 0;

    test_spi_setup();

    k_gpio_write(CS, 0);

    ret = kprv_spi_read(SPI_BUS, &id, sizeof id);

    k_gpio_write(CS, 1);

    kprv_spi_dev_terminate(SPI_BUS);

    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to read from SPI_BUS");
    TEST_ASSERT_EQUAL_INT_MESSAGE(0xFF, id, "Read request returned unexpected value");
}


/*
 * test_spi_readMasterOverflow
 *
 * Purpose:  Test reading more bytes than the read buffer contains
 *
 */

static void test_spi_readMasterOverflow(void)
{
    int ret;
    char buffer[100] = {0};
    uint8_t chipReg = 0xD0;
    chipReg |= 0x80;

    test_spi_setup();

    k_gpio_write(CS, 0);

    ret = kprv_spi_write(SPI_BUS, &chipReg, sizeof chipReg);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");

    ret = kprv_spi_read(SPI_BUS, buffer, 100);

    k_gpio_write(CS, 1);

    kprv_spi_dev_terminate(SPI_BUS);

    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to read from SPI_BUS");
}

/*
 * test_spi_writeReadMaster
 *
 * Purpose:  Test the write_read function from a properly initialized SPI port
 *
 * Note:  This test is not currently being called because it doesn't currently work.
 * The BME280 sensor doesn't support writeRead calls and attempting to make a call will
 * return zero and appear successful, but will cause all subsequent tests to hang.
 * TODO:  Once something capable of writeRead calls is available (like the MSP430 in slave
 * mode), fully implement this test.
 */

static void test_spi_writeReadMaster(void)
{
    int ret;
    uint8_t id = 0;
    uint8_t chipReg = 0xD0; //Chip ID register
    chipReg |= 0x80; //Turn on high bit for read request

    test_spi_setup();

    k_gpio_write(CS, 0);

    ret = kprv_spi_write_read(SPI_BUS, &chipReg, &id, sizeof chipReg);

    k_gpio_write(CS, 1);

    kprv_spi_dev_terminate(SPI_BUS);

    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Write/read failed from SPI_BUS");
}

/*
 * test_spi_slave
 *
 * Purpose:  Test running SPI connection in slave mode
 *
 * Expectation: This should fail since slave mode isn't currently supported by Kubos-HAL
 */

static void test_spi_slave(void)
{
    int ret = 0;

    k_gpio_init(CS, K_GPIO_OUTPUT, K_GPIO_PULL_UP);
    k_gpio_write(CS, 1);

    KSPI * k_spi = kprv_spi_get(SPI_BUS);
    k_spi->config.role = K_SPI_SLAVE;
    k_spi->config.direction = K_SPI_DIRECTION_2LINES;
    k_spi->config.data_size = K_SPI_DATASIZE_8BIT;
    k_spi->config.speed = 10000;

    k_spi->bus_num = SPI_BUS;
    k_spi->spi_lock = xSemaphoreCreateMutex();

    ret = kprv_spi_dev_init(SPI_BUS);

    kprv_spi_dev_terminate(SPI_BUS);

    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_ERROR_CONFIG, ret, "Failed to init SPI_BUS");
}

/*
 * test_spi_bidiMode
 *
 * Purpose:  Test SPI communication using the bidirectional mode (communicates only over MOSI line)
 *
 * Expectation:  Should return configuration error.  The MSP430 doesn't support bidirectional mode.
 *
 */

static void test_spi_bidiMode(void)
{
    int ret = 0;
    uint8_t id = 0;

    //Setup
    k_gpio_init(CS, K_GPIO_OUTPUT, K_GPIO_PULL_UP);
    k_gpio_write(CS, 1);

    KSPI * k_spi = kprv_spi_get(SPI_BUS);
    k_spi->config.role = K_SPI_MASTER;
    k_spi->config.direction = K_SPI_DIRECTION_1LINE;
    k_spi->config.data_size = K_SPI_DATASIZE_8BIT;
    k_spi->config.speed = 10000;

    k_spi->bus_num = SPI_BUS;
    k_spi->spi_lock = xSemaphoreCreateMutex();

    ret = kprv_spi_dev_init(SPI_BUS);

    kprv_spi_dev_terminate(SPI_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_ERROR_CONFIG, ret, "Failed to init SPI_BUS");
}

/*
 * test_spi_rxOnly
 *
 * Purpose:  Test SPI communication in RX-only mode
 *
 * Expectation:  Initialization should be successful, but writing should fail
 *
 */

static void test_spi_rxOnly(void)
{
    int ret;
    uint8_t id;
    uint8_t chipReg = 0xD0;
    chipReg |= 0x80;

    //Setup
    k_gpio_init(CS, K_GPIO_OUTPUT, K_GPIO_PULL_UP);
    k_gpio_write(CS, 1);

    KSPI * k_spi = kprv_spi_get(SPI_BUS);
    k_spi->config.role = K_SPI_MASTER;
    k_spi->config.direction = K_SPI_DIRECTION_2LINES_RXONLY;
    k_spi->config.data_size = K_SPI_DATASIZE_8BIT;
    k_spi->config.speed = 10000;

    k_spi->bus_num = SPI_BUS;
    k_spi->spi_lock = xSemaphoreCreateMutex();

    ret = kprv_spi_dev_init(SPI_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to init SPI_BUS");
    //End of setup

    k_gpio_write(CS, 0);

    ret = kprv_spi_write(SPI_BUS, &chipReg, 1);

    k_gpio_write(CS, 1);

    kprv_spi_dev_terminate(SPI_BUS);

    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_ERROR, ret, "Failed to write from SPI_BUS");

}

/*
 * test_spi_clock
 *
 * Purpose:  Test SPI communication with non-default clock phase and clock polarity
 *
 * Note:  BME280 sensor only supports 1edge/low and 2edge/high for clock phase/polarity.
 *   Expand to other two cases once slave mode has been implemented
 */

static void test_spi_clock(void)
{
    int ret = 0;
    uint8_t id;
    uint8_t chipReg = 0xD0;
    chipReg |= 0x80;

    //Setup
    k_gpio_init(CS, K_GPIO_OUTPUT, K_GPIO_PULL_UP);
    k_gpio_write(CS, 1);

    KSPI * k_spi = kprv_spi_get(SPI_BUS);
    k_spi->config.role = K_SPI_MASTER;
    k_spi->config.direction = K_SPI_DIRECTION_2LINES;
    k_spi->config.data_size = K_SPI_DATASIZE_8BIT;
    k_spi->config.speed = 10000;
    k_spi->config.clock_phase = K_SPI_CPHA_2EDGE;
    k_spi->config.clock_polarity = K_SPI_CPOL_HIGH;
    k_spi->config.first_bit = K_SPI_FIRSTBIT_MSB;

    k_spi->bus_num = SPI_BUS;
    k_spi->spi_lock = xSemaphoreCreateMutex();

    ret = kprv_spi_dev_init(SPI_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to init SPI_BUS");

    soft_reset();
    //End of setup

    k_gpio_write(CS, 0);

    ret = kprv_spi_write(SPI_BUS, &chipReg, sizeof chipReg);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");

    ret = kprv_spi_read(SPI_BUS, &id, sizeof id);

    k_gpio_write(CS, 1);

    soft_reset();

    kprv_spi_dev_terminate(SPI_BUS);

    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to read from SPI_BUS");
    TEST_ASSERT_EQUAL_INT_MESSAGE(0x60, id, "ID incorrect");
}

/*
 * test_spi_LSBFirst
 *
 * Purpose:  Test SPI communication sending the LSB first
 *
 */

static void test_spi_LSBFirst(void)
{
    int ret = 0;
    uint8_t id;
    uint8_t chipReg = 0xD0;
    chipReg |= 0x80;

    //Setup
    k_gpio_init(CS, K_GPIO_OUTPUT, K_GPIO_PULL_UP);
    k_gpio_write(CS, 1);

    KSPI * k_spi = kprv_spi_get(SPI_BUS);
    k_spi->config.role = K_SPI_MASTER;
    k_spi->config.direction = K_SPI_DIRECTION_2LINES;
    k_spi->config.data_size = K_SPI_DATASIZE_8BIT;
    k_spi->config.speed = 10000;
    k_spi->config.clock_phase = K_SPI_CPHA_1EDGE;
    k_spi->config.clock_polarity = K_SPI_CPOL_LOW;
    k_spi->config.first_bit = K_SPI_FIRSTBIT_LSB;

    k_spi->bus_num = SPI_BUS;
    k_spi->spi_lock = xSemaphoreCreateMutex();

    ret = kprv_spi_dev_init(SPI_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to init SPI_BUS");

    soft_reset();
    //End of setup

    k_gpio_write(CS, 0);

    ret = kprv_spi_write(SPI_BUS, &chipReg, sizeof chipReg);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");

    ret = kprv_spi_read(SPI_BUS, &id, sizeof id);

    k_gpio_write(CS, 1);

    soft_reset();

    kprv_spi_dev_terminate(SPI_BUS);

    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to read from SPI_BUS");
}

/*
 * test_spi_16bitMode
 *
 * Purpose:  Test SPI communication using 16-bit mode
 *
 * Expectation: Should return configuration error.  The MSP430 doesn't support 16-bit mode.
 *
 */

static void test_spi_16bitMode(void)
{
    int ret = 0;
    uint8_t id;
    uint8_t chipReg = 0xD0;
    chipReg |= 0x80;

    //Setup
    k_gpio_init(CS, K_GPIO_OUTPUT, K_GPIO_PULL_UP);
    k_gpio_write(CS, 1);

    KSPI * k_spi = kprv_spi_get(SPI_BUS);
    k_spi->config.role = K_SPI_MASTER;
    k_spi->config.direction = K_SPI_DIRECTION_2LINES;
    k_spi->config.data_size = K_SPI_DATASIZE_16BIT;
    k_spi->config.speed = 10000;
    k_spi->config.clock_phase = K_SPI_CPHA_2EDGE;
    k_spi->config.clock_polarity = K_SPI_CPOL_HIGH;
    k_spi->config.first_bit = K_SPI_FIRSTBIT_MSB;

    k_spi->bus_num = SPI_BUS;
    k_spi->spi_lock = xSemaphoreCreateMutex();

    ret = kprv_spi_dev_init(SPI_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_ERROR_CONFIG, ret, "Initialization returned unexpected value");
}

/*
 * test_spi_clockSpeedHigh
 *
 * Purpose:  Test SPI communication using the highest clock speed possible
 *
 * Note: This is currently limited to the BME280's max speed, 10MHz. The MSP430's max speed is 1MHz
 *  (set by SMCLK), so the actual clock speed should be forced to 1MHz.
 */
static void test_spi_clockSpeedHigh(void)
{
    int ret = 0;
    uint8_t id;
    uint8_t chipReg = 0xD0;
    chipReg |= 0x80;

    //Setup
    k_gpio_init(CS, K_GPIO_OUTPUT, K_GPIO_PULL_UP);
    k_gpio_write(CS, 1);

    KSPI * k_spi = kprv_spi_get(SPI_BUS);
    k_spi->config.role = K_SPI_MASTER;
    k_spi->config.direction = K_SPI_DIRECTION_2LINES;
    k_spi->config.data_size = K_SPI_DATASIZE_8BIT;
    k_spi->config.speed = 10000000; //10MHz, max frequency for BME280 sensor
    k_spi->config.clock_phase = K_SPI_CPHA_1EDGE;
    k_spi->config.clock_polarity = K_SPI_CPOL_LOW;
    k_spi->config.first_bit = K_SPI_FIRSTBIT_MSB;

    k_spi->bus_num = SPI_BUS;
    k_spi->spi_lock = xSemaphoreCreateMutex();

    ret = kprv_spi_dev_init(SPI_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to init SPI_BUS");

    soft_reset();
    //End of setup

    k_gpio_write(CS, 0);

    ret = kprv_spi_write(SPI_BUS, &chipReg, sizeof chipReg);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");

    ret = kprv_spi_read(SPI_BUS, &id, sizeof id);

    k_gpio_write(CS, 1);

    soft_reset();

    kprv_spi_dev_terminate(SPI_BUS);

    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to read from SPI_BUS");
    TEST_ASSERT_EQUAL_INT_MESSAGE(0x60, id, "ID incorrect");
}

/*
 * test_spi_clockSpeedLow
 *
 * Purpose:  Test SPI communication using the lowest clock speed possible
 *
 * Note: This will try to set the speed to 1Hz, but the prescaler calculation will
 *   actually end up setting it to SMCLK/256 since we cannot directly set the SPI clock
 *   speed.
 */

static void test_spi_clockSpeedLow(void)
{
    int ret = 0;
    uint8_t id;
    uint8_t chipReg = 0xD0;
    chipReg |= 0x80;

    //Setup
    k_gpio_init(CS, K_GPIO_OUTPUT, K_GPIO_PULL_UP);
    k_gpio_write(CS, 1);

    KSPI * k_spi = kprv_spi_get(SPI_BUS);
    k_spi->config.role = K_SPI_MASTER;
    k_spi->config.direction = K_SPI_DIRECTION_2LINES;
    k_spi->config.data_size = K_SPI_DATASIZE_8BIT;
    k_spi->config.speed = 1; //1Hz
    k_spi->config.clock_phase = K_SPI_CPHA_1EDGE;
    k_spi->config.clock_polarity = K_SPI_CPOL_LOW;
    k_spi->config.first_bit = K_SPI_FIRSTBIT_MSB;

    k_spi->bus_num = SPI_BUS;
    k_spi->spi_lock = xSemaphoreCreateMutex();

    ret = kprv_spi_dev_init(SPI_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to init SPI_BUS");

    soft_reset();
    //End of setup

    k_gpio_write(CS, 0);

    ret = kprv_spi_write(SPI_BUS, &chipReg, sizeof chipReg);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");

    ret = kprv_spi_read(SPI_BUS, &id, sizeof id);

    k_gpio_write(CS, 1);

    soft_reset();

    kprv_spi_dev_terminate(SPI_BUS);

    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to read from SPI_BUS");
    TEST_ASSERT_EQUAL_INT_MESSAGE(0x60, id, "ID incorrect");
}

/*
 * test_spi_clockSpeedZero
 *
 * Purpose:  Test SPI communication a clock speed of zero
 *
 * Expectation: The test should pass because internally we'll change the 0 to 1 to prevent a divide-by-zero error
 */

static void test_spi_clockSpeedZero(void)
{
    int ret = 0;
    uint8_t id;
    uint8_t chipReg = 0xD0;
    chipReg |= 0x80;

    //Setup
    k_gpio_init(CS, K_GPIO_OUTPUT, K_GPIO_PULL_UP);
    k_gpio_write(CS, 1);

    KSPI * k_spi = kprv_spi_get(SPI_BUS);
    k_spi->config.role = K_SPI_MASTER;
    k_spi->config.direction = K_SPI_DIRECTION_2LINES;
    k_spi->config.data_size = K_SPI_DATASIZE_8BIT;
    k_spi->config.speed = 0;
    k_spi->config.clock_phase = K_SPI_CPHA_1EDGE;
    k_spi->config.clock_polarity = K_SPI_CPOL_LOW;
    k_spi->config.first_bit = K_SPI_FIRSTBIT_MSB;

    k_spi->bus_num = SPI_BUS;
    k_spi->spi_lock = xSemaphoreCreateMutex();

    ret = kprv_spi_dev_init(SPI_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to init SPI_BUS");

    soft_reset();
    //End of setup

    k_gpio_write(CS, 0);

    ret = kprv_spi_write(SPI_BUS, &chipReg, sizeof chipReg);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");

    ret = kprv_spi_read(SPI_BUS, &id, sizeof id);

    k_gpio_write(CS, 1);

    soft_reset();

    kprv_spi_dev_terminate(SPI_BUS);

    TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to read from SPI_BUS");
    TEST_ASSERT_EQUAL_INT_MESSAGE(0x60, id, "ID incorrect");
}

K_TEST_MAIN() {
    UNITY_BEGIN();

    printf("\r\n---------------------------------\r\n");
    printf("MSP430 Kubos-HAL SPI Tests:\r\n");
    printf("---------------------------------\r\n");

    RUN_TEST(test_spi_initGood);
    RUN_TEST(test_spi_initBad);
    RUN_TEST(test_spi_termInit);
    RUN_TEST(test_spi_termNoninit);
    RUN_TEST(test_spi_termBad);
    RUN_TEST(test_spi_writeMaster);
    RUN_TEST(test_spi_writeMasterNoCS);
    RUN_TEST(test_spi_writeMasterNoninit);
    RUN_TEST(test_spi_writeMasterOverflow);
    RUN_TEST(test_spi_readMaster);
    RUN_TEST(test_spi_readMasterNoCS);
    RUN_TEST(test_spi_readMasterNoWrite);
    RUN_TEST(test_spi_readMasterOverflow);
    RUN_TEST(test_spi_slave);
    RUN_TEST(test_spi_bidiMode);
    RUN_TEST(test_spi_rxOnly);
    RUN_TEST(test_spi_clock);
    RUN_TEST(test_spi_LSBFirst);
    RUN_TEST(test_spi_16bitMode);
    RUN_TEST(test_spi_clockSpeedHigh);
    RUN_TEST(test_spi_clockSpeedLow);
    RUN_TEST(test_spi_clockSpeedZero);

    return UNITY_END();
}

int main(void) {

    /* Stop the watchdog. */
    WDTCTL = WDTPW + WDTHOLD;

    __enable_interrupt();

    K_TEST_RUN_MAIN();

}
