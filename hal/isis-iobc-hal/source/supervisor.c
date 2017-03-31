#include "isis-iobc-hal/supervisor.h"
#include "isis-iobc-hal/checksum.h"
#include <fcntl.h>
#include <linux/spi/spidev.h>
#include <stdio.h>
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

static bool spi_comms(uint8_t * tx_buffer, uint32_t tx_length, uint8_t * rx_buffer, uint8_t rx_length)
{
    int fd, ret;
    uint16_t i;
    static uint8_t mode = SPI_MODE_0;
    static uint8_t bits = 8;
    static uint32_t speed = 1000000;
    static uint32_t order;
    static uint32_t mode2;
    // static uint32_t delay = 100000;
    static uint32_t delay = 100000;
    uint8_t receive[64];

    if ((tx_buffer == NULL) || (rx_buffer == NULL))
    {
        return false;
    }

    // char checksum = checksum_calculateCRC8(tx_buffer, tx_length - 1, CRC8_POLYNOMIAL, CRC8_DEFAULT_STARTREMAINDER, true);
    // tx_buffer[tx_length - 1] = checksum;

    char checksum = supervisor_calculate_CRC(tx_buffer, tx_length - 1);
    tx_buffer[tx_length - 1] = checksum;

    fd = open(SPI_DEV, O_RDWR);
    if (fd < 0) {
        printf("can't open device\n");
        return false;
    }

    // /*
    //  * spi mode
    //  */
    // ret = ioctl(fd, SPI_IOC_WR_MODE, &mode);
    // if (ret == -1) {
    //     perror("can't set spi mode");
    //     return false;
    // }

    ret = ioctl(fd, SPI_IOC_RD_MODE, &mode);
    if (ret == -1) {
        perror("can't get spi mode");
        return false;
    }

    // /*
    //  * bits per word
    //  */
    // ret = ioctl(fd, SPI_IOC_WR_BITS_PER_WORD, &bits);
    // if (ret == -1) {
    // 	perror("can't set bits per word");
    //     return false;
    // }

    ret = ioctl(fd, SPI_IOC_RD_BITS_PER_WORD, &bits);
    if (ret == -1) {
        perror("can't get bits per word");
        return false;
    }

    // /*
    //  * max speed hz
    //  */
    // ret = ioctl(fd, SPI_IOC_WR_MAX_SPEED_HZ, &speed);
    // if (ret == -1) {
    // 	perror("can't set max speed hz");
    //     return false;
    // }

    ret = ioctl(fd, SPI_IOC_RD_MAX_SPEED_HZ, &speed);
    if (ret == -1) {
        perror("can't get max speed hz");
        return false;
    }

    ret = ioctl(fd, SPI_IOC_RD_LSB_FIRST, &order);
    ret = ioctl(fd, SPI_IOC_RD_MODE32, &mode2);

    usleep(10);

    printf("spi mode: %d\n", mode);
    printf("spi mode2: %d\n", mode2);
    printf("bits per word: %d\n", bits);
    printf("lsb first: %d\n", order);
    printf("max speed: %d Hz (%d KHz)\n", speed, speed / 1000);

    // printf("normal command\r\n");
    // for (i = 0; i < tx_length; i++) {
    //     printf("0x%02X ", tx_buffer[i]);
    //     if (i % 2)
    //         printf("\n");
    // }
    // printf("\r\n");

    printf("command with checksum\r\n");
    for (i = 0; i < tx_length; i++) {
        printf("0x%02X ", tx_buffer[i]);
        if (i % 2)
            printf("\n");
    }

    printf("\r\n");

    struct spi_ioc_transfer tr = {
        .tx_buf = (unsigned long)tx_buffer,
        .rx_buf = (unsigned long)rx_buffer,
        .len = tx_length,
        .delay_usecs = 1,
        .speed_hz = 1000000,
        .tx_nbits = 0,
        .rx_nbits = 0,
        .pad = 0,
        .cs_change = 1
    };

    ret = ioctl(fd, SPI_IOC_MESSAGE(1), &tr);
    printf("Spi ret %d\r\n", ret);
    if (ret < 1) {
        printf("Can't send spi message\n");
        return false;
    }

    // struct spi_ioc_transfer tr_tx = {
    //     .tx_buf = (unsigned long)tx_buffer,
    //     .rx_buf = (unsigned long)NULL,
    //     .len = tx_length
    // };

    // struct spi_ioc_transfer tr_rx = {
    //     .tx_buf = (unsigned long)NULL,
    //     .rx_buf = (unsigned long)rx_buffer,
    //     .len = tx_length
    // };

    // ret = ioctl(fd, SPI_IOC_MESSAGE(1), &tr_tx);
    // printf("Spi ret %d\r\n", ret);
    // if (ret < 1) {
    //     printf("Can't send spi message\n");
    //     return false;
    // }

    // ret = ioctl(fd, SPI_IOC_MESSAGE(1), &tr_rx);
    // printf("Spi ret %d\r\n", ret);
    // if (ret < 1) {
    //     printf("Can't send spi message\n");
    //     return false;
    // }

    printf("response\r\n");
    for (i = 0; i < rx_length; i++) {
        printf("0x%02X ", rx_buffer[i]);
        if (i % 2)
            printf("\n");
    }

    printf("\r\n");

    close(fd);

    return true;
}

