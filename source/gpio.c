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

#include "stm32f4xx.h"
#include "stm32f4xx_hal_gpio.h"
#include "stm32f4xx_hal_rcc.h"

static KPinDesc pins[] = {
    {GPIOA, GPIO_PIN_0, RCC_AHB1ENR_GPIOAEN},
    {GPIOA, GPIO_PIN_1, RCC_AHB1ENR_GPIOAEN},
    {GPIOA, GPIO_PIN_2, RCC_AHB1ENR_GPIOAEN},
    {GPIOA, GPIO_PIN_3, RCC_AHB1ENR_GPIOAEN},
    {GPIOA, GPIO_PIN_4, RCC_AHB1ENR_GPIOAEN},
    {GPIOA, GPIO_PIN_5, RCC_AHB1ENR_GPIOAEN},
    {GPIOA, GPIO_PIN_6, RCC_AHB1ENR_GPIOAEN},
    {GPIOA, GPIO_PIN_7, RCC_AHB1ENR_GPIOAEN},
    {GPIOA, GPIO_PIN_8, RCC_AHB1ENR_GPIOAEN},
    {GPIOA, GPIO_PIN_9, RCC_AHB1ENR_GPIOAEN},
    {GPIOA, GPIO_PIN_10, RCC_AHB1ENR_GPIOAEN},
    {GPIOA, GPIO_PIN_11, RCC_AHB1ENR_GPIOAEN},
    {GPIOA, GPIO_PIN_12, RCC_AHB1ENR_GPIOAEN},
    {GPIOA, GPIO_PIN_13, RCC_AHB1ENR_GPIOAEN},
    {GPIOA, GPIO_PIN_14, RCC_AHB1ENR_GPIOAEN},
    {GPIOA, GPIO_PIN_15, RCC_AHB1ENR_GPIOAEN},
    {GPIOB, GPIO_PIN_0, RCC_AHB1ENR_GPIOBEN},
    {GPIOB, GPIO_PIN_1, RCC_AHB1ENR_GPIOBEN},
    {GPIOB, GPIO_PIN_2, RCC_AHB1ENR_GPIOBEN},
    {GPIOB, GPIO_PIN_3, RCC_AHB1ENR_GPIOBEN},
    {GPIOB, GPIO_PIN_4, RCC_AHB1ENR_GPIOBEN},
    {GPIOB, GPIO_PIN_5, RCC_AHB1ENR_GPIOBEN},
    {GPIOB, GPIO_PIN_6, RCC_AHB1ENR_GPIOBEN},
    {GPIOB, GPIO_PIN_7, RCC_AHB1ENR_GPIOBEN},
    {GPIOB, GPIO_PIN_8, RCC_AHB1ENR_GPIOBEN},
    {GPIOB, GPIO_PIN_9, RCC_AHB1ENR_GPIOBEN},
    {GPIOB, GPIO_PIN_10, RCC_AHB1ENR_GPIOBEN},
    {GPIOB, GPIO_PIN_11, RCC_AHB1ENR_GPIOBEN},
    {GPIOB, GPIO_PIN_12, RCC_AHB1ENR_GPIOBEN},
    {GPIOB, GPIO_PIN_13, RCC_AHB1ENR_GPIOBEN},
    {GPIOB, GPIO_PIN_14, RCC_AHB1ENR_GPIOBEN},
    {GPIOB, GPIO_PIN_15, RCC_AHB1ENR_GPIOBEN},
    {GPIOC, GPIO_PIN_0, RCC_AHB1ENR_GPIOCEN},
    {GPIOC, GPIO_PIN_1, RCC_AHB1ENR_GPIOCEN},
    {GPIOC, GPIO_PIN_2, RCC_AHB1ENR_GPIOCEN},
    {GPIOC, GPIO_PIN_3, RCC_AHB1ENR_GPIOCEN},
    {GPIOC, GPIO_PIN_4, RCC_AHB1ENR_GPIOCEN},
    {GPIOC, GPIO_PIN_5, RCC_AHB1ENR_GPIOCEN},
    {GPIOC, GPIO_PIN_6, RCC_AHB1ENR_GPIOCEN},
    {GPIOC, GPIO_PIN_7, RCC_AHB1ENR_GPIOCEN},
    {GPIOC, GPIO_PIN_8, RCC_AHB1ENR_GPIOCEN},
    {GPIOC, GPIO_PIN_9, RCC_AHB1ENR_GPIOCEN},
    {GPIOC, GPIO_PIN_10, RCC_AHB1ENR_GPIOCEN},
    {GPIOC, GPIO_PIN_11, RCC_AHB1ENR_GPIOCEN},
    {GPIOC, GPIO_PIN_12, RCC_AHB1ENR_GPIOCEN},
    {GPIOC, GPIO_PIN_13, RCC_AHB1ENR_GPIOCEN},
    {GPIOC, GPIO_PIN_14, RCC_AHB1ENR_GPIOCEN},
    {GPIOC, GPIO_PIN_15, RCC_AHB1ENR_GPIOCEN},
    {GPIOD, GPIO_PIN_0, RCC_AHB1ENR_GPIODEN},
    {GPIOD, GPIO_PIN_1, RCC_AHB1ENR_GPIODEN},
    {GPIOD, GPIO_PIN_2, RCC_AHB1ENR_GPIODEN},
    {GPIOD, GPIO_PIN_3, RCC_AHB1ENR_GPIODEN},
    {GPIOD, GPIO_PIN_4, RCC_AHB1ENR_GPIODEN},
    {GPIOD, GPIO_PIN_5, RCC_AHB1ENR_GPIODEN},
    {GPIOD, GPIO_PIN_6, RCC_AHB1ENR_GPIODEN},
    {GPIOD, GPIO_PIN_7, RCC_AHB1ENR_GPIODEN},
    {GPIOD, GPIO_PIN_8, RCC_AHB1ENR_GPIODEN},
    {GPIOD, GPIO_PIN_9, RCC_AHB1ENR_GPIODEN},
    {GPIOD, GPIO_PIN_10, RCC_AHB1ENR_GPIODEN},
    {GPIOD, GPIO_PIN_11, RCC_AHB1ENR_GPIODEN},
    {GPIOD, GPIO_PIN_12, RCC_AHB1ENR_GPIODEN},
    {GPIOD, GPIO_PIN_13, RCC_AHB1ENR_GPIODEN},
    {GPIOD, GPIO_PIN_14, RCC_AHB1ENR_GPIODEN},
    {GPIOD, GPIO_PIN_15, RCC_AHB1ENR_GPIODEN},
    {GPIOE, GPIO_PIN_0, RCC_AHB1ENR_GPIOEEN},
    {GPIOE, GPIO_PIN_1, RCC_AHB1ENR_GPIOEEN},
    {GPIOE, GPIO_PIN_2, RCC_AHB1ENR_GPIOEEN},
    {GPIOE, GPIO_PIN_3, RCC_AHB1ENR_GPIOEEN},
    {GPIOE, GPIO_PIN_4, RCC_AHB1ENR_GPIOEEN},
    {GPIOE, GPIO_PIN_5, RCC_AHB1ENR_GPIOEEN},
    {GPIOE, GPIO_PIN_6, RCC_AHB1ENR_GPIOEEN},
    {GPIOE, GPIO_PIN_7, RCC_AHB1ENR_GPIOEEN},
    {GPIOE, GPIO_PIN_8, RCC_AHB1ENR_GPIOEEN},
    {GPIOE, GPIO_PIN_9, RCC_AHB1ENR_GPIOEEN},
    {GPIOE, GPIO_PIN_10, RCC_AHB1ENR_GPIOEEN},
    {GPIOE, GPIO_PIN_11, RCC_AHB1ENR_GPIOEEN},
    {GPIOE, GPIO_PIN_12, RCC_AHB1ENR_GPIOEEN},
    {GPIOE, GPIO_PIN_13, RCC_AHB1ENR_GPIOEEN},
    {GPIOE, GPIO_PIN_14, RCC_AHB1ENR_GPIOEEN},
    {GPIOE, GPIO_PIN_15, RCC_AHB1ENR_GPIOEEN}
};

