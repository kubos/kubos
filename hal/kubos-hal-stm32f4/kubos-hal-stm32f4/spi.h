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
  * @defgroup STM32F4_HAL_SPI
  * @addtogroup STM32F4_HAL_SPI
  * @{
  */
 #if (defined YOTTA_CFG_HARDWARE_SPI) && (YOTTA_CFG_HARDWARE_SPI_COUNT > 0)
#ifndef K_SPI_HAL_H
#define K_SPI_HAL_H

#include "kubos-hal/spi.h"

#include "stm32f4xx.h"
#include "stm32f4xx_hal_spi.h"

typedef struct
{
    uint16_t miso;
    uint16_t mosi;
    uint16_t sck;
    GPIO_TypeDef * port;
    uint16_t alt;
} hal_spi_pins;

typedef struct {
    /* KubOS-HAL structure */
    KSPI * kspi;
    /* STM32CubeF4's special structure */
    SPI_HandleTypeDef hal_handle;
    /* spi pins struct */
    hal_spi_pins pins;
} hal_spi_handle;

#endif
#endif

/* @} */