bool supervisor_get_version(supervisor_version_configuration_t * versionReply)
{
    uint8_t bytesToSendSampleVersion[LENGTH_TELEMETRY_SAMPLE_VERSION] = { CMD_SUPERVISOR_OBTAIN_VERSION_CONFIG, 0x00, 0x00 };
    uint8_t bytesToReceiveSampleVersion[LENGTH_TELEMETRY_SAMPLE_VERSION] = { CMD_SUPERVISOR_OBTAIN_VERSION_CONFIG, 0x00, 0x00 };
    uint8_t bytesToSendObtainVersion[LENGTH_TELEMETRY_GET_VERSION] = { 0 };
    uint8_t bytesToReceiveObtainVersion[LENGTH_TELEMETRY_GET_VERSION] = { 0 };

    if (!spi_comms(bytesToSendSampleVersion, LENGTH_TELEMETRY_SAMPLE_VERSION, bytesToReceiveSampleVersion, LENGTH_TELEMETRY_SAMPLE_VERSION))
    {
        printf("Failed to sample version\n");
        return false;
    }

    usleep(10000);

    if (!spi_comms(bytesToSendObtainVersion, LENGTH_TELEMETRY_GET_VERSION, bytesToReceiveObtainVersion, LENGTH_TELEMETRY_GET_VERSION))
    {
        printf("Failed to obtain version\n");
        return false;
    }

    return true;
}

bool supervisor_get_housekeeping(supervisor_housekeeping_t * versionReply)
{
    uint8_t bytesToSendSampleHousekeepingTelemetry[LENGTH_TELEMETRY_SAMPLE_HOUSEKEEPING] = { CMD_SUPERVISOR_OBTAIN_HK_TELEMETRY, 0x00, 0x00 };
    uint8_t bytesToReceiveSampleHousekeepingTelemetry[LENGTH_TELEMETRY_SAMPLE_HOUSEKEEPING] = { CMD_SUPERVISOR_OBTAIN_HK_TELEMETRY, 0x00, 0x00 };
    uint8_t bytesToSendObtainHousekeepingTelemetry[LENGTH_TELEMETRY_HOUSEKEEPING] = { 0 };
    uint8_t bytesToReceiveObtainHousekeepingTelemetry[LENGTH_TELEMETRY_HOUSEKEEPING] = { 0 };

    if (!spi_comms(bytesToSendSampleHousekeepingTelemetry, LENGTH_TELEMETRY_SAMPLE_HOUSEKEEPING, bytesToReceiveSampleHousekeepingTelemetry, LENGTH_TELEMETRY_SAMPLE_HOUSEKEEPING))
    {
        printf("Failed to sample housekeeping\n");
        return false;
    }

    usleep(10000);

    if (!spi_comms(bytesToSendObtainHousekeepingTelemetry, LENGTH_TELEMETRY_HOUSEKEEPING, bytesToReceiveObtainHousekeepingTelemetry, LENGTH_TELEMETRY_HOUSEKEEPING))
    {
        printf("Failed to obtain housekeeping\n");
        return false;
    }

    return true;
}

