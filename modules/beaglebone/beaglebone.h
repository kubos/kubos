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

// BeagleBone specific config and APIs
#ifndef BEAGLEBONE_H
#define BEAGLEBONE_H

#define UART_NUMOF    (5U)
#define UART_0_EN     1
#define UART_1_EN     1
#define UART_2_EN     1
#define UART_3_EN     1
#define UART_4_EN     1
#define UART_IRQ_PRIO 1

// Beaglebone UARTS are numbered 1-5, so we have to subtract 1 to use the RIOT
// 0-4 constants

#define UART_0_PATH "/dev/ttyO1"
#define UART_1_PATH "/dev/ttyO2"
#define UART_2_PATH "/dev/ttyO3"
#define UART_3_PATH "/dev/ttyO4"
#define UART_4_PATH "/dev/ttyO5"

#endif
