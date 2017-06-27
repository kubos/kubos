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

/**
 * DEPRECATED. Research needed. This file appears to have been replaced by the Kubos HAL UART interface
 */

#ifndef K_UART_H
#define K_UART_H

#include <stdint.h>

/**
 * Usart configuration, to be used with the uart_init call.
 */
struct uart_conf {
	const char *device;
	uint32_t baudrate;
	uint8_t databits;
	uint8_t stopbits;
	uint8_t paritysetting;
	uint8_t checkparity;
};

/**
 * Initialize UART with the uart_conf data structure
 * @param conf full uart configuration structure
 */
void uart_init(struct uart_conf *conf);

/**
 * UART callback function components
 * @param[in] extra_data Extra input arguments
 * @param[out] buf Pointer to data buffer
 * @param[out] len Currently unused
 * @param pxTaskWoken NULL if task context, pointer to variable if ISR
 */
typedef void (*uart_callback_t) (void * extra_data, uint8_t *buf, int len, void *pxTaskWoken);

/**
 * Set callback function to be used when characters are received
 * Note: Only one callback allowed per interface.
 * @param arg Pointer to arguments to passthrough to callback function
 * @param callback Pointer to callback function
 */
void uart_set_callback(void * arg, uart_callback_t callback);

/**
 * Insert a character to the RX buffer of a uart
 * @param c Character to insert
 * @param pxTaskWoken NULL if task context, pointer to variable if ISR
 */
void uart_insert(char c, void *pxTaskWoken);

/**
 * Polling putchar
 *
 * @param c Character to transmit
 */
void uart_putc(char c);

/**
 * Send char buffer on UART
 *
 * @param buf Pointer to data
 * @param len Length of data
 */
void uart_putstr(char *buf, int len);

/**
 * Buffered getchar
 *
 * @return Character received
 */
char uart_getc(void);

int uart_messages_waiting(int handle);

static inline int uart_stdio_msgwaiting(void) {
	return uart_messages_waiting(0);
}

#endif /* K_UART_H_ */

/* @} */
