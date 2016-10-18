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
/**
  * @defgroup GPIO
  * @addtogroup GPIO
  * @{
  */

/**
  *
  * @file       gpio.h
  * @brief      MSP430F5529-HAL - GPIO module
  *
  * @author     kubos.co
  */

#ifndef HAL_UART_H
#define HAL_UART_H

#include <stdint.h>

/**
  * @brief Read in bit value on gpio pin.
  * @param pin
  * @param bit
  * @return uint8_t value
  */
uint8_t hal_gpio_read(volatile uint8_t * pin, uint8_t bit);

/**
  * @brief Write value to bit position on gpio pin.
  * @param pin
  * @param bit
  * @param value
  */
void hal_gpio_write(volatile uint8_t * pin, uint8_t bit, uint8_t value);


#endif
