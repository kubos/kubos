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
#include <csp/arch/csp_thread.h>

KUARTNum uart; /* global device num */

usart_callback_t usart_callback;

CSP_DEFINE_TASK(task_csp)
{
    portBASE_TYPE task_woken = pdFALSE;
    uint8_t len = 0;
    char csp_buf = 0;

    while(1)
    {
        len = k_uart_read(uart, &csp_buf, 1);
        if (usart_callback != NULL && len == 1)
        {
            usart_callback((uint8_t*)&csp_buf, 1, &task_woken);
        }
    }

    return CSP_TASK_RETURN;
}

void usart_init(struct usart_conf *conf)
{
    /* check if conf is valid */
    if (conf == NULL)
    {
        return;
    }

    /* set dev num */
    uart = (KUARTNum)*(conf->device);

    /* set params for k_uart */
    KUARTConf k_uart_conf = {
            .baud_rate = conf->baudrate,
            .word_len = conf->databits,
            .stop_bits = conf->stopbits,
            .parity = conf->paritysetting,
            .rx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_RXQUEUELEN,
            .tx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_TXQUEUELEN,
        };

    /* initialize uart */
    k_uart_init(uart, &k_uart_conf);

    /* create csp thread to look for csp message */
    csp_thread_handle_t handle_csp;
    csp_thread_create(task_csp, "CSP", configMINIMAL_STACK_SIZE, NULL, 0, &handle_csp);
}

void usart_set_callback(usart_callback_t callback)
{
    usart_callback = callback;
}

void usart_insert(char c, void *pxTaskWoken)
{
    /* put back into UART rx queue */
    k_uart_rx_queue_push(uart, c, pxTaskWoken);
}

void usart_putc(char c)
{
    k_uart_write(uart, &c, 1);
}

void usart_putstr(char *buf, int len)
{
    k_uart_write(uart, buf, len);
}

char usart_getc(void)
{
    char buf = 0;
    k_uart_read(uart, &buf, 1);
    return buf;
}

int usart_messages_waiting(int handle)
{
    /* get rx queue len */
    return k_uart_rx_queue_len((KUARTNum) handle);
}
