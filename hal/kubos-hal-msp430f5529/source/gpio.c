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
 * @defgroup MSP430F5529_HAL_GPIO MSP430F5529 HAL GPIO Interface
 * @addtogroup MSP430F5529_HAL_GPIO
 * @{
 */

#include "kubos-hal/gpio.h"
#include "msp430f5529-hal/gpio.h"

#include <stddef.h>
#include <msp430f5529.h>
#include <msp430.h>


/**
  * Static array of GPIO setup (dir, out, in pins, specific bit).
  *
  * Note - This (only) describes digital pins exposed on the MSP430F5529 launchpad.
  *
  */
static KPinDesc pins[] = {
    /* P10 */ {(uint8_t*)&P1DIR, (uint8_t*)&P1OUT, (uint8_t*)&P1IN, (uint8_t*)&P1REN, BIT0},
    /* P11 */ {(uint8_t*)&P1DIR, (uint8_t*)&P1OUT, (uint8_t*)&P1IN, (uint8_t*)&P1REN, BIT1},
    /* P12 */ {(uint8_t*)&P1DIR, (uint8_t*)&P1OUT, (uint8_t*)&P1IN, (uint8_t*)&P1REN, BIT2},
    /* P13 */ {(uint8_t*)&P1DIR, (uint8_t*)&P1OUT, (uint8_t*)&P1IN, (uint8_t*)&P1REN, BIT3},
    /* P14 */ {(uint8_t*)&P1DIR, (uint8_t*)&P1OUT, (uint8_t*)&P1IN, (uint8_t*)&P1REN, BIT4},
    /* P15 */ {(uint8_t*)&P1DIR, (uint8_t*)&P1OUT, (uint8_t*)&P1IN, (uint8_t*)&P1REN, BIT5},
    /* P16 */ {(uint8_t*)&P1DIR, (uint8_t*)&P1OUT, (uint8_t*)&P1IN, (uint8_t*)&P1REN, BIT6},

    /* P20 */ {(uint8_t*)&P2DIR, (uint8_t*)&P2OUT, (uint8_t*)&P2IN, (uint8_t*)&P2REN, BIT0},
    /* P21 */ {(uint8_t*)&P2DIR, (uint8_t*)&P2OUT, (uint8_t*)&P2IN, (uint8_t*)&P2REN, BIT1},
    /* P22 */ {(uint8_t*)&P2DIR, (uint8_t*)&P2OUT, (uint8_t*)&P2IN, (uint8_t*)&P2REN, BIT2},
    /* P23 */ {(uint8_t*)&P2DIR, (uint8_t*)&P2OUT, (uint8_t*)&P2IN, (uint8_t*)&P2REN, BIT3},
    /* P24 */ {(uint8_t*)&P2DIR, (uint8_t*)&P2OUT, (uint8_t*)&P2IN, (uint8_t*)&P2REN, BIT4},
    /* P25 */ {(uint8_t*)&P2DIR, (uint8_t*)&P2OUT, (uint8_t*)&P2IN, (uint8_t*)&P2REN, BIT5},
    /* P26 */ {(uint8_t*)&P2DIR, (uint8_t*)&P2OUT, (uint8_t*)&P2IN, (uint8_t*)&P2REN, BIT6},
    /* P27 */ {(uint8_t*)&P2DIR, (uint8_t*)&P2OUT, (uint8_t*)&P2IN, (uint8_t*)&P2REN, BIT7},

    /* P30 */ {(uint8_t*)&P3DIR, (uint8_t*)&P3OUT, (uint8_t*)&P3IN, (uint8_t*)&P3REN, BIT0},
    /* P31 */ {(uint8_t*)&P3DIR, (uint8_t*)&P3OUT, (uint8_t*)&P3IN, (uint8_t*)&P3REN, BIT1},
    /* P32 */ {(uint8_t*)&P3DIR, (uint8_t*)&P3OUT, (uint8_t*)&P3IN, (uint8_t*)&P3REN, BIT2},
    /* P33 */ {(uint8_t*)&P3DIR, (uint8_t*)&P3OUT, (uint8_t*)&P3IN, (uint8_t*)&P3REN, BIT3},
    /* P34 */ {(uint8_t*)&P3DIR, (uint8_t*)&P3OUT, (uint8_t*)&P3IN, (uint8_t*)&P3REN, BIT4},
    /* P35 */ {(uint8_t*)&P3DIR, (uint8_t*)&P3OUT, (uint8_t*)&P3IN, (uint8_t*)&P3REN, BIT5},
    /* P36 */ {(uint8_t*)&P3DIR, (uint8_t*)&P3OUT, (uint8_t*)&P3IN, (uint8_t*)&P3REN, BIT6},
    /* P37 */ {(uint8_t*)&P3DIR, (uint8_t*)&P3OUT, (uint8_t*)&P3IN, (uint8_t*)&P3REN, BIT7},

    /* P40 */ {(uint8_t*)&P4DIR, (uint8_t*)&P4OUT, (uint8_t*)&P4IN, (uint8_t*)&P4REN, BIT0},
    /* P41 */ {(uint8_t*)&P4DIR, (uint8_t*)&P4OUT, (uint8_t*)&P4IN, (uint8_t*)&P4REN, BIT1},
    /* P42 */ {(uint8_t*)&P4DIR, (uint8_t*)&P4OUT, (uint8_t*)&P4IN, (uint8_t*)&P4REN, BIT2},
    /* P43 */ {(uint8_t*)&P4DIR, (uint8_t*)&P4OUT, (uint8_t*)&P4IN, (uint8_t*)&P4REN, BIT3},
    /* P44 */ {(uint8_t*)&P4DIR, (uint8_t*)&P4OUT, (uint8_t*)&P4IN, (uint8_t*)&P4REN, BIT4},
    /* P45 */ {(uint8_t*)&P4DIR, (uint8_t*)&P4OUT, (uint8_t*)&P4IN, (uint8_t*)&P4REN, BIT5},
    /* P47 */ {(uint8_t*)&P4DIR, (uint8_t*)&P4OUT, (uint8_t*)&P4IN, (uint8_t*)&P4REN, BIT7},

    /* P60 */ {(uint8_t*)&P6DIR, (uint8_t*)&P6OUT, (uint8_t*)&P6IN, (uint8_t*)&P6REN, BIT0},
    /* P61 */ {(uint8_t*)&P6DIR, (uint8_t*)&P6OUT, (uint8_t*)&P6IN, (uint8_t*)&P6REN, BIT1},
    /* P62 */ {(uint8_t*)&P6DIR, (uint8_t*)&P6OUT, (uint8_t*)&P6IN, (uint8_t*)&P6REN, BIT2},
    /* P63 */ {(uint8_t*)&P6DIR, (uint8_t*)&P6OUT, (uint8_t*)&P6IN, (uint8_t*)&P6REN, BIT3},
    /* P64 */ {(uint8_t*)&P6DIR, (uint8_t*)&P6OUT, (uint8_t*)&P6IN, (uint8_t*)&P6REN, BIT4},
    /* P65 */ {(uint8_t*)&P6DIR, (uint8_t*)&P6OUT, (uint8_t*)&P6IN, (uint8_t*)&P6REN, BIT5},
    /* P67 */ {(uint8_t*)&P6DIR, (uint8_t*)&P6OUT, (uint8_t*)&P6IN, (uint8_t*)&P6REN, BIT7},

    /* P70 */ {(uint8_t*)&P7DIR, (uint8_t*)&P7OUT, (uint8_t*)&P7IN, (uint8_t*)&P7REN, BIT0},
    /* P74 */ {(uint8_t*)&P7DIR, (uint8_t*)&P7OUT, (uint8_t*)&P7IN, (uint8_t*)&P7REN, BIT4},

    /* P81 */ {(uint8_t*)&P8DIR, (uint8_t*)&P8OUT, (uint8_t*)&P8IN, (uint8_t*)&P8REN, BIT1},
    /* P82 */ {(uint8_t*)&P8DIR, (uint8_t*)&P8OUT, (uint8_t*)&P8IN, (uint8_t*)&P8REN, BIT2},
};

/**
  * Initialize GPIO pin to specified mode.
  *
  * I/O pin configuration described in MSP430x5xx Family Guide Section 12.2.4
  *
  * @param[in] pin number of pin to initialize
  * @param[in] mode pin mode to set
  * @param[in] pullup pin pullup value to set
  */
void k_gpio_init(int pin, KGPIOMode mode, KGPIOPullup pullup)
{
    if (K_GPIO_OUTPUT == mode)
    {
        hal_gpio_write(pins[pin].dir_pin, pins[pin].bit, 1);
    }
    else if (K_GPIO_INPUT == mode)
    {
        hal_gpio_write(pins[pin].dir_pin, pins[pin].bit, 0);

        if (K_GPIO_PULL_UP == pullup)
        {
            hal_gpio_write(pins[pin].pull_pin, pins[pin].bit, 1);
            hal_gpio_write(pins[pin].out_pin, pins[pin].bit, 1);
        }
        else if (K_GPIO_PULL_DOWN == pullup)
        {
            hal_gpio_write(pins[pin].pull_pin, pins[pin].bit, 1);
            hal_gpio_write(pins[pin].out_pin, pins[pin].bit, 0);
        }
    }
}

/**
  * Read in GPIO pin.
  * @param[in] pin number of pin to read from
  * @return unsigned int value read from pin
  */
unsigned int k_gpio_read(int pin)
{
    return hal_gpio_read(pins[pin].in_pin, pins[pin].bit);
}

/**
  * Write to GPIO pin.
  * @param[in] pin number of pin to write to
  * @param[in] val value to write
  */
void k_gpio_write(int pin, unsigned int val)
{
    hal_gpio_write(pins[pin].out_pin, pins[pin].bit, val);
}

/* @} */
