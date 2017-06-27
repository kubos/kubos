/*
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
 * @defgroup MSP430F5529_HAL_UART MSP430F5529 HAL UART Interface
 * @addtogroup MSP430F5529_HAL_UART
 * @{
 */

#if (defined YOTTA_CFG_HARDWARE_UART) && (YOTTA_CFG_HARDWARE_UART_COUNT > 0)
#include "kubos-hal/uart.h"
#include "msp430f5529-hal/uart.h"
#include <msp430.h>

/**
  * Helper function to convert UART device option.
  * @param[in] uart UART device number
  * @return hal_uart_device handle for UART device
  */
static inline hal_uart_device uart_dev(KUARTNum uart)
{
    switch(uart)
    {
        case K_UART1: return HAL_UART_A0;
        case K_UART2: return HAL_UART_A1;
        default: return 0;
    }
}

/**
  * Helper function to get UART handle.
  * @param[in] uart UART device number
  * @return hal_uart_handle* pointer to UART device structure
  */
hal_uart_handle * uart_handle(KUARTNum uart)
{
    switch(uart)
    {
        case K_UART1: return &hal_uart_dev[HAL_UART_A0];
        case K_UART2: return &hal_uart_dev[HAL_UART_A1];
        default: return NULL;
    }
}

/**
  * Helper function to convert baud option.
  * @param[in] baud UART baud rate
  * @return hal_uart_baudrate converted baud rate
  */
static inline hal_uart_baudrate uart_baud(long int baud)
{
    if(baud <= 9600)
    {
        return HAL_UART_9600;
    }
    else if(baud <= 19200)
    {
        return HAL_UART_19200;
    }
    else if(baud <= 38400)
    {
        return HAL_UART_38400;
    }
    else if(baud <= 57600)
    {
        return HAL_UART_57600;
    }
    else
    {
        return HAL_UART_115200;
    }
}

/**
  * Helper function to convert parity option.
  * @param[in] parity UART parity setting
  * @return hal_uart_parity converted party value
  */
static inline hal_uart_parity uart_parity(KParity parity)
{
    switch (parity)
    {
        case K_PARITY_NONE: return HAL_UART_PARITY_NONE;
        case K_PARITY_EVEN: return HAL_UART_PARITY_EVEN;
        case K_PARITY_ODD:  return HAL_UART_PARITY_ODD;
        default: return HAL_UART_PARITY_NONE;
    }
}

/**
  * Helper function to convert stop bits option.
  * @param[in] stopbits UART stop bits setting
  * @return hal_uart_stopbits converted stop bits value
  */
static inline hal_uart_stopbits uart_stopbits(KStopBits stopbits)
{
    switch (stopbits)
    {
        case K_STOP_BITS_1: return HAL_UART_STOP_BITS_1;
        case K_STOP_BITS_2:  return HAL_UART_STOP_BITS_2;
        default: return HAL_UART_STOP_BITS_1;
    }
}

/**
  * Helper function to convert word len option.
  * @param[in] wordlen UART word length setting
  * @return hal_uart_wordlen converted word length value
  */
static inline hal_uart_wordlen uart_wordlen(KWordLen wordlen)
{
    switch (wordlen)
    {
        case K_WORD_LEN_7BIT: return HAL_UART_WORD_LEN_7;
        case K_WORD_LEN_8BIT: return HAL_UART_WORD_LEN_8;
        default: return HAL_UART_WORD_LEN_8;
    }
}

/**
  * Creates and sets up specified UART device.
  * @param[in] uart Number of UART device to setup.
  * @return KUARTStatus UART_OK on success, otherwise failure
  */
KUARTStatus kprv_uart_dev_init(KUARTNum uart)
{
    KUART * k_uart = kprv_uart_get(uart);
    if (k_uart == NULL)
    {
        return UART_ERROR_NULL_HANDLE;
    }

    hal_uart_config config = {
        .device = uart_dev(uart),
        .baudrate = uart_baud(k_uart->conf.baud_rate),
        .parity = uart_parity(k_uart->conf.parity),
        .wordlen = uart_wordlen(k_uart->conf.word_len),
        .stopbits = uart_stopbits(k_uart->conf.stop_bits)
    };

    //Reject unsupported configurations
    if(k_uart->conf.word_len == K_WORD_LEN_9BIT)
    {
        return UART_ERROR_CONFIG;
    }

    hal_uart_handle * handle = hal_uart_init(config);
    if (handle != NULL)
    {
        hal_uart_setup(handle);
        return UART_OK;
    }
    return UART_ERROR_NULL_HANDLE;
}

/**
 * Terminates the specified UART device.
 * @param[in] uart Number of UART device to terminate.
 */
void kprv_uart_dev_terminate(KUARTNum uart)
{
    hal_uart_handle * handle = uart_handle(uart);
    if (handle == NULL)
    {
        return;
    }

    hal_uart_terminate(handle);
}



/**
  * Enables UART TX interrupt.
  * @param[in] uart Number of UART device.
  */
void kprv_uart_enable_tx_int(KUARTNum uart)
{
    hal_uart_handle * handle = uart_handle(uart);
    handle->reg->interruptEnable |= UCTXIE;
}



/**
  * This function handles the rx/tx interrupts using FreeRTOS queues.
  * @param[in] handle UART handle
  */
void hal_uart_interrupt(hal_uart_handle * handle)
{
    static portBASE_TYPE task_woken;
    KUART *k_uart = kprv_uart_get(handle->config.device);
    task_woken = pdFALSE;

    // Check for UCxRXBUF overrun error
    // According to MSP430x5xx Family Guide Section 40.4.3
    // Reading from UCxRXBUF will clear this up.
    if (HAL_UART_STAT(handle, UCOE))
    {
        // Force a read to clear error
        hal_uart_read_raw(handle);
    }

    // RX Interrupt
    if (HAL_UART_INT_FLAG(handle, UCRXIFG))
    {
        char c = hal_uart_read_raw(handle);
        csp_queue_enqueue_isr(k_uart->rx_queue, (void*) &c, &task_woken);
    }

    // TX Interrupt
    if (HAL_UART_INT_FLAG(handle, UCTXIFG))
    {
        char c;
        BaseType_t result = csp_queue_dequeue_isr(k_uart->tx_queue,
                                                (void *) &c,
                                                &task_woken);

        if (result == pdTRUE)
        {
            hal_uart_write_raw(handle, c);
        }
        else
        {
            handle->reg->interruptEnable &= ~UCTXIE;
        }
    }


    if (task_woken != pdFALSE)
    {
        portYIELD();
    }
}

/**
 * Write a character directly to the uart interface
 * @param[in] uart UART bus
 * @param[in] c character to write
 * @return KUARTStatus UART_OK if success, otherwise failure
 */
KUARTStatus k_uart_write_immediate(KUARTNum uart, char c)
{
    hal_uart_handle *handle = uart_handle(uart);
    if (handle == NULL) {
        return UART_ERROR_NULL_HANDLE;
    }

    hal_uart_write(handle, c);

    return UART_OK;
}

/**
 * Write a string directly to the UART interface
 * @param[in] uart UART bus
 * @param[in] ptr buffer to write data from
 * @param[in] len length of data to write
 * @return KUARTStatus UART_OK if success, otherwise failure
 */
KUARTStatus k_uart_write_immediate_str(KUARTNum uart, uint8_t * ptr, uint8_t len)
{
    hal_uart_handle *handle = uart_handle(uart);
    if (handle == NULL) {
        return UART_ERROR_NULL_HANDLE;
    }

    hal_uart_write_str(handle, ptr, len);

    return UART_OK;
}

#endif

/* @} */
