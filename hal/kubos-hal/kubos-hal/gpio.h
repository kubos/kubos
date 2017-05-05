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
 * @defgroup GPIO HAL GPIO Interface
 * @addtogroup GPIO
 * @{
 */

 /**
   *
   * @file       gpio.h
   * @brief      GPIO interface
   *
   * @author     kubos.co
   */

#ifndef K_GPIO_H
#define K_GPIO_H

#include "pins.h"

/**
 * Options for configuring GPIO pin mode.
 */
typedef enum {
    K_GPIO_INPUT = 0,
    K_GPIO_OUTPUT,
    K_GPIO_OUTPUT_OD,
    K_GPIO_ALT,
    K_GPIO_ALT_OD
} KGPIOMode;

/**
 * Options for configuring GPIO pull ups
 */
typedef enum {
    K_GPIO_PULL_NONE = 0,
    K_GPIO_PULL_UP,
    K_GPIO_PULL_DOWN
} KGPIOPullup;

/**
 * Initializes GPIO pin
 * @param [in] pin pin to initialize
 * @param [in] mode mode setting for pin
 * @param [in] pullup pullup setting for pin
 */
void k_gpio_init(int pin, KGPIOMode mode, KGPIOPullup pullup);

/**
 * Reads value from GPIO pin
 * @param [in] pin gpio pin to read from
 * @return int value which was read
 */
unsigned int k_gpio_read(int pin);

/**
 * Writes value to GPIO pin
 * @param [in] pin gpio pin to write to
 * @param [in] val value to write to pin
 */
void k_gpio_write(int pin, unsigned int val);

/**
 * Inverts current value of gpio pin
 * @param [in] pin gpio pin to invert
 */
void k_gpio_toggle(int pin);

#endif

/* @} */
