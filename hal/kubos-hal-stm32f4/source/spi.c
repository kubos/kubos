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
 * @addtogroup STM32F4_HAL_SPI
 * @{
 */
#if (defined YOTTA_CFG_HARDWARE_SPI) && (YOTTA_CFG_HARDWARE_SPI_COUNT > 0)
#include "kubos-hal-stm32f4/spi.h"
#include "kubos-hal-stm32f4/pins.h"

/**
 * Fetches SPI bus data structure
 * @param[in] spi SPI bus num to fetch
 * @return hal_spi_handle* pointer to data structure
 */
static hal_spi_handle * hal_spi_get_handle(KSPINum spi);

/**
 * Initializes SPI bus structure with data needed to setup hardware
 * @param[in] spi higher level HAL SPI data
 * @return hal_spi_handle* NULL if bad bus num, otherwise data ready for dev setup
 */
static hal_spi_handle * hal_spi_device_init(KSPI * spi);

/**
 * Initializes the SPI according to the specified parameters
 * in the configuration and creates the associated handle.
 * @param[in,out] handle pointer to hal_spi_handle containing config information
 * @return KSPIStatus SPI_OK if success, otherwise a specific error flag
 */
static KSPIStatus hal_spi_hw_init(hal_spi_handle * handle);

/**
 * SPI hardware cleanup and disabling
 * @param[in] handle pointer to hal_spi_handle containing config information
 */
static void hal_spi_terminate(hal_spi_handle * handle);

/**
 * Initializes the SPI bus pins.
 * @param[in] handle pointer to hal_spi_handle containing config information
 */
static void hal_spi_gpio_init(hal_spi_handle * handle);

/**
 * Static array of SPI bus handles
 */
static hal_spi_handle hal_spi_dev[K_NUM_SPI];

/**
 * Default SPI request timeout value
 */
static uint32_t spi_timeout = 1000;

/* Functions implemented from KubOS-HAL SPI Interface */

/**
 * Setup and enable SPI bus
 * @param[in] spi_num SPI bus to initialize
 * @return KSPIStatus SPI_OK if success, otherwise a specific error flag
 */
KSPIStatus kprv_spi_dev_init(KSPINum spi_num)
{
    KSPI * spi = kprv_spi_get(spi_num);
    if(spi == NULL)
    {
    	return SPI_ERROR;
    }
    hal_spi_handle * handle = hal_spi_device_init(spi);
    if(handle == NULL)
    {
    	return SPI_ERROR;
    }
    return hal_spi_hw_init(handle);
}

/**
 * SPI hardware cleanup and disabling
 * @param[in] spi bus num to terminate
 * @return KSPIStatus SPI_OK if success, otherwise a specific error flag
 */
KSPIStatus kprv_spi_dev_terminate(KSPINum spi)
{
    hal_spi_handle * handle = hal_spi_get_handle(spi);
    if(handle == NULL)
    {
    	return SPI_ERROR;
    }
    hal_spi_terminate(handle);
    return SPI_OK;
}

/**
 * Write data over SPI bus
 * @param[in] spi SPI bus to write to
 * @param[in] buffer pointer to data buffer
 * @param[in] len length of data to write
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus kprv_spi_write(KSPINum spi, uint8_t * buffer, uint32_t len)
{
    hal_spi_handle * handle = hal_spi_get_handle(spi);
    if(handle == NULL)
    {
    	return SPI_ERROR;
    }
    HAL_StatusTypeDef status = HAL_SPI_Transmit(&(handle->hal_handle), buffer, len, spi_timeout);
    return (KSPIStatus)status;
}

/**
 * Read data over SPI bus
 * @param[in] spi SPI bus to read from
 * @param[out] buffer pointer to data buffer
 * @param[in] len length of data to read
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus kprv_spi_read(KSPINum spi, uint8_t * buffer, uint32_t len)
{
    hal_spi_handle * handle = hal_spi_get_handle(spi);
    if(handle == NULL)
    {
    	return SPI_ERROR;
    }
    HAL_StatusTypeDef status = HAL_SPI_Receive(&(handle->hal_handle), buffer, len, spi_timeout);
    return (KSPIStatus)status;
}

/**
 * Write and read data over SPI bus
 * @param[in] spi SPI bus to write to
 * @param[in] txBuffer pointer to data buffer to write from
 * @param[out] rxBuffer pointer to data buffer to read into
 * @param[in] len length of data to write and read
 * @return KSPIStatus SPI_OK on success, otherwise failure
 */
KSPIStatus kprv_spi_write_read(KSPINum spi, uint8_t * txBuffer, uint8_t * rxBuffer, uint32_t len)
{
    hal_spi_handle * handle = hal_spi_get_handle(spi);
    if(handle == NULL)
    {
    	return SPI_ERROR;
    }
    HAL_StatusTypeDef status = HAL_SPI_TransmitReceive(&(handle->hal_handle), txBuffer, rxBuffer, len, spi_timeout);
    return (KSPIStatus)status;
}

/* Private functions */

static hal_spi_handle * hal_spi_get_handle(KSPINum spi)
{
	if(spi > K_NUM_SPI-1)
	{
		return 0;
	}
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
#ifdef YOTTA_CFG_HARDWARE_SPI_SPI1
                case K_SPI1:
                {
                    handle->hal_handle.Instance = SPI1;
                    handle->pins.mosi = STM32F4_PIN_MASK(YOTTA_CFG_HARDWARE_SPI_SPI1_MOSI);
                    handle->pins.miso = STM32F4_PIN_MASK(YOTTA_CFG_HARDWARE_SPI_SPI1_MISO);
                    handle->pins.sck = STM32F4_PIN_MASK(YOTTA_CFG_HARDWARE_SPI_SPI1_SCK);
                    handle->pins.port = YOTTA_CFG_HARDWARE_SPI_SPI1_PORT;
                    handle->pins.alt = YOTTA_CFG_HARDWARE_SPI_SPI1_ALT;
                    break;
                }
#endif
#ifdef YOTTA_CFG_HARDWARE_SPI_SPI2
                case K_SPI2:
                {
                    handle->hal_handle.Instance = SPI2;
                    handle->pins.mosi = STM32F4_PIN_MASK(YOTTA_CFG_HARDWARE_SPI_SPI2_MOSI);
                    handle->pins.miso = STM32F4_PIN_MASK(YOTTA_CFG_HARDWARE_SPI_SPI2_MISO);
                    handle->pins.sck = STM32F4_PIN_MASK(YOTTA_CFG_HARDWARE_SPI_SPI2_SCK);
                    handle->pins.port = YOTTA_CFG_HARDWARE_SPI_SPI2_PORT;
                    handle->pins.alt = YOTTA_CFG_HARDWARE_SPI_SPI2_ALT;
                    break;
                }
#endif
#ifdef YOTTA_CFG_HARDWARE_SPI_SPI3
                case K_SPI3:
                {
                    handle->hal_handle.Instance = SPI3;
                    handle->pins.mosi = STM32F4_PIN_MASK(YOTTA_CFG_HARDWARE_SPI_SPI3_MOSI);
                    handle->pins.miso = STM32F4_PIN_MASK(YOTTA_CFG_HARDWARE_SPI_SPI3_MISO);
                    handle->pins.sck = STM32F4_PIN_MASK(YOTTA_CFG_HARDWARE_SPI_SPI3_SCK);
                    handle->pins.port = YOTTA_CFG_HARDWARE_SPI_SPI3_PORT;
                    handle->pins.alt = YOTTA_CFG_HARDWARE_SPI_SPI3_ALT;
                    break;
                }
#endif
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
    switch(handle->kspi->bus_num)
    {
#ifdef YOTTA_CFG_HARDWARE_SPI_SPI1
        case K_SPI1:
        {
            __HAL_RCC_SPI1_FORCE_RESET();
            __HAL_RCC_SPI1_RELEASE_RESET();
            break;
        }
#endif
#ifdef YOTTA_CFG_HARDWARE_SPI_SPI2
        case K_SPI2:
        {
            __HAL_RCC_SPI2_FORCE_RESET();
            __HAL_RCC_SPI2_RELEASE_RESET();
            break;
        }
#endif
#ifdef YOTTA_CFG_HARDWARE_SPI_SPI3
        case K_SPI3:
        {
            __HAL_RCC_SPI3_FORCE_RESET();
            __HAL_RCC_SPI3_RELEASE_RESET();
            break;
        }
#endif
        default:
        {
        }
    }

    /* de-init pins */
    HAL_GPIO_DeInit(handle->pins.port, handle->pins.mosi);
    HAL_GPIO_DeInit(handle->pins.port, handle->pins.miso);
    HAL_GPIO_DeInit(handle->pins.port, handle->pins.sck);
}

static KSPIStatus hal_spi_hw_init(hal_spi_handle * handle)
{
    SPI_HandleTypeDef * SPIHandle = &(handle->hal_handle);
    uint32_t freq;
    float prescaler;

    switch(handle->kspi->bus_num)
    {
#ifdef YOTTA_CFG_HARDWARE_SPI_SPI1
        case K_SPI1:
        {
            __HAL_RCC_SPI1_CLK_ENABLE();
            freq = HAL_RCC_GetPCLK2Freq();
            break;
        }
#endif
#ifdef YOTTA_CFG_HARDWARE_SPI_SPI2
        case K_SPI2:
        {
            __HAL_RCC_SPI2_CLK_ENABLE();
            freq = HAL_RCC_GetPCLK1Freq();
            break;
        }
#endif
#ifdef YOTTA_CFG_HARDWARE_SPI_SPI3
        case K_SPI3:
        {
            __HAL_RCC_SPI3_CLK_ENABLE();
            freq = HAL_RCC_GetPCLK1Freq();
            break;
        }
#endif
        default:
        {
            return SPI_ERROR; /* wrong bus num */
        }
    }
    /* Init pins */
    hal_spi_gpio_init(handle);

    /* Set options */
    KSPIConf conf = handle->kspi->config;

    //For the moment the STM32F4 only supports SPI in master mode
    if(conf.role == K_SPI_MASTER)
    {
    	SPIHandle->Init.Mode = SPI_MODE_MASTER;
    }
    else
    {
    	return SPI_ERROR;
    }

    if (conf.data_size == K_SPI_DATASIZE_8BIT)
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
        SPIHandle->Init.CLKPhase = SPI_PHASE_2EDGE;
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

    //Calculate closest prescaler value based on desired clock frequency
    if(conf.speed == 0)
    {
    	return SPI_ERROR;
    }

    prescaler = (float) freq / (float) conf.speed;

    if(prescaler <= 2)
    {
    	SPIHandle->Init.BaudRatePrescaler = SPI_BAUDRATEPRESCALER_2;
    }
    else if(prescaler <= 4)
    {
    	SPIHandle->Init.BaudRatePrescaler = SPI_BAUDRATEPRESCALER_4;
    }
    else if(prescaler <= 8)
	{
		SPIHandle->Init.BaudRatePrescaler = SPI_BAUDRATEPRESCALER_8;
	}
    else if(prescaler <= 16)
	{
		SPIHandle->Init.BaudRatePrescaler = SPI_BAUDRATEPRESCALER_16;
	}
    else if(prescaler <= 32)
	{
		SPIHandle->Init.BaudRatePrescaler = SPI_BAUDRATEPRESCALER_32;
	}
    else if(prescaler <= 64)
	{
		SPIHandle->Init.BaudRatePrescaler = SPI_BAUDRATEPRESCALER_64;
	}
    else if(prescaler <= 128)
	{
		SPIHandle->Init.BaudRatePrescaler = SPI_BAUDRATEPRESCALER_128;
	}
    else
	{
		SPIHandle->Init.BaudRatePrescaler = SPI_BAUDRATEPRESCALER_256;
	}

    /* Fill aditional SPI settings */
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

#endif

/* @} */
