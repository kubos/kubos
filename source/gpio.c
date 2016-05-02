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

#include "kubos-hal/gpio.h"

#include <stddef.h>
#include <msp430f5529.h>
#include <msp430.h>


static KPinDesc pins[] = {
    {(uint8_t*)&P1DIR, (uint8_t*)&P1OUT, (uint8_t*)&P1IN, NULL, BIT0},
    {(uint8_t*)&P4DIR, (uint8_t*)&P4OUT, (uint8_t*)&P4IN, NULL, BIT7},
    {(uint8_t*)&P2DIR, (uint8_t*)&P2OUT, (uint8_t*)&P2IN, (uint8_t*)&P2REN, BIT1}
};

void k_gpio_init(int pin, KGPIOMode mode, KGPIOPullup pullup)
{
    *(pins[pin].dir_pin) = pins[pin].bit;
    if (K_GPIO_PULL_UP == pullup)
    {
        *(pins[pin].pull_pin) = pins[pin].bit;
    }
    else if (K_GPIO_PULL_DOWN == pullup)
    {
        *(pins[pin].pull_pin) = ~pins[pin].bit;
    }
}

unsigned int k_gpio_read(int pin)
{
    return !(*(pins[pin].in_pin) & pins[pin].bit);
}

void k_gpio_write(int pin, unsigned int val)
{
    if (0 == val)
    {
        *(pins[pin].out_pin) = ~pins[pin].bit;
    }
    else if (1 == val)
    {
        *(pins[pin].out_pin) = pins[pin].bit;
    }
}
