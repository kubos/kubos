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
  * @defgroup KUBOS_CORE_UART Kubos Core UART Interface
  * @addtogroup KUBOS_CORE_UART
  * @{
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
 * Initialise UART with the uart_conf data structure
 * @param uart_conf full configuration structure
 */
void uart_init(struct uart_conf *conf);

/**
 * In order to catch incoming chars use the callback.
 * Only one callback per interface.
 * @param handle uart[0,1,2,3]
 * @param callback function pointer
 */
typedef void (*uart_callback_t) (void * extra_data, uint8_t *buf, int len, void *pxTaskWoken);
void uart_set_callback(void * arg, uart_callback_t callback);

/**
 * Insert a character to the RX buffer of a uart
 * @param handle uart[0,1,2,3]
 * @param c Character to insert
 */
void uart_insert(char c, void *pxTaskWoken);

/**
 * Polling putchar
 *
 * @param handle uart[0,1,2,3]
 * @param c Character to transmit
 */
void uart_putc(char c);

/**
 * Send char buffer on UART
 *
 * @param handle uart[0,1,2,3]
 * @param buf Pointer to data
 * @param len Length of data
 */
void uart_putstr(char *buf, int len);

/**
 * Buffered getchar
 *
 * @param handle uart[0,1,2,3]
 * @return Character received
 */
char uart_getc(void);

int uart_messages_waiting(int handle);

static inline int uart_stdio_msgwaiting(void) {
	return uart_messages_waiting(0);
}

#endif /* K_UART_H_ */

/* @} */
