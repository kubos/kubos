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
 * @defgroup STM32F4_HAL_SPI STM32F4 HAL SPI Interface
 * @addtogroup STM32F4_HAL_SPI
 * @{
 */
#if (defined YOTTA_CFG_HARDWARE_SPI) && (YOTTA_CFG_HARDWARE_SPI_COUNT > 0)
#ifndef K_SPI_HAL_H
#define K_SPI_HAL_H

#include "kubos-hal/spi.h"
#include "stm32f4xx.h"
#include "stm32f4xx_hal_spi.h"

/** Structure defining pins for SPI device */
typedef struct
{
    /** Master-In Slave-Out */
    uint16_t miso;
    /** Master-Out Slave-In */
    uint16_t mosi;
    /** Clock */
    uint16_t sck;
    /** STM32CubeF4 GPIO port */
    GPIO_TypeDef * port;
    /** Alternate pin setting */
    uint16_t alt;
} hal_spi_pins;

/** Structure for SPI device */
typedef struct
{
    /** KubOS-HAL structure */
    KSPI * kspi;
    /** STM32CubeF4's special structure */
    SPI_HandleTypeDef hal_handle;
    /** SPI pins struct */
    hal_spi_pins pins;
} hal_spi_handle;

#endif
#endif

/* @} */