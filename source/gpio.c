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
   * @defgroup GPIO
   * @addtogroup GPIO
   * @{
   */
#include "kubos-hal/gpio.h"
#include "kubos-hal-stm32f4/stm32f4_gpio.h"

#include "stm32f4xx.h"
#include "stm32f4xx_hal_gpio.h"
#include "stm32f4xx_hal_rcc.h"

void k_gpio_init(int pin, KGPIOMode mode, KGPIOPullup pullup)
{
    // First enable the GPIO clock in RCC AHB1

    CHECK_SET_BIT(RCC->AHB1ENR, STM32F4_PIN_AHB1ENR_BIT(pin));

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

    params.Pin = STM32F4_PIN_MASK(pin);
    params.Speed = GPIO_SPEED_HIGH;
    HAL_GPIO_Init(STM32F4_PIN_GPIO(pin), &params);
}

unsigned int k_gpio_read(int pin)
{
    return HAL_GPIO_ReadPin(STM32F4_PIN_GPIO(pin),
                            STM32F4_PIN_MASK(pin)) == GPIO_PIN_SET ? 1 : 0;
}

void k_gpio_write(int pin, unsigned int val)
{
    HAL_GPIO_WritePin(STM32F4_PIN_GPIO(pin),
                      STM32F4_PIN_MASK(pin),
                      val == 0 ? GPIO_PIN_RESET : GPIO_PIN_SET);
}

void kprv_gpio_alt_config(GPIO_TypeDef* GPIOx, uint16_t GPIO_PinSource, uint8_t GPIO_AF)
{
    uint32_t temp = 0x00;
    uint32_t temp_2 = 0x00;

    temp = ((uint32_t)(GPIO_AF) << ((uint32_t)((uint32_t)GPIO_PinSource & (uint32_t)0x07) * 4));
    GPIOx->AFR[GPIO_PinSource >> 0x03] &= ~((uint32_t)0xF << ((uint32_t)((uint32_t)GPIO_PinSource & (uint32_t)0x07) * 4));
    temp_2 = GPIOx->AFR[GPIO_PinSource >> 0x03] | temp;
    GPIOx->AFR[GPIO_PinSource >> 0x03] = temp_2;
}

/* @} */
