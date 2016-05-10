/*
 * @file    uart.c
 * @brief   UART HAL module
 *
 *******************************************************************************
 * @attention
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
 *******************************************************************************
 */



#include <msp430.h>
#include "msp430f5529-hal/uart.h"

#include <stddef.h>

#define BC_RX_WAKE_THRESH 1

static hal_uart_handle hal_uart_a0;
static hal_uart_handle hal_uart_a1;

/**
  * @brief Creates a UART handle and fills in details associated with
  *        the specified device.
  * @param device Instance of hal_uart_device, specifying device to use.
  * @retval hal_uart_handle
  */
hal_uart_handle hal_uart_device_init(hal_uart_device device)
{
    hal_uart_handle handle;
    if (HAL_UART_A1 == device)
    {
        handle.config.device = device;
        handle.select = &P4SEL;
        handle.selectVal = BIT4 + BIT5;
        handle.reg = (hal_uart_mem_reg *)__MSP430_BASEADDRESS_USCI_A1__;
    }
    return handle;
}

/**
  * @brief Creates a UART handle according to details specified in config.
  * @param config Instance of hal_uart_config with init details.
  * @retval hal_uart_handle
  */
hal_uart_handle hal_uart_init(hal_uart_config config)
{
    hal_uart_handle handle = hal_uart_device_init(config.device);
    handle.config = config;
    return handle;
}

/**
  * @brief Low level hardware setup of UART device.
  * @param handle Instance of initilaized hal_uart_handle containing hardware
  *               registers and config values.
  * @retval status
  */
uint8_t hal_uart_setup(hal_uart_handle * handle)
{
    if (NULL != handle)
    {
        handle->rxBufferSize = 128;
        handle->rxBuffer = malloc(sizeof(uint8_t)*handle->rxBufferSize);
        // Put the USCI state machine in reset
        handle->reg->control1 = UCSWRST;
        // Use SMCLK as the bit clock
        handle->reg->control1 |= UCSSEL__SMCLK;

        // Set baudrate
        hal_uart_set_speed(handle);

        // Configure pins as TXD/RXD
        *(handle->select) |= handle->selectVal;

        // Take the USCI out of reset
        handle->reg->control1 &= ~UCSWRST;

        // Enable the RX interrupt.
        handle->reg->interruptFlags |= UCRXIE;
    }
}

/**
  * @brief Low level hardware setup of UART baudrate.
  * @param handle Instance of initilaized hal_uart_handle containing hardware
  *               registers and config values.
  */
void hal_uart_set_speed(hal_uart_handle * handle)
{
    switch(handle->config.baudrate)
    {
        case HAL_UART_9600:
        {
            handle->reg->baudrate0 = 6;
            handle->reg->baudrate1 = 0;
            handle->reg->modControl = UCBRS_0 + UCBRF_13 + UCOS16;
            break;
        }
        case HAL_UART_115200:
        {
            handle->reg->baudrate0 = 9;
            handle->reg->baudrate1 = 0;
            handle->reg->modControl = UCBRS_0 + UCBRF_1;
            break;
        }
    }
}

/**
  * @brief Transmit single character
  * @param handle UART handle to transmit over
  * @param c character to transmit
  * @retval status
  */
void hal_uart_putc(hal_uart_handle * handle, uint8_t c)
{
    handle->reg->tx = c;
    // Wait until bit has been clocked out...
    while (!(UCTXIFG == (UCTXIFG && (handle->reg->interruptFlags)))
        && (UCBUSY == ((handle->reg->status) & UCBUSY)));
}

/**
  * @brief Transmit buffer
  * @param handle UART handle to transmit over
  * @param buf buffer to transmit
  * @param len number of characters to transmit
  * @retval status
  */
void hal_uart_putstr(hal_uart_handle * handle, uint8_t * buf, uint8_t len)
{
    uint8_t i = 0;

    for (i = 0; i < len; i++)
        hal_uart_putc(handle, buf[i]);
}

/**
  * @brief Receive a buffer
  * @param handle UART handle to receieve from
  * @param buf Buffer to receive characters into.
  * @retval count Number of characters received
  */
uint16_t hal_uart_read(hal_uart_handle * handle, uint8_t * buf)
{
    uint16_t i, count;

    handle->reg->interruptFlags &= ~UCRXIE;

    for (i = 0; i < handle->rxBufferIndex; i++)
    {
        buf[i] = handle->rxBuffer[i];
    }

    count = handle->rxBufferIndex;
    handle->rxBufferIndex = 0;

    handle->reg->interruptFlags |= UCRXIE;

    return count;
}

/**
  * @brief This function handles UART interrut requests.
  */
void hal_uart_interrupt(hal_uart_handle * handle)
{
    handle->rxBuffer[handle->rxBufferIndex++] = (handle->reg->rx);

    // // Wake main, to fetch data from the buffer.
    // if (handle->rxBufferIndex >= BC_RX_WAKE_THRESH)
    // {
    //     __bic_SR_register_on_exit(LPM3_bits);
    // }
}


/**
  * @brief This function handles UART interrut requests.
  */
__attribute__ (( interrupt ( USCI_A1_VECTOR ))) void bcUartISR(void)
{
    hal_uart_interrupt(&hal_uart_a1);
}
