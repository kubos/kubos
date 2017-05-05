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
 * @addtogroup MSP430F5529_HAL_UART
 * @{
 */

#if (defined YOTTA_CFG_HARDWARE_UART) && (YOTTA_CFG_HARDWARE_UART_COUNT > 0)
#ifndef KUBOS_HAL_MSP430F5529_UART_H
#define KUBOS_HAL_MSP430F5529_UART_H

#include "kubos-hal/uart.h"

/**
 * Internal function to get appropriate UART handle based on UART num
 * @param[in] uart UART bus num
 * @return hal_uart_handle pointer
 */
hal_uart_handle *uart_handle(KUARTNum uart);

#endif
#endif

/* @} */