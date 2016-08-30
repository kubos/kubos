/*
 * @file    uart.c
 * @brief   MSP430F5529 UART HAL module
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

#if (defined YOTTA_CFG_HARDWARE_UART) && (YOTTA_CFG_HARDWARE_UART_COUNT > 0)
#include <msp430.h>
#include "msp430f5529-hal/uart.h"
#include <stddef.h>
#include <stdbool.h>

/**
  * @brief Low level hardware setup of UART baudrate.
  * @param handle Instance of initilaized hal_uart_handle containing hardware
  *               registers and config values.
  */
static void hal_uart_set_baudrate(hal_uart_handle * handle);

/**
  * @brief Low level hardware setup of UART parity.
  * @param handle Instance of initilaized hal_uart_handle containing hardware
  *               registers and config values.
  */
static void hal_uart_set_parity(hal_uart_handle * handle);

/**
  * @brief Low level hardware setup of UART stopbits.
  * @param handle Instance of initilaized hal_uart_handle containing hardware
  *               registers and config values.
  */
static void hal_uart_set_stopbits(hal_uart_handle * handle);


/**
  * @brief Low level hardware setup of UART word length.
  * @param handle Instance of initilaized hal_uart_handle containing hardware
  *               registers and config values.
  */
static void hal_uart_set_wordlen(hal_uart_handle * handle);

hal_uart_handle hal_uart_dev[YOTTA_CFG_HARDWARE_UART_COUNT];

hal_uart_handle * hal_uart_device_init(hal_uart_device device)
{
    hal_uart_handle * handle = NULL;

    handle = &hal_uart_dev[device];
    if (HAL_UART_A0 == device)
    {
        handle->config.device = device;
        handle->select = &P3SEL;
        handle->selectVal = BIT3 + BIT4;
        handle->reg = (hal_uart_mem_reg *)__MSP430_BASEADDRESS_USCI_A0__;
    }
    else if (HAL_UART_A1 == device)
    {
        handle->config.device = device;
        handle->select = &P4SEL;
        handle->selectVal = BIT4 + BIT5;
        handle->reg = (hal_uart_mem_reg *)__MSP430_BASEADDRESS_USCI_A1__;
    }
    return handle;
}


hal_uart_handle * hal_uart_init(hal_uart_config config)
{
    hal_uart_handle * handle = hal_uart_device_init(config.device);
    handle->config = config;
    return handle;
}


void hal_uart_setup(hal_uart_handle * handle)
{
    if (NULL != handle)
    {
        // Put the USCI state machine in reset
        handle->reg->control1 |= UCSWRST;
        // Use SMCLK as the bit clock
        handle->reg->control1 |= UCSSEL__SMCLK;

        hal_uart_set_baudrate(handle);

        hal_uart_set_parity(handle);

        hal_uart_set_wordlen(handle);

        hal_uart_set_stopbits(handle);

        // Configure pins as TXD/RXD
        *(handle->select) |= handle->selectVal;

        // Take the USCI out of reset
        handle->reg->control1 &= ~UCSWRST;

        // Enable the RX interrupt.
        handle->reg->interruptEnable |= UCRXIE;
    }
}

void hal_uart_terminate(hal_uart_handle * handle)
{
    if (NULL != handle)
    {
        // Disable RX and TX interrutps
        handle->reg->interruptEnable &= ~ (UCRXIE | UCTXIE);

        // Put the USCI peripheral into reset
        handle->reg->control1 |= UCSWRST;
    }
}


static void hal_uart_set_baudrate(hal_uart_handle * handle)
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

static void hal_uart_set_parity(hal_uart_handle * handle)
{
    switch(handle->config.parity)
    {
        case HAL_UART_PARITY_NONE:
        {
            handle->reg->control0 &= ~UCPEN;
            break;
        }
        case HAL_UART_PARITY_EVEN:
        {
            // enable parity
            handle->reg->control0 |= UCPEN;
            // set even parity
            handle->reg->control0 |= UCPAR;
            break;
        }
        case HAL_UART_PARITY_ODD:
        {
            // enable parity
            handle->reg->control0 |= UCPEN;
            // set odd parity
            handle->reg->control0 &= ~UCPAR;
            break;
        }
    }
}

static void hal_uart_set_wordlen(hal_uart_handle * handle)
{
    switch (handle->config.wordlen)
    {
        case HAL_UART_WORD_LEN_7:
        {
            handle->reg->control0 |= UC7BIT;
            break;
        }
        case HAL_UART_WORD_LEN_8:
        {
            handle->reg->control0 &= ~UC7BIT;
            break;
        }
    }
}

static void hal_uart_set_stopbits(hal_uart_handle * handle)
{
    switch (handle->config.stopbits)
    {
        case HAL_UART_STOP_BITS_1:
        {
            handle->reg->control0 &= ~UCSPB;
            break;
        }
        case HAL_UART_STOP_BITS_2:
        {
            handle->reg->control0 |= UCSPB;
            break;
        }
    }
}


uint8_t hal_uart_read_raw(hal_uart_handle * handle)
{
    return handle->reg->rx;
}


void hal_uart_write_raw(hal_uart_handle * handle, uint8_t c)
{
    handle->reg->tx = c;
}


void hal_uart_write(hal_uart_handle * handle, uint8_t c)
{
    hal_uart_write_raw(handle, c);
    // Wait until bit has been clocked out...
    while (!HAL_UART_INT_FLAG(handle, UCTXIFG) && HAL_UART_STAT(handle, UCBUSY));
}


void hal_uart_write_str(hal_uart_handle * handle, uint8_t * buf, uint8_t len)
{
    uint8_t i = 0;

    for (i = 0; i < len; i++)
        hal_uart_write(handle, buf[i]);
}


/**
  * @brief This function is the USCI A0 interrupt handler.
  */
__attribute__ (( interrupt ( USCI_A0_VECTOR ))) void hal_uart_a0_interrupt(void)
{
    hal_uart_interrupt(&hal_uart_dev[HAL_UART_A0]);
}


/**
  * @brief This function is the USCI A1 interrupt handler.
  */
__attribute__ (( interrupt ( USCI_A1_VECTOR ))) void hal_uart_a1_interrupt(void)
{
    hal_uart_interrupt(&hal_uart_dev[HAL_UART_A1]);
}

#endif
