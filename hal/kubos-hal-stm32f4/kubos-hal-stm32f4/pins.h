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
 * @defgroup STM32F4_HAL_GPIO STM32F4 HAL GPIO Interface
 * @addtogroup STM32F4_HAL_GPIO
 * @{
 */
#ifndef KUBOS_HAL_STM32F4_PINS_H
#define KUBOS_HAL_STM32F4_PINS_H

#include "stm32f4xx.h"
#include "stm32f4xx_hal_gpio.h"

/** Number of available GPIO pins */
#define K_NUM_PINS (16 * 5)

/** Green led pin */
#define K_LED_GREEN  K_LED_0
/** Led 0 pin */
#define K_LED_0      YOTTA_CFG_HARDWARE_PINS_LED1

/** Orange led pin */
#define K_LED_ORANGE K_LED_1
/** Led 1 pin */
#define K_LED_1      YOTTA_CFG_HARDWARE_PINS_LED2

/** Red led pin */
#define K_LED_RED    K_LED_2
/** Led 2 pin */
#define K_LED_2      YOTTA_CFG_HARDWARE_PINS_LED3

/** Blue led pin */
#define K_LED_BLUE   K_LED_3
/** Led 3 pin */
#define K_LED_3      YOTTA_CFG_HARDWARE_PINS_LED4

/** Number of available leds */
#define K_LED_COUNT  4

/** User button pin */
#define K_BUTTON_0   YOTTA_CFG_HARDWARE_PINS_USER_BUTTON

/** Derives GPIO pin offset (within respective port) */
#define STM32F4_PIN_OFFSET(p) ((p) % 16)

/** Derives bit mask for GPIO pin */
#define STM32F4_PIN_MASK(p) (1 << STM32F4_PIN_OFFSET(p))

/** Derives GPIO port from pin number */
#define STM32F4_PIN_GPIO(p) ((GPIO_TypeDef *) (GPIOA_BASE + (((p) / 16) * 0x400)))

/** Derives AHB (hardware clock) bit from pin number */
#define STM32F4_PIN_AHB1ENR_BIT(p) (1 << ((p) / 16))

/** Checks if bit value in src is set */
#define CHECK_BIT(src, bit) (READ_BIT((src), (bit)) == (bit))

/** Checks until bit value in dest is set */
#define CHECK_SET_BIT(dest, bit) do { \
    uint32_t tmpreg = READ_BIT(dest, bit); \
    if (!tmpreg) { \
        SET_BIT(dest, bit); \
        /* Delay after an RCC peripheral clock enabling */ \
        tmpreg = READ_BIT(dest, bit);\
    } \
} while(0)

/**
 * Structure describing GPIO pin
 */
typedef struct {
    /**
     * GPIO structure from STM32CubeF4 interface
     */
    GPIO_TypeDef *gpio;
    /**
     * Pin number
     */
    uint16_t pin;
    /**
     * AHB bit setting
     */
    uint32_t ahb1enr_bit;
} KPinDesc;

/**
 * All available GPIO pins for STM32F4
 */
typedef enum {
    PA0 = 0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PA10, PA11, PA12, PA13, PA14, PA15,
    PB0, PB1, PB2, PB3, PB4, PB5, PB6, PB7, PB8, PB9, PB10, PB11, PB12, PB13, PB14, PB15,
    PC0, PC1, PC2, PC3, PC4, PC5, PC6, PC7, PC8, PC9, PC10, PC11, PC12, PC13, PC14, PC15,
    PD0, PD1, PD2, PD3, PD4, PD5, PD6, PD7, PD8, PD9, PD10, PD11, PD12, PD13, PD14, PD15,
    PE0, PE1, PE2, PE3, PE4, PE5, PE6, PE7, PE8, PE9, PE10, PE11, PE12, PE13, PE14, PE15
} KPin;

#endif
/* @} */
