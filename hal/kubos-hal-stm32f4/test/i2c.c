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
 * Unit tests for the STM32F4 I2C bus
 *
 * Wiring:
 * 	- PB6 to SCL
 * 	- PB7 to SDA
 * 	- 3V to Vin
 * 	- Gnd to Gnd
 *
 * Note:
 * 	Kubos-HAL doesn't currently support I2C slave mode, so all of these tests were created to be
 * 	run with the I2C bus connected to a BNO055 sensor.  Once slave mode is implemented, these tests
 * 	should be updated to keep the setup entirely contained within the STM32F4 board.
 */
#include "unity/unity.h"
#include "unity/k_test.h"
#include <string.h>

#include "kubos-hal/i2c.h"

#define I2C_BUS K_I2C1
#define BNO055_ADDRESS_A (0x28)
#define BNO055_CHIP_ID_ADDR (0x00)
#define BNO055_ID (0xA0)

#define NUM_BNO055_OFFSET_REGISTERS (22)

//Establishes configuration and initialization for the tests
void test_i2c_setup(void)
{
	int ret;

	KI2CConf conf = {
		.addressing_mode = K_ADDRESSINGMODE_7BIT,
		.role = K_MASTER,
		.clock_speed = 100000
	};

	KI2C *k_i2c = kprv_i2c_get(I2C_BUS);
	memcpy(&k_i2c->conf, &conf, sizeof(KI2CConf));
	k_i2c->bus_num = I2C_BUS;
	k_i2c->i2c_lock = xSemaphoreCreateMutex();

	ret = kprv_i2c_dev_init(I2C_BUS);
	TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Failed to init I2C_BUS");
}

/*
 * test_i2c_initGood
 *
 * Purpose:  Test the base level I2C port initialization
 *
 */

static void test_i2c_initGood(void)
{
    int ret;

    KI2CConf conf = {
        .addressing_mode = K_ADDRESSINGMODE_7BIT,
        .role = K_MASTER,
        .clock_speed = 100000
    };

    KI2C *k_i2c = kprv_i2c_get(I2C_BUS);
    memcpy(&k_i2c->conf, &conf, sizeof(KI2CConf));
    k_i2c->bus_num = I2C_BUS;
    k_i2c->i2c_lock = xSemaphoreCreateMutex();

    ret = kprv_i2c_dev_init(I2C_BUS);
    kprv_i2c_dev_terminate(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Failed to init I2C_BUS");

}

/*
 * test_i2c_initBad
 *
 * Purpose:  Test initializing a fake I2C port
 *
 */

static void test_i2c_initBad(void)
{
    int ret;

    ret = kprv_i2c_dev_init(K_NUM_I2CS+1);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_ERROR, ret, "Successfully initialized fake I2C port?");

}

/*
 * test_i2c_termInit
 *
 * Purpose:  Test terminating a port that wasn't initialized
 *
 * Expectation: Nothing is returned except for a null-handle check, but the system at least shouldn't crash
 *
 */