bool supervisor_powercycle()
{
    uint8_t bytesToSendPowerCycleIobc[LENGTH_POWER_CYCLE_IOBC] = { CMD_SUPERVISOR_POWER_CYCLE_IOBC, 0x00, 0x00 };
    uint8_t bytesToReceivePowerCycleIobc[LENGTH_POWER_CYCLE_IOBC] = { 0 };
    uint8_t bytesToSendDummyByte[LENGTH_TELEMETRY_DUMMY] = { 0x00, 0x00, 0x00 };
    uint8_t bytesToReceiveDummyByte[LENGTH_TELEMETRY_DUMMY] = { 0x00, 0x00, 0x00 };

    if (!spi_comms(bytesToSendPowerCycleIobc, LENGTH_POWER_CYCLE_IOBC, bytesToReceivePowerCycleIobc, LENGTH_POWER_CYCLE_IOBC))
    {
        printf("Failed to send power cycle\n");
        return false;
    }

    usleep(10000);

    if (!spi_comms(bytesToSendDummyByte, LENGTH_TELEMETRY_DUMMY, bytesToReceiveDummyByte, LENGTH_TELEMETRY_DUMMY))
    {
        printf("Failed to get dummy bytes\n");
        return false;
    }

    return true;
}

bool supervisor_reset()
{
    uint8_t bytesToSendReset[LENGTH_RESET] = { CMD_SUPERVISOR_RESET, 0x00, 0x00 };
    uint8_t bytesToReceiveReset[LENGTH_RESET] = { 0 };
    uint8_t bytesToSendDummyByte[LENGTH_TELEMETRY_DUMMY] = { 0x00, 0x00, 0x00 };
    uint8_t bytesToReceiveDummyByte[LENGTH_TELEMETRY_DUMMY] = { 0x00, 0x00, 0x00 };

    if (!spi_comms(bytesToSendReset, LENGTH_RESET, bytesToReceiveReset, LENGTH_RESET))
    {
        printf("Failed to send reset\n");
        return false;
    }

    usleep(10000);

    if (!spi_comms(bytesToSendDummyByte, LENGTH_TELEMETRY_DUMMY, bytesToReceiveDummyByte, LENGTH_TELEMETRY_DUMMY))
    {
        printf("Failed to get dummy bytes\n");
        return false;
    }

    return true;
}

bool supervisor_emergency_reset()
{
    uint8_t bytesToSendEmergencyReset[LENGTH_EMERGENCY_RESET] = { CMD_SUPERVISOR_EMERGENCY_RESET, 'M', 'E', 'R', 'G', 'E', 'N', 'C', 'Y', 0x00 };
    uint8_t bytesToReceiveEmergencyReset[LENGTH_EMERGENCY_RESET] = { 0 };
    uint8_t bytesToSendDummyByte[LENGTH_TELEMETRY_DUMMY] = { 0x00, 0x00, 0x00 };
    uint8_t bytesToReceiveDummyByte[LENGTH_TELEMETRY_DUMMY] = { 0x00, 0x00, 0x00 };

    if (!spi_comms(bytesToSendEmergencyReset, LENGTH_EMERGENCY_RESET, bytesToReceiveEmergencyReset, LENGTH_EMERGENCY_RESET))
    {
        printf("Failed to send emergency reset\n");
        return false;
    }

    usleep(10000);

    if (!spi_comms(bytesToSendDummyByte, LENGTH_TELEMETRY_DUMMY, bytesToReceiveDummyByte, LENGTH_TELEMETRY_DUMMY))
    {
        printf("Failed to get dummy bytes\n");
        return false;
    }

    return true;
}