/*
 * KubOS HAL
 * Copyright (C) 2016 Kubos Corporation
 
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
 * @defgroup STM32F4_HAL_I2C STM32F4 HAL I2C Interface
 * @addtogroup STM32F4_HAL_I2C
 * @{
 */
#if (defined YOTTA_CFG_HARDWARE_I2C) && (YOTTA_CFG_HARDWARE_I2C_COUNT > 0)
#ifndef K_I2C_STM32F4
#define K_I2C_STM32F4

#include "stm32f4xx_hal.h"
#include "stm32f4xx_hal_i2c.h"

/** Structure defining pins + config for I2C device */
typedef struct
{
    /** SCL pin number */
    uint16_t scl;
    /** SCL pin mode */
    uint16_t scl_mode;
    /** SCL pin pullup setting */
    uint16_t scl_pullup;
    /** SCL pin speed */
    uint16_t scl_speed;
    /** SDA pin number */
    uint16_t sda;
    /** SDA pin mode */
    uint16_t sda_mode;
    /** SDA pin pullup setting */
    uint16_t sda_pullup;
    /** SDA pin speed */
    uint16_t sda_speed;
    /** Alternate pin number */
    uint16_t alt;
    /** EV IRQ number */
    uint16_t ev_irqn;
    /** ER IRQ number */
    uint16_t er_irqn;
    /** STM32CubeF4 defined GPIO port */
    GPIO_TypeDef * gpio_port;
} hal_i2c_pins;

/** HAL structure for I2C device definition/configuration */
typedef struct
{
    /** Kubos-HAL structure */
    KI2C * ki2c;
    /** STM32CubeF4 HAL structure */
    I2C_HandleTypeDef hal_handle;
    /** GPIO config pins/ports */
    hal_i2c_pins pins;
} hal_i2c_handle;

#endif
#endif

/* @} */