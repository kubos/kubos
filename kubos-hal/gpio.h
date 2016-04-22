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
   *
   * @file       gpio.h
   * @brief      GPIO interface
   *
   * @author     kubos.co
   */

#ifndef K_GPIO_H
#define K_GPIO_H

#include "pins.h"

typedef enum {
    K_GPIO_INPUT = 0,
    K_GPIO_OUTPUT,
    K_GPIO_OUTPUT_OD,
    K_GPIO_ALT,
    K_GPIO_ALT_OD
} KGPIOMode;

typedef enum {
    K_GPIO_PULL_NONE = 0,
    K_GPIO_PULL_UP,
    K_GPIO_PULL_DOWN
} KGPIOPullup;

void k_gpio_init(int pin, KGPIOMode mode, KGPIOPullup pullup);

unsigned int k_gpio_read(int pin);
void k_gpio_write(int pin, unsigned int val);
void k_gpio_toggle(int pin);

#endif
