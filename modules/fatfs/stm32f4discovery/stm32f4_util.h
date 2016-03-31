/*
 * KubOS Core Flight Services
 * Copyright (C) 2015 Kubos Corporation
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
#ifndef STM32F4_UTIL_H
#define STM32F4_UTIL_H

#include <stdint.h>
#include "cpu_conf.h"

#define CR1_CLEAR_MASK             ((uint16_t)0x3040)

#define GPIO_Pin_0                 ((uint16_t)0x0001)  /* Pin 0 selected */
#define GPIO_Pin_1                 ((uint16_t)0x0002)  /* Pin 1 selected */
#define GPIO_Pin_2                 ((uint16_t)0x0004)  /* Pin 2 selected */
#define GPIO_Pin_3                 ((uint16_t)0x0008)  /* Pin 3 selected */
#define GPIO_Pin_4                 ((uint16_t)0x0010)  /* Pin 4 selected */
#define GPIO_Pin_5                 ((uint16_t)0x0020)  /* Pin 5 selected */
#define GPIO_Pin_6                 ((uint16_t)0x0040)  /* Pin 6 selected */
#define GPIO_Pin_7                 ((uint16_t)0x0080)  /* Pin 7 selected */
#define GPIO_Pin_8                 ((uint16_t)0x0100)  /* Pin 8 selected */
#define GPIO_Pin_9                 ((uint16_t)0x0200)  /* Pin 9 selected */
#define GPIO_Pin_10                ((uint16_t)0x0400)  /* Pin 10 selected */
#define GPIO_Pin_11                ((uint16_t)0x0800)  /* Pin 11 selected */
#define GPIO_Pin_12                ((uint16_t)0x1000)  /* Pin 12 selected */
#define GPIO_Pin_13                ((uint16_t)0x2000)  /* Pin 13 selected */
#define GPIO_Pin_14                ((uint16_t)0x4000)  /* Pin 14 selected */
#define GPIO_Pin_15                ((uint16_t)0x8000)  /* Pin 15 selected */
#define GPIO_Pin_All               ((uint16_t)0xFFFF)  /* All pins selected */

#define GPIO_PinSource5            ((uint8_t)0x05)
#define GPIO_PinSource6            ((uint8_t)0x06)
#define GPIO_PinSource7            ((uint8_t)0x07)
#define GPIO_AF_SPI1               ((uint8_t)0x05)  /* SPI1 Alternate Function mapping      */

#define RCC_APB2Periph_SPI1             ((uint32_t)0x00001000)
#define RCC_AHB1Periph_GPIOA            ((uint32_t)0x00000001)
#define RCC_AHB1Periph_GPIOB            ((uint32_t)0x00000002)

#define SPI_Direction_2Lines_FullDuplex ((uint16_t)0x0000)
#define SPI_Mode_Master                 ((uint16_t)0x0104)
#define SPI_DataSize_8b                 ((uint16_t)0x0000)
#define SPI_CPOL_High                   ((uint16_t)0x0002)
#define SPI_CPHA_2Edge                  ((uint16_t)0x0001)
#define SPI_NSS_Soft                    ((uint16_t)0x0200)
#define SPI_BaudRatePrescaler_2         ((uint16_t)0x0000)
#define SPI_BaudRatePrescaler_4         ((uint16_t)0x0008)
#define SPI_BaudRatePrescaler_8         ((uint16_t)0x0010)
#define SPI_BaudRatePrescaler_16        ((uint16_t)0x0018)
#define SPI_BaudRatePrescaler_32        ((uint16_t)0x0020)
#define SPI_BaudRatePrescaler_64        ((uint16_t)0x0028)
#define SPI_BaudRatePrescaler_128       ((uint16_t)0x0030)
#define SPI_BaudRatePrescaler_256       ((uint16_t)0x0038)
#define SPI_FirstBit_MSB                ((uint16_t)0x0000)

typedef enum {
    DISABLE = 0,
    ENABLE = !DISABLE
} FunctionalState;

typedef enum {
    GPIO_Mode_IN   = 0x00, /*!< GPIO Input Mode */
    GPIO_Mode_OUT  = 0x01, /*!< GPIO Output Mode */
    GPIO_Mode_AF   = 0x02, /*!< GPIO Alternate function Mode */
    GPIO_Mode_AN   = 0x03  /*!< GPIO Analog Mode */
} GPIOMode_TypeDef;

typedef enum {
    GPIO_OType_PP = 0x00,
    GPIO_OType_OD = 0x01
} GPIOOType_TypeDef;

typedef enum {
    GPIO_Speed_2MHz   = 0x00, /*!< Low speed */
    GPIO_Speed_25MHz  = 0x01, /*!< Medium speed */
    GPIO_Speed_50MHz  = 0x02, /*!< Fast speed */
    GPIO_Speed_100MHz = 0x03  /*!< High speed on 30 pF (80 MHz Output max speed on 15 pF) */
} GPIOSpeed_TypeDef;

