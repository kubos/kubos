/*
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

#include "kubos-hal-iobc/supervisor.h"
#include "kubos-hal-iobc/checksum.h"
#include <fcntl.h>
#include <linux/spi/spidev.h>
#include <stdio.h>
#include <string.h>
#include <sys/ioctl.h>
#include <unistd.h>

#define SPI_DEV "/dev/spidev0.2"

/** Emergency Reset in hexadecimal. */
#define CMD_SUPERVISOR_EMERGENCY_RESET 0x45
/** Reset Command in hexadecimal. */
#define CMD_SUPERVISOR_RESET 0xAA
/** Write Output Control Command in hexadecimal. */
#define CMD_SUPERVISOR_WRITE_OUTPUT_CTRL 0xA0
/** Power Cycle IOBC Command in hexadecimal. */
#define CMD_SUPERVISOR_POWER_CYCLE_IOBC 0xA1
/** Obtain Housekeeping telemetry Command in hexadecimal. */
#define CMD_SUPERVISOR_OBTAIN_HK_TELEMETRY 0xB0
/** Obtain Version and Configuration Command in hexadecimal. */
#define CMD_SUPERVISOR_OBTAIN_VERSION_CONFIG 0x55

static bool spi_comms(const uint8_t * tx_buffer, uint8_t * rx_buffer, uint16_t tx_length)
{
    int fd, ret;
    static uint32_t speed = 1000000;

    if ((tx_buffer == NULL) || (rx_buffer == NULL))
    {
        return false;
    }

    char checksum = supervisor_calculate_CRC(tx_buffer, tx_length - 1);

    fd = open(SPI_DEV, O_RDWR);
    if (fd < 0) {
        perror("Can't open device ");
        return false;
    }

    /*
     * Setting SPI bus speed
     */
    ret = ioctl(fd, SPI_IOC_WR_MAX_SPEED_HZ, &speed);
    if (ret == -1) {
        perror("Can't set max speed hz");
        return false;
    }

    /**
     * Messages are sent across one byte per ioctl call
     * This is to introduce inter-byte delays, as per
     * discussion with ISIS on 3/31. They suggested
     * at least 1 ms between bytes.
     */
    for (uint16_t i = 0; i < tx_length - 1; i++)
    {
        struct spi_ioc_transfer tr = {
            .tx_buf = (unsigned long)&tx_buffer[i],
            .rx_buf = (unsigned long)&rx_buffer[i],
            .len = 1,
            .delay_usecs = 0,
            .cs_change = 1
        };
        ret = ioctl(fd, SPI_IOC_MESSAGE(1), &tr);
        if (ret < 1)
        {
            perror("Can't send spi message ");
            return false;
        }
        usleep(1000);
    }

    /**
     * Send checksum last
     */
    struct spi_ioc_transfer tr = {
        .tx_buf = (unsigned long)&checksum,
        .rx_buf = (unsigned long)&rx_buffer[tx_length - 1],
        .len = 1,
        .delay_usecs = 0,
        .cs_change = 1
    };
    ret = ioctl(fd, SPI_IOC_MESSAGE(1), &tr);
    if (ret < 1)
    {
        perror("Can't send spi message ");
        return false;
    }

    close(fd);

    return true;
}

static bool verify_checksum(const uint8_t * buffer, int buffer_length)
{
    uint8_t checksum = supervisor_calculate_CRC(buffer + 1, buffer_length - 2);
    return true ? (checksum == buffer[buffer_length - 1]) : false;
}

bool supervisor_get_version(supervisor_version_t * version)
{
    uint8_t bytesToSendSampleVersion[LENGTH_TELEMETRY_SAMPLE_VERSION] = { CMD_SUPERVISOR_OBTAIN_VERSION_CONFIG, 0x00, 0x00 };
    uint8_t bytesToReceiveSampleVersion[LENGTH_TELEMETRY_SAMPLE_VERSION] = { CMD_SUPERVISOR_OBTAIN_VERSION_CONFIG, 0x00, 0x00 };
    uint8_t bytesToSendObtainVersion[LENGTH_TELEMETRY_GET_VERSION] = { 0 };
    uint8_t bytesToReceiveObtainVersion[LENGTH_TELEMETRY_GET_VERSION] = { 0 };

    if (!spi_comms(bytesToSendSampleVersion, bytesToReceiveSampleVersion, LENGTH_TELEMETRY_SAMPLE_VERSION))
    {
        printf("Failed to sample version\n");
        return false;
    }

    usleep(10000);

    if (!spi_comms(bytesToSendObtainVersion, bytesToReceiveObtainVersion, LENGTH_TELEMETRY_GET_VERSION))
    {
        printf("Failed to obtain version\n");
        return false;
    }

    if (!verify_checksum(bytesToReceiveObtainVersion, LENGTH_TELEMETRY_GET_VERSION))
    {
        printf("Checksum failed\n");
        return false;
    }

    memcpy(version, bytesToReceiveObtainVersion, LENGTH_TELEMETRY_GET_VERSION);

    return true;
}

