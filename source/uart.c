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

#if (defined YOTTA_CFG_HARDWARE_UART) && (YOTTA_CFG_HARDWARE_UART_COUNT > 0)
#include "kubos-hal/uart.h"
#include <string.h>

static KUART k_uarts[K_NUM_UARTS];

static inline int queue_push(csp_queue_handle_t *queue, char c,
                                    int timeout, void *task_woken)
{
    if (!task_woken) {
        return csp_queue_enqueue(queue, &c, timeout);
    } else {
        return csp_queue_enqueue_isr(queue, &c, task_woken);
    }
}

KUART* kprv_uart_get(KUARTNum uart)
{
    return &k_uarts[uart];
}

KUARTConf k_uart_conf_defaults(void)
{
    return (KUARTConf) {
        .baud_rate = YOTTA_CFG_HARDWARE_UART_DEFAULTS_BAUDRATE,
        .word_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_WORDLEN,
        .stop_bits = YOTTA_CFG_HARDWARE_UART_DEFAULTS_STOPBITS,
        .parity = YOTTA_CFG_HARDWARE_UART_DEFAULTS_PARITY,
        .rx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_RXQUEUELEN,
        .tx_queue_len = YOTTA_CFG_HARDWARE_UART_DEFAULTS_TXQUEUELEN,
    };
}

void k_uart_init(KUARTNum uart, KUARTConf *conf)
{
    KUART *k_uart = &k_uarts[uart];
    memcpy(&k_uart->conf, conf, sizeof(KUARTConf));

    k_uart->dev = uart;
    k_uart->rx_queue = csp_queue_create(k_uart->conf.rx_queue_len, sizeof(char));
    k_uart->tx_queue = csp_queue_create(k_uart->conf.tx_queue_len, sizeof(char));

    kprv_uart_dev_init(uart);
}

void k_uart_terminate(KUARTNum uart)
{
    KUART *k_uart = &k_uarts[uart];

    kprv_uart_dev_terminate(uart);

    csp_queue_remove(k_uart->rx_queue);
    k_uart->rx_queue = NULL;
    csp_queue_remove(k_uart->tx_queue);
    k_uart->tx_queue = NULL;
}

void k_uart_console_init(void)
{
    KUARTConf conf = k_uart_conf_defaults();
    conf.baud_rate = K_UART_CONSOLE_BAUDRATE;
    // TODO: allow more configuration of console UART device

    k_uart_init(K_UART_CONSOLE, &conf);
}

int k_uart_read(KUARTNum uart, char *ptr, int len)
{
    int i = 0;
    int result;
    if ((k_uarts[uart].rx_queue != NULL) && (ptr != NULL))
    {
        for (; i < len; i++, ptr++) {
            result = csp_queue_dequeue(k_uarts[uart].rx_queue, ptr, 0);
            if (result != CSP_QUEUE_OK) {
                return i;
            }
        }
    }

    return i;
}

int k_uart_write(KUARTNum uart, char *ptr, int len)
{
    int i = 0;
    if ((k_uarts[uart].tx_queue != NULL) && (ptr != NULL))
    {
        for (; i < len; i++, ptr++) {
            int result = queue_push(k_uarts[uart].tx_queue, *ptr, CSP_MAX_DELAY, NULL);
            if (result != CSP_QUEUE_OK) {
                return i;
            }
            kprv_uart_enable_tx_int(uart);
        }
    }
    return i;
}

int k_uart_rx_queue_len(KUARTNum uart)
{
    if (k_uarts[uart].rx_queue != NULL)
    {
        return (int) csp_queue_size(k_uarts[uart].rx_queue);
    }
    return 0;
}

void k_uart_rx_queue_push(KUARTNum uart, char c, void *task_woken)
{
    queue_push(k_uarts[uart].rx_queue, c, 0, task_woken);
}

#endif
