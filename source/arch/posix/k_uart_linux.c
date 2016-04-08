/*
Cubesat Space Protocol - A small network-layer protocol designed for Cubesats
Copyright (C) 2012 GomSpace ApS (http://www.gomspace.com)
Copyright (C) 2012 AAUSAT3 Project (http://aausat3.space.aau.dk)

This library is free software; you can redistribute it and/or
modify it under the terms of the GNU Lesser General Public
License as published by the Free Software Foundation; either
version 2.1 of the License, or (at your option) any later version.

This library is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public
License along with this library; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
*/

#include "kubos-core/arch/k_uart.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <pthread.h>
#include <unistd.h>
#include <errno.h>
#include <termios.h>
#include <fcntl.h>

#include <sys/time.h>

int uart_stdio_id = 0;
int fd;
uart_callback_t uart_callback = NULL;
void * uart_callback_arg = NULL;

static void *serial_rx_thread(void *vptr_args);

int getbaud(int fd) {
	struct termios termAttr;
	int inputSpeed = -1;
	speed_t baudRate;
	tcgetattr(fd, &termAttr);
	/* Get the input speed. */
	baudRate = cfgetispeed(&termAttr);
	switch (baudRate) {
	case B0:
		inputSpeed = 0;
		break;
	case B50:
		inputSpeed = 50;
		break;
	case B110:
		inputSpeed = 110;
		break;
	case B134:
		inputSpeed = 134;
		break;
	case B150:
		inputSpeed = 150;
		break;
	case B200:
		inputSpeed = 200;
		break;
	case B300:
		inputSpeed = 300;
		break;
	case B600:
		inputSpeed = 600;
		break;
	case B1200:
		inputSpeed = 1200;
		break;
	case B1800:
		inputSpeed = 1800;
		break;
	case B2400:
		inputSpeed = 2400;
		break;
	case B4800:
		inputSpeed = 4800;
		break;
	case B9600:
		inputSpeed = 9600;
		break;
	case B19200:
		inputSpeed = 19200;
		break;
	case B38400:
		inputSpeed = 38400;
		break;
	case B57600:
		inputSpeed = 57600;
		break;
	case B115200:
		inputSpeed = 115200;
		break;
	}

	return inputSpeed;

}

void uart_init(struct uart_conf * conf) {

	struct termios options;
	pthread_t rx_thread;

	fd = open(conf->device, O_RDWR | O_NOCTTY | O_NONBLOCK);

	if (fd < 0) {
		printf("Failed to open %s: %s\r\n", conf->device, strerror(errno));
		return;
	}

	int brate = 0;
    switch(conf->baudrate) {
    case 4800:    brate=B4800;    break;
    case 9600:    brate=B9600;    break;
    case 19200:   brate=B19200;   break;
    case 38400:   brate=B38400;   break;
    case 57600:   brate=B57600;   break;
    case 115200:  brate=B115200;  break;
    }

	tcgetattr(fd, &options);
	cfsetispeed(&options, brate);
	cfsetospeed(&options, brate);
	options.c_cflag |= (CLOCAL | CREAD);
	options.c_cflag &= ~PARENB;
	options.c_cflag &= ~CSTOPB;
	options.c_cflag &= ~CSIZE;
	options.c_cflag |= CS8;
	options.c_lflag &= ~(ECHO | ECHONL | ICANON | IEXTEN | ISIG);
	options.c_iflag &= ~(IGNBRK | BRKINT | ICRNL | INLCR | PARMRK | INPCK | ISTRIP | IXON);
	options.c_oflag &= ~(OCRNL | ONLCR | ONLRET | ONOCR | OFILL | OPOST);
	options.c_cc[VTIME] = 0;
	options.c_cc[VMIN] = 1;
	tcsetattr(fd, TCSANOW, &options);
	if (tcgetattr(fd, &options) == -1)
		perror("error setting options");
	fcntl(fd, F_SETFL, 0);

	/* Flush old transmissions */
	if (tcflush(fd, TCIOFLUSH) == -1)
		printf("Error flushing serial port - %s(%d).\n", strerror(errno), errno);

	if (pthread_create(&rx_thread, NULL, serial_rx_thread, NULL) != 0)
		return;

}

void uart_set_callback(void * arg, uart_callback_t callback) {
	uart_callback = callback;
	uart_callback_arg = arg;
}

void uart_insert(char c, void * pxTaskWoken) {
	printf("%c", c);
}

void uart_putstr(char * buf, int len) {
	if (write(fd, buf, len) != len)
		return;
}

void uart_putc(char c) {
	if (write(fd, &c, 1) != 1)
		return;
}

char uart_getc(void) {
	char c;
	if (read(fd, &c, 1) != 1) return 0;
	return c;
}

int uart_messages_waiting(int handle) {
  struct timeval tv;
  fd_set fds;
  tv.tv_sec = 0;
  tv.tv_usec = 0;
  FD_ZERO(&fds);
  FD_SET(STDIN_FILENO, &fds);
  select(STDIN_FILENO+1, &fds, NULL, NULL, &tv);
  return (FD_ISSET(0, &fds));
}

static void *serial_rx_thread(void *vptr_args) {
	unsigned int length;
	uint8_t * cbuf = malloc(100000);

	// Receive loop
	while (1) {
		length = read(fd, cbuf, 300);
		if (length <= 0) {
			perror("Error: ");
			exit(1);
		}
		if (uart_callback)
			uart_callback(uart_callback_arg, cbuf, length, NULL);
	}
	return NULL;
}
