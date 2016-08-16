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

#include "kubos-hal-stm32f4/spi.h"


static hal_spi_handle * hal_spi_get_handle(KSPINum spi);
static hal_spi_handle * hal_spi_device_init(KSPI * spi);
static KSPIStatus hal_spi_hw_init(hal_spi_handle * handle);
static void hal_spi_terminate(hal_spi_handle * handle);
static void hal_spi_gpio_init(hal_spi_handle * handle);

static hal_spi_handle hal_spi_dev[K_NUM_SPI];
static uint32_t spi_timeout = 1000;

/** Functions implemented from KubOS-HAL SPI Interface **/

void kprv_spi_dev_init(KSPINum spi_num)
{
    KSPI * spi = kprv_spi_get(spi_num);
    hal_spi_handle * handle = hal_spi_device_init(spi);
    hal_spi_hw_init(handle);
}

void kprv_spi_dev_terminate(KSPINum spi)
{
    hal_spi_handle * handle = hal_spi_get_handle(spi);
    hal_spi_terminate(handle);
}

KSPIStatus kprv_spi_write(KSPINum spi, uint8_t * buffer, uint32_t len)
{
    hal_spi_handle * handle = hal_spi_get_handle(spi);
    HAL_StatusTypeDef status = HAL_SPI_Transmit(&(handle->hal_handle), buffer, len, spi_timeout);
    return (KSPIStatus)status;
}

KSPIStatus kprv_spi_read(KSPINum spi, uint8_t * buffer, uint32_t len)
{
    hal_spi_handle * handle = hal_spi_get_handle(spi);
    HAL_StatusTypeDef status = HAL_SPI_Receive(&(handle->hal_handle), buffer, len, spi_timeout);
    return (KSPIStatus)status;
}

KSPIStatus kprv_spi_write_read(KSPINum spi, uint8_t * txBuffer, uint8_t * rxBuffer, uint32_t len)
{
    hal_spi_handle * handle = hal_spi_get_handle(spi);
    HAL_StatusTypeDef status = HAL_SPI_TransmitReceive(&(handle->hal_handle), txBuffer, rxBuffer, len, spi_timeout);
    return (KSPIStatus)status;
}

/** Private functions **/

static hal_spi_handle * hal_spi_get_handle(KSPINum spi)
{
    return &hal_spi_dev[spi];
}

static hal_spi_handle * hal_spi_device_init(KSPI * spi)
{
    hal_spi_handle * handle = NULL;
    if (spi != NULL)
    {
        handle = hal_spi_get_handle(spi->bus_num);
        if (handle != NULL)
        {
            handle->kspi = spi;
            switch(spi->bus_num)
            {
                case K_SPI1:
                {
                    handle->hal_handle.Instance = SPI1;
                    handle->pins.mosi = YOTTA_CFG_HARDWARE_SPI_SPI1_MOSI;
                    handle->pins.miso = YOTTA_CFG_HARDWARE_SPI_SPI1_MISO;
                    handle->pins.sck = YOTTA_CFG_HARDWARE_SPI_SPI1_SCK;
                    handle->pins.port = YOTTA_CFG_HARDWARE_SPI_SPI1_PORT;
                    handle->pins.alt = YOTTA_CFG_HARDWARE_SPI_SPI1_ALT;
                    break;
                }
                case K_SPI2:
                {
                    handle->hal_handle.Instance = SPI2;
                    handle->pins.mosi = YOTTA_CFG_HARDWARE_SPI_SPI2_MOSI;
                    handle->pins.miso = YOTTA_CFG_HARDWARE_SPI_SPI2_MISO;
                    handle->pins.sck = YOTTA_CFG_HARDWARE_SPI_SPI2_SCK;
                    handle->pins.port = YOTTA_CFG_HARDWARE_SPI_SPI2_PORT;
                    handle->pins.alt = YOTTA_CFG_HARDWARE_SPI_SPI2_ALT;
                    break;
                }
                case K_SPI3:
                {
                    handle->hal_handle.Instance = SPI3;
                    handle->pins.mosi = YOTTA_CFG_HARDWARE_SPI_SPI3_MOSI;
                    handle->pins.miso = YOTTA_CFG_HARDWARE_SPI_SPI3_MISO;
                    handle->pins.sck = YOTTA_CFG_HARDWARE_SPI_SPI3_SCK;
                    handle->pins.port = YOTTA_CFG_HARDWARE_SPI_SPI3_PORT;
                    handle->pins.alt = YOTTA_CFG_HARDWARE_SPI_SPI3_ALT;
                    break;
                }
                default:
                {
                    handle = NULL;
                }
            }
        }
    }
    return handle;
}

