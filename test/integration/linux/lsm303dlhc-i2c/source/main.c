/*
 * KubOS Linux
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
#include <errno.h>
#include <string.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>
#include <linux/i2c-dev.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/ioctl.h>
#include <fcntl.h>

#define LSM303DLHC_ADDRESS_A (0x19)
#define LSM303DLHC_NAME "LSM303DLHC"

#define TO_STR(x) UNWRAP(x)
#define UNWRAP(x) #x

#define CREATE_CMD "echo " LSM303DLHC_NAME " " TO_STR(LSM303DLHC_ADDRESS_A) " > /sys/bus/i2c/devices/i2c-0/new_device"
#define DESTROY_CMD "echo " TO_STR(LSM303DLHC_ADDRESS_A) " > /sys/bus/i2c/devices/i2c-0/delete_device"

#define msleep(time_ms) usleep(time_ms * 1000);

#define POWER_DOWN 0x00
#define SPEED_1Hz 0x10
#define SPEED_10Hz 0x20
#define SPEED_25Hz 0x30
#define SPEED_50Hz 0x40
#define SPEED_100Hz 0x50
#define SPEED_200Hz 0x60
#define SPEED_400Hz 0x70
#define SPEED_L1_6kHz 0x80
#define SPEED_N1_3kHz_L5_3kHz 0x90

#define NORMAL_POWER 0x00
#define LOW_POWER 0x08

#define X_AXIS_ENABLE 0x01
#define Y_AXIS_ENABLE 0x02
#define Z_AXIS_ENABLE 0x04


typedef enum
{
    CTRL_REG1_A = 0x20
} LSM303DLHC_reg_t;

static int write_byte(int file, LSM303DLHC_reg_t reg, uint8_t value)
{
    /* Write buffer: reg and write value */
    uint8_t buffer[2] = {(uint8_t)reg, value};

    /* Transmit reg and value */
    if (write(file, buffer, sizeof(buffer)) != sizeof(buffer))
    {
		printf("Write failed. RC=%s\n", strerror(errno));
        return -1;
    }

    return 0;
}

static int read_byte(int file, LSM303DLHC_reg_t reg, uint8_t* value)
{
    if (value == NULL)
    {
        return -1;
    }
    /* Transmit reg */
    if (write(file, (uint8_t*)&reg, 1) != 1)
    {
		printf("Write (read) failed. RC=%s\n", strerror(errno));
        return -1;
    }
    msleep(5);

    /* Receive value */
    if (read(file, value, 1) != 1)
    {
		printf("Read failed. RC=%s\n", strerror(errno));
        return -1;
    }
    return 0;
}

int init_sensor(int file) {
	
    /* Prep power/frequency mode */
    uint8_t mode = SPEED_100Hz + NORMAL_POWER + X_AXIS_ENABLE + Y_AXIS_ENABLE + Z_AXIS_ENABLE;
	
    int ret = -1;

    /* Set the requested operating mode */
    if ((ret = write_byte(file, CTRL_REG1_A, mode)) != 0)
    {
        return ret;
    }
    msleep(20);
	
	/* Fetch the operating mode */
    if ((ret = read_byte(file, CTRL_REG1_A, &mode)) != 0)
    {
        return ret;
    }
    msleep(20);

    return ret;
}

int main(void)
{

    int file;
    char filename[20];
	
	//Define the peripheral to the system
	system(CREATE_CMD);

    /*
     * The iOBC only has one i2c bus, so this will always be i2c-0
     * If a future board has more than one i2c bus, change this to take
     * an input parameter
     */
	snprintf(filename, 19, "/dev/i2c-0");
	
	file = open(filename, O_RDWR);
	
	if (file < 0) {
		printf("Couldn't open /dev/i2c-0. RC=%s\n", strerror(errno));
		system(DESTROY_CMD);
		exit(1);
	}
	
	/* Set the desired slave's address */
	if (ioctl(file, I2C_SLAVE, LSM303DLHC_ADDRESS_A) < 0) {
		printf("Couldn't reach address %x. RC=%s\n", LSM303DLHC_ADDRESS_A, strerror(errno));
		close(file);
		system(DESTROY_CMD);
		exit(-1);
	}
	
	if (init_sensor(file) != 0)
	{
		printf("Something went wrong\n");
		close(file);
		system(DESTROY_CMD);
		exit(1);
	}
	
	printf("LSM303DLHC I2C test completed successfully!\n");
	close(file);
	system(DESTROY_CMD);

    return 0;
}
