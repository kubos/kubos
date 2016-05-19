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
#ifndef K_UART_H
#define K_UART_H

#include "FreeRTOS.h"
#include "queue.h"
#include <stdint.h>

#include "pins.h"

#ifndef K_NUM_UARTS
#define K_NUM_UARTS YOTTA_CFG_HARDWARE_UARTCOUNT
#endif

#ifndef K_UART_CONSOLE
#define K_UART_CONSOLE YOTTA_CFG_HARDWARE_CONSOLE_UART
#endif

#ifndef K_UART_CONSOLE_BAUDRATE
#define K_UART_CONSOLE_BAUDRATE YOTTA_CFG_HARDWARE_CONSOLE_BAUDRATE
#endif

typedef enum {
    K_UART1 = 0,
    K_UART2,
    K_UART3,
    K_UART4,
    K_UART5,
    K_UART6
} KUARTNum;

typedef enum {
    K_WORD_LEN_7BIT = 0,
    K_WORD_LEN_8BIT,
    K_WORD_LEN_9BIT
} KWordLen;

typedef enum {
    K_STOP_BITS_1 = 0,
    K_STOP_BITS_1_5,
    K_STOP_BITS_2
} KStopBits;

typedef enum {
    K_PARITY_NONE = 0,
    K_PARITY_EVEN,
    K_PARITY_ODD
} KParity;

typedef struct {
    const char *dev_path;
    uint32_t baud_rate;
    KWordLen word_len;
    KStopBits stop_bits;
    KParity parity;

    uint8_t rx_queue_len;
    uint8_t tx_queue_len;
} KUARTConf;

typedef struct {
    int dev;
    KUARTConf conf;
    QueueHandle_t rx_queue;
    QueueHandle_t tx_queue;
} KUART;

KUARTConf k_uart_conf_defaults(void);
void k_uart_init(KUARTNum uart, KUARTConf *conf);
void k_uart_console_init(void);

int k_uart_read(KUARTNum uart, char *ptr, int len);
int k_uart_write(KUARTNum uart, char *ptr, int len);
void k_uart_write_immediate(KUARTNum uart, char c);

int k_uart_rx_queue_len(KUARTNum uart);
void k_uart_rx_queue_push(KUARTNum uart, char c, void *task_woken);

inline int k_uart_rx_pin(KUARTNum uart) {
    switch (uart) {
#ifdef YOTTA_CFG_HARDWARE_PINS_UART1_RX
        case K_UART1: return YOTTA_CFG_HARDWARE_PINS_UART1_RX;
#endif
#ifdef YOTTA_CFG_HARDWARE_PINS_UART2_RX
        case K_UART2: return YOTTA_CFG_HARDWARE_PINS_UART2_RX;
#endif
#ifdef YOTTA_CFG_HARDWARE_PINS_UART3_RX
        case K_UART3: return YOTTA_CFG_HARDWARE_PINS_UART3_RX;
#endif
#ifdef YOTTA_CFG_HARDWARE_PINS_UART4_RX
        case K_UART4: return YOTTA_CFG_HARDWARE_PINS_UART4_RX;
#endif
#ifdef YOTTA_CFG_HARDWARE_PINS_UART5_RX
        case K_UART5: return YOTTA_CFG_HARDWARE_PINS_UART5_RX;
#endif
#ifdef YOTTA_CFG_HARDWARE_PINS_UART6_RX
        case K_UART6: return YOTTA_CFG_HARDWARE_PINS_UART6_RX;
#endif
    }
    return -1;
}

inline int k_uart_tx_pin(KUARTNum uart) {
    switch (uart) {
#ifdef YOTTA_CFG_HARDWARE_PINS_UART1_TX
        case K_UART1: return YOTTA_CFG_HARDWARE_PINS_UART1_TX;
#endif
#ifdef YOTTA_CFG_HARDWARE_PINS_UART2_TX
        case K_UART2: return YOTTA_CFG_HARDWARE_PINS_UART2_TX;
#endif
#ifdef YOTTA_CFG_HARDWARE_PINS_UART3_TX
        case K_UART3: return YOTTA_CFG_HARDWARE_PINS_UART3_TX;
#endif
#ifdef YOTTA_CFG_HARDWARE_PINS_UART4_TX
        case K_UART4: return YOTTA_CFG_HARDWARE_PINS_UART4_TX;
#endif
#ifdef YOTTA_CFG_HARDWARE_PINS_UART5_TX
        case K_UART5: return YOTTA_CFG_HARDWARE_PINS_UART5_TX;
#endif
#ifdef YOTTA_CFG_HARDWARE_PINS_UART6_TX
        case K_UART6: return YOTTA_CFG_HARDWARE_PINS_UART6_TX;
#endif
    }
    return -1;
}

// private APIs
KUART* kprv_uart_get(KUARTNum uart);
void kprv_uart_dev_init(KUARTNum uart);
void kprv_uart_enable_tx_int(KUARTNum uart);
#endif
