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
   *
   * @file       uart.h
   * @brief      MSP430F5529 HAL - UART module
   *
   * @author     kubos.co
   */

#if (defined YOTTA_CFG_HARDWARE_UART) && (YOTTA_CFG_HARDWARE_UART_COUNT > 0)
#ifndef HAL_UART_H
#define HAL_UART_H

#include <stdint.h>

/**
  * @brief Type by which UART devices are numbered.
  */
typedef enum
{
    HAL_UART_A0 = 0,
    HAL_UART_A1
} hal_uart_device;


/**
  * @brief Type by which UART baudrates are referenced.
  */
typedef enum
{
    HAL_UART_9600 = 0,
    HAL_UART_115200
} hal_uart_baudrate;

/**
  * @brief UART parity options
  */
typedef enum
{
    HAL_UART_PARITY_NONE = 0,
    HAL_UART_PARITY_EVEN,
    HAL_UART_PARITY_ODD
} hal_uart_parity;

/**
  * @brief UART stop bits options
  */
typedef enum
{
    HAL_UART_STOP_BITS_1 = 0,
    HAL_UART_STOP_BITS_2
} hal_uart_stopbits;

/**
  * @brief UART wordlen options
  */
typedef enum
{
    HAL_UART_WORD_LEN_7 = 0,
    HAL_UART_WORD_LEN_8
} hal_uart_wordlen;

/**
  * @brief This type is a map of the USCI_Ax UART registers.
  */
typedef struct
{
    /** UCAxCTL1 */
    volatile uint8_t control1;
    /** UCAxCTL0 */
    volatile uint8_t control0;
    uint8_t padding1[4];
    /** UCAxBR0 */
    volatile uint8_t baudrate0;
    /** UCAxBR1 */
    volatile uint8_t baudrate1;
    /** UCAxMCTL */
    volatile uint8_t modControl;
    uint8_t padding2;
    /** UCAxSTAT */
    volatile uint8_t status;
    uint8_t padding3;
    /** UCAxRXBUF */
    volatile uint8_t rx;
    uint8_t padding4;
    /** UCAxTXBUF */
    volatile uint8_t tx;
    /** UCAxABCTL
        UCAxRxCTL */
    uint8_t padding5[13];
    /** UCAxIE */
    volatile uint8_t interruptEnable;
    /** UCAxIFG */
    volatile uint8_t interruptFlags;
    /** UCAxIV */
    volatile uint8_t interruptVector;
} hal_uart_mem_reg;

/**
  * @brief This type contains UART device config data.
  */
typedef struct
{
    hal_uart_device device;
    hal_uart_baudrate baudrate;
    hal_uart_wordlen wordlen;
    hal_uart_stopbits stopbits;
    hal_uart_parity parity;
} hal_uart_config;

/**
  * @brief This type contains all uart config and register details for this layer.
  */
typedef struct
{
    hal_uart_config config;
    volatile hal_uart_mem_reg * reg;
    volatile uint8_t * select;
    uint8_t selectVal;
} hal_uart_handle;

/**
  * @brief Static array of avaiable uart handles.
  */
extern hal_uart_handle hal_uart_dev[];

/**
  * @brief Creates a UART handle and fills in details associated with
  *        the specified device.
  * @param device Instance of hal_uart_device, specifying device to use.
  * @retval hal_uart_handle *
  */
hal_uart_handle * hal_uart_device_init(hal_uart_device device);

/**
  * @brief Creates a UART handle according to details specified in config.
  * @param config Instance of hal_uart_config with init details.
  * @retval hal_uart_handle
  */
hal_uart_handle * hal_uart_init(hal_uart_config config);

/**
  * @brief Low level hardware setup of UART device.
  * @param handle Instance of initilaized hal_uart_handle containing hardware
  *               registers and config values.
  */
void hal_uart_setup(hal_uart_handle * handle);

/**
  * @brief Reads a single character from UART.
  * @param handle UART handle to read from
  * @retval uint8_t character read
  */
uint8_t hal_uart_read_raw(hal_uart_handle * handle);

/**
  * @brief Inserts a single character into UART TX buffer.
  * @param handle UART handle to transmit over
  * @param c character to transmit
  */
void hal_uart_write_raw(hal_uart_handle * handle, uint8_t c);

/**
  * @brief Transmits a single character over UART.
  * @param handle UART handle to transmit over
  * @param c character to transmit
  */
void hal_uart_write(hal_uart_handle * handle, uint8_t c);

/**
  * @brief Transmits a buffer over UART.
  * @param handle UART handle to transmit over
  * @param buf buffer to transmit
  * @param len number of characters to transmit
  */
void hal_uart_write_str(hal_uart_handle * handle, uint8_t * buf, uint8_t len);

/**
  * @brief Externally implemented interrupt processing function
  * @param handle UART handle
  */
extern void hal_uart_interrupt(hal_uart_handle * handle);

/**
  * @brief Checks for existence of interrupt flag
  */
#define HAL_UART_INT_FLAG(handle, flag) (flag == (flag & handle->reg->interruptFlags))

/**
  * @brief Checks for existence of interrupt status
  */
#define HAL_UART_STAT(handle, flag) (flag == (flag & handle->reg->status))

#endif
#endif
/* @} */
