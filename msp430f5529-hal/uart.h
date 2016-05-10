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


 #ifndef HAL_UART_H
 #define HAL_UART_H

#include <stdint.h>

typedef enum
{
    HAL_UART_A1 = 0
} hal_uart_device;

typedef enum
{
    HAL_UART_9600 = 9600U,
    HAL_UART_115200 = 115200U
} hal_uart_baudrate;

typedef struct
{
    volatile uint8_t control1;
    volatile uint8_t control2;
    uint8_t padding1[4];
    volatile uint8_t baudrate0;
    volatile uint8_t baudrate1;
    volatile uint8_t modControl;
    uint8_t padding2;
    volatile uint8_t status;
    uint8_t padding3;
    volatile uint8_t rx;
    uint8_t padding4;
    volatile uint8_t tx;
    uint8_t padding5[14];
    volatile uint8_t interruptEnable;
    volatile uint8_t interruptFlags;
} hal_uart_mem_reg;

typedef struct
{
    hal_uart_device device;
    hal_uart_baudrate baudrate;
    uint8_t databits;
    uint8_t stopbits;
    uint8_t paritysetting;
    uint8_t checkparity;
} hal_uart_config;

typedef struct
{
    hal_uart_config config;
    volatile hal_uart_mem_reg * reg;
    volatile uint8_t * select;
    uint8_t * rxBuffer;
    uint16_t rxBufferSize;
    uint16_t rxBufferIndex;
    uint8_t selectVal;
} hal_uart_handle;


hal_uart_handle hal_uart_device_init(hal_uart_device device);

hal_uart_handle hal_uart_init(hal_uart_config config);

uint8_t hal_uart_setup(hal_uart_handle * handle);

void hal_uart_set_speed(hal_uart_handle * handle);

void hal_uart_putc(hal_uart_handle * handle, uint8_t c);

void hal_uart_putstr(hal_uart_handle * handle, uint8_t * buf, uint8_t len);

uint16_t hal_uart_read(hal_uart_handle * handle, uint8_t * buf);

 #endif
