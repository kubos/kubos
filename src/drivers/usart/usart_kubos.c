/*
 * KubOS HAL
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

#include <csp/csp.h>
#include <csp/drivers/usart.h>
#include "kubos-hal/uart.h"
#include <stdlib.h>

KUARTNum uart;

void usart_init(struct usart_conf *conf) {
    /* only need dev num */
    uart = (KUARTNum)*conf->device;
}

void usart_insert(char c, void *pxTaskWoken) {
    /* put back into UART rx queue */
    k_uart_rx_queue_push(uart, c, pxTaskWoken);
}

void usart_putc(char c) {
    k_uart_write(uart, &c, 1);
}

void usart_putstr(char *buf, int len) {
    k_uart_write(uart, buf, len);
}

char usart_getc(void) {
    char buf = 0;
    k_uart_read(K_UART6, &buf, 1);
    return buf;
}

int usart_messages_waiting(int handle) {
    /* get rx queue len */
    return k_uart_rx_queue_len((KUARTNum)handle);
}