void k_gpio_init(int pin, KGPIOMode mode, KGPIOPullup pullup)
{
    // First enable the GPIO clock in RCC AHB1

    uint32_t tmpreg = READ_BIT(RCC->AHB1ENR, pins[pin].ahb1enr_bit);
    if (!tmpreg) {
        SET_BIT(RCC->AHB1ENR, pins[pin].ahb1enr_bit);
        /* Delay after an RCC peripheral clock enabling */
        tmpreg = READ_BIT(RCC->AHB1ENR, pins[pin].ahb1enr_bit);
    }

    GPIO_InitTypeDef params;
    switch (mode) {
        case K_GPIO_INPUT:
            params.Mode = GPIO_MODE_INPUT; break;
        case K_GPIO_OUTPUT:
            params.Mode = GPIO_MODE_OUTPUT_PP; break;
        case K_GPIO_OUTPUT_OD:
            params.Mode = GPIO_MODE_OUTPUT_OD; break;
        case K_GPIO_ALT:
            params.Mode = GPIO_MODE_AF_PP; break;
        case K_GPIO_ALT_OD:
            params.Mode = GPIO_MODE_AF_OD; break;
    }

    switch (pullup) {
        case K_GPIO_PULL_NONE:
            params.Pull = GPIO_NOPULL; break;
        case K_GPIO_PULL_UP:
            params.Pull = GPIO_PULLUP; break;
        case K_GPIO_PULL_DOWN:
            params.Pull = GPIO_PULLDOWN; break;
    }

    params.Pin = pins[pin].pin;
    params.Speed = GPIO_SPEED_HIGH;
    HAL_GPIO_Init(pins[pin].gpio, &params);
}

unsigned int k_gpio_read(int pin)
{
    return HAL_GPIO_ReadPin(pins[pin].gpio, pins[pin].pin) == GPIO_PIN_SET ? 1 : 0;
}

void k_gpio_write(int pin, unsigned int val)
{
    HAL_GPIO_WritePin(pins[pin].gpio, pins[pin].pin,
                      val == 0 ? GPIO_PIN_RESET : GPIO_PIN_SET);
}
