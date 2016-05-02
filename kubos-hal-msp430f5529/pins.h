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

#ifndef KUBOS_HAL_MSP430F5529_PINS_H
#define KUBOS_HAL_MSP430F5529_PINS_H


#include <stdint.h>

#define K_LED_0 P1_0
#define K_LED_RED P1_0

#define K_LED_1 P4_7
#define K_LED_GREEN P4_7

#define K_LED_COUNT 2

#define K_BUTTON_0 P2_1


typedef struct {
    volatile uint8_t * dir_pin;
    volatile uint8_t * out_pin;
    volatile uint8_t * in_pin;
    volatile uint8_t * pull_pin;
    uint8_t  bit;
} KPinDesc;

typedef enum {
    P1_0 = 0, P4_7, P2_1
} KPin;

#endif
