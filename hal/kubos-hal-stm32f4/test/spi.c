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
 * Unit tests for the STM32F4 SPI bus
 *
 * Wiring:
 * 	- PA7 to SDI
 * 	- PA6 to SDO
 * 	- PA5 to SCK
 * 	- CS to CS
 *
 * Note:
 * 	Kubos-HAL doesn't currently support SPI slave mode, so all of these tests were created to be
 * 	run with the SPI bus connected to a BME280 sensor.  Once slave mode is implemented, these tests
 * 	should be updated to keep the setup entirely contained within the STM32F4 board.
 */
#include "unity/unity.h"
#include "unity/k_test.h"
#include <string.h>

#include "kubos-hal-stm32f4/spi.h"
#include "kubos-hal/gpio.h"

#define CS PA4
#define SPI_BUS K_SPI1

void test_spi_setup(void)
{
	int ret = 0;
    uint8_t resetReg = 0xE0 & ~0x80; //Reset register, high bit low for write request
    uint8_t resetValue = 0xB6;       //Soft reset

	k_gpio_init(CS, K_GPIO_OUTPUT, K_GPIO_PULL_UP);
	k_gpio_write(CS, 1);

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

	//Soft-reset the BME280 sensor
	k_gpio_write(CS, 0);  //drive CS low
	ret = k_spi_write(SPI_BUS, &resetReg, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write chipReg SPI_BUS");
	ret = k_spi_write(SPI_BUS, &resetValue, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write resetValue SPI_BUS");

	k_gpio_write(CS, 1);  //drive CS high

	vTaskDelay(5);

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
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_ERROR, ret, "Successfully initialized fake SPI port?");
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
 * test_spi_termNoninit
 *
 * Purpose:  Test terminating a SPI port that doesn't exist
 *
 */

static void test_spi_termBad(void)
{
	int ret;

	ret = kprv_spi_dev_terminate(K_NUM_SPI+1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_ERROR, ret, "Successfully terminated fake SPI port?");
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

	ret = kprv_spi_write(SPI_BUS, &chipReg, 1);

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

	ret = kprv_spi_write(SPI_BUS, &chipReg, 1);

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

	ret = kprv_spi_write(SPI_BUS, &chipReg, 1);

	k_gpio_write(CS, 1);

	kprv_spi_dev_terminate(SPI_BUS);

	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_ERROR_TIMEOUT, ret, "Unexpected value returned from write");
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
 * 	value should be 0x60.
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

	ret = kprv_spi_write(SPI_BUS, &chipReg, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");

	ret = kprv_spi_read(SPI_BUS, &id, sizeof id);

	k_gpio_write(CS, 1);

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
 * Expectation: The read call should complete, but the slave should only give a value of 0xFF
 *  since it won't know what was supposed to be sent
 *
 */

static void test_spi_readMasterNoCS(void)
{
	int ret;
	uint8_t id;
	uint8_t chipReg = 0xD0;
	chipReg |= 0x80;

	test_spi_setup();

	ret = kprv_spi_write(SPI_BUS, &chipReg, 1);
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
 * test_spi_writeReadMaster
 *
 * Purpose:  Test the write_read function from a properly initialized SPI port
 *
 * Note:  Once slave mode has been implemented on the STM32F4, create a more robust test of the
 *   write_read function. For now we just want to know that it won't crash the system if we try
 *   to talk to a device that doesn't support it, like the BME280
 */

static void test_spi_writeReadMaster(void)
{
	int ret;
	uint8_t id = 0;
	uint8_t buffer[1] = {0xD0};

	test_spi_setup();

	k_gpio_write(CS, 0);

	ret = kprv_spi_write_read(SPI_BUS, buffer, &id, sizeof buffer);

	k_gpio_write(CS, 1);

	kprv_spi_dev_terminate(SPI_BUS);

	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Write/read failed from SPI_BUS");
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

	ret = kprv_spi_write(SPI_BUS, &chipReg, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");

	ret = kprv_spi_read(SPI_BUS, buffer, 100);

	k_gpio_write(CS, 1);

	kprv_spi_dev_terminate(SPI_BUS);

	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to read from SPI_BUS");
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

	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_ERROR, ret, "Failed to init SPI_BUS");
}

/*
 * test_spi_bidiMode
 *
 * Purpose:  Test SPI communication using the bidirectional mode (communicates only over MOSI line)
 *
 */

static void test_spi_bidiMode(void)
{
	int ret = 0;
	uint8_t id = 0;
	uint8_t chipReg = 0xD0; //Chip ID register
	chipReg |= 0x80; //Turn on high bit

    uint8_t resetReg = 0xE0 & ~0x80; //Reset register
    uint8_t resetValue = 0xB6;		 //Soft reset

    uint8_t configReg = 0xF5 &~0x80; //Configuration register
    uint8_t configValue = 0x01;      //3-wire mode


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
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to init SPI_BUS");

	//Soft-reset the BME280 sensor
	k_gpio_write(CS, 0);
	ret = k_spi_write(SPI_BUS, &resetReg, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write reset register to SPI_BUS");
	ret = k_spi_write(SPI_BUS, &resetValue, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write reset value to SPI_BUS");

	k_gpio_write(CS, 1);

	vTaskDelay(20);

	//Put the sensor in 3-wire mode
	k_gpio_write(CS, 0);
	ret = k_spi_write(SPI_BUS, &configReg, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write config register SPI_BUS");
	ret = k_spi_write(SPI_BUS, &configValue, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write config value SPI_BUS");

	k_gpio_write(CS, 1);

	vTaskDelay(20);

	k_gpio_write(CS, 0);

	//Read the chip ID value
	ret = kprv_spi_write(SPI_BUS, &chipReg, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");

	ret = kprv_spi_read(SPI_BUS, &id, sizeof id);

	k_gpio_write(CS, 1);

	kprv_spi_dev_terminate(SPI_BUS);

	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to read from SPI_BUS");
	TEST_ASSERT_EQUAL_INT_MESSAGE(0x60, id, "ID incorrect");
}

/*
 * test_spi_rxOnly
 *
 * Purpose:  Test SPI communication in RX-only mode
 *
 * Expectation:  Initialization should be successful, but writing should timeout
 *
 */

static void test_spi_rxOnly(void)
{
	int ret;
	uint8_t id;
	uint8_t chipReg = 0xD0;
	chipReg |= 0x80;
    uint8_t resetReg = 0xE0 & ~0x80;
    uint8_t resetValue = 0xB6;

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

	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_ERROR_TIMEOUT, ret, "Failed to write from SPI_BUS");

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
    uint8_t resetReg = 0xE0 & ~0x80;
    uint8_t resetValue = 0xB6;
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

	//Soft-reset the BME280 sensor
	k_gpio_write(CS, 0);
	ret = k_spi_write(SPI_BUS, &resetReg, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write reset register to SPI_BUS");
	ret = k_spi_write(SPI_BUS, &resetValue, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write reset value to SPI_BUS");

	k_gpio_write(CS, 1);

	vTaskDelay(20);
	//End of setup

	k_gpio_write(CS, 0);

	ret = kprv_spi_write(SPI_BUS, &chipReg, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");

	ret = kprv_spi_read(SPI_BUS, &id, sizeof id);

	k_gpio_write(CS, 1);

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
    uint8_t resetReg = 0xE0 & ~0x80;
    uint8_t resetValue = 0xB6;
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

	//Soft-reset the BME280 sensor
	k_gpio_write(CS, 0);
	ret = k_spi_write(SPI_BUS, &resetReg, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write reset register SPI_BUS");
	ret = k_spi_write(SPI_BUS, &resetValue, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write reset value SPI_BUS");

	k_gpio_write(CS, 1);

	vTaskDelay(20);
	//End of setup

	k_gpio_write(CS, 0);

	ret = kprv_spi_write(SPI_BUS, &chipReg, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");

	ret = kprv_spi_read(SPI_BUS, &id, sizeof id);

	k_gpio_write(CS, 1);

	kprv_spi_dev_terminate(SPI_BUS);

	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to read from SPI_BUS");
}

/*
 * test_spi_16bitMode
 *
 * Purpose:  Test SPI communication using 16-bit mode
 *
 */

static void test_spi_16bitMode(void)
{
	int ret = 0;
    uint8_t resetReg = 0xE0 & ~0x80;
    uint8_t resetValue = 0xB6;
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
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to init SPI_BUS");

	//Soft-reset the BME280 sensor
	k_gpio_write(CS, 0);
	ret = k_spi_write(SPI_BUS, &resetReg, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write reset register SPI_BUS");
	ret = k_spi_write(SPI_BUS, &resetValue, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write reset value SPI_BUS");

	k_gpio_write(CS, 1);

	vTaskDelay(20);
	//End of setup

	k_gpio_write(CS, 0);

	ret = kprv_spi_write(SPI_BUS, &chipReg, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");

	ret = kprv_spi_read(SPI_BUS, &id, sizeof id);

	k_gpio_write(CS, 1);

	kprv_spi_dev_terminate(SPI_BUS);

	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to read from SPI_BUS");
}

/*
 * test_spi_clockSpeedHigh
 *
 * Purpose:  Test SPI communication using the highest clock speed possible
 *
 * Note: This is currently limited to the BME280's max speed, 10MHz. Test
 *   absolute max frequency (42MHz using SPI1) once slave mode has been implemented
 */
static void test_spi_clockSpeedHigh(void)
{
	int ret = 0;
    uint8_t resetReg = 0xE0 & ~0x80;
    uint8_t resetValue = 0xB6;
	uint8_t id;
	uint8_t chipReg = 0xD0;
	chipReg |= 0x80;
	uint32_t freq = 0;

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

	//Soft-reset the BME280 sensor
	k_gpio_write(CS, 0);
	ret = k_spi_write(SPI_BUS, &resetReg, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write reset register SPI_BUS");
	ret = k_spi_write(SPI_BUS, &resetValue, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write reset value SPI_BUS");

	k_gpio_write(CS, 1);

	vTaskDelay(20);
	//End of setup

	k_gpio_write(CS, 0);

	ret = kprv_spi_write(SPI_BUS, &chipReg, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");

	ret = kprv_spi_read(SPI_BUS, &id, sizeof id);

	k_gpio_write(CS, 1);

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
 *   actually end up setting it to PCLK/256 since we cannot directly set the SPI clock
 *   speed.
 */

static void test_spi_clockSpeedLow(void)
{
	int ret = 0;
    uint8_t resetReg = 0xE0 & ~0x80;
    uint8_t resetValue = 0xB6;
	uint8_t id;
	uint8_t chipReg = 0xD0;
	chipReg |= 0x80;
	uint32_t freq = 0;

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

	//Soft-reset the BME280 sensor
	k_gpio_write(CS, 0);
	ret = k_spi_write(SPI_BUS, &resetReg, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write reset register SPI_BUS");
	ret = k_spi_write(SPI_BUS, &resetValue, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write reset value SPI_BUS");

	k_gpio_write(CS, 1);

	vTaskDelay(20);
	//End of setup

	k_gpio_write(CS, 0);

	ret = kprv_spi_write(SPI_BUS, &chipReg, 1);
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to write from SPI_BUS");

	ret = kprv_spi_read(SPI_BUS, &id, sizeof id);

	k_gpio_write(CS, 1);

	kprv_spi_dev_terminate(SPI_BUS);

	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_OK, ret, "Failed to read from SPI_BUS");
	TEST_ASSERT_EQUAL_INT_MESSAGE(0x60, id, "ID incorrect");
}

/*
 * test_spi_clockSpeedZero
 *
 * Purpose:  Test SPI communication a clock speed of zero
 *
 * Expectation:  The initialization should fail
 */

static void test_spi_clockSpeedZero(void)
{
	int ret = 0;
    uint8_t resetReg = 0xE0 & ~0x80;
    uint8_t resetValue = 0xB6;
	uint8_t id;
	uint8_t chipReg = 0xD0;
	chipReg |= 0x80;
	uint32_t freq = 0;

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
	TEST_ASSERT_EQUAL_INT_MESSAGE(SPI_ERROR, ret, "Failed to init SPI_BUS");
}

K_TEST_MAIN() {
    UNITY_BEGIN();

    printf("\r\n---------------------------------\r\n");
    printf("STM32F4 Kubos-HAL SPI Tests:\r\n");
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
    RUN_TEST(test_spi_writeReadMaster);
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

    K_TEST_RUN_MAIN();

}
