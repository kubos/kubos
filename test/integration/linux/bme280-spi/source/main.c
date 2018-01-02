/*
 * KubOS Linux
 * Copyright (C) 2017 Kubos Corporation
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

#include <fcntl.h>
#include <linux/spi/spidev.h>
#include <stdint.h>
#include <stdio.h>
#include <sys/ioctl.h>
#include <time.h>
#include <unistd.h>

#define BME280_REGISTER_CHIPID    0xD0
#define BME280_REGISTER_SOFTRESET 0xE0

uint8_t chip_select;

static int spi_comms(uint8_t * tx_buffer, uint32_t tx_length,
                     uint8_t * rx_buffer, uint8_t rx_length)
{
    int             i, fd, ret;
    static uint8_t  mode  = SPI_MODE_0;
    static uint8_t  bits  = 8;
    static uint32_t speed = 1000000;
    static uint16_t delay = 0;

    if ((tx_buffer == NULL) || (rx_buffer == NULL))
    {
        return -2;
    }

    char spi_dev[] = "/dev/spidev1.n";
    sprintf(spi_dev, "/dev/spidev1.%d", chip_select);

    fd = open(spi_dev, O_RDWR);
    if (fd < 0)
    {
        perror("Can't open SPI device");
        return -1;
    }

    struct spi_ioc_transfer tr = {
        .tx_buf        = (unsigned long) tx_buffer,
        .rx_buf        = (unsigned long) rx_buffer,
        .len           = tx_length,
        .speed_hz      = speed,
        .bits_per_word = bits,
        .cs_change     = 0,
        .delay_usecs   = delay,
    };

    ret = ioctl(fd, SPI_IOC_MESSAGE(1), &tr);
    if (ret < 0)
    {
        perror("Failed to send SPI message");
        return -1;
    }

    close(fd);

    return 0;
}

static int write_byte(uint8_t reg, uint8_t value)
{
    uint8_t tx_buf[2] = { 0 };
    uint8_t rx_buf[2] = { 0 }; /* Dummy rx buffer */

    uint8_t shift_reg = reg & ~0x80; /* write, bit 7 low */
    tx_buf[0]         = shift_reg;
    tx_buf[1]         = value;

    return spi_comms(tx_buf, 2, rx_buf, 2);
}

static int read_byte(uint8_t reg, uint8_t * value)
{
    uint8_t tx_buf[2] = { 0 };
    uint8_t rx_buf[2] = { 0 };

    uint8_t shift_reg = reg | 0x80; /* read, bit 7 high */
    tx_buf[0]         = shift_reg;

    if (spi_comms(tx_buf, 2, rx_buf, 2) != 0)
    {
        return -1;
    }

    *value = rx_buf[1];

    return 0;
}

int main(int argc, char * argv[])
{
    const struct timespec delay = {.tv_sec = 0, .tv_nsec = 50000 };

    /* Get the chip select to use for this test */
    if (argc == 2)
    {
        chip_select = argv[1][0] - '0';
    }
    else
    {
        chip_select = 0;
    }

    /* Do soft reset of chip to initialize it */
    if (write_byte(BME280_REGISTER_SOFTRESET, 0xB6) != 0)
    {
        fprintf(stderr, "Couldn't send soft reset\n");
        return -1;
    }
    nanosleep(&delay, NULL);

    /* Get chip ID to verify successfully communication */
    int     timeout = 20;
    uint8_t chip_id = 0;
    read_byte(BME280_REGISTER_CHIPID, &chip_id);

    /*
     * Allow up to 1 millisecond for the chip to
     * initialize successfully and respond to our
     * request
     */
    while (chip_id != 0x60)
    {
        nanosleep(&delay, NULL);

        read_byte(BME280_REGISTER_CHIPID, &chip_id);

        if (timeout <= 0)
        {
            fprintf(stderr, "Timed out while trying to get chipid\n");
            return -3;
        }
        timeout--;
    }

    printf("BME280 SPI test completed successfully!\n");

    return 0;
}