bool supervisor_get_housekeeping(supervisor_housekeeping_t * housekeeping)
{
    uint8_t bytesToSendSampleHousekeepingTelemetry[LENGTH_TELEMETRY_SAMPLE_HOUSEKEEPING] = { CMD_SUPERVISOR_OBTAIN_HK_TELEMETRY, 0x00, 0x00 };
    uint8_t bytesToReceiveSampleHousekeepingTelemetry[LENGTH_TELEMETRY_SAMPLE_HOUSEKEEPING] = { CMD_SUPERVISOR_OBTAIN_HK_TELEMETRY, 0x00, 0x00 };
    uint8_t bytesToSendObtainHousekeepingTelemetry[LENGTH_TELEMETRY_HOUSEKEEPING] = { 0 };
    uint8_t bytesToReceiveObtainHousekeepingTelemetry[LENGTH_TELEMETRY_HOUSEKEEPING] = { 0 };

    if (!spi_comms(bytesToSendSampleHousekeepingTelemetry, bytesToReceiveSampleHousekeepingTelemetry, LENGTH_TELEMETRY_SAMPLE_HOUSEKEEPING))
    {
        printf("Failed to sample housekeeping\n");
        return false;
    }

    usleep(10000);

    if (!spi_comms(bytesToSendObtainHousekeepingTelemetry, bytesToReceiveObtainHousekeepingTelemetry, LENGTH_TELEMETRY_HOUSEKEEPING))
    {
        printf("Failed to obtain housekeeping\n");
        return false;
    }

    if (!verify_checksum(bytesToReceiveObtainHousekeepingTelemetry, LENGTH_TELEMETRY_HOUSEKEEPING))
    {
        printf("Checksum failed\n");
        return false;
    }

    memcpy(housekeeping, bytesToReceiveObtainHousekeepingTelemetry, LENGTH_TELEMETRY_HOUSEKEEPING);

    return true;
}

bool supervisor_powercycle()
{
    uint8_t bytesToSendPowerCycleIobc[LENGTH_POWER_CYCLE_IOBC] = { CMD_SUPERVISOR_POWER_CYCLE_IOBC, 0x00, 0x00 };
    uint8_t bytesToReceivePowerCycleIobc[LENGTH_POWER_CYCLE_IOBC] = { 0 };

    if (!spi_comms(bytesToSendPowerCycleIobc, bytesToReceivePowerCycleIobc, LENGTH_POWER_CYCLE_IOBC))
    {
        printf("Failed to send power cycle\n");
        return false;
    }
    return true;
}

bool supervisor_reset()
{
    uint8_t bytesToSendReset[LENGTH_RESET] = { CMD_SUPERVISOR_RESET, 0x00, 0x00 };
    uint8_t bytesToReceiveReset[LENGTH_RESET] = { 0 };

    if (!spi_comms(bytesToSendReset, bytesToReceiveReset, LENGTH_RESET))
    {
        printf("Failed to send reset\n");
        return false;
    }
    return true;
}

bool supervisor_emergency_reset()
{
    uint8_t bytesToSendEmergencyReset[LENGTH_EMERGENCY_RESET] = { CMD_SUPERVISOR_EMERGENCY_RESET, 'M', 'E', 'R', 'G', 'E', 'N', 'C', 'Y', 0x00 };
    uint8_t bytesToReceiveEmergencyReset[LENGTH_EMERGENCY_RESET] = { 0 };

    if (!spi_comms(bytesToSendEmergencyReset, bytesToReceiveEmergencyReset, LENGTH_EMERGENCY_RESET))
    {
        printf("Failed to send emergency reset\n");
        return false;
    }
    return true;
}