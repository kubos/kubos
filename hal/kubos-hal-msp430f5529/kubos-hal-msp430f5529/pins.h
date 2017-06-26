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
 * @addtogroup MSP430F5529_HAL_GPIO
 * @{
 */

#ifndef KUBOS_HAL_MSP430F5529_PINS_H
#define KUBOS_HAL_MSP430F5529_PINS_H

#include <stdint.h>

/** LED 0 pin */
#define K_LED_0 P10
/** Red LED pin */
#define K_LED_RED P10

/** LED 1 pin */
#define K_LED_1 P47
/** Green LED pin */
#define K_LED_GREEN P47

/** Number of LEDs available */
#define K_LED_COUNT 2

/** User button pin */
#define K_BUTTON_0 P21


/**
  * Struct which contains GPIO details (dir/out/in/pull pins and bit)
  */
typedef struct {
    /** PxDIR */
    volatile uint8_t * dir_pin;
    /** PxOUT */
    volatile uint8_t * out_pin;
    /** PxIN */
    volatile uint8_t * in_pin;
    /** PxREN */
    volatile uint8_t * pull_pin;
    /** Specific bit to read/write from */
    uint8_t  bit;
} KPinDesc;

/**
  * Enum containing GPIOs currently setup.
  *
  * Note - This describes digital pins exposed on the MSP430F5529 launchpad.
  *
  */
typedef enum {
    P10 = 0, P11, P12, P13, P14, P15, P16,
    P20, P21, P22, P23, P24, P25, P26, P27,
    P30, P31, P32, P33, P34, P35, P36, P37,
    P40, P41, P42, P43, P44, P45, P47,
    P60, P61, P62, P63, P64, P65, P66,
    P70, P74,
    P81, P82
} KPin;

#endif

/* @} */