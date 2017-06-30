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

/**
 * Returns rx pin for specified uart interface
 * @param uart uart interface number
 * @return int rx pin or -1 to indicate uart not found
 */
int k_uart_rx_pin(KUARTNum uart) {
    switch (uart) {
#ifdef YOTTA_CFG_HARDWARE_UART_UART1_RX
        case K_UART1: return YOTTA_CFG_HARDWARE_UART_UART1_RX;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART2_RX
        case K_UART2: return YOTTA_CFG_HARDWARE_UART_UART2_RX;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART3_RX
        case K_UART3: return YOTTA_CFG_HARDWARE_UART_UART3_RX;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART4_RX
        case K_UART4: return YOTTA_CFG_HARDWARE_UART_UART4_RX;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART5_RX
        case K_UART5: return YOTTA_CFG_HARDWARE_UART_UART5_RX;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART6_RX
        case K_UART6: return YOTTA_CFG_HARDWARE_UART_UART6_RX;
#endif
    }
    return -1;
}

/**
 * Returns tx pin for specified uart interface
 * @param uart uart interface number
 * @return int tx pin or -1 to indicate uart not found
 */
int k_uart_tx_pin(KUARTNum uart) {
    switch (uart) {
#ifdef YOTTA_CFG_HARDWARE_UART_UART1_TX
        case K_UART1: return YOTTA_CFG_HARDWARE_UART_UART1_TX;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART2_TX
        case K_UART2: return YOTTA_CFG_HARDWARE_UART_UART2_TX;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART3_TX
        case K_UART3: return YOTTA_CFG_HARDWARE_UART_UART3_TX;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART4_TX
        case K_UART4: return YOTTA_CFG_HARDWARE_UART_UART4_TX;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART5_TX
        case K_UART5: return YOTTA_CFG_HARDWARE_UART_UART5_TX;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART6_TX
        case K_UART6: return YOTTA_CFG_HARDWARE_UART_UART6_TX;
#endif
    }
    return -1;
}

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
	//Validate UART number
	if(uart > (K_NUM_UARTS-1))
	{
		return NULL;
	}

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

KUARTStatus k_uart_init(KUARTNum uart, KUARTConf *conf)
{
    KUARTStatus ret = UART_ERROR;

    KUART *k_uart = kprv_uart_get(uart);
    if(k_uart == NULL)
    {
        return UART_ERROR_NULL_HANDLE;
    }

    memcpy(&k_uart->conf, conf, sizeof(KUARTConf));

    k_uart->dev = uart;
    k_uart->rx_queue = csp_queue_create(k_uart->conf.rx_queue_len, sizeof(char));
    k_uart->tx_queue = csp_queue_create(k_uart->conf.tx_queue_len, sizeof(char));

    ret = kprv_uart_dev_init(uart);

    if(ret != UART_OK)
    {
        csp_queue_remove(k_uart->rx_queue);
        k_uart->rx_queue = NULL;
        csp_queue_remove(k_uart->tx_queue);
        k_uart->tx_queue = NULL;
    }

    return ret;
}

void k_uart_terminate(KUARTNum uart)
{
    KUART *k_uart = kprv_uart_get(uart);
    if(k_uart == NULL)
    {
      return;
    }

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
    KUART *k_uart = kprv_uart_get(uart);

    if(k_uart == NULL)
    {
      return -1;
    }

    if ((k_uart->rx_queue != NULL) && (ptr != NULL))
    {
        for (; i < len; i++, ptr++) {
            result = csp_queue_dequeue(k_uart->rx_queue, ptr, 0);
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
    KUART *k_uart = kprv_uart_get(uart);

    if(k_uart == NULL)
    {
      return -1;
    }

    if ((k_uart->tx_queue != NULL) && (ptr != NULL))
    {
        for (; i < len; i++, ptr++) {
            int result = queue_push(k_uart->tx_queue, *ptr, CSP_MAX_DELAY, NULL);
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
    KUART *k_uart = kprv_uart_get(uart);
    if(k_uart == NULL)
    {
      return -1;
    }

    if (k_uart->rx_queue != NULL)
    {
        return (int) csp_queue_size(k_uart->rx_queue);
    }
    return -2;
}

void k_uart_rx_queue_push(KUARTNum uart, char c, void *task_woken)
{
    KUART *k_uart = kprv_uart_get(uart);

    if(k_uart == NULL)
    {
      return;
    }

    queue_push(k_uart->rx_queue, c, 0, task_woken);
}

#endif