static void hal_spi_terminate(hal_spi_handle * handle)
{
    __HAL_RCC_SPI1_FORCE_RESET();
    __HAL_RCC_SPI1_RELEASE_RESET();

    /* de-init pins */
    HAL_GPIO_DeInit(GPIOA, GPIO_PIN_5);
    HAL_GPIO_DeInit(GPIOA, GPIO_PIN_6);
    HAL_GPIO_DeInit(GPIOA, GPIO_PIN_7);
}

static KSPIStatus hal_spi_hw_init(hal_spi_handle * handle)
{
    SPI_HandleTypeDef * SPIHandle = &(handle->hal_handle);

    switch(handle->kspi->bus_num)
    {
        case K_SPI1:
        {
            __HAL_RCC_SPI1_CLK_ENABLE();
            break;
        }
        case K_SPI2:
        {
            __HAL_RCC_SPI2_CLK_ENABLE();
            break;
        }
        case K_SPI3:
        {
            __HAL_RCC_SPI3_CLK_ENABLE();
            break;
        }
        default:
        {
            return SPI_ERROR; /* wrong bus num */
        }
    }
    /* Init pins */
    hal_spi_gpio_init(handle);

    /* Set options */
    KSPIConf conf = handle->kspi->config;

    if (conf.clock_phase == K_SPI_DATASIZE_8BIT)
    {
        SPIHandle->Init.DataSize = SPI_DATASIZE_8BIT;
    }
    else /* 16 bit */
    {
        SPIHandle->Init.DataSize = SPI_DATASIZE_16BIT;
    }

    if (conf.clock_phase == K_SPI_CPHA_1EDGE)
    {
        SPIHandle->Init.CLKPhase = SPI_PHASE_1EDGE;
    }
    else /* 2 edge */
    {
        SPIHandle->Init.CLKPhase = SPI_PHASE_1EDGE;
    }

    if (conf.clock_polarity == K_SPI_CPOL_LOW)
    {
        SPIHandle->Init.CLKPolarity = SPI_POLARITY_LOW;
    }
    else /* high pol */
    {
        SPIHandle->Init.CLKPolarity = SPI_POLARITY_HIGH;
    }

    if (conf.first_bit == K_SPI_FIRSTBIT_MSB)
    {
        SPIHandle->Init.FirstBit = SPI_FIRSTBIT_MSB;
    }
    else /* LSB */
    {
        SPIHandle->Init.FirstBit = SPI_FIRSTBIT_LSB;
    }

    switch(conf.direction)
    {
        case K_SPI_DIRECTION_2LINES:
        {
            SPIHandle->Init.Direction = SPI_DIRECTION_2LINES;
            break;
        }
        case K_SPI_DIRECTION_2LINES_RXONLY:
        {
            SPIHandle->Init.Direction = SPI_DIRECTION_2LINES_RXONLY;
            break;
        }
        case K_SPI_DIRECTION_1LINE:
        {
            SPIHandle->Init.Direction = SPI_DIRECTION_1LINE;
            break;
        }
        default: /* default to 2line */
        {
            SPIHandle->Init.Direction = SPI_DIRECTION_2LINES;
        }
    }

    /* Fill aditional SPI settings */
    SPIHandle->Init.BaudRatePrescaler = SPI_BAUDRATEPRESCALER_256;
    SPIHandle->Init.Mode = SPI_MODE_MASTER;
    SPIHandle->Init.CRCCalculation = SPI_CRCCALCULATION_DISABLE;
    SPIHandle->Init.CRCPolynomial = 7;
    SPIHandle->Init.TIMode = SPI_TIMODE_DISABLE;
    SPIHandle->Init.NSS = SPI_NSS_SOFT;

    /* Disable first */
    __HAL_SPI_DISABLE(SPIHandle);

    if (HAL_SPI_Init(SPIHandle) != HAL_OK)
    {
        return SPI_ERROR;
    }

    /* Enable SPI */
    __HAL_SPI_ENABLE(SPIHandle);

    return SPI_OK;
}


static void hal_spi_gpio_init(hal_spi_handle * handle)
{

    GPIO_InitTypeDef  GPIO_InitStruct;
    /* SPI GPIO Init */
    GPIO_InitStruct.Pin       = handle->pins.mosi | handle->pins.miso | handle->pins.sck;
    GPIO_InitStruct.Mode      = GPIO_MODE_AF_PP;
    GPIO_InitStruct.Pull      = GPIO_NOPULL;
    GPIO_InitStruct.Speed     = GPIO_SPEED_HIGH;
    GPIO_InitStruct.Alternate = handle->pins.alt;
    HAL_GPIO_Init(handle->pins.port, &GPIO_InitStruct);

}
