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
#include "FreeRTOS.h"
#include "kubos-hal/uart.h"

static KUART k_uarts[K_NUM_UARTS];

static inline BaseType_t queue_push(QueueHandle_t *queue, char c,
                                    TickType_t timeout, void *task_woken)
{
    if (!task_woken) {
        return xQueueSendToBack(queue, &c, timeout);
    } else {
        return xQueueSendToBackFromISR(queue, &c, task_woken);
    }
}

KUART* kprv_uart_get(int uart)
{
    return &k_uarts[uart];
}

KUARTConf k_uart_conf_defaults(void)
{
    return (KUARTConf) {
        .baud_rate = YOTTA_CFG_HARDWARE_UARTDEFAULTS_BAUDRATE,
        .word_len = YOTTA_CFG_HARDWARE_UARTDEFAULTS_WORDLEN,
        .stop_bits = YOTTA_CFG_HARDWARE_UARTDEFAULTS_STOPBITS,
        .parity = YOTTA_CFG_HARDWARE_UARTDEFAULTS_PARITY,
        .rx_queue_len = YOTTA_CFG_HARDWARE_UARTDEFAULTS_RXQUEUELEN,
        .tx_queue_len = YOTTA_CFG_HARDWARE_UARTDEFAULTS_TXQUEUELEN,
    };
}

void k_uart_init(int uart, KUARTConf *conf)
{
    KUART *k_uart = &k_uarts[uart];
    memcpy(&k_uart->conf, conf, sizeof(KUARTConf));

    k_uart->dev = uart;
    k_uart->rx_queue = xQueueCreate(k_uart->conf.rx_queue_len, sizeof(char));
    k_uart->tx_queue = xQueueCreate(k_uart->conf.tx_queue_len, sizeof(char));

    kprv_uart_dev_init(uart);
}

void k_uart_console_init(void)
{
    KUARTConf conf = k_uart_conf_defaults();
    conf.baud_rate = K_UART_CONSOLE_BAUDRATE;
    // TODO: allow more configuration of console UART device

    k_uart_init(K_UART_CONSOLE, &conf);
}

int k_uart_read(int uart, char *ptr, int len)
{
    int i = 0;
    BaseType_t result;

    for (; i < len; i++, ptr++) {
        result = xQueueReceive(k_uarts[uart].rx_queue, ptr, portMAX_DELAY);
        if (result != pdTRUE) {
            return i;
        }
    }

    return i;
}

int k_uart_write(int uart, char *ptr, int len)
{
    int i = 0;
    for (; i < len; i++, ptr++) {
        BaseType_t result = queue_push(k_uarts[uart].tx_queue, *ptr, portMAX_DELAY, NULL);
        if (result != pdTRUE) {
            return i;
        }
        kprv_uart_enable_tx_int(uart);
    }
    return i;
}

int k_uart_rx_queue_len(int uart)
{
    return (int) uxQueueMessagesWaiting(k_uarts[uart].rx_queue);
}

void k_uart_rx_queue_push(int uart, char c, void *task_woken)
{
    queue_push(k_uarts[uart].rx_queue, c, 0, task_woken);
}
