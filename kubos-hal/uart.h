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
/**
 * @defgroup UART
 * @addtogroup UART
 * @{
 */
/**
 * @brief KubOS-HAL UART Interface
 * @author kubos.co
 */

#if (defined YOTTA_CFG_HARDWARE_UART) && (YOTTA_CFG_HARDWARE_UART_COUNT > 0)
#ifndef K_UART_H
#define K_UART_H

#include "FreeRTOS.h"
#include "queue.h"
#include <stdint.h>

#include "pins.h"

/**
 * Number of uart interfaces available. Derived from value in target.json
 * @code
 * "config": {
 *   "hardware": {
 *     "uart": {
 *       "count": 2
 *     }
 *   }
 * }
 * @endcode
 */
#ifndef K_NUM_UARTS
#define K_NUM_UARTS YOTTA_CFG_HARDWARE_UART_COUNT
#endif

/**
 * Uart interface used for console output.
 * @code
 * "config": {
 *   "hardware": {
 *     "console": {
 *       "uart": "K_UART1"
 *     }
 *   }
 * }
 * @endcode
 */
#ifndef K_UART_CONSOLE
#define K_UART_CONSOLE YOTTA_CFG_HARDWARE_CONSOLE_UART
#endif

/**
 * Baudrate used for console output.
 * @code
 * "config": {
 *   "hardware": {
 *     "console": {
 *       "baudrate": "115200"
 *     }
 *   }
 * }
 * @endcode
 */
#ifndef K_UART_CONSOLE_BAUDRATE
#define K_UART_CONSOLE_BAUDRATE YOTTA_CFG_HARDWARE_CONSOLE_BAUDRATE
#endif

/**
 * Available uart interfaces
 */
typedef enum {
#ifdef YOTTA_CFG_HARDWARE_UART_UART1
    K_UART1 = 0,
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART2
    K_UART2,
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART3
    K_UART3,
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART4
    K_UART4,
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART5
    K_UART5,
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART6
    K_UART6
#endif
} KUARTNum;

/**
 * Word length
 */
typedef enum {
    K_WORD_LEN_7BIT = 0,
    K_WORD_LEN_8BIT,
    K_WORD_LEN_9BIT
} KWordLen;

/**
 * Number of stop bits
 */
typedef enum {
    K_STOP_BITS_1 = 0,
    K_STOP_BITS_2
} KStopBits;

/**
 * Parity setting
 */
typedef enum {
    K_PARITY_NONE = 0,
    K_PARITY_EVEN,
    K_PARITY_ODD
} KParity;

/**
 * Uart configuration structure
 */
typedef struct {
    const char *dev_path;
    uint32_t baud_rate;
    KWordLen word_len;
    KStopBits stop_bits;
    KParity parity;
    uint8_t rx_queue_len;
    uint8_t tx_queue_len;
} KUARTConf;

/**
 * Uart interface data structure
 */
typedef struct {
    int dev;
    KUARTConf conf;
    QueueHandle_t rx_queue;
    QueueHandle_t tx_queue;
} KUART;

/**
 * Generate KUARTConf with default uart values
 * @return KUARTConf
 */
KUARTConf k_uart_conf_defaults(void);

/**
 * Setup and enable uart interface
 * @param uart uart interface to initialize
 * @param conf config values to initialize with
 */
int k_uart_init(KUARTNum uart, KUARTConf *conf);

/**
 * Setup and enable console uart interface
 */
void k_uart_console_init(void);

/**
 * Interrupt driven function for reading data from a uart interface.
 * This function reads from a queue which is filled up via the uart
 * interrupt handler.
 *
 * @param uart uart interface to read from
 * @param ptr buffer to read data into
 * @param len length of data to read
 * @return int number of characters read
 */
int k_uart_read(KUARTNum uart, char *ptr, int len);

/**
 * Interrupt driven function for writing data to a uart interface.
 * This function writes data into a queue which is then written out in the
 * interrupt handler.
 *
 * @param uart uart interface to write to
 * @param ptr buffer to write data from
 * @param len length of data to write
 * @return int number of characters written
 */
int k_uart_write(KUARTNum uart, char *ptr, int len);

/**
 * Write data directly to a uart interface
 * @param uart uart interface to write to
 * @param c character to write
 */
int k_uart_write_immediate(KUARTNum uart, char c);

/**
 * Returns the number of characters currently in the uart rx queue
 * @param uart uart interface number
 */
int k_uart_rx_queue_len(KUARTNum uart);

/**
 * Pushes a character into the uart rx queue
 * @param uart uart interface number
 * @param c character to push
 * @param task_woken used by FreeRTOS to determine task blocking
 */
void k_uart_rx_queue_push(KUARTNum uart, char c, void *task_woken);

/**
 * Returns rx pin for specified uart interface
 * @param uart uart interface number
 * @return int rx pin
 */
int k_uart_rx_pin(KUARTNum uart);

/**
 * Returns tx pin for specified uart interface
 * @param uart uart interface number
 * @return int tx pin
 */
int k_uart_tx_pin(KUARTNum uart);

// private APIs
/**
 * Returns uart data structure for specified interface
 * @param uart uart interface number
 * @return KUART* pointer to uart data structure
 */
KUART* kprv_uart_get(KUARTNum uart);

/**
 * Performs low level uart hardware initialization
 * @param uart uart interface to initialize
 * @return Error code
 */
int kprv_uart_dev_init(KUARTNum uart);

/**
 * Enables uart transmit interrupt
 * @param uart uart interface number
 */
void kprv_uart_enable_tx_int(KUARTNum uart);
#endif // #ifndef K_UART_H
#endif // #ifdef YOTTA_CFG_HARDWARE_UART && YOTTA_CFG_HARDE_UART_COUNT > 0
/* @} */