static void test_i2c_termInit(void)
{
    int ret;

    KI2CConf conf = {
        .addressing_mode = K_ADDRESSINGMODE_7BIT,
        .role = K_MASTER,
        .clock_speed = 100000
    };

    KI2C *k_i2c = kprv_i2c_get(I2C_BUS);
    memcpy(&k_i2c->conf, &conf, sizeof(KI2CConf));
    k_i2c->bus_num = I2C_BUS;
    k_i2c->i2c_lock = xSemaphoreCreateMutex();

    ret = kprv_i2c_dev_init(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Failed to init I2C_BUS");

    ret = kprv_i2c_dev_terminate(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Failed to term I2C_BUS");
}

/*
 * test_i2c_termNoninit
 *
 * Purpose:  Test terminating a port that wasn't initialized
 *
 * Expectation: Nothing is returned except for a null-handle check, but the system at least shouldn't crash
 *
 */

static void test_i2c_termNoninit(void)
{
    int ret;

    ret = kprv_i2c_dev_terminate(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Failed to term I2C_BUS");
}

/*
 * test_i2c_termBad
 *
 * Purpose:  Test terminating a fake I2C port
 *
 */

static void test_i2c_termBad(void)
{
    int ret;

    ret = kprv_i2c_dev_terminate(K_NUM_I2CS+1);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_ERROR_NULL_HANDLE, ret, "Successfully terminated fake I2C port?");

}

/*
 * test_i2c_writeMasterGood
 *
 * Purpose:  Test writing to slave address
 *
 */

static void test_i2c_writeMasterGood(void)
{
    int ret;
    uint8_t buffer[2] = {(uint8_t)61, 0x00}; //cmd (0x3D): Set bno055 sensor to config mode (0x00)

	test_i2c_setup();

    //Send request for data
    ret = kprv_i2c_master_write(I2C_BUS, BNO055_ADDRESS_A, (uint8_t*)buffer, 2);

    kprv_i2c_dev_terminate(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Write failed");
}

/*
 * test_i2c_writeMasterBad
 *
 * Purpose:  Test writing to slave address that doesn't exist
 *
 */

static void test_i2c_writeMasterBad(void)
{
    int ret;
    uint8_t cmd = 0xE3;

    ret = kprv_i2c_dev_init(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Failed to init I2C_BUS");

    ret = kprv_i2c_master_write(I2C_BUS, 0x80, &cmd, sizeof cmd);
    kprv_i2c_dev_terminate(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_ERROR_AF, ret, "Was expecting I2C_ERROR_AF");
}

/*
 * test_i2c_writeMasterOverflow
 *
 * Purpose:  Test writing more bytes than the write buffer contains
 *
 */

static void test_i2c_writeMasterOverflow(void)
{
    int ret;
    uint8_t buffer[2] = {(uint8_t)61, 0x00}; //cmd (0x3D): Set bno055 sensor to config mode (0x00)

	test_i2c_setup();

    //Send request for data
    ret = kprv_i2c_master_write(I2C_BUS, BNO055_ADDRESS_A, (uint8_t*)buffer, 200);

    kprv_i2c_dev_terminate(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Write failed");
}

/*
 * test_i2c_readMasterGood
 *
 * Purpose:  Test reading from slave address
 *
 * The write is requesting the chip ID number from the bno055 sensor.  The returned value should be 0xA0
 *
 */

static void test_i2c_readMasterGood(void)
{
	int ret;
	uint8_t id;
	uint8_t reg = BNO055_CHIP_ID_ADDR;

	test_i2c_setup();

    //Send request for data
    ret = kprv_i2c_master_write(I2C_BUS, BNO055_ADDRESS_A, (uint8_t*)&reg, 1);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Write failed");

    vTaskDelay(5);

    ret = kprv_i2c_master_read(I2C_BUS, BNO055_ADDRESS_A, &id, 1);

	TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Read failed");
	TEST_ASSERT_EQUAL_INT_MESSAGE(BNO055_ID, id, "ID incorrect");

}

/*
 * test_i2c_readMasterBad
 *
 * Purpose:  Test reading from slave address that doesn't exist
 *
 */

static void test_i2c_readMasterBad(void)
{
    int ret;
    uint8_t value = 0;

    test_i2c_setup();

	ret = kprv_i2c_master_read(I2C_BUS, 0x80, &value, 1);

    kprv_i2c_dev_terminate(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_ERROR_AF, ret, "Read returned unexpected value");
}

/*
 * test_i2c_readMasterNoWrite
 *
 * Purpose:  Test reading from slave address without having written the register to read from
 *
 * The read should return the value from either the last requested register, or register 0 (chip ID)
 *
 */

static void test_i2c_readMasterNoWrite(void)
{
    int ret;
    uint8_t value = 0;

    test_i2c_setup();

	ret = kprv_i2c_master_read(I2C_BUS, BNO055_ADDRESS_A, &value, 1);

    kprv_i2c_dev_terminate(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Read returned unexpected value");
}

/*
 * test_i2c_readMasterOverflow
 *
 * Purpose:  Test reading more bytes than the read buffer contains
 *
 */

static void test_i2c_readMasterOverflow(void)
{
	int ret;
	char buffer[100] = {0};
	uint8_t reg = BNO055_CHIP_ID_ADDR;

	test_i2c_setup();

    //Send request for data
    ret = kprv_i2c_master_write(I2C_BUS, BNO055_ADDRESS_A, (uint8_t*)&reg, 1);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Write failed");

    vTaskDelay(5);

    ret = kprv_i2c_master_read(I2C_BUS, BNO055_ADDRESS_A, buffer, sizeof buffer);

	TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Read failed");

}

/*
 * test_i2c_addrModeRead
 *
 * Purpose:  Test reading from slave address in 10-bit addressing mode (normally 7-bit)
 *
 * Expectation: The bno055 sensor only supports 7-bit mode, so this should fail
 *
 * Note:  Once slave mode is supported, a valid 10-bit test case should be created
 */

static void test_i2c_addrModeRead(void)
{
    int ret;

    uint8_t value = 0;

    //Set up I2C master port configuration
    KI2CConf conf = {
        .addressing_mode = K_ADDRESSINGMODE_10BIT,
        .role = K_MASTER,
        .clock_speed = 100000
    };

    KI2C *k_i2c = kprv_i2c_get(I2C_BUS);
    memcpy(&k_i2c->conf, &conf, sizeof(KI2CConf));
    k_i2c->bus_num = I2C_BUS;
    k_i2c->i2c_lock = xSemaphoreCreateMutex();

    ret = kprv_i2c_dev_init(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Failed to init I2C_BUS");

	ret = kprv_i2c_master_read(I2C_BUS, BNO055_ADDRESS_A, &value, 1);

    kprv_i2c_dev_terminate(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_ERROR_ADDR_TIMEOUT, ret, "Read returned unexpected value");
}

/*
 * test_i2c_addrModeWrite
 *
 * Purpose:  Test writing to slave address in 10-bit addressing mode (normally 7-bit)
 *
 * Expectation: The bno055 sensor only supports 7-bit mode, so this should fail
 *
 * Note:  Once slave mode is supported, a valid 10-bit test case should be created
 */

static void test_i2c_addrModeWrite(void)
{
    int ret;
    uint8_t buffer[2] = {(uint8_t)61, 0x00};

    //Set up I2C master port configuration
    KI2CConf conf = {
        .addressing_mode = K_ADDRESSINGMODE_10BIT,
        .role = K_MASTER,
        .clock_speed = 100000
    };

    KI2C *k_i2c = kprv_i2c_get(I2C_BUS);
    memcpy(&k_i2c->conf, &conf, sizeof(KI2CConf));
    k_i2c->bus_num = I2C_BUS;
    k_i2c->i2c_lock = xSemaphoreCreateMutex();

    ret = kprv_i2c_dev_init(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Failed to init I2C_BUS");

    ret = kprv_i2c_master_write(I2C_BUS, BNO055_ADDRESS_A, buffer, 2);

    kprv_i2c_dev_terminate(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_ERROR_ADDR_TIMEOUT, ret, "Write returned unexpected value");
}

/*
 * test_i2c_slave
 *
 * Purpose:  Test running I2C connection in slave mode
 *
 * Expectation: This should fail since slave mode isn't currently supported by Kubos-HAL
 */

static void test_i2c_slave(void)
{
    int ret;

    //Set up I2C master port configuration
    KI2CConf conf = {
        .addressing_mode = K_ADDRESSINGMODE_7BIT,
        .role = K_SLAVE,
        .clock_speed = 100000
    };

    KI2C *k_i2c = kprv_i2c_get(I2C_BUS);
    memcpy(&k_i2c->conf, &conf, sizeof(KI2CConf));
    k_i2c->bus_num = I2C_BUS;
    k_i2c->i2c_lock = xSemaphoreCreateMutex();

    ret = kprv_i2c_dev_init(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_ERROR, ret, "Successfully inititalized I2C_BUS?");
}

/*
 * test_i2c_clockHigh
 *
 * Purpose:  Test I2C communication when the clock speed is above the max limit (400k)
 *
 * Expectation: The speed should be lowered to the max and the test should complete successfully
 */

static void test_i2c_clockHigh(void)
{
    int ret;

    uint8_t buffer[2] = {(uint8_t)61, 0x00};
    uint8_t value;

    //Set up I2C master port configuration
    KI2CConf conf = {
        .addressing_mode = K_ADDRESSINGMODE_7BIT,
        .role = K_MASTER,
        .clock_speed = 500000
    };

    KI2C *k_i2c = kprv_i2c_get(I2C_BUS);
    memcpy(&k_i2c->conf, &conf, sizeof(KI2CConf));
    k_i2c->bus_num = I2C_BUS;
    k_i2c->i2c_lock = xSemaphoreCreateMutex();

    ret = kprv_i2c_dev_init(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Failed to init I2C_BUS");

    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, kprv_i2c_master_write(I2C_BUS, BNO055_ADDRESS_A, buffer, 2),
    		"Failed to write from I2C_BUS");

	vTaskDelay(5);

	/* receive value */
	ret = kprv_i2c_master_read(I2C_BUS, BNO055_ADDRESS_A, &value, 1);


    kprv_i2c_dev_terminate(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Read returned unexpected value");
}

/*
 * test_i2c_clockLow
 *
 * Purpose:  Test I2C communication when the clock speed is at the minimum (1)
 *
 * Expectation: The test should complete successfully
 */

static void test_i2c_clockLow(void)
{
    int ret;

    uint8_t buffer[2] = {(uint8_t)61, 0x00};
    uint8_t value;

    KI2CConf conf = {
        .addressing_mode = K_ADDRESSINGMODE_7BIT,
        .role = K_MASTER,
        .clock_speed = 1
    };

    KI2C *k_i2c = kprv_i2c_get(I2C_BUS);
    memcpy(&k_i2c->conf, &conf, sizeof(KI2CConf));
    k_i2c->bus_num = I2C_BUS;
    k_i2c->i2c_lock = xSemaphoreCreateMutex();

    ret = kprv_i2c_dev_init(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Failed to init I2C_BUS");

    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, kprv_i2c_master_write(I2C_BUS, BNO055_ADDRESS_A, buffer, 2),
    		"Failed to write from I2C_BUS");

	vTaskDelay(5);

	ret = kprv_i2c_master_read(I2C_BUS, BNO055_ADDRESS_A, &value, 1);


    kprv_i2c_dev_terminate(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Read returned unexpected value");
}

/*
 * test_i2c_clockZero
 *
 * Purpose:  Test I2C communication when the clock speed is zero
 *
 * Expectation: The test should fail due to timeout
 */

static void test_i2c_clockZero(void)
{
    int ret;

    uint8_t buffer[2] = {(uint8_t)61, 0x00};;
    uint8_t value;

    //Set up I2C master port configuration
    KI2CConf conf = {
        .addressing_mode = K_ADDRESSINGMODE_7BIT,
        .role = K_MASTER,
        .clock_speed = 0
    };

    KI2C *k_i2c = kprv_i2c_get(I2C_BUS);
    memcpy(&k_i2c->conf, &conf, sizeof(KI2CConf));
    k_i2c->bus_num = I2C_BUS;
    k_i2c->i2c_lock = xSemaphoreCreateMutex();

    ret = kprv_i2c_dev_init(I2C_BUS);
    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_OK, ret, "Failed to init I2C_BUS");

    TEST_ASSERT_EQUAL_INT_MESSAGE(I2C_ERROR_ADDR_TIMEOUT, kprv_i2c_master_write(I2C_BUS, BNO055_ADDRESS_A, buffer, 2),
    		"Write returned unexpected value");

    kprv_i2c_dev_terminate(I2C_BUS);
}

K_TEST_MAIN() {
    UNITY_BEGIN();

    printf("\r\n---------------------------------\r\n");
    printf("STM32F4 Kubos-HAL I2C Tests:\r\n");
    printf("---------------------------------\r\n");

    RUN_TEST(test_i2c_initGood);
    RUN_TEST(test_i2c_initBad);
    RUN_TEST(test_i2c_termInit);
    RUN_TEST(test_i2c_termNoninit);
    RUN_TEST(test_i2c_termBad);
    RUN_TEST(test_i2c_writeMasterGood);
    RUN_TEST(test_i2c_writeMasterBad);
    RUN_TEST(test_i2c_writeMasterOverflow);
    RUN_TEST(test_i2c_readMasterGood);
    RUN_TEST(test_i2c_readMasterBad);
    RUN_TEST(test_i2c_readMasterNoWrite);
    RUN_TEST(test_i2c_readMasterOverflow);
    RUN_TEST(test_i2c_addrModeWrite);
    RUN_TEST(test_i2c_addrModeRead);
    RUN_TEST(test_i2c_slave);
    RUN_TEST(test_i2c_clockHigh);
    RUN_TEST(test_i2c_clockLow);
    RUN_TEST(test_i2c_clockZero);

    return UNITY_END();
}

int main(void) {

    K_TEST_RUN_MAIN();

}
