/*
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
#include "msp430f5529-hal/gpio.h"

uint8_t hal_gpio_read(volatile uint8_t * pin, uint8_t bit)
{
    return !(*pin & bit);
}

void hal_gpio_write(volatile uint8_t * pin, uint8_t bit, uint8_t value)
{
    if (0 == value)
    {
        *pin &= ~bit;
    }
    else if (1 == value)
    {
        *pin |= bit;
    }
}
