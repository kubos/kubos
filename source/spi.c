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

#include "kubos-hal/spi.h"
#include "msp430f5529-hal/spi.h"
#include "FreeRTOS.h"
#include "task.h"
#include <msp430.h>

//hal_spi_handle hal_spi_buses[YOTTA_CFG_HARDWARE_SPICOUNT];
hal_spi_handle hal_spi_buses[2];

/* defines for register timeout mode */
#define SET 0
#define RELEASE 1

static void hal_spi_set_clock(hal_spi_handle * handle);
static hal_spi_status hal_spi_register_timeout(hal_spi_handle * handle, uint8_t flag, uint8_t mode);


hal_spi_handle * hal_spi_device_init(hal_spi_bus spi)
{
    hal_spi_handle * handle = NULL;

    handle = &hal_spi_buses[spi];

    if (HAL_SPI_B0 == spi)
    {
        handle->select = &P3SEL;
        handle->selectVal = BIT0 + BIT1 + BIT2; /* somi, simo, clk */
        handle->reg = (hal_spi_mem_reg *)__MSP430_BASEADDRESS_USCI_B0__;
    }
    else if (HAL_SPI_B1 == spi)
    {
        handle->select = &P4SEL;
        handle->selectVal = BIT1 + BIT2 + BIT3; /* simo, somi, clk */
        handle->reg = (hal_spi_mem_reg *)__MSP430_BASEADDRESS_USCI_B1__;
    }
    return handle;
}

hal_spi_handle * hal_spi_init(hal_spi_conf config, hal_spi_bus spi)
{
    hal_spi_handle * handle = hal_spi_device_init(spi);
    handle->conf = config;
    return handle;
}

static void hal_spi_set_clock(hal_spi_handle * handle)
{
    /* SMCLK FREQ constant 1 MHz for F5529 */
    const uint32_t SMCLK_FREQ = 1000000;
    uint8_t preScalar;
    preScalar = (uint8_t)(SMCLK_FREQ/handle->conf.speed);

    handle->reg->control1 |= UCSSEL_2 | UCSWRST; /* SMCLK + keep reset */
    handle->reg->baudrate0 = preScalar;
    handle->reg->baudrate1 = 0;
}

void hal_spi_setup(hal_spi_handle * handle)
{
    /* configure pins */
    *(handle->select) |= handle->selectVal;

    handle->reg->control1 |= UCSWRST; /* software reset */
    /* Mode3 2line, 8-bit SPI master, clock polarity=1, clock phase=0 */
    handle->reg->control0 |= UCMST | UCSYNC | UCCKPL | UCMSB;

    hal_spi_set_clock(handle);

    handle->reg->control1 &= ~UCSWRST; /* enable spi by releasing reset */
}

void hal_spi_dev_terminate(hal_spi_handle * handle)
{
    handle->reg->control1 |= UCSWRST; /* software reset */
    handle->reg->control0 &= ~(UCMST | UCSYNC | UCCKPL | UCMSB); /* clear CTL0 */
    handle->reg->control1 &= ~UCSWRST; /* releasing reset */

    /* de-select pins */
    *(handle->select) &= ~handle->selectVal;
}

static hal_spi_status hal_spi_register_timeout(hal_spi_handle * handle, uint8_t flag, uint8_t mode)
{
    /* timeout counter */
    int timeout = 50;

    /* set register based on mode */
    if(mode == RELEASE)
    {
        /* while waiting for status register to clear */
        while((handle->reg->status & flag) && timeout > 0)
        {
            vTaskDelay(5); /* wait */
            timeout--; /* decrease counter */
        }
    }
    else /* SET */
    {
        /* while waiting for interrupt register to set */
        while(!(handle->reg->interruptFlags & flag) && timeout > 0)
        {
            vTaskDelay(5); /* wait */
            timeout--; /* decrease counter */
        }
    }

    /* if we timed out */
    if(timeout <= 0)
    {
        return HAL_SPI_ERROR_TIMEOUT;
    }

    /* success */
    return HAL_SPI_OK;
}

hal_spi_status hal_spi_master_write(hal_spi_handle * handle, uint8_t *buffer, int len)
{
    hal_spi_status ret = HAL_SPI_ERROR;

    int i = 0; /* loop variable */

    /* send data */
    for (; i < len; i++, buffer++)
    {
        /* wait for TX ready to set */
        if((ret = hal_spi_register_timeout(handle, UCTXIFG, SET)) != HAL_SPI_OK)
        {
            return ret; /* error */
        }

        /* put byte into register */
        handle->reg->txBuffer = *buffer;

        /* wait for TX to finish */
        if((ret = hal_spi_register_timeout(handle, UCBUSY, RELEASE)) != HAL_SPI_OK)
        {
            return ret; /* error */
        }
    }

    return HAL_SPI_OK;
}

hal_spi_status hal_spi_master_read(hal_spi_handle * handle, uint8_t *buffer, int len)
{
    hal_spi_status ret = HAL_SPI_ERROR;

    int i = 0; /* loop variable */


    /* send dummy data and receive data */
    for (; i < len; i++, buffer++)
    {
        /* wait for TX ready to set */
        if((ret = hal_spi_register_timeout(handle, UCTXIFG, SET)) != HAL_SPI_OK)
        {
            return ret; /* error */
        }

        /* put dummy byte into register */
        handle->reg->txBuffer = 0xFF;

        /* wait for RX ready to set */
        if((ret = hal_spi_register_timeout(handle, UCRXIFG, SET)) != HAL_SPI_OK)
        {
            return ret; /* error */
        }

        /* put rx'd byte into buffer */
        *buffer = handle->reg->rxBuffer;
    }

    return HAL_SPI_OK;
}

hal_spi_status hal_spi_master_write_read(hal_spi_handle * handle, uint8_t *txBuffer, uint8_t *rxBuffer, int len)
{
    hal_spi_status ret = HAL_SPI_ERROR;

    int i = 0; /* loop variable */


    /* send data and receive data */
    for (; i < len; i++, txBuffer++, rxBuffer++)
    {
        /* wait for TX ready to set */
        if((ret = hal_spi_register_timeout(handle, UCTXIFG, SET)) != HAL_SPI_OK)
        {
            return ret; /* error */
        }

        /* put data byte into register */
        handle->reg->txBuffer = *txBuffer;

        /* wait for RX ready to set */
        if((ret = hal_spi_register_timeout(handle, UCRXIFG, SET)) != HAL_SPI_OK)
        {
            return ret; /* error */
        }

        /* put rx'd byte into buffer */
        *rxBuffer = handle->reg->rxBuffer;
    }

    return HAL_SPI_OK;
}
