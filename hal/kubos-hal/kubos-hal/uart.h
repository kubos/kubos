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
 * @defgroup UART HAL UART Interface
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

#include "pins.h"
#include <csp/arch/csp_queue.h>
#include <stdint.h>

/**
 * Number of UART interfaces available. Derived from value in target.json
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
 * Available UART interfaces
 */
typedef enum {
// @warning - need to add K_UART_NO_BUS
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
 * @note MSP430F5 does not support 9-bit mode
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
 * Uart status values
 */
typedef enum {
    UART_OK,
    UART_ERROR,
    UART_ERROR_NULL_HANDLE,
    UART_ERROR_CONFIG
} KUARTStatus;

/**
 * Uart configuration structure
 */
typedef struct
{
    /**
     * The path of the UART bus
     */
    const char * dev_path;
    /**
     * The buad rate of the UART bus
     * @warning For the <b>MSP430F5 microcontroller</b>, the speed of the SPI bus can only be defined
     * as a factor of the peripheral clock to which it's connected (SMCLK for MSP430F5 SPI buses).
     * For example, SMCLK_speed / 2.  To make things easier, this speed field will take a normal baud rate number and
     * then it will automatically be converted to the nearest available system speed without exceeding the original
     * value. <br />
     */
    uint32_t baud_rate;
    /**
     * The number of data bits in each transmit/receive of the UART bus.
     * Can be 7-, 8-, or 9-bits, as specified by the KWordLen enumerator
     */
    KWordLen word_len;
    /**
     * The number of stop bits at the end of each transmit/receive of the UART bus.
     * Can be 1 or 2 bits, as specified by the KStopBits enumerator
     */
    KStopBits stop_bits;
    /**
     * The presence and state of the parity bit in each transmit/receive of the UART bus.
     * Can be none, odd, or even, as specified by the KParity enumerator
     */
    KParity parity;
    /**
     * The size of the queue for incoming messages
     */
    uint8_t rx_queue_len;
    /**
     * The size of the queue for outgoing messages
     */
    uint8_t tx_queue_len;
} KUARTConf;

/**
 * Uart interface data structure
 */
typedef struct
{
    /**
     * UART device number
     */
    int dev;
    /**
     * Copy of UART configuration options
     */
    KUARTConf conf;
    /**
     * Queue filled with received UART data
     */
    csp_queue_handle_t rx_queue;
    /**
     * Queue filled with data to be sent
     */
    csp_queue_handle_t tx_queue;
} KUART;

/**
 * Generate KUARTConf with default UART values
 * @return KUARTConf
 */
KUARTConf k_uart_conf_defaults(void);

/**
 * Setup and enable UART interface
 * @param uart UART interface to initialize
 * @param conf config values to initialize with
 * @return KUARTStatus UART_OK if OK, failure otherwise
 */
KUARTStatus k_uart_init(KUARTNum uart, KUARTConf * conf);

/**
 * Terminates UART interface
 * @param uart UART interface to terminate
 */
void k_uart_terminate(KUARTNum uart);

/**
 * Setup and enable console UART interface
 */
void k_uart_console_init(void);

/**
 * Interrupt driven function for reading data from a UART interface.
 * This function reads from a queue which is filled up via the UART
 * interrupt handler.
 *
 * @param uart UART interface to read from
 * @param ptr buffer to read data into
 * @param len length of data to read
 * @return int number of characters read or -1 to indicate a null UART handle
 */
int k_uart_read(KUARTNum uart, char * ptr, int len);

/**
 * Interrupt driven function for writing data to a UART interface.
 * This function writes data into a queue which is then written out in the
 * interrupt handler.
 *
 * @param uart UART interface to write to
 * @param ptr buffer to write data from
 * @param len length of data to write
 * @return int number of characters written or -1 to indicate a null UART handle
 */
int k_uart_write(KUARTNum uart, char * ptr, int len);

/**
 * Write data directly to a UART interface
 * @param uart UART interface to write to
 * @param c character to write
 * @return KUARTStatus UART_OK if success, otherwise failure
 */
KUARTStatus k_uart_write_immediate(KUARTNum uart, char c);

/**
 * Write data directly to a UART interface
 * @param uart UART interface to write to
 * @param ptr buffer to write data from
 * @param len length of data to write
 * @return KUARTStatus UART_OK if success, otherwise failure
 */
KUARTStatus k_uart_write_immediate_str(KUARTNum uart, uint8_t * ptr, uint8_t len);

/**
 * Returns the number of characters currently in the UART rx queue
 * @param uart UART interface number or -1 to indicate a null UART handle or -2
 * to indicate a null rx queue pointer
 * @return int length of UART's rx_queue
 */
int k_uart_rx_queue_len(KUARTNum uart);

/**
 * Pushes a character into the UART rx queue
 * @param uart UART interface number
 * @param c character to push
 * @param task_woken used by FreeRTOS to determine task blocking
 */
void k_uart_rx_queue_push(KUARTNum uart, char c, void * task_woken);

/**
 * Returns rx pin for specified UART interface
 * @param uart UART interface number
 * @return int rx pin
 */
int k_uart_rx_pin(KUARTNum uart);

/**
 * Returns tx pin for specified UART interface
 * @param uart UART interface number
 * @return int tx pin
 */
int k_uart_tx_pin(KUARTNum uart);

// private APIs
/**
 * Returns UART data structure for specified interface
 * @param uart UART interface number
 * @return KUART* pointer to UART data structure
 */
KUART * kprv_uart_get(KUARTNum uart);

/**
 * Performs low-level UART hardware initialization
 * @param uart UART interface to initialize
 * @return KUARTStatus UART_OK if OK, failure otherwise
 */
KUARTStatus kprv_uart_dev_init(KUARTNum uart);

/**
 * Performs low-level UART hardware termination
 * @param uart UART interface to initialize
 */
void kprv_uart_dev_terminate(KUARTNum uart);

/**
 * Enables UART transmit interrupt
 * @param uart UART interface number
 */
void kprv_uart_enable_tx_int(KUARTNum uart);

#endif // #ifndef K_UART_H
#endif // #ifdef YOTTA_CFG_HARDWARE_UART && YOTTA_CFG_HARDE_UART_COUNT > 0
/* @} */