typedef enum {
    GPIO_PuPd_NOPULL = 0x00,
    GPIO_PuPd_UP     = 0x01,
    GPIO_PuPd_DOWN   = 0x02
} GPIOPuPd_TypeDef;

typedef struct {
    uint32_t GPIO_Pin;              /*!< Specifies the GPIO pins to be configured.
                                      This parameter can be any value of @ref GPIO_pins_define */

    GPIOMode_TypeDef GPIO_Mode;     /*!< Specifies the operating mode for the selected pins.
                                       This parameter can be a value of @ref GPIOMode_TypeDef */

    GPIOSpeed_TypeDef GPIO_Speed;   /*!< Specifies the speed for the selected pins.
                                       This parameter can be a value of @ref GPIOSpeed_TypeDef */

    GPIOOType_TypeDef GPIO_OType;   /*!< Specifies the operating output type for the selected pins.
                                       This parameter can be a value of @ref GPIOOType_TypeDef */

    GPIOPuPd_TypeDef GPIO_PuPd;     /*!< Specifies the operating Pull-up/Pull down for the selected pins.
                                       This parameter can be a value of @ref GPIOPuPd_TypeDef */
} GPIO_InitTypeDef;

typedef struct {
    uint16_t SPI_Direction;           /*!< Specifies the SPI unidirectional or bidirectional data mode.
                                        This parameter can be a value of @ref SPI_data_direction */

    uint16_t SPI_Mode;                /*!< Specifies the SPI operating mode.
                                        This parameter can be a value of @ref SPI_mode */

    uint16_t SPI_DataSize;            /*!< Specifies the SPI data size.
                                        This parameter can be a value of @ref SPI_data_size */

    uint16_t SPI_CPOL;                /*!< Specifies the serial clock steady state.
                                        This parameter can be a value of @ref SPI_Clock_Polarity */

    uint16_t SPI_CPHA;                /*!< Specifies the clock active edge for the bit capture.
                                        This parameter can be a value of @ref SPI_Clock_Phase */

    uint16_t SPI_NSS;                 /*!< Specifies whether the NSS signal is managed by
                                        hardware (NSS pin) or by software using the SSI bit.
                                        This parameter can be a value of @ref SPI_Slave_Select_management */

    uint16_t SPI_BaudRatePrescaler;   /*!< Specifies the Baud Rate prescaler value which will be
                                        used to configure the transmit and receive SCK clock.
                                        This parameter can be a value of @ref SPI_BaudRate_Prescaler
                                        @note The communication clock is derived from the master
                                        clock. The slave clock does not need to be set. */

    uint16_t SPI_FirstBit;            /*!< Specifies whether data transfers start from MSB or LSB bit.
                                        This parameter can be a value of @ref SPI_MSB_LSB_transmission */

    uint16_t SPI_CRCPolynomial;       /*!< Specifies the polynomial used for the CRC calculation. */
} SPI_InitTypeDef;

inline void RCC_AHB1PeriphClockCmd(uint32_t RCC_AHB1Periph,
                                   FunctionalState NewState)
{
    if (NewState != DISABLE) {
        RCC->AHB1ENR |= RCC_AHB1Periph;
    } else {
        RCC->AHB1ENR &= ~RCC_AHB1Periph;
    }
}

inline void RCC_APB2PeriphClockCmd(uint32_t RCC_APB2Periph,
                                   FunctionalState NewState)
{
    if (NewState != DISABLE) {
        RCC->APB2ENR |= RCC_APB2Periph;
    } else {
        RCC->APB2ENR &= ~RCC_APB2Periph;
    }
}

void GPIO_Init(GPIO_TypeDef* GPIOx, GPIO_InitTypeDef* GPIO_InitStruct);

inline void GPIO_SetBits(GPIO_TypeDef* GPIOx, uint16_t GPIO_Pin) {
    GPIOx->BSRRL = GPIO_Pin;
}

void GPIO_PinAFConfig(GPIO_TypeDef* GPIOx, uint16_t GPIO_PinSource,
                      uint8_t GPIO_AF);

inline void SPI_Cmd(SPI_TypeDef* SPIx, FunctionalState NewState) {
    if (NewState != DISABLE) {
        /* Enable the selected SPI peripheral */
        SPIx->CR1 |= SPI_CR1_SPE;
    } else {
        /* Disable the selected SPI peripheral */
        SPIx->CR1 &= (uint16_t)~((uint16_t)SPI_CR1_SPE);
    }
}

void SPI_Init(SPI_TypeDef* SPIx, SPI_InitTypeDef* SPI_InitStruct);

#endif
