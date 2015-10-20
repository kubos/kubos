#include <errno.h>
#include <termios.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include "kernel.h"

#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

#include "../ham/aprs.h"
#include "../ham/ax25.h"
#include "../ham/kiss.h"
#include "gpio.h"


int set_interface_attribs (int fd, int speed, int parity)
{
        struct termios tty;
        memset (&tty, 0, sizeof tty);
        if (tcgetattr (fd, &tty) != 0)
        {
                printf ("error %d from tcgetattr", errno);
                return -1;
        }

        cfsetospeed (&tty, speed);
        cfsetispeed (&tty, speed);

        tty.c_cflag = (tty.c_cflag & ~CSIZE) | CS8;     // 8-bit chars
        // disable IGNBRK for mismatched speed tests; otherwise receive break
        // as \000 chars
        tty.c_iflag &= ~IGNBRK;         // disable break processing
        tty.c_lflag = 0;                // no signaling chars, no echo,
                                        // no canonical processing
        tty.c_oflag = 0;                // no remapping, no delays
        tty.c_cc[VMIN]  = 0;            // read doesn't block
        tty.c_cc[VTIME] = 5;            // 0.5 seconds read timeout

        tty.c_iflag &= ~(IXON | IXOFF | IXANY); // shut off xon/xoff ctrl

        tty.c_cflag |= (CLOCAL | CREAD);// ignore modem controls,
                                        // enable reading
        tty.c_cflag &= ~(PARENB | PARODD);      // shut off parity
        tty.c_cflag |= parity;
        tty.c_cflag &= ~CSTOPB;
        tty.c_cflag &= ~CRTSCTS;

        if (tcsetattr (fd, TCSANOW, &tty) != 0)
        {
                printf ("error %d from tcsetattr", errno);
                return -1;
        }
        return 0;
}

void set_blocking (int fd, int should_block)
{
        struct termios tty;
        memset (&tty, 0, sizeof tty);
        if (tcgetattr (fd, &tty) != 0)
        {
                printf ("error %d from tggetattr", errno);
                return;
        }

        tty.c_cc[VMIN]  = should_block ? 1 : 0;
        tty.c_cc[VTIME] = 5;            // 0.5 seconds read timeout

        if (tcsetattr (fd, TCSANOW, &tty) != 0)
                printf ("error %d setting term attributes", errno);
}

int testGPIO(unsigned int gpio)
{
    // export and configure the pin for our usage
    gpio_export(gpio);
    gpio_set_dir(gpio, GPIO_DIR_OUTPUT);
    gpio_set_value(gpio, 1);

    return 0;
}

int test_gpio(int argc, char **argv) {

    unsigned int gpio;

    // gpio pin number
    if (argc < 2) {
        printf("Usage: gpio-int <gpio-pin>\n\n");
        printf("Set pin high\n");
        printf("See this article for beaglebone pins...http://www.armhf.com/using-beaglebone-black-gpios & http://kilobaser.com/blog/2014-07-15-beaglebone-black-gpios/\n");
        return -1;
    }

    gpio = atoi(argv[1]);

    testGPIO(gpio);

    return 0;
}

