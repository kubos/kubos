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
 *
 * UART Communication Demo Code - Transmitter
 *
 * This program will write an incrementing message "Test message nnn" over
 * UART via the uart device /dev/ttyS3 every 5 seconds. The counter will
 * reset at 255.
 *
 * Use Ctrl+C to exit the program cleanly.
 *
 */

#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <termios.h>
#include <unistd.h>

struct sigaction saio;

int running;
int fd;

int uart_init(void)
{
    char *         device = "/dev/ttyS3";
    struct termios uart;
    speed_t        speed = B115200;

    /*
     * Open UART (terminal)
     * O_WRONLY - Write only
     * O_NOCTTY - This terminal should not be the controlling terminal of the
     * process
     *            (That should still be the debug UART)
     * O_DSYNC  - Writes should block until all data has been successfully
     *            written out of the buffer to the underlying hardware
     */
    fd = open(device, O_WRONLY | O_NOCTTY | O_DSYNC);
    if (fd < 0)
    {
        perror("Error opening device");
        return -1;
    }

    /*
     * Get the current terminal settings. There are a bunch, and we only want
     * to update a few, so we'll preserve everything else that's currently set.
     */
    if (tcgetattr(fd, &uart) < 0)
    {
        perror("Error from tcgetattr");
        return -1;
    }

    /* Set output baudrate */
    cfsetospeed(&uart, speed);

    /* Character processing flags */
    uart.c_cflag |= (CLOCAL | CREAD);   /* Ignore modem controls */
    uart.c_cflag &= ~CSIZE;             /* Clear current char size mask */
    uart.c_cflag |= CS8;                /* 8-bit characters */
    uart.c_cflag &= ~PARENB;            /* No parity bit */
    uart.c_cflag &= ~CSTOPB;            /* 1 stop bit */
    uart.c_cflag &= ~CRTSCTS;           /* No hardware flowcontrol */

    /* Output flags */
    uart.c_oflag = 0; /* Raw output */

    /* Clear anything that's currently in the UART buffers */
    tcflush(fd, TCIOFLUSH);

    /* Update the terminal settings */
    if (tcsetattr(fd, TCSANOW, &uart) != 0)
    {
        perror("Error from tcsetattr");
        return -1;
    }

    return 0;
}

int uart_close(void)
{
    close(fd);
}

void sigint_handler(int sig)
{
    running = 0;
    sleep(2);
    signal(SIGINT, SIG_DFL);
    kill(getpid(), SIGINT);
}

int main(int argc, char * argv[])
{

    int  opt;
    char counter = 0;

    running = 1;

    /* Set up Ctrl+C interrupt handler for exiting cleanly */
    signal(SIGINT, sigint_handler);

    /* Open connection to receiver */
    uart_init();

    while (running)
    {

        char testmsg[] = "Test Message nnn\n";

        printf("Writing message %d\n", counter);

        sprintf(testmsg, "Test Message %03d\n", counter++);

        if(write(fd, testmsg, sizeof(testmsg) < 0)
        {
            perror("Error from write");
            break;
        }

        printf("Wrote %d bytes\n", ret);

        sleep(5);
    }

    /* Cleanup */
    uart_close();

    return 0;
}